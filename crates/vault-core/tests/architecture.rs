use std::fs;
use std::path::Path;

/// Architecture: `vault-core` must not link `anki` (see `docs/ARCHITECTURE.md`).
#[test]
fn vault_core_must_not_depend_on_anki() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let content = fs::read_to_string(&manifest).expect("read vault-core Cargo.toml");
    assert!(
        !manifest_declares_anki_crate(&content),
        "vault-core must not depend on the `anki` crate; use `anki-bridge` instead"
    );
}

fn manifest_declares_anki_crate(manifest: &str) -> bool {
    let mut in_dependency_table = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_dependency_table = matches!(
                trimmed,
                "[dependencies]" | "[dev-dependencies]" | "[build-dependencies]"
            );
            continue;
        }
        if !in_dependency_table || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let key = trimmed.split('=').next().unwrap_or("").trim();
        if key == "anki" {
            return true;
        }
    }
    false
}
