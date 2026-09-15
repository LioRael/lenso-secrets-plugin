# Workers Plugin adaptation: first wave

Source inventory: local main checkouts on 2026-09-15. The feature column is a
manifest signal, not a compatibility certification. Transitive dependencies,
actual Wasm compilation and workerd behavior must be checked before declaring
support. Existing Auth and Marketplace proofs were handled in the previous batch.

## Selected first implementation

Env Secrets is the first additional Plugin. The Auth Workers Host currently has
a test-only Secrets provider; a production allowlisted binding factory closes
that composition gap. It preserves the Secrets contract and provider logic,
receives an event-owned lookup, and leaves values out of Plans and diagnostics.
Keychain, local encrypted files and command execution retain native requirements.

The implementation and executable proof live in `lenso-secrets-plugin` on
`release-plz-workers-bindings`, documented in `docs/workers.md`.

## Stateful sequence

| Order | Owner | Required equivalence proof before declaring Workers support |
| --- | --- | --- |
| 1 | Access Control | Default deny; protected bootstrap role/binding; current authorization and mutation in one atomic operation; exactly one revision increase per effective change; role-page snapshot consistency |
| 2 | Organization | Membership changes and authorization retain their transaction semantics; external Access Control ownership stays intact |
| 3 | Feature Flag | Flag/environment CAS and immutable ruleset publication remain atomic; evaluation batches and receipts commit together; idempotency and deterministic targeting vectors match native |

The candidate manifests pin older Git cohorts of Kernel, Native Adapter and Auth.
Before adding D1 implementations, align their dependency graph with the released
Workers-compatible cohort. Do not change locked Git identities independently
without checking the full graph for duplicated Capability and Kernel types.

The existing executable packages and Plugin IDs are explicitly PostgreSQL-named.
A D1 target needs an explicit authoring decision about backend-specific Plugin
identity and configuration, while preserving the provided Capability contracts.
Do not silently reinterpret a PostgreSQL schema/database URL setting as a D1
binding. SQL remains owned by each Plugin under backend-specific migration paths.
Use `lenso-migration` plus its D1 adapter for lifecycle; do not move business SQL
or authorization into the migration library.

Access Control currently authorizes and writes inside a transaction after
`SELECT ... FOR UPDATE`. Feature Flag uses transaction-scoped idempotency, CAS,
and atomic bounded evaluations. D1 implementation must prove equivalent atomic
checks and writes; replacing these with separate read/write calls is not support.

## Candidate inventory

| Repository | Declared Workers signal | Dependencies requiring inspection |
| --- | --- | --- |
| `lenso-access-control-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-access-request-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-audit-log-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-auth-passkey-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; UUID randomness: inspect target features |
| `lenso-auth-plugin` | workers feature in 9 crates | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-business-approval-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-content-vault-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-customer-directory-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-data-export-retention-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-email-smtp-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-entitlements-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-feature-flag-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-github-project-sync-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-help-center-web-plugin` | No workers feature found; not a verdict | Inspect source and transitive graph |
| `lenso-jobs-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx |
| `lenso-knowledge-author-web-plugin` | No workers feature found; not a verdict | Inspect source and transitive graph |
| `lenso-knowledge-base-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-legal-hold-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-notification-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-notification-template-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-organization-invitation-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-organization-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-organization-provisioning-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-otel-plugin` | No workers feature found; not a verdict | Inspect source and transitive graph |
| `lenso-outbound-webhook-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx |
| `lenso-privacy-request-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-project-automation-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-project-portfolio-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-projects-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-projects-web-plugin` | No workers feature found; not a verdict | Tokio dependency: inspect feature gates; HTTP client: inspect egress path; UUID randomness: inspect target features |
| `lenso-scim-provisioning-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-search-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |
| `lenso-secrets-plugin` | No workers feature found; not a verdict | Inspect source and transitive graph |
| `lenso-service-account-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; UUID randomness: inspect target features |
| `lenso-stripe-subscription-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; UUID randomness: inspect target features |
| `lenso-support-attachment-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-support-case-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-support-email-resend-plugin` | No workers feature found; not a verdict | Inspect source and transitive graph |
| `lenso-support-sla-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-support-web-plugin` | No workers feature found; not a verdict | Inspect source and transitive graph |
| `lenso-usage-billing-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates; UUID randomness: inspect target features |
| `lenso-usage-meter-plugin` | No workers feature found; not a verdict | PostgreSQL / SQLx; Tokio dependency: inspect feature gates |

## Source anchors

- `lenso-auth-plugin/experiments/workers-g4/host/src/lib.rs`: test Secrets factory.
- `lenso-secrets-plugin/crates/lenso-secrets-env-plugin/src/lib.rs`: shared allowlist, preparation and invocation.
- `lenso-access-control-plugin/crates/lenso-access-control-postgres-plugin/src/storage.rs`: authorized transaction and policy revisions.
- `lenso-feature-flag-plugin/crates/lenso-feature-flag-postgres-plugin/src/storage.rs`: transactions, CAS and receipts.
- `lenso-organization-plugin/crates/lenso-organization-postgres-plugin/src/lib.rs`: native lifecycle and SQL ownership.

## Completion gates for each Plugin

Native regression and actual Wasm checks; real workerd success and failure paths;
packaged artifact checks; event isolation and teardown; explicit unsupported
target failure; a real Host-derived Plan and removal proof. Publication and
consumer registry adoption are separate delivery steps.
