Sim. Isso aqui é o **informativo de preenchimento** dos contratos — não é schema ainda, é a **cartilha para não preencher errado**.

# Control Plane Authority Contracts — Filling Guide v0

## 0. Lei de preenchimento

Nenhum contrato deve “inventar mundo”. Ele só pode referenciar mundo já reconhecido, observado ou declarado como ghost.

```txt
Registry resolve existência.
Doppler resolve valores, sem revelar valores.
Policy resolve permissão.
Gate resolve consequência.
ExecutionWindow resolve poder temporário.
Evidence resolve observação.
Receipt resolve fechamento.
Ghost resolve ausência.
```

Base doutrinária: Doppler guarda delegação operacional, mas não significado institucional; segredo não entra em repo, banco, receipt ou logs. LABs executam, não governam. Linguagem natural não executa. Provider success não é closure. Receipts vencem histórias.  

---

# 1. Quem pode preencher o quê

Essa é a parte mais importante.

```json
{
  "caller_may_fill": [
    "command",
    "input",
    "intent",
    "requested_at",
    "dry_run",
    "actor_claim",
    "resource_target"
  ],
  "authority_core_must_fill": [
    "policy_decision",
    "gate_request",
    "execution_window",
    "authority_response",
    "ghost",
    "error_contract"
  ],
  "database_must_resolve": [
    "registry_entity_ref",
    "release_artifact_ref",
    "known_lab_host",
    "known_config_requirement",
    "receipt_index"
  ],
  "runtime_must_observe": [
    "lab_event",
    "evidence_ref",
    "doctor_result",
    "smoke_result",
    "healthcheck_result"
  ],
  "never_user_supplied_as_truth": [
    "verified=true",
    "receipt_status=closed",
    "execution_window.status=open",
    "policy_decision=ok",
    "secret_redacted=true without scan",
    "release_artifact.status=published",
    "evidence_mode=verified"
  ]
}
```

Regra curta:

```txt
Usuário pede.
Authority decide.
Runtime observa.
Receipt fecha.
```

---

# 2. Resolvedores canônicos

Todo contrato deve preencher IDs e estados a partir destas fontes:

```json
{
  "identity_source": "registry.entities",
  "relationship_source": "registry.links",
  "runtime_source": "registry.runtimes",
  "mandate_source": "registry.mandates",
  "config_requirement_source": "registry.config_requirements + config_registry.keys",
  "secret_value_source": "Doppler only",
  "release_source": "release_registry.artifacts",
  "installation_source": "release_registry.installations",
  "lab_life_source": "lab_observability.events + lab_observability.current_state",
  "receipt_source": "ops.receipt_index",
  "logline_act_source": "ops.logline_acts"
}
```

Os personagens e papéis vêm do universo Minilab: Dan é autoridade humana/signatário de consequência; Dan Control Plane renderiza autoridade, mas não é executor nem source of truth; LABs são máquinas de execução; Doppler fornece material, mas não autoriza; Supabase/Postgres projeta e armazena mundo operacional. 

---

# 3. Vocabulário canônico inicial

## Pessoas / agentes / máquinas

```json
{
  "dan": {
    "entity_id": "dan",
    "kind": "human",
    "default_roles": ["lab_director", "approver", "consequence_signer"]
  },
  "chatgpt": {
    "entity_id": "chatgpt",
    "kind": "llm",
    "may": ["translate", "propose", "summarize"],
    "may_not": ["execute", "approve", "close_receipt"]
  },
  "lab8gb": {
    "host_id": "lab8gb",
    "entity_id": "lab_8gb",
    "role": "supervisor"
  },
  "lab256": {
    "host_id": "lab256",
    "entity_id": "lab_256",
    "role": "workbench"
  },
  "lab512": {
    "host_id": "lab512",
    "entity_id": "lab_512",
    "role": "inference"
  }
}
```

## Serviços principais

```json
{
  "minilab_database": {
    "kind": "database",
    "role": "operational_truth_store",
    "may": ["store_registry", "store_observability", "store_release_metadata"],
    "may_not": ["govern", "execute", "be_canon"]
  },
  "doppler": {
    "kind": "vault",
    "role": "operational_delegation_vault",
    "may": ["store_secrets", "store_config", "store_brakes", "store_pointers"],
    "may_not": ["authorize", "define_meaning", "close_evidence"]
  },
  "lab_host_runtime": {
    "kind": "runtime",
    "role": "local_lab_execution_surface",
    "may": ["run_allowlisted_scripts", "emit_events", "serve_health"],
    "may_not": ["govern", "approve_itself"]
  }
}
```

