//! Black-box JSON-RPC 2.0 conformance contracts for the opt-in MCP server.
//! The malformed requests exercise the wire handler rather than private helpers.
#![cfg(feature = "mcp")]

use pdfcraft_automation::{Automation, mcp::McpServer};
use serde_json::{Value, json};

fn send(server: &mut McpServer, message: Value) -> Option<Value> {
    server.handle_line(&message.to_string()).map(|line| serde_json::from_str(&line).expect("valid JSON reply"))
}

fn invalid(response: Option<Value>, expected_id: Value) {
    let response = response.expect("invalid request must not disappear");
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], expected_id);
    assert_eq!(response["error"]["code"], -32600);
    assert!(response.get("result").is_none());
}

#[test]
fn wrong_version_never_executes_a_request_or_becomes_a_notification() {
    let mut server = McpServer::new(Automation::new());
    // All four are malformed requests, including the one that looks like a
    // notification. They must not be silently ignored or run as a valid ping.
    for message in [
        json!({ "id": 1, "method": "ping" }),
        json!({ "jsonrpc": "1.0", "id": 1, "method": "ping" }),
        json!({ "jsonrpc": null, "id": 1, "method": "ping" }),
        json!({ "jsonrpc": "1.0", "method": "notifications/initialized" }),
    ] {
        invalid(send(&mut server, message), Value::Null);
    }
}

#[test]
fn invalid_ids_are_rejected_with_null_response_ids() {
    let mut server = McpServer::new(Automation::new());
    for id in [json!(true), json!(false), json!([]), json!({}), json!(1.5)] {
        invalid(send(&mut server, json!({ "jsonrpc": "2.0", "id": id, "method": "ping" })), Value::Null);
    }
}

#[test]
fn methodless_requests_are_errors_even_without_an_id() {
    let mut server = McpServer::new(Automation::new());
    invalid(send(&mut server, json!({ "jsonrpc": "2.0" })), Value::Null);
    invalid(send(&mut server, json!({ "jsonrpc": "2.0", "id": "request-17" })), json!("request-17"));
    invalid(send(&mut server, json!({ "jsonrpc": "2.0", "id": 5, "method": 9 })), json!(5));
}

#[test]
fn valid_ids_notifications_and_unknown_methods_keep_normal_behavior() {
    let mut server = McpServer::new(Automation::new());
    for id in [json!(0), json!(7), json!("request-string"), Value::Null] {
        let reply = send(&mut server, json!({ "jsonrpc": "2.0", "id": id, "method": "ping" })).expect("request reply");
        assert_eq!(reply["id"], id);
        assert_eq!(reply["result"], json!({}));
        assert!(reply.get("error").is_none());
    }
    assert!(send(&mut server, json!({ "jsonrpc": "2.0", "method": "notifications/initialized" })).is_none());
    assert!(send(&mut server, json!({ "jsonrpc": "2.0", "method": "notifications/cancelled", "params": { "requestId": 1 } })).is_none());
    let unknown = send(&mut server, json!({ "jsonrpc": "2.0", "id": "unknown", "method": "not/implemented" })).expect("reply");
    assert_eq!(unknown["id"], "unknown");
    assert_eq!(unknown["error"]["code"], -32601);
}

#[test]
fn stdio_recovers_from_invalid_messages_without_losing_subsequent_requests() {
    let input = [
        r#"{"jsonrpc":"1.0","id":1,"method":"ping"}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{not-json"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"ping"}"#,
    ]
    .join("\n");
    let mut output = Vec::new();
    McpServer::new(Automation::new()).serve(input.as_bytes(), &mut output).expect("stdio serving");
    let replies: Vec<Value> = String::from_utf8(output)
        .expect("UTF-8 wire")
        .lines()
        .map(|line| serde_json::from_str(line).expect("valid reply"))
        .collect();
    assert_eq!(replies.len(), 3, "one valid notification has no reply");
    assert_eq!(replies[0]["error"]["code"], -32600);
    assert_eq!(replies[1]["error"]["code"], -32700);
    assert_eq!(replies[2]["id"], 2);
    assert_eq!(replies[2]["result"], json!({}));
}
