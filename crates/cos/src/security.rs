//! Bridging the COS object model and `pdfcraft-crypt`: reading `/Encrypt` dictionaries and
//! decrypting / encrypting the strings and streams of an object (§7.6.2).
//!
//! Not encrypted, per the spec: the `/Encrypt` dictionary itself, cross-reference streams, the
//! trailer `/ID`, objects inside object streams (the object stream is encrypted as a whole),
//! the `/Contents` of signature dictionaries, and metadata streams when `/EncryptMetadata` is
//! false. Streams naming a crypt filter (`/Crypt` in `/Filter`) use that filter; embedded files
//! use `/EFF`.

use pdfcraft_crypt::{EncryptDict, Method, SecurityHandler, StreamKind};

use crate::{Dict, Document, Object, PdfString, Stream};

fn bytes_of(doc: &Document, d: &Dict, key: &[u8]) -> Vec<u8> {
    d.get(key).map(|v| doc.resolve(v)).and_then(|v| v.as_string().map(|s| s.bytes.clone())).unwrap_or_default()
}

/// Read the handler parameters from an `/Encrypt` dictionary.
pub(crate) fn encrypt_dict(doc: &Document, d: &Dict) -> EncryptDict {
    let int = |k: &[u8], default: i64| d.get(k).map(|v| doc.resolve(v)).and_then(|v| v.as_int()).unwrap_or(default);
    let name = |k: &[u8]| d.get(k).and_then(|v| v.as_name()).map(|n| n.to_vec()).unwrap_or_else(|| b"Identity".to_vec());
    let mut crypt_filters = Vec::new();
    if let Some(cf) = d.get(b"CF").map(|c| doc.resolve(c)).and_then(|c| c.as_dict().cloned()) {
        for (k, v) in cf.iter() {
            let Some(f) = doc.resolve(v).as_dict().cloned() else { continue };
            let method = match f.name(b"CFM") {
                Some(b"V2") => Method::Rc4,
                Some(b"AESV2") => Method::Aes128,
                Some(b"AESV3") => Method::Aes256,
                _ => Method::Identity,
            };
            crypt_filters.push((k.clone(), method));
        }
    }
    let v = int(b"V", 0);
    // V4: the key length comes from the standard crypt filter's /Length (bytes, or bits in
    // some files); /Length on the dictionary is often a stale 40.
    let cf_len = d
        .get(b"CF")
        .map(|c| doc.resolve(c))
        .and_then(|c| c.as_dict().and_then(|c| c.get(b"StdCF").map(|f| doc.resolve(f))))
        .and_then(|f| f.as_dict().and_then(|f| f.int(b"Length")));
    let length_bits = match (v, cf_len) {
        (5.., _) => 256,
        (4, Some(n)) if n <= 32 => n * 8,
        (4, Some(n)) => n,
        (4, None) => 128,
        _ => int(b"Length", 40),
    };
    EncryptDict {
        filter: d.name(b"Filter").map(|n| n.to_vec()).unwrap_or_default(),
        v,
        r: int(b"R", 0),
        length_bits,
        o: bytes_of(doc, d, b"O"),
        u: bytes_of(doc, d, b"U"),
        oe: bytes_of(doc, d, b"OE"),
        ue: bytes_of(doc, d, b"UE"),
        perms: bytes_of(doc, d, b"Perms"),
        p: int(b"P", -1) as i32,
        encrypt_metadata: !matches!(d.get(b"EncryptMetadata"), Some(Object::Bool(false))),
        crypt_filters,
        stm_f: name(b"StmF"),
        str_f: name(b"StrF"),
        ef_f: d.get(b"EFF").and_then(|v| v.as_name()).map(|n| n.to_vec()).unwrap_or_default(),
    }
}

/// The crypt filter a stream names in its `/Filter` chain (`/Crypt` + `/DecodeParms /Name`).
fn named_crypt_filter(dict: &Dict) -> Option<Vec<u8>> {
    // Preserve the *original* /Filter array index when pairing with /DecodeParms.
    // Dropping malformed non-name entries shifts later parameters to the wrong
    // filter; in particular it can silently turn a named /Crypt filter into Identity.
    let i = match dict.get(b"Filter") {
        Some(Object::Name(n)) if n == b"Crypt" => 0,
        Some(Object::Array(filters)) => filters.iter().position(|f| f.as_name() == Some(&b"Crypt"[..]))?,
        _ => return None,
    };
    let parms = match dict.get(b"DecodeParms") {
        Some(Object::Dict(d)) if i == 0 => Some(d.clone()),
        Some(Object::Array(a)) => a.get(i).and_then(|p| p.as_dict().cloned()),
        _ => None,
    };
    Some(parms.and_then(|p| p.name(b"Name").map(|n| n.to_vec())).unwrap_or_else(|| b"Identity".to_vec()))
}