---

# 4. Contratos P0 e como preencher

## 4.1 `ServiceContract`

Identifica o binário Rust.

```json
{
  "filled_by": "authority_core",
  "required_source": "compile-time metadata + runtime flags",
  "rules": {
    "service_id": "must be stable",
    "version": "from Cargo.toml",
    "role": "control_plane_authority_core",
    "external_effects_enabled": "false unless explicitly admitted"
  },
  "example": {
    "service_id": "minilab-control-plane-authority",
    "version": "0.1.0",
    "role": "control_plane_authority_core",
    "mode": "cli",
    "external_effects_enabled": false
  }
}
```

---

## 4.2 `CommandEnvelope`

Envelope comum para CLI e HTTP.

```json
{
  "filled_by": "caller",
  "validated_by": "authority_core",
  "rules": {
    "command_id": "uuid or ulid generated at ingress",
    "command": "namespaced verb, e.g. gate.decide",
    "actor": "must resolve to PrincipalRef",
    "dry_run": "default true unless gate/window allows mutation",
    "correlation_id": "reuse when part of larger flow"
  },
  "must_not": [
    "contain secret values",
    "claim policy decision",
    "claim receipt closure"
  ]
}
```

Exemplo:

```json
{
  "command_id": "cmd_01",
  "command": "gate.decide",
  "actor": {
    "entity_id": "dan",
    "kind": "human"
  },
  "input": {
    "verb": "update_host_runtime",
    "resource_id": "lab256"
  },
  "requested_at": "2026-05-16T15:00:00Z",
  "dry_run": true
}
```

---

## 4.3 `PrincipalRef`

Quem está pedindo.

```json
{
  "filled_by": "caller claim + registry/auth resolution",
  "required_source": "registry.entities + auth_context",
  "rules": {
    "entity_id": "must exist in registry.entities or become ghost",
    "kind": "must match registered entity kind",
    "auth_context": "must not upgrade actor kind"
  },
  "forbidden": [
    "LLM claiming human",
    "host claiming Dan",
    "token subject treated as mandate"
  ]
}
```

Correto:

```json
{
  "entity_id": "dan",
  "kind": "human",
  "display_name": "Dan"
}
```

Correto para LLM:

```json
{
  "entity_id": "chatgpt",
  "kind": "llm",
  "display_name": "ChatGPT"
}
```

---

## 4.4 `AuthContext`

O que foi autenticado.

```json
{
  "filled_by": "auth boundary",
  "rules": {
    "token_present": "boolean only",
    "token_value_printed": false,
    "verified": "false unless verifier actually ran",
    "scopes": "derived from token/registry, not caller text"
  },
  "never": [
    "print token",
    "store raw token in receipt",
    "treat local_dev as production auth"
  ]
}
```

Exemplo dev honesto:

```json
{
  "method": "local_dev",
  "verified": false,
  "scopes": [],
  "token_present": false,
  "token_value_printed": false
}
```

---

## 4.5 `ActionRequest`

O pedido institucional.

```json
{
  "filled_by": "authority_core from command input",
  "required_source": "CommandEnvelope + RegistryEntityRef + ResourceRef",
  "rules": {
    "verb": "must be normalized",
    "risk_class": "derived by authority, not by caller",
    "dry_run": "true by default",
    "metadata": "no secrets"
  }
}
```

Risk class derivation:

```json
{
  "read": ["status", "get", "list", "inspect"],
  "diagnostic": ["doctor", "smoke", "health", "verify"],
  "write": ["register", "index", "project"],
  "install": ["install"],
  "update": ["update"],
  "delete": ["delete", "drop", "remove"],
  "external_effect": ["publish", "deploy", "provider_mutation"],
  "protected_power": ["execute", "run_command", "protected"]
}
```

Exemplo:

```json
{
  "action_id": "actreq_01",
  "actor": {
    "entity_id": "dan",
    "kind": "human"
  },
  "verb": "update_host_runtime",
  "resource": {
    "kind": "lab",
    "id": "lab256"
  },
  "risk_class": "update",
  "requested_at": "2026-05-16T15:00:00Z",
  "dry_run": false
}
```

---

## 4.6 `ResourceRef`

