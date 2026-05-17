Agora sim. **Contratos necessários para esse pacote genérico virar o Control Plane Authority Core.**

Não vou falar de feature. Isso aqui é a lista de **contratos** que precisam existir para o pacote Rust saber falar com o mundo.

# Contratos P0 — obrigatórios

## 1. `ServiceContract`

Identifica o próprio binário.

```ts
type ServiceContract = {
  service_id: "minilab-control-plane-authority";
  version: string;
  role: "control_plane_authority_core";
  mode: "cli" | "server" | "test" | "dry_run";
  external_effects_enabled: boolean;
  started_at: string;
};
```

Serve para:

```txt
status
health
diagnóstico
Artifact Honesty
```

---

## 2. `CommandEnvelope`

Formato comum de entrada para CLI e HTTP.

```ts
type CommandEnvelope<T> = {
  command_id: string;
  command: string;
  actor: PrincipalRef;
  input: T;
  requested_at: string;
  correlation_id?: string;
  dry_run: boolean;
};
```

Exemplo:

```txt
gate.decide
config.doctor
policy.check
release.verify
receipt.verify
```

Sem isso, cada comando inventa seu payload.

---

## 3. `PrincipalRef`

Quem está pedindo.

```ts
type PrincipalRef = {
  entity_id: string;      // ex: dan, chatgpt, lab_256
  kind: "human" | "llm" | "service" | "lab" | "runtime";
  display_name?: string;
  auth_context?: AuthContext;
};
```

Esse contrato amarra com:

```txt
registry.entities
registry.mandates
auth boundary
policy
gate
```

---

## 4. `AuthContext`

O que foi autenticado.

```ts
type AuthContext = {
  method:
    | "none"
    | "bearer_token"
    | "jwt"
    | "oidc"
    | "passkey"
    | "mcp_token"
    | "local_dev";
  verified: boolean;
  subject?: string;
  issuer?: string;
  scopes: string[];
  token_present: boolean;
  token_value_printed: false;
};
```

Regra:

```txt
token nunca aparece
auth ausente não vira Dan
LLM nunca vira operador humano
```

---

## 5. `ActionRequest`

O pedido institucional.

```ts
type ActionRequest = {
  action_id: string;
  actor: PrincipalRef;
  verb: string;
  resource: ResourceRef;
  intent?: string;
  risk_class:
    | "read"
    | "diagnostic"
    | "write"
    | "install"
    | "update"
    | "delete"
    | "external_effect"
    | "protected_power";
  requested_at: string;
  dry_run: boolean;
  metadata?: Record<string, unknown>;
};
```

Exemplos:

```txt
update_host_runtime lab256
publish_release minilab-host-runtime
verify_config doppler
read_registry_entity dan
```

Esse é um dos contratos centrais.

---

## 6. `ResourceRef`

Coisa alvo do pedido.

```ts
type ResourceRef = {
  kind:
    | "entity"
    | "lab"
    | "runtime"
    | "release_artifact"
    | "config_key"
    | "receipt"
    | "database"
    | "storage_object"
    | "mcp_tool";
  id: string;
  table?: string;
  schema?: string;
};
```

Exemplo:

```ts
{
  kind: "lab",
  id: "lab256"
}
```

ou:

```ts
{
  kind: "release_artifact",
  id: "minilab-host-runtime@0.1.0"
}
```

---

## 7. `PolicyDecision`

Resultado da política.

```ts
type PolicyDecision = {
  decision_id: string;
  action_id: string;
  engine: "cedar" | "stub_policy_v0";
  decision: "ok" | "denied" | "needs_approval" | "ghost" | "error";
  reasons: string[];
  matched_policy_ids?: string[];
  evaluated_at: string;
};
```

Esse contrato é o output do `policy check`.

---

## 8. `GateRequest`

Quando a policy diz “precisa aprovação”.

```ts
type GateRequest = {
  gate_id: string;
  action_id: string;
  actor: PrincipalRef;
  resource: ResourceRef;
  policy_decision_id: string;
  status: "pending" | "approved" | "denied" | "ghosted" | "expired";
  requested_at: string;
  expires_at?: string;
  required_approver?: string; // ex: dan
  reason: string;
};
```

Esse é o contrato que impede botão virar execução.

---

## 9. `ExecutionWindow`

Permissão curta, escopada, consumível.

```ts
type ExecutionWindow = {
  window_id: string;
  gate_id: string;
  actor_entity_id: string;
  allowed_action: string;
  allowed_resource_id: string;
  scope_hash: string;
  expires_at: string;
  consumed_at?: string;
  status: "open" | "consumed" | "expired" | "revoked";
};
```

Regra:

```txt
aprovação não é permissão global
window é single-use
sem window, nada protegido executa
```

---

