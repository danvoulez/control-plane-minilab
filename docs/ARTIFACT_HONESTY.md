# Artifact Honesty

## Phase

Control Plane Authority Contracts v0.

## What is real

- Contract JSON files exist.
- Rust structs and enums compile.
- Pure validators run without external effects.
- The CLI can report service status, list contracts, check a contract JSON file, run an offline config doctor, and return stub policy/gate decisions.

## What is stub

- `stub_policy_v0` is not Cedar.
- Auth verification is not real.
- Receipt verification is not real.
- Config doctor reads process environment names only.
- Registry, release, and receipt resolution are not wired to a database.

## What is not implemented

- Supabase adapter.
- Doppler adapter.
- Cedar policy loading.
- JWT/OIDC verification.
- Receipt verification.
- LAB Host Runtime package/update code.
- UI.
- Migrations.
- Storage upload.
- External effects.

## Ghosts remaining

- `real_policy_engine_not_wired`
- `real_auth_not_wired`
- `real_receipt_verifier_not_wired`
- `registry_resolution_not_wired`
- `doppler_schema_not_wired`
- `database_adapter_not_wired`