Alvo do pedido.

```json
{
  "filled_by": "authority_core",
  "required_source": "registry/release/config/lab tables",
  "rules": {
    "lab": "must exist in lab_observability.hosts",
    "entity": "must exist in registry.entities",
    "release_artifact": "must exist in release_registry.artifacts",
    "config_key": "must exist in config_registry.keys",
    "receipt": "must exist in ops.receipt_index or be ghost"
  }
}
```

Exemplos:

```json
{
  "kind": "lab",
  "id": "lab256"
}
```

```json
{
  "kind": "release_artifact",
  "id": "minilab-host-runtime@0.1.0"
}
```

---

## 4.7 `PolicyDecision`

Resultado da política. Nunca vem do caller.

```json
{
  "filled_by": "authority_core policy engine",
  "rules": {
    "decision": "must be one of ok, denied, needs_approval, ghost, error",
    "engine": "cedar when real; stub_policy_v0 when stub",
    "reasons": "human-readable and machine-stable enough",
    "matched_policy_ids": "only if actually evaluated"
  },
  "decision_defaults": {
    "delete": "needs_approval",
    "drop": "needs_approval",
    "install": "needs_approval",
    "update": "needs_approval",
    "publish": "needs_approval",
    "llm_execute": "denied",
    "unknown_entity": "ghost"
  }
}
```

---

## 4.8 `GateRequest`

Criado quando policy exige aprovação.

```json
{
  "filled_by": "authority_core",
  "rules": {
    "status": "pending on creation",
    "required_approver": "dan for protected or consequential actions",
    "expires_at": "required for operational power",
    "reason": "must reference policy decision"
  },
  "must_not": [
    "create execution window automatically",
    "claim approval from click alone",
    "allow LLM approval"
  ]
}
```

Exemplo:

```json
{
  "gate_id": "gate_01",
  "action_id": "actreq_01",
  "actor": {
    "entity_id": "dan",
    "kind": "human"
  },
  "resource": {
    "kind": "lab",
    "id": "lab256"
  },
  "policy_decision_id": "pdec_01",
  "status": "pending",
  "requested_at": "2026-05-16T15:00:00Z",
  "required_approver": "dan",
  "reason": "host runtime update requires human approval"
}
```

---

## 4.9 `ExecutionWindow`

Só nasce após aprovação válida.

```json
{
  "filled_by": "authority_core after approval",
  "rules": {
    "single_use": true,
    "scope_hash": "hash of actor/action/resource/version",
    "expires_at": "mandatory",
    "status": "open only until consumed",
    "consumed_at": "set exactly once"
  },
  "must_not": [
    "be reused",
    "authorize different resource",
    "authorize broad shell",
    "exist without gate"
  ]
}
```

---

## 4.10 `ConfigDoctorResult`

Resultado do doctor de Doppler/config.

```json
{
  "filled_by": "config doctor",
  "required_source": "process.env injected by Doppler + config_registry.keys",
  "rules": {
    "secret_values_printed": false,
    "keys_present": "names only",
    "keys_missing": "names only",
    "keys_unknown": "names only",
    "keys_forbidden": "must hard fail if present",
    "legacy_keys_used": "allowed only when explicitly mapped"
  },
  "canonical_supabase_keys": {
    "server_secret_preferred": "SUPABASE_SECRET_KEY",
    "server_secret_legacy_fallback": "SUPABASE_SERVICE_ROLE_KEY",
    "client_publishable_preferred": "SUPABASE_PUBLISHABLE_KEY",
    "client_publishable_legacy_fallback": "SUPABASE_ANON_KEY"
  },
  "forbidden_client_exposure": [
    "NEXT_PUBLIC_SUPABASE_SECRET_KEY",
    "NEXT_PUBLIC_SUPABASE_SERVICE_ROLE_KEY"
  ]
}
```

Doppler tem projeto único `minilab`, configs como `dev_local`, `lab8gb`, `lab512`, `lab256`, e suas variáveis são classificadas como `secret`, `config`, `flag`, `brake`, `pointer`, `identity`, `window` ou `provider`. 

---

## 4.11 `RegistryEntityRef`

Referência leve a uma entidade.

```json
{
  "filled_by": "registry repository",
  "required_source": "registry.entities",
  "rules": {
    "entity_id": "stable slug/id",
    "status": "from registry",
    "evidence_status": "from registry",
    "kind": "must not be inferred from folder names"
  }
}
```