## 10. `ConfigDoctorResult`

Contrato do `config doctor`.

```ts
type ConfigDoctorResult = {
  run_id: string;
  status: "ok" | "warn" | "error";
  doppler_project?: string;
  doppler_config?: string;
  keys_present: string[];
  keys_missing: string[];
  keys_unknown: string[];
  keys_forbidden: string[];
  canonical_keys_used: string[];
  legacy_keys_used: string[];
  secret_values_printed: false;
  checked_at: string;
};
```

Inclui a regra nova:

```txt
SUPABASE_SECRET_KEY preferido
SUPABASE_SERVICE_ROLE_KEY fallback legacy
SUPABASE_PUBLISHABLE_KEY front
SUPABASE_ANON_KEY fallback legacy
```

E proibido:

```txt
NEXT_PUBLIC_SUPABASE_SECRET_KEY
NEXT_PUBLIC_SUPABASE_SERVICE_ROLE_KEY
```

---

## 11. `RegistryEntityRef`

O Authority Core não precisa carregar a entity inteira sempre. Precisa de referência estável.

```ts
type RegistryEntityRef = {
  entity_id: string;
  name?: string;
  kind:
    | "person"
    | "computer"
    | "service"
    | "runtime"
    | "database"
    | "vault"
    | "app"
    | "llm"
    | "package"
    | "object";
  status: "active" | "planned" | "partial" | "ghost" | "retired";
  evidence_status: "declared" | "observed" | "verified" | "unverified";
};
```

Usa:

```txt
registry.entities
```

---

## 12. `ReleaseArtifactRef`

Para verificar se pacote pode ser instalado/update.

```ts
type ReleaseArtifactRef = {
  artifact_id: string;
  package_name: string; // minilab-host-runtime
  version: string;
  storage_bucket: string;
  storage_path: string;
  sha256: string;
  size_bytes?: number;
  status: "planned" | "built" | "published" | "retired" | "ghost";
};
```

Usa:

```txt
release_registry.artifacts
```

Regra:

```txt
só published pode virar install/update
planned não instala
sha256 obrigatório
```

---

## 13. `HostRuntimeUpdateRequest`

Pedido específico de update do Host Runtime.

```ts
type HostRuntimeUpdateRequest = {
  request_id: string;
  host_id: "lab8gb" | "lab256" | "lab512";
  target_version: string;
  artifact: ReleaseArtifactRef;
  requested_by: PrincipalRef;
  gate_id?: string;
  execution_window_id?: string;
  dry_run: boolean;
};
```

Esse contrato é para o Authority Core decidir:

```txt
pode preparar?
precisa gate?
pode chamar MCP?
```

Ele não executa o update. Ele autoriza/nega/prepara.

---

## 14. `LabEvent`

Evento vindo do Host Runtime.

```ts
type LabEvent = {
  id: string;
  host_id: "lab8gb" | "lab256" | "lab512";
  host_role: "supervisor" | "workbench" | "inference";
  runtime_version?: string;
  event_kind: string;
  component: string;
  severity: "debug" | "info" | "warn" | "error" | "critical";
  status: "ok" | "degraded" | "failed" | "ghost";
  observed_at: string;
  duration_ms?: number;
  evidence?: Record<string, unknown>;
  next_action?: string;
  secret_redacted: true;
};
```

Usa:

```txt
lab_observability.events
```

---

## 15. `EvidenceRef`

Referência a prova, sem carregar tudo.

```ts
type EvidenceRef = {
  evidence_id: string;
  kind:
    | "lab_event"
    | "receipt"
    | "checksum"
    | "doctor"
    | "smoke"
    | "healthcheck"
    | "database_row"
    | "storage_object"
    | "stdout"
    | "stderr";
  source: string;
  hash?: string;
  uri?: string;
  observed_at: string;
  secret_redacted: true;
};
```

---

## 16. `ReceiptRef`

Índice de receipt.

```ts
type ReceiptRef = {
  receipt_hash: string;
  receipt_kind: string;
  tuple_hash?: string;
  actor_entity_id?: string;
  target_entity_id?: string;
  evidence_mode: "declared" | "observed" | "verified" | "unverified";
  receipt_status: "draft" | "closed" | "ghosted" | "rejected";
  storage_uri?: string;
};
```

Usa:

```txt
ops.receipt_index
```

---

## 17. `Ghost`

Quando algo falta.

```ts
type Ghost = {
  ghost_id: string;
  kind:
    | "missing_registry_entity"
    | "missing_config"
    | "forbidden_config"
    | "missing_evidence"
    | "runtime_unavailable"
    | "release_unpublished"
    | "auth_unverified"
    | "receipt_unverified"
    | "unknown";
  summary: string;
  source_ids: string[];
  created_at: string;
  status: "open" | "resolved" | "ignored";
};
```

