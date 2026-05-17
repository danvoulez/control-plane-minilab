# Contract Filling Guide v0

No contract may invent the world. It may only reference a world that is recognized by the registry, observed by runtime evidence, or declared as a ghost.

## Who fills what

- **Caller** may fill command, input, intent, requested time, dry-run intent, actor claim, and resource target.
- **Authority Core** must fill policy decisions, gate requests, execution windows, authority responses, ghosts, and error contracts.
- **Database/registry** must resolve registry entities, release artifacts, known LAB hosts, known config requirements, and receipt indexes when those adapters exist.
- **Runtime** must observe lab events, evidence refs, doctor results, smoke results, and healthcheck results.

Never accept user-supplied truth for `verified=true`, closed receipts, open execution windows, `policy_decision=ok`, verified evidence, published release artifacts, or unscanned `secret_redacted=true`.

## Source of truth

- Identity: `registry.entities`
- Relationships: `registry.links`
- Runtime: `registry.runtimes`
- Mandates: `registry.mandates`
- Config requirements: `registry.config_requirements + config_registry.keys`
- Operational values: Doppler only, never printed by this package
- Releases: `release_registry.artifacts`
- Installations: `release_registry.installations`
- LAB life: `lab_observability.events + lab_observability.current_state`
- Receipts: `ops.receipt_index`
- LogLine acts: `ops.logline_acts`

## Forbidden shortcuts

- Do not treat provider success as receipt closure.
- Do not let natural language execute.
- Do not let an LLM approve or execute protected power.
- Do not print operational values.
- Do not create an execution window from a gate request alone.
- Do not mark unknown state as success; create a ghost.

## Flow order and success conditions

### host_runtime_update

Order: CommandEnvelope, PrincipalRef, AuthContext, ActionRequest, PolicyDecision, GateRequest, ExecutionWindow, ReleaseArtifactRef, HostRuntimeUpdateRequest, LabEvent, EvidenceRef, ConfigDoctorResult, ReceiptRef, AuthorityResponse.

Success requires `PACKAGE_DOWNLOADED`, `PACKAGE_VERIFIED`, `PACKAGE_INSTALLED`, `DOCTOR_PASSED`, and `SMOKE_PASSED`.

### config_doctor

Order: CommandEnvelope, ActionRequest, ConfigDoctorResult, EvidenceRef, AuthorityResponse.

Success requires no forbidden keys, no printed values, required keys present or warned, and valid brakes.

### release_publish

Order: CommandEnvelope, ActionRequest, PolicyDecision, GateRequest, ExecutionWindow, ReleaseArtifactRef, EvidenceRef, ReceiptRef, AuthorityResponse.

Success requires a passed secret scan, generated sha256, verified storage upload, and release registry artifact status set to published.

### bootstrap_lab

Order: CommandEnvelope, RegistryEntityRef, ReleaseArtifactRef, HostRuntimeUpdateRequest, LabEvent, EvidenceRef, ReceiptRef, AuthorityResponse.

Success requires package verification, host runtime installation, doctor pass, and smoke pass.

## Ghosts

A ghost is structured absence. Missing registry entities, missing config, forbidden config, missing evidence, unavailable runtime, unpublished release, unverified auth, unverified receipt, and unknown state must be represented as ghosts rather than silent success.