Exemplo:

```json
{
  "entity_id": "lab_256",
  "name": "LAB-256",
  "kind": "computer",
  "status": "active",
  "evidence_status": "observed"
}
```

---

## 4.12 `ReleaseArtifactRef`

Referência a pacote publicável/instalável.

```json
{
  "filled_by": "release registry repository",
  "required_source": "release_registry.artifacts",
  "rules": {
    "status": "only published can be installed",
    "sha256": "mandatory for install/update",
    "storage_bucket": "must be known bucket",
    "storage_path": "must point to registered artifact",
    "version": "must be exact"
  }
}
```

Exemplo:

```json
{
  "artifact_id": "artifact_01",
  "package_name": "minilab-host-runtime",
  "version": "0.1.0",
  "storage_bucket": "minilab-release-artifacts",
  "storage_path": "releases/minilab-host-runtime/0.1.0/minilab-host-runtime-v0.1.0-darwin-arm64.tar.gz",
  "sha256": "sha256...",
  "status": "published"
}
```

---

## 4.13 `HostRuntimeUpdateRequest`

Pedido de update de runtime de um LAB.

```json
{
  "filled_by": "control plane action builder",
  "validated_by": "authority_core",
  "rules": {
    "host_id": "must exist in lab_observability.hosts",
    "artifact": "must be published release artifact",
    "requested_by": "must resolve to PrincipalRef",
    "gate_id": "required for real update",
    "execution_window_id": "required for dispatch",
    "dry_run": "allowed without execution window"
  },
  "must_not": [
    "contain arbitrary shell",
    "contain arbitrary URL",
    "contain secret values",
    "claim install success"
  ]
}
```

---

## 4.14 `LabEvent`

Evento vindo do Host Runtime.

```json
{
  "filled_by": "LAB Host Runtime",
  "validated_by": "Control Plane ingest",
  "required_source": "runtime observation",
  "rules": {
    "host_id": "lab8gb | lab256 | lab512",
    "secret_redacted": true,
    "observed_at": "from host clock",
    "received_at": "added by server/database",
    "evidence": "must not contain token/api_key/password/secret/service_role_key",
    "runtime_version": "required for package/install/heartbeat when known"
  }
}
```

Event kinds mínimos:

```json
[
  "PACKAGE_DOWNLOADED",
  "PACKAGE_VERIFIED",
  "PACKAGE_INSTALLED",
  "DOCTOR_PASSED",
  "DOCTOR_FAILED",
  "SMOKE_PASSED",
  "SMOKE_FAILED",
  "HEALTH_CHECK_PASSED",
  "HEALTH_CHECK_FAILED",
  "HEARTBEAT_EMITTED",
  "BREAK_GLASS_CONFIRMED_OFF"
]
```

---

## 4.15 `EvidenceRef`

Referência a prova.

```json
{
  "filled_by": "runtime/control plane after observation",
  "rules": {
    "kind": "must match observed source",
    "hash": "required for artifact/checksum/receipt-like evidence",
    "uri": "allowed for storage/db references",
    "secret_redacted": true,
    "observed_at": "actual observation time"
  },
  "must_not": [
    "use narrative as evidence",
    "use provider success as closure",
    "include raw stdout with secrets"
  ]
}
```

---

## 4.16 `ReceiptRef`

Referência a receipt indexado.

```json
{
  "filled_by": "receipt verifier/indexer",
  "required_source": "ops.receipt_index or receipt store",
  "rules": {
    "receipt_hash": "mandatory",
    "receipt_status": "closed only after verification",
    "evidence_mode": "must distinguish declared/observed/verified/unverified",
    "storage_uri": "pointer only; no secret"
  }
}
```

---

## 4.17 `Ghost`

Ausência estruturada.

```json
{
  "filled_by": "any validator/authority/runtime when truth is missing",
  "rules": {
    "kind": "specific, not generic when possible",
    "summary": "short factual reason",
    "source_ids": "related command/action/resource/evidence ids",
    "status": "open until resolved"
  },
  "use_when": [
    "registry entity missing",
    "config missing",
    "auth unverified",
    "release unpublished",
    "runtime unavailable",
    "evidence missing",
    "receipt unverified"
  ]
}
```

Regra do Lab:

```txt
Ghost não é fracasso.
Ghost é impedir mentira.
```

---

## 4.18 `AuthorityResponse`

