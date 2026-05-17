# minilab-control-plane-authority

`minilab-control-plane-authority` is the Control Plane Authority Core package for the Minilab control plane.

It is a Rust CLI/service kit for contracts, safe validators, stub policy/gate decisions, offline config doctor output, evidence/receipt/ghost references, and authority responses.

It is not the UI, the LAB Host Runtime, Supabase, or Doppler.

## Offline build

This repository is vendored and configured to use `vendor/` through `.cargo/config.toml`.

```bash
cargo check --offline
cargo test --offline
```

## CLI examples

```bash
cargo run --offline -- status
cargo run --offline -- contracts list
cargo run --offline -- contracts check --file contracts/service.contract.json
cargo run --offline -- --json status
cargo run --offline -- policy check --actor dan --action update_host_runtime --resource lab256 --json
cargo run --offline -- gate decide --actor dan --action update_host_runtime --resource lab256 --json
cargo run --offline -- config doctor --dry-run --json
```

## Current limitations

- Policy is `stub_policy_v0`; no Cedar policy is loaded.
- Auth context is typed and validated, but JWT/OIDC/passkey verification is not implemented.
- Config doctor reads current process environment key names only and never calls Doppler.
- Registry/release/receipt lookup adapters are not wired.
- Receipt verification is not implemented.
- No LAB Host Runtime code, UI, migrations, storage upload, or external effect is implemented.

Do not build more institution. Materialize the contract language already decided.
