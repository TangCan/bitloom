//! FR167 — ChiselSim coupling + multi IDE-store descriptors (CLI product path).
//!
//! Kept in the `bitloom` CLI crate (not `bitloom-viz`) so `cargo publish -p bitloom --dry-run`
//! still resolves crates.io `bitloom-viz` 1.0.0 without requiring a mid-story viz republish.

/// FR134 G1 VS Code marketplace id (retained under FR167 multi-store).
pub const TYWAVES_IDE_PLUGIN_ID: &str = "surfer-project.surfer";
pub const TYWAVES_IDE_PLUGIN_CHANNEL: &str =
    "https://marketplace.visualstudio.com/items?itemName=surfer-project.surfer";

pub const CHISELSIM_PACKAGE_ID: &str = "chisel3-chiselsim";
pub const CHISELSIM_VERSION: &str = "7.15.0";
pub const CHISELSIM_CHANNEL: &str =
    "https://github.com/chipsalliance/chisel/tree/v7.15.0/src/main/scala/chisel3/simulator";
pub const CHISELSIM_PAIRING_NOTE: &str = "pairs with AD-9 Chisel 7.15.0 / firtool-1.158.0; peek/poke on generated SV (not Bitloom tick golden)";

pub const IDE_STORE_OPENVSX_ID: &str = "surfer-project.surfer";
pub const IDE_STORE_OPENVSX_CHANNEL: &str = "https://open-vsx.org/extension/surfer-project/surfer";
pub const IDE_STORE_JETBRAINS_ID: &str = "org.surferproject.surfer";
pub const IDE_STORE_JETBRAINS_CHANNEL: &str =
    "https://plugins.jetbrains.com/plugin/org.surferproject.surfer";

fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn chiselsim_manifest(title: &str) -> String {
    format!(
        "{{\n\
         \"product\": \"Bitloom\",\n\
         \"fr\": \"FR167\",\n\
         \"schemaVersion\": \"bitloom-chiselsim-1\",\n\
         \"marker\": \"data-bitloom-chiselsim\",\n\
         \"title\": \"{title}\",\n\
         \"chiselsim\": {{\n\
         \"enabled\": true,\n\
         \"packageId\": \"{CHISELSIM_PACKAGE_ID}\",\n\
         \"version\": \"{CHISELSIM_VERSION}\",\n\
         \"channel\": \"{CHISELSIM_CHANNEL}\",\n\
         \"pairing\": \"{CHISELSIM_PAIRING_NOTE}\"\n\
         }},\n\
         \"beyond\": \"FR162 Tywaves GUI primary alone is not FR167 ChiselSim coupling\",\n\
         \"note\": \"ChiselSim runtime must not enter bitloom-prelude design-crate deps\"\n\
         }}\n",
        title = escape_json_string(title),
    )
}

pub fn chiselsim_install_json() -> String {
    format!(
        "{{\n\
         \"product\": \"Bitloom\",\n\
         \"fr\": \"FR167\",\n\
         \"schemaVersion\": \"bitloom-chiselsim-install-1\",\n\
         \"chiselsim\": {{\n\
         \"packageId\": \"{CHISELSIM_PACKAGE_ID}\",\n\
         \"version\": \"{CHISELSIM_VERSION}\",\n\
         \"channel\": \"{CHISELSIM_CHANNEL}\",\n\
         \"installMarker\": \"BITLOOM_CHISELSIM_OK\",\n\
         \"envRoot\": \"BITLOOM_CHISELSIM_ROOT\",\n\
         \"forceMissingEnv\": \"BITLOOM_CHISELSIM_FORCE_MISSING\"\n\
         }},\n\
         \"beyond\": \"FR134 G1–G4 / FR162 default wave alone ≠ FR167\"\n\
         }}\n"
    )
}