Envelope de saída de todo comando.

```json
{
  "filled_by": "authority_core",
  "rules": {
    "ok": "true only if requested operation reached its honest success condition",
    "status": "ok | warn | error | ghost",
    "decision": "include when policy/gate involved",
    "evidence": "include when observed",
    "ghosts": "include open uncertainty",
    "secret_values_printed": false,
    "messages": "short, operator-safe"
  }
}
```

Exemplo para update que precisa aprovação:

```json
{
  "ok": false,
  "status": "warn",
  "decision": {
    "decision_id": "pdec_01",
    "action_id": "actreq_01",
    "engine": "stub_policy_v0",
    "decision": "needs_approval",
    "reasons": ["host runtime update requires approval"],
    "evaluated_at": "2026-05-16T15:00:00Z"
  },
  "gate_request": {
    "gate_id": "gate_01",
    "action_id": "actreq_01",
    "status": "pending",
    "required_approver": "dan"
  },
  "messages": ["update prepared; approval required"],
  "secret_values_printed": false
}
```

---

# 5. Contratos P1

## 5.1 `LogLineActCandidate`

Preencher quando uma mutação semântica for proposta.

```json
{
  "filled_by": "action builder / LLM translator / CLI",
  "validated_by": "LogLine/runtime",
  "rules": {
    "field_count": 9,
    "required_fields": [
      "who",
      "did",
      "this",
      "when",
      "confirmed_by",
      "if_ok",
      "if_doubt",
      "if_not",
      "status"
    ],
    "natural_language": "may create candidate only",
    "dispatch": "not allowed from candidate alone"
  }
}
```

O plano estratégico define os 9 slots como protocolo mandatório para comunicação, admissibilidade, decisão, execução e evidência; “mouth” não é décimo slot, é a jurisdição executável que invoca a gramática. 

---

## 5.2 `WalkResult`

Resultado do walk.

```json
{
  "filled_by": "LogLine runtime / authority core adapter",
  "rules": {
    "selected_branch": "if_ok | if_doubt | if_not",
    "if_doubt": "never hidden ok",
    "simulation_receipt": "required when doubt becomes simulation"
  }
}
```

---

## 5.3 `BootstrapPlan`

Plano de reconstrução operacional.

```json
{
  "filled_by": "bootstrap planner",
  "required_source": [
    "registry.entities",
    "registry.runtimes",
    "registry.config_requirements",
    "release_registry.artifacts"
  ],
  "rules": {
    "restores": "operational capability, not perfect memory",
    "actions": "allowlisted only",
    "optional_failures": "become ghosts",
    "core_failures": "fail bootstrap"
  }
}
```

---

## 5.4 `BootstrapAction`

Ação permitida de bootstrap.

```json
{
  "filled_by": "bootstrap planner from allowlist",
  "allowed_actions": [
    "verify_doppler",
    "download_release_artifact",
    "verify_sha256",
    "install_host_runtime",
    "install_launchd",
    "run_doctor",
    "run_smoke",
    "emit_event",
    "record_ghost"
  ],
  "forbidden": [
    "arbitrary_shell",
    "arbitrary_url",
    "eval",
    "provider_mutation_without_gate"
  ]
}
```

---

## 5.5 `McpToolRequest`

Pedido para MCP local.

```json
{
  "filled_by": "control plane adapter",
  "validated_by": "host runtime",
  "rules": {
    "tool": "must be allowlisted",
    "host_id": "must match local LAB identity",
    "gate_id": "required for protected update",
    "execution_window_id": "required for real dispatch",
    "input": "no arbitrary command"
  }
}
```

Exemplo:

```json
{
  "tool": "minilab.host_runtime_update",
  "host_id": "lab256",
  "input": {
    "target_version": "0.1.0",
    "artifact_id": "artifact_01"
  },
  "gate_id": "gate_01",
  "execution_window_id": "win_01"
}
```

---

## 5.6 `ErrorContract`

Erro estruturado.

```json
{
  "filled_by": "any layer",
  "rules": {
    "code": "stable snake_case",
    "message": "safe for operator",
    "retryable": "true only if retry makes sense",
    "ghost_kind": "required when error means unknown/missing truth"
  }
}
```

Exemplos:

```json
{
  "code": "missing_registry_entity",
  "message": "resource lab999 is not recognized in registry",
  "severity": "error",
  "retryable": false,
  "ghost_kind": "missing_registry_entity"
}
```

