# Sensor Pipeline Specification (`specs/sensors/sensor-pipeline.spec.md`)

## 1. Passive Sensor Architecture
* **Target Crate**: `crates/matis-sensors`

Sensors observe external engineering activity passively without altering developer workflows, modifying files, executing AI, or altering Git states.

## 2. Sensor Trait Contract

```rust
pub trait SensorAdapter {
    fn initialize(&mut self) -> Result<()>;
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn health(&self) -> HealthStatus;
    fn capabilities(&self) -> Vec<Capability>;
}
```

## 3. Mandatory Invariants
1. **Passive & Read-Only**: Sensors never modify files or send prompts.
2. **Secret Redaction**: Sensors MUST sanitize API keys, SSH keys, passwords, and secrets before publishing events.
3. **Event Normalization**: All sensors normalize observed activity into standard `EngineeringEvent` schemas (`crates/matis-objects`).
