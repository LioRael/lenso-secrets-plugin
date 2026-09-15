//! Request-owned Workers bindings for the existing Env Secrets contract.

use std::{fmt, rc::Rc};

use wasm_bindgen::JsValue;

use super::{EnvSecretsFactory, SecretSource, SourceUnavailable};

/// Creates an Env Secrets factory using an explicit Workers binding lookup.
///
/// The Host must create a fresh lookup and factory for each event. The lookup
/// accepts one configured source name and returns a string, or throws/returns a
/// non-string when unavailable. It must reject calls after its event closes.
/// No global environment, fallback provider, or secret-bearing configuration is
/// consulted. Lookup exceptions are discarded rather than copied to diagnostics.
#[must_use]
pub fn factory(lookup: js_sys::Function) -> EnvSecretsFactory {
    EnvSecretsFactory {
        source: Rc::new(BindingLookup(lookup)),
    }
}

struct BindingLookup(js_sys::Function);

impl fmt::Debug for BindingLookup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WorkersSecretBindings(redacted)")
    }
}

impl SecretSource for BindingLookup {
    fn read(&self, name: &str) -> Result<String, SourceUnavailable> {
        self.0
            .call1(&JsValue::UNDEFINED, &JsValue::from_str(name))
            .ok()
            .and_then(|value| value.as_string())
            .ok_or(SourceUnavailable)
    }
}