```json
{
  "code": "release_artifact_unpublished",
  "message": "target artifact exists but is not published",
  "severity": "warn",
  "retryable": false,
  "ghost_kind": "release_unpublished"
}
```

---

# 6. Defaults canônicos

```json
{
  "default_modes": {
    "MINILAB_SAFE_MODE": true,
    "MINILAB_DRY_RUN": true,
    "MINILAB_EXECUTION_MODE": "draft_only"
  },
  "default_decisions": {
    "protected_or_mutating_action": "needs_approval",
    "unknown_actor": "ghost",
    "unknown_resource": "ghost",
    "llm_execute": "denied",
    "missing_evidence": "ghost"
  },
  "default_evidence_status": {
    "seeded_registry_row": "declared",
    "browser_or_runtime_seen": "observed",
    "receipt_verified": "verified",
    "claimed_without_probe": "unverified"
  },
  "default_secret_policy": {
    "secret_values_printed": false,
    "allowed_in_repo": false,
    "allowed_in_database_value": false,
    "allowed_in_receipt_value": false,
    "allowed_in_logs": false
  }
}
```

---

# 7. Preenchimento por fluxo

## Update do Host Runtime

```json
{
  "flow": "host_runtime_update",
  "contracts_in_order": [
    "CommandEnvelope",
    "PrincipalRef",
    "AuthContext",
    "ActionRequest",
    "ResourceRef",
    "ReleaseArtifactRef",
    "PolicyDecision",
    "GateRequest",
    "ExecutionWindow",
    "McpToolRequest",
    "LabEvent",
    "EvidenceRef",
    "ReceiptRef",
    "AuthorityResponse"
  ],
  "success_condition": [
    "PACKAGE_DOWNLOADED",
    "PACKAGE_VERIFIED",
    "PACKAGE_INSTALLED",
    "DOCTOR_PASSED",
    "SMOKE_PASSED"
  ],
  "must_not_claim_success_before": "PACKAGE_INSTALLED + DOCTOR_PASSED + SMOKE_PASSED"
}
```

## Config doctor

```json
{
  "flow": "config_doctor",
  "contracts_in_order": [
    "CommandEnvelope",
    "PrincipalRef",
    "ConfigDoctorResult",
    "EvidenceRef",
    "AuthorityResponse"
  ],
  "success_condition": [
    "no_forbidden_keys",
    "no_secret_values_printed",
    "required_keys_present_or_warned",
    "brakes_valid"
  ]
}
```

## Release publish

```json
{
  "flow": "release_publish",
  "contracts_in_order": [
    "CommandEnvelope",
    "PrincipalRef",
    "ActionRequest",
    "ReleaseArtifactRef",
    "PolicyDecision",
    "GateRequest",
    "ExecutionWindow",
    "EvidenceRef",
    "ReceiptRef",
    "AuthorityResponse"
  ],
  "success_condition": [
    "secret_scan_passed",
    "sha256_generated",
    "storage_upload_verified",
    "release_registry_artifact_status_published"
  ]
}
```

## Bootstrap

```json
{
  "flow": "bootstrap_lab",
  "contracts_in_order": [
    "CommandEnvelope",
    "PrincipalRef",
    "BootstrapPlan",
    "BootstrapAction",
    "ReleaseArtifactRef",
    "LabEvent",
    "EvidenceRef",
    "Ghost",
    "AuthorityResponse"
  ],
  "success_condition": [
    "package_verified",
    "host_runtime_installed",
    "doctor_passed",
    "smoke_passed"
  ],
  "acceptable_result": "partial_with_ghosts_if_core_restored"
}
```

---

# 8. JSON compacto para virar arquivo

Nome sugerido:

```txt
contracts/contract_filling_guide.v0.json
```

