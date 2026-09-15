//! Local-only real Workers Driver qualification for the production Secrets factory.
#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use lenso_app_plan::{
    AppComposition, CapabilityBinding, CapabilityEndpointPlan, CapabilityRequirementPlan,
    PluginInstancePlan,
};
use lenso_capability_secrets::{self as secrets, ResolveError, ResolveRequest, Secrets};
use lenso_kernel::{Kernel, RuntimeFailure, ShutdownOutcome};
use lenso_native_adapter::{
    NativePluginFactory, NativePluginFactoryContext, NativePluginInstance, NativePluginRegistry,
};
use lenso_workers_driver::WorkersDriver;
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Caller;
impl NativePluginFactory for Caller {
    fn package_id(&self) -> &'static str {
        "test.secrets-caller"
    }
    fn instantiate(
        &self,
        _: NativePluginFactoryContext<'_>,
    ) -> Result<NativePluginInstance, RuntimeFailure> {
        Ok(NativePluginInstance::default())
    }
}
struct Event(WorkersDriver);
impl Drop for Event {
    fn drop(&mut self) {
        self.0.request_shutdown();
    }
}
fn failure() -> JsValue {
    JsValue::from_str("Secrets qualification failed")
}

#[wasm_bindgen]
pub async fn exercise(
    lookup: js_sys::Function,
    mutate: js_sys::Function,
    mode: String,
    expected: String,
) -> Result<String, JsValue> {
    let factory = lenso_secrets_env_plugin::workers::factory(lookup);
    if format!("{factory:?}").contains(&expected) && !expected.is_empty() {
        return Err(failure());
    }
    let configuration = if mode == "invalid-config" {
        r#"{"references":{"../bad":"APP_SECRET"}}"#
    } else {
        r#"{"references":{"test/key":"APP_SECRET"}}"#
    };
    let caller = PluginInstancePlan::new("caller", "test.secrets-caller").with_requirement(
        CapabilityRequirementPlan::one(secrets::CAPABILITY_ID, secrets::DESCRIPTOR_VERSION),
    );
    let provider = PluginInstancePlan::new("secrets", lenso_secrets_env_plugin::PACKAGE_ID)
        .with_configuration(configuration)
        .with_capability(CapabilityEndpointPlan::new(
            secrets::CAPABILITY_ID,
            secrets::DESCRIPTOR_VERSION,
            [secrets::RESOLVE_OPERATION],
        ));
    let plan = AppComposition::new(
        vec![caller, provider],
        vec![CapabilityBinding::new(
            "caller",
            secrets::CAPABILITY_ID,
            secrets::DESCRIPTOR_VERSION,
            "secrets",
        )],
    )
    .resolve()
    .map_err(|_| failure())?;
    let driver = WorkersDriver::new();
    let _event = Event(driver.clone());
    let result = Kernel::start_native(
        plan,
        driver.clone(),
        NativePluginRegistry::new()
            .with_factory(Caller)
            .with_factory(factory),
    )
    .await;
    if matches!(
        mode.as_str(),
        "missing" | "wrong-type" | "throws" | "invalid-config"
    ) {
        if let Err(error) = result {
            if !expected.is_empty() && format!("{error:?}").contains(&expected) {
                return Err(failure());
            }
            return Ok("startup-rejected".into());
        }
        return Err(failure());
    }
    let app = result.map_err(|_| failure())?;
    let invoke = |reference: &str| {
        app.invoke::<Secrets>(
            "caller",
            secrets::RESOLVE_OPERATION,
            ResolveRequest {
                reference: reference.into(),
            },
        )
    };
    let value = invoke("test/key")
        .await
        .map_err(|_| failure())?
        .map_err(|_| failure())?;
    if value.value != expected || (!expected.is_empty() && format!("{value:?}").contains(&expected))
    {
        return Err(failure());
    }
    if !matches!(
        invoke("../bad").await,
        Ok(Err(ResolveError::InvalidReference))
    ) || !matches!(
        invoke("test/unknown").await,
        Ok(Err(ResolveError::UnknownReference))
    ) {
        return Err(failure());
    }
    mutate.call0(&JsValue::UNDEFINED).map_err(|_| failure())?;
    if mode == "rotate" {
        if invoke("test/key")
            .await
            .map_err(|_| failure())?
            .map_err(|_| failure())?
            .value
            != "rotated-fixture"
        {
            return Err(failure());
        }
    } else if matches!(mode.as_str(), "loss" | "closed")
        && !matches!(
            invoke("test/key").await,
            Err(RuntimeFailure::PluginFailure { .. })
        )
    {
        return Err(failure());
    }
    if app.shutdown(Duration::from_secs(1)).await != ShutdownOutcome::Clean {
        return Err(failure());
    }
    // Removing both the provider and its consuming requirement yields a bootable App.
    let remaining = AppComposition::new(
        vec![PluginInstancePlan::new("caller", "test.secrets-caller")],
        vec![],
    )
    .resolve()
    .map_err(|_| failure())?;
    let app = Kernel::start_native(
        remaining,
        driver,
        NativePluginRegistry::new().with_factory(Caller),
    )
    .await
    .map_err(|_| failure())?;
    if app.shutdown(Duration::from_secs(1)).await != ShutdownOutcome::Clean {
        return Err(failure());
    }
    Ok("passed".into())
}