pub fn chiselsim_check_sh() -> String {
    format!(
        "#!/usr/bin/env bash\n\
         # Bitloom FR167 — validate ChiselSim coupling root (or stub)\n\
         set -euo pipefail\n\
         if [[ -n \"${{BITLOOM_CHISELSIM_FORCE_MISSING:-}}\" ]]; then\n\
           echo \"bitloom.chiselsim-missing: BITLOOM_CHISELSIM_FORCE_MISSING set (refusing silent-Ok)\" >&2\n\
           exit 2\n\
         fi\n\
         ROOT=\"${{BITLOOM_CHISELSIM_ROOT:-}}\"\n\
         if [[ -z \"$ROOT\" ]]; then\n\
           echo \"bitloom.chiselsim-missing: set BITLOOM_CHISELSIM_ROOT to ChiselSim install (or stub)\" >&2\n\
           exit 2\n\
         fi\n\
         if [[ ! -d \"$ROOT\" ]]; then\n\
           echo \"bitloom.chiselsim-missing: BITLOOM_CHISELSIM_ROOT is not a directory: $ROOT\" >&2\n\
           exit 2\n\
         fi\n\
         if [[ ! -f \"$ROOT/BITLOOM_CHISELSIM_OK\" ]]; then\n\
           echo \"bitloom.chiselsim-missing: marker BITLOOM_CHISELSIM_OK not found under $ROOT\" >&2\n\
           exit 2\n\
         fi\n\
         echo \"bitloom.chiselsim: FR167 root OK ($ROOT) — {CHISELSIM_PACKAGE_ID}@{CHISELSIM_VERSION}\"\n"
    )
}

pub fn ide_stores_manifest(title: &str) -> String {
    format!(
        "{{\n\
         \"product\": \"Bitloom\",\n\
         \"fr\": \"FR167\",\n\
         \"schemaVersion\": \"bitloom-ide-stores-1\",\n\
         \"marker\": \"data-bitloom-ide-stores\",\n\
         \"title\": \"{title}\",\n\
         \"stores\": [\n\
         {{\n\
         \"kind\": \"vscode-marketplace\",\n\
         \"id\": \"{TYWAVES_IDE_PLUGIN_ID}\",\n\
         \"channel\": \"{TYWAVES_IDE_PLUGIN_CHANNEL}\",\n\
         \"role\": \"FR134-G1-retained\"\n\
         }},\n\
         {{\n\
         \"kind\": \"open-vsx\",\n\
         \"id\": \"{IDE_STORE_OPENVSX_ID}\",\n\
         \"channel\": \"{IDE_STORE_OPENVSX_CHANNEL}\",\n\
         \"role\": \"FR167-extra\"\n\
         }},\n\
         {{\n\
         \"kind\": \"jetbrains\",\n\
         \"id\": \"{IDE_STORE_JETBRAINS_ID}\",\n\
         \"channel\": \"{IDE_STORE_JETBRAINS_CHANNEL}\",\n\
         \"role\": \"FR167-extra\"\n\
         }}\n\
         ],\n\
         \"beyond\": \"FR134 G1 VS Code Marketplace alone is not FR167 multi-store\",\n\
         \"publishScript\": \"scripts/publish-tywaves-ide-stores.sh\"\n\
         }}\n",
        title = escape_json_string(title),
    )
}

pub fn ide_stores_install_json() -> String {
    format!(
        "{{\n\
         \"product\": \"Bitloom\",\n\
         \"fr\": \"FR167\",\n\
         \"schemaVersion\": \"bitloom-ide-stores-install-1\",\n\
         \"stores\": {{\n\
         \"vscode\": {{\n\
         \"id\": \"{TYWAVES_IDE_PLUGIN_ID}\",\n\
         \"channel\": \"{TYWAVES_IDE_PLUGIN_CHANNEL}\",\n\
         \"tokenEnv\": \"VSCE_PAT\"\n\
         }},\n\
         \"openVsx\": {{\n\
         \"id\": \"{IDE_STORE_OPENVSX_ID}\",\n\
         \"channel\": \"{IDE_STORE_OPENVSX_CHANNEL}\",\n\
         \"tokenEnv\": \"OVSX_PAT\"\n\
         }},\n\
         \"jetbrains\": {{\n\
         \"id\": \"{IDE_STORE_JETBRAINS_ID}\",\n\
         \"channel\": \"{IDE_STORE_JETBRAINS_CHANNEL}\",\n\
         \"tokenEnv\": \"JETBRAINS_TOKEN\"\n\
         }}\n\
         }},\n\
         \"forceMissingEnv\": \"BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING\",\n\
         \"honesty\": \"missing token → non-zero bitloom.ide-store-missing-token; never silent-Ok\"\n\
         }}\n"
    )
}
