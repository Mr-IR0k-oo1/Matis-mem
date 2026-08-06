# Engineering Knowledge Standard (EKS)

# Purpose

The Engineering Knowledge Standard (EKS) defines the canonical, vendor-neutral representation of engineering knowledge, workflows, context, and intelligence.

It exists to ensure that engineering information remains portable, interoperable, and durable across tools, organizations, and decades.

Matis is the reference implementation of EKS.

---

# Vision: Strategic Standard Positioning

Just as software engineering established universal open standards for other domains:
* **Git**: Source code version history
* **OCI**: Container packaging & runtimes
* **OpenAPI**: REST API contracts
* **OpenTelemetry**: Distributed tracing & metrics
* **SPDX**: Software licensing & bill of materials

EKS establishes the universal open standard for engineering decisions, episodes, memory, context, and reasoning.

```text
                                Engineering Knowledge Standard (EKS)
 ┌────────────────────────────────────────────────────────────────────────────────────────┐
 │                                                                                        │
 │   Engineering Events Standard     ──►   Engineering Episodes Standard                  │
 │   Engineering Object Standard     ──►   Engineering Context Protocol (ECP)             │
 │   Engineering Memory Standard     ──►   Engineering Replay Standard                    │
 │                                                                                        │
 └───────────────────────────────────────────┬────────────────────────────────────────────┘
                                             │
                                             ▼
                              Reference Implementation: Matis
```

---

# The 6 EKS Standard Specifications

1. **Engineering Events Standard**: Canonical schemas, timestamps, identity, provenance, ordering, checksums, and replay behavior.
2. **Engineering Episodes Standard**: Canonical units of work (`title`, `objective`, `timeline`, `relationships`, `outcome`).
3. **Engineering Object Standard (EOS)**: Universal base envelope (`Identity`, `schema_version`, cryptographic `checksum`, `ProvenanceMetadata`).
4. **Engineering Context Protocol (ECP)**: Transport-neutral wire protocol framing, version handshake, and streaming RPC semantics.
5. **Engineering Memory Standard**: Universal schemas for ADRs, constraints, patterns, lessons, and milestones.
6. **Engineering Replay Standard**: Verifiable, deterministic event log replay specification ($\text{Replay}(\text{EventStore}) \equiv \text{Original}(\text{EventStore})$).

---

# Conformance, Interoperability & Governance

* **Vendor Independence**: Two independent EKS implementations must exchange Engineering Objects and replay event history without data loss or semantic drift.
* **Open Governance**: EKS evolves through open RFCs (`docs/12-rfcs/`), community reviews, ADRs (`docs/01-adrs/`), formal specs (`specs/`), and test harnesses (`conformance/`).

---

# Specification Freeze Rule

Starting today, no new foundational architecture documents will be authored unless they resolve a blocker in code or formal contracts.

Every new effort must move through the strict 4-step pipeline:

$$\text{Architecture (Frozen)} \Longrightarrow \text{Formal Specs (\texttt{specs/})} \Longrightarrow \text{Rust Workspace (\texttt{crates/})} \Longrightarrow \text{Conformance \& Replay (\texttt{tests/})}$$
