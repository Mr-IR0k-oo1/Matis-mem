# Engineering Context Protocol (ECP) Specification (`specs/protocol/ecp.spec.md`)

## 1. Scope & Wire Abstraction
* **Specification Version**: 1.0.0
* **Target Crate**: `crates/matis-protocol`

ECP defines the open, transport-neutral wire framing for engineering memory communication over Unix Sockets, Named Pipes, WebSockets, HTTP/REST, and MCP.

## 2. Frame Structure

```text
┌─────────────────┬──────────────────┬──────────────────┬───────────────────────────┐
│ Magic (4 Bytes) │ Version (2 Bytes)│ Msg Type (2 Bytes)│ Payload Length (4 Bytes)  │
│  "ECP1"         │  0x0001          │  0x0010 (Command)│  N Bytes                  │
├─────────────────┴──────────────────┴──────────────────┴───────────────────────────┤
│                               Payload (Serialized ECP Object)                     │
└───────────────────────────────────────────────────────────────────────────────────┘
```

## 3. Message Types
* `0x0001`: Handshake Request / Response
* `0x0002`: Heartbeat / Telemetry
* `0x0010`: Command Execution (`SubmitEvent`, `RegisterSensor`)
* `0x0020`: Query Request (`MqlQuery`, `ContextRequest`)
* `0x0021`: Context Response (`AssembledContext` with Citations)
* `0x0030`: Subscription Request (`TimelineStream`, `EventStream`)
* `0x0040`: Error Envelope (`MatisError`)

## 4. Error Envelope Schema
```json
{
  "error": {
    "code": "INVALID_EVENT_CHECKSUM",
    "message": "SHA-256 payload digest mismatch",
    "cause": "Calculated digest 'a8f9...' != 'b1e2...'",
    "suggestion": "Check sensor event serialization pipeline",
    "trace_id": "tr_8f910a"
  }
}
```
