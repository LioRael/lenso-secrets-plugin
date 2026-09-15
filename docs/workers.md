# Workers secret bindings

## Plugin boundary

Available in `lenso-secrets-env-plugin` 0.1.8.

`lenso.secrets.env` retains the same `lenso.secrets@1` contract, configuration,
reference allowlist, errors and lifecycle on native and Workers Hosts. It owns
logical-reference resolution. It does not own authentication, business
permissions, persistence, a secret manager, or environment discovery.

Native Hosts use `EnvSecretsFactory::new()`. Workers Hosts enable the `workers`
feature and explicitly install `lenso_secrets_env_plugin::workers::factory(lookup)`
in the request's registry. The default linked factory still reads the process
environment; it is not a Workers binding provider and will fail preparation when
its configured sources are unavailable. No fallback is selected automatically.

Configuration contains only names, for example:

```json
{"references":{"auth/signing":"AUTH_SIGNING_KEY"}}
```

The Host supplies a function taking one source name and returning a string.
Non-strings and exceptions are unavailable sources. Exception payloads never
enter Plugin diagnostics. The provider validates all required mappings during
prepare; source loss after readiness is a Runtime Failure. Invalid and unknown
logical references remain distinct Domain Errors and never reach the lookup.
Values are read on demand; each event receives its own factory and lookup.

## Host assembly

Create the lookup within the shared Runtime's event scope. Keep `env` in the
Host closure, never in the App Plan or configuration:

```js
createScope(request, env) {
  return createEventScope(resources => ({
    lookup(name) {
      if (resources.closed) throw new Error("event_scope_closed");
      const property = Object.getOwnPropertyDescriptor(env, name);
      return property && typeof property.value === "string"
        ? property.value : undefined;
    },
  }));
}
```

Pass `scope.lookup` into the Rust factory constructor. Use a fresh factory for
each event, install it with `with_factory` in a registry without the default
linked factory (or replace that factory with `with_factory_override`), and shut
down the App before the event ends. The Host must fence the callback after
closure. There is no module-global map or resolved secret in a generated file.

## Qualification

```sh
pnpm --dir workers install --frozen-lockfile
CARGO=/path/to/lenso-cargo bash workers/build.sh
node workers/proof.mjs
```

The private `lenso-secrets-workers-smoke` Host uses actual Wasm, the Workers
Driver and the published Runtime 0.1.2. It checks successful resolution,
independent and concurrent requests, invalid/unknown references, missing and
non-string sources, thrown lookup errors, rotation, loss, a closed callback,
clean shutdown and removal of the provider and consuming requirement.
All values in this local fixture are synthetic. Responses contain only outcomes
and access counts. Production credentials and Cloudflare deployment are not used.

Native contract tests continue to run through the same provider and lifecycle.
The Workers feature is opt-in; other Secrets providers (command, Keychain and
encrypted file) retain their platform requirements.

The [recorded local receipt](workers-proof.json) identifies the exact tested Wasm
and Runtime version. The first-wave inventory and stateful sequence are recorded
in the [first-wave plan](workers-adaptation-plan.md).