/// A signature dictionary (field signature, document timestamp or usage-rights signature),
/// whose `/Contents` is never encrypted (ISO 32000-2 §7.6.2, §12.8.1): the signer writes a
/// placeholder, hashes the file around it and patches the value in place, so encrypting it
/// would make signing impossible. Identified as other readers do: by `/Type`, or by a string
/// `/Contents` alongside an array `/ByteRange` (`/Type` is optional).
fn is_signature_dict(d: &Dict) -> bool {
    matches!(d.name(b"Type"), Some(b"Sig" | b"DocTimeStamp"))
        || (matches!(d.get(b"Contents"), Some(Object::String(_))) && matches!(d.get(b"ByteRange"), Some(Object::Array(_))))
}

/// Decrypt (`decrypt = true`) or encrypt every string and stream in an indirect object,
/// except a signature dictionary's `/Contents` (see [`is_signature_dict`]).
pub(crate) fn transform(h: &SecurityHandler, o: &Object, num: u32, generation: u16, decrypt: bool) -> Object {
    let strings = |s: &PdfString| {
        let bytes = if decrypt { h.decrypt_string(num, generation, &s.bytes) } else { h.encrypt_string(num, generation, &s.bytes) };
        PdfString { bytes, hex: s.hex || !decrypt }
    };
    walk(o, &strings, h, num, generation, decrypt)
}

fn walk(o: &Object, strings: &dyn Fn(&PdfString) -> PdfString, h: &SecurityHandler, num: u32, generation: u16, decrypt: bool) -> Object {
    match o {
        Object::String(s) => Object::String(strings(s)),
        Object::Array(a) => Object::Array(a.iter().map(|x| walk(x, strings, h, num, generation, decrypt)).collect()),
        Object::Dict(d) if is_signature_dict(d) => Object::Dict(
            d.iter()
                .map(|(k, v)| match (k.as_slice(), v) {
                    (b"Contents", Object::String(_)) => (k.clone(), v.clone()),
                    _ => (k.clone(), walk(v, strings, h, num, generation, decrypt)),
                })
                .collect(),
        ),
        Object::Dict(d) => Object::Dict(d.iter().map(|(k, v)| (k.clone(), walk(v, strings, h, num, generation, decrypt))).collect()),
        Object::Stream(s) => {
            let dict: Dict = s.dict.iter().map(|(k, v)| (k.clone(), walk(v, strings, h, num, generation, decrypt))).collect();
            let ty = s.dict.name(b"Type");
            if ty == Some(b"XRef") || (ty == Some(b"Metadata") && !h.encrypts_metadata()) {
                return Object::Stream(Stream { dict, raw: s.raw.clone() });
            }
            let named = named_crypt_filter(&s.dict);
            let kind = match (&named, ty) {
                (Some(n), _) => StreamKind::Named(n),
                (None, Some(b"EmbeddedFile")) => StreamKind::EmbeddedFile,
                _ => StreamKind::Normal,
            };
            let raw = if decrypt { h.decrypt_stream(num, generation, &s.raw, kind) } else { h.encrypt_stream(num, generation, &s.raw, kind) };
            Object::Stream(Stream { dict, raw: raw.into() })
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named_params(name: &str) -> Object {
        let mut parms = Dict::new();
        parms.set(b"Name".to_vec(), Object::name(name));
        Object::Dict(parms)
    }

    #[test]
    fn named_crypt_filter_preserves_original_filter_array_offsets() {
        let mut d = Dict::new();
        // An invalid entry is ignored as a filter, but must not shift the
        // /DecodeParms array. Previously the /Crypt filter read index 0.
        d.set(b"Filter".to_vec(), Object::Array(vec![Object::Null, Object::name("Crypt")]));
        d.set(b"DecodeParms".to_vec(), Object::Array(vec![Object::Null, named_params("StdCF")]));
        assert_eq!(named_crypt_filter(&d), Some(b"StdCF".to_vec()));

        // A valid preceding filter also keeps the second entry's parameters.
        d.set(b"Filter".to_vec(), Object::Array(vec![Object::name("FlateDecode"), Object::name("Crypt")]));
        assert_eq!(named_crypt_filter(&d), Some(b"StdCF".to_vec()));
    }

    #[test]
    fn named_crypt_filter_supports_scalar_and_missing_parameters() {
        let mut d = Dict::new();
        d.set(b"Filter".to_vec(), Object::name("Crypt"));
        d.set(b"DecodeParms".to_vec(), named_params("StdCF"));
        assert_eq!(named_crypt_filter(&d), Some(b"StdCF".to_vec()));

        d.set(b"DecodeParms".to_vec(), Object::Null);
        assert_eq!(named_crypt_filter(&d), Some(b"Identity".to_vec()));

        d.set(b"Filter".to_vec(), Object::name("FlateDecode"));
        assert_eq!(named_crypt_filter(&d), None);
    }

    #[test]
    fn signature_detection_covers_timestamps_and_untyped_signatures() {
        let mut d = Dict::new();
        d.set(b"Type".to_vec(), Object::name("DocTimeStamp"));
        assert!(is_signature_dict(&d));

        d.set(b"Type".to_vec(), Object::name("Annot"));
        assert!(!is_signature_dict(&d));
        d.set(b"Contents".to_vec(), PdfString::literal(b"synthetic".to_vec()));
        assert!(!is_signature_dict(&d));
        d.set(b"ByteRange".to_vec(), Object::Array(vec![Object::Int(0), Object::Int(10)]));
        assert!(is_signature_dict(&d));
    }
}
