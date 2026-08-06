# Event Store Specification (`specs/storage/event-store.spec.md`)

## 1. Storage Contract & Repository Interface
* **Target Crate**: `crates/matis-storage`

```rust
pub trait EventRepository {
    fn append(&mut self, event: &EngineeringEvent) -> Result<()>;
    fn get_by_id(&self, id: &EventId) -> Result<Option<EngineeringEvent>>;
    fn query_range(&self, start: Timestamp, end: Timestamp) -> Result<Vec<EngineeringEvent>>;
    fn stream_all(&self) -> Result<Box<dyn Iterator<Item = EngineeringEvent>>>;
}
```

## 2. Invariants
1. `EventRepository` implementations MUST be 100% append-only.
2. Updates and deletes on raw event records are STRICTLY PROHIBITED.
3. Storage backends (SQLite default, RocksDB, Postgres) MUST expose identical query behavior and SHA-256 integrity verification.
4. Payload payloads exceeding 64KB MUST be stored in `BlobStore` and referenced by `BlobId`.
