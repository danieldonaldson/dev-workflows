//! Env helpers: an empty-but-present var (`SMTP_HOST=` in a template/ConfigMap) is UNSET.
//! `std::env::var(k).ok()` returns Some("") for it, which silently enables half-configured
//! adapters. Route every read through these.

pub fn optional_env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.trim().is_empty())
}

pub fn env_or(key: &str, default: &str) -> String {
    optional_env(key).unwrap_or_else(|| default.to_owned())
}

/// Aggregate, don't fail fast one-at-a-time: collect every missing var, report once at startup.
pub struct Missing(pub Vec<&'static str>);

pub fn require_all<const N: usize>(keys: [&'static str; N]) -> Result<[String; N], Missing> {
    let missing: Vec<_> = keys
        .iter()
        .copied()
        .filter(|k| optional_env(k).is_none())
        .collect();
    if !missing.is_empty() {
        return Err(Missing(missing));
    }
    Ok(keys.map(|k| optional_env(k).expect("checked above")))
}

/// Feature gate: `FOO_ENABLED=true` requires its vars; otherwise fall back to a no-op adapter,
/// which `main` must refuse in production (DEV_MODE != true).
pub fn enabled(flag: &str) -> bool {
    matches!(optional_env(flag).as_deref(), Some("true" | "1"))
}
