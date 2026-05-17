# Control Plane Authority Contracts v0

This package is the **Control Plane Authority Core**. It is the generic Rust service kit that understands the contract language for authority decisions, policy/gate boundaries, config doctor output, evidence, receipts, ghosts, and safe responses.

It is explicitly **not**:

- the UI;
- the LAB Host Runtime;
- Supabase;
- Doppler.

## Contract doctrine

Contracts are the language of authority. They prevent commands, policy decisions, gates, evidence, receipts, and ghosts from becoming ad-hoc payloads.

The v0 doctrine is:

- Registry resolves recognized existence.
- Doppler stores operational values, not institutional meaning.
- Policy decides permission.
- Gate contains consequence.
- ExecutionWindow is scoped, temporary, and single-use.
- Evidence is observed output.
- Receipt is proof closure.
- Ghost is structured absence.
- LABs execute; they do not govern.
- Natural language never executes.
- Provider success is evidence input, not closure.
- Receipts beat stories.

## Minimum honesty rules

Validators enforce minimum honesty rules only. They do not call the network, a database, Supabase, or Doppler. They reject raw credential-shaped fields, unsafe config doctor output, unapproved consequence, unpublished release artifacts, unredacted lab events, and success responses that still contain ghosts or errors.

## P0 contracts

The P0 contract set lives under `contracts/` and has Rust equivalents under `src/contracts/`:

1. ServiceContract
2. CommandEnvelope
3. PrincipalRef
4. AuthContext
5. ActionRequest
6. ResourceRef
7. PolicyDecision
8. GateRequest
9. ExecutionWindow
10. ConfigDoctorResult
11. RegistryEntityRef
12. ReleaseArtifactRef
13. HostRuntimeUpdateRequest
14. LabEvent
15. EvidenceRef
16. ReceiptRef
17. Ghost
18. AuthorityResponse