```json
{
  "version": "control-plane-contract-filling-guide.v0",
  "doctrine": {
    "registry": "recognized existence",
    "doppler": "operational values only",
    "policy": "permission decision",
    "gate": "consequence boundary",
    "execution_window": "scoped single-use power",
    "evidence": "observed output",
    "receipt": "proof closure",
    "ghost": "structured absence"
  },
  "global_rules": [
    "do_not_invent_entities",
    "do_not_print_secret_values",
    "do_not_accept_natural_language_as_dispatch",
    "do_not_claim_success_without_evidence",
    "do_not_treat_provider_success_as_closure",
    "do_not_allow_llm_execution",
    "do_not_reuse_execution_window",
    "do_not_install_unpublished_release",
    "do_not_run_arbitrary_shell_from_registry"
  ],
  "resolvers": {
    "principal": "registry.entities + auth_context",
    "resource": "registry/entities/lab/release/config tables",
    "config": "Doppler env + config_registry.keys",
    "release": "release_registry.artifacts",
    "installation": "release_registry.installations",
    "lab_state": "lab_observability.current_state",
    "lab_events": "lab_observability.events",
    "receipt": "ops.receipt_index",
    "act": "ops.logline_acts"
  },
  "contracts": {
    "ServiceContract": {
      "filled_by": "authority_core",
      "truth_source": "binary metadata",
      "must_include": ["service_id", "version", "role", "mode", "external_effects_enabled"]
    },
    "CommandEnvelope": {
      "filled_by": "caller",
      "validated_by": "authority_core",
      "must_include": ["command_id", "command", "actor", "input", "requested_at", "dry_run"]
    },
    "PrincipalRef": {
      "filled_by": "caller_claim_plus_auth",
      "resolved_by": "registry.entities",
      "must_not": ["upgrade_llm_to_human", "treat_token_as_mandate"]
    },
    "AuthContext": {
      "filled_by": "auth_boundary",
      "must": ["token_value_printed_false"],
      "must_not": ["print_raw_token", "store_raw_token"]
    },
    "ActionRequest": {
      "filled_by": "authority_core",
      "risk_class": "derived_not_user_supplied",
      "default_dry_run": true
    },
    "ResourceRef": {
      "filled_by": "authority_core",
      "must_resolve_against": ["registry.entities", "lab_observability.hosts", "release_registry.artifacts", "config_registry.keys", "ops.receipt_index"]
    },
    "PolicyDecision": {
      "filled_by": "policy_engine",
      "decisions": ["ok", "denied", "needs_approval", "ghost", "error"]
    },
    "GateRequest": {
      "filled_by": "authority_core",
      "created_when": "policy_decision_needs_approval",
      "default_status": "pending"
    },
    "ExecutionWindow": {
      "filled_by": "authority_core_after_approval",
      "rules": ["single_use", "scoped", "expires", "not_global_permission"]
    },
    "ConfigDoctorResult": {
      "filled_by": "config_doctor",
      "must": ["secret_values_printed_false", "forbidden_keys_fail", "names_only"],
      "supabase_key_preference": ["SUPABASE_SECRET_KEY", "SUPABASE_SERVICE_ROLE_KEY_legacy_fallback"]
    },
    "RegistryEntityRef": {
      "filled_by": "registry_repository",
      "source": "registry.entities"
    },
    "ReleaseArtifactRef": {
      "filled_by": "release_repository",
      "source": "release_registry.artifacts",
      "installable_only_when": "status_published"
    },
    "HostRuntimeUpdateRequest": {
      "filled_by": "control_plane_action_builder",
      "requires": ["known_host", "published_artifact", "gate_for_real_update", "execution_window_for_dispatch"]
    },
    "LabEvent": {
      "filled_by": "lab_host_runtime",
      "validated_by": "control_plane_ingest",
      "must": ["secret_redacted_true", "known_host_id", "no_secret_fields_in_evidence"]
    },
    "EvidenceRef": {
      "filled_by": "runtime_or_control_plane_after_observation",
      "must_not": ["use_narrative_as_evidence", "include_secret_values"]
    },
    "ReceiptRef": {
      "filled_by": "receipt_verifier_or_indexer",
      "source": "ops.receipt_index",
      "closed_only_after": "verification"
    },
    "Ghost": {
      "filled_by": "any_layer_when_truth_missing",
      "purpose": "structured_absence_not_failure"
    },
    "AuthorityResponse": {
      "filled_by": "authority_core",
      "must": ["secret_values_printed_false", "honest_status", "ghosts_when_unknown"]
    }
  }
}
```

---

# 9. Frase de uso

```txt
Preencher contrato não é preencher formulário.
É resolver autoridade.

Toda linha deve responder:
quem pediu,
com que identidade,
sobre qual coisa reconhecida,
com qual risco,
sob qual policy,
com qual gate,
com qual evidência,
e o que ficou ghost.
```

Isso é o suficiente para o pacote genérico Rust não virar framework vazio. Ele passa a ter uma gramática operacional de preenchimento.