Esse contrato é fundamental para não fingir sucesso.

---

## 18. `AuthorityResponse`

Resposta comum de qualquer comando.

```ts
type AuthorityResponse<T> = {
  ok: boolean;
  status: "ok" | "warn" | "error" | "ghost";
  data?: T;
  decision?: PolicyDecision;
  gate_request?: GateRequest;
  evidence?: EvidenceRef[];
  ghosts?: Ghost[];
  receipt?: ReceiptRef;
  messages: string[];
  secret_values_printed: false;
};
```

Esse é o envelope de saída.

---

# Contratos P1 — logo depois

Esses não precisam nascer no primeiro minuto, mas são próximos.

## 19. `LogLineActCandidate`

```ts
type LogLineActCandidate = {
  who: string;
  did: string;
  this: Record<string, unknown>;
  when: string;
  confirmed_by: Record<string, unknown>;
  if_ok: Record<string, unknown>;
  if_doubt: Record<string, unknown>;
  if_not: Record<string, unknown>;
  status: string;
};
```

Regra:

```txt
exatamente 9 campos
sem décimo campo
```

---

## 20. `WalkResult`

```ts
type WalkResult = {
  act_id?: string;
  selected_branch: "if_ok" | "if_doubt" | "if_not";
  status_transition?: {
    from: string;
    to: string;
  };
  runtime_slots?: Record<string, unknown>;
  doubt_trace?: Record<string, unknown>;
  simulation_receipt?: ReceiptRef;
};
```

---

## 21. `BootstrapPlan`

```ts
type BootstrapPlan = {
  host_id: string;
  host_role: string;
  required_config: string[];
  required_artifacts: ReleaseArtifactRef[];
  actions: BootstrapAction[];
  optional: BootstrapAction[];
  ghosts: Ghost[];
};
```

---

## 22. `BootstrapAction`

```ts
type BootstrapAction = {
  action:
    | "verify_doppler"
    | "download_release_artifact"
    | "verify_sha256"
    | "install_host_runtime"
    | "install_launchd"
    | "run_doctor"
    | "run_smoke"
    | "emit_event"
    | "record_ghost";
  required: boolean;
};
```

Regra:

```txt
allowlist only
sem shell arbitrário
```

---

## 23. `McpToolRequest`

```ts
type McpToolRequest = {
  tool: "minilab.host_runtime_update" | "minilab.host_runtime_status";
  host_id: string;
  input: Record<string, unknown>;
  auth: AuthContext;
  gate_id?: string;
  execution_window_id?: string;
};
```

---

## 24. `ErrorContract`

```ts
type ErrorContract = {
  code: string;
  message: string;
  severity: "info" | "warn" | "error" | "critical";
  retryable: boolean;
  ghost_kind?: Ghost["kind"];
};
```

---

# Pacote final de contratos

Eu colocaria assim no repo:

```txt
contracts/
  service.contract.json
  command_envelope.contract.json
  principal.contract.json
  auth_context.contract.json
  action_request.contract.json
  resource_ref.contract.json
  policy_decision.contract.json
  gate_request.contract.json
  execution_window.contract.json
  config_doctor_result.contract.json
  registry_entity_ref.contract.json
  release_artifact_ref.contract.json
  host_runtime_update_request.contract.json
  lab_event.contract.json
  evidence_ref.contract.json
  receipt_ref.contract.json
  ghost.contract.json
  authority_response.contract.json
```

E no Rust:

```txt
src/contracts/
  service.rs
  command.rs
  principal.rs
  auth.rs
  action.rs
  resource.rs
  policy.rs
  gate.rs
  config.rs
  registry.rs
  release.rs
  lab.rs
  evidence.rs
  receipt.rs
  ghost.rs
  response.rs
```

# O mínimo absoluto

Se quiser começar sem inflar:

```txt
P0 mínimo real:

1. PrincipalRef
2. ResourceRef
3. ActionRequest
4. PolicyDecision
5. GateRequest
6. ExecutionWindow
7. ConfigDoctorResult
8. ReleaseArtifactRef
9. LabEvent
10. EvidenceRef
11. ReceiptRef
12. Ghost
13. AuthorityResponse
```

Com esses 13, o pacote já consegue falar:

```txt
quem pediu
o que pediu
sobre o quê
pode ou não pode
precisa gate
tem window
config está segura
release é válido
evento é prova
receipt fecha
ghost sobra
```

# Resposta final

```txt
Esse pacote genérico precisa de contratos de:
- identidade
- ação
- recurso
- auth
- policy
- gate
- execution window
- config
- release
- lab event
- evidence
- receipt
- ghost
- response envelope
```

Isso é o esqueleto contratual do Control Plane sem UI.
