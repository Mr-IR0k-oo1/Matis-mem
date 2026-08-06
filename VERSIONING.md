# Matis Versioning Policy

Matis adheres to strict [Semantic Versioning 2.0.0](https://semver.org/) (`MAJOR.MINOR.PATCH`).

## Version Contracts

- **MAJOR version (`MAJOR.0.0`)**: Incompatible changes to the Engineering Object Specification (`EOS`), Event Store schema, ECP wire protocol, or core API trait boundaries.
- **MINOR version (`0.MINOR.0`)**: Additive, backward-compatible additions (new event kinds, new sensors, new CLI subcommands, new query filters).
- **PATCH version (`0.0.PATCH`)**: Backward-compatible bug fixes, performance optimizations, and documentation updates.

## Specification Versioning

Formal behavioral specifications under `specs/` are versioned independently (`v1.0.0`). Specification updates require an RFC (`docs/12-rfcs/`) and an accepted ADR (`docs/01-adrs/`).
