use tauri::State;

use crate::hermes_config;
use crate::store::AppState;

#[tauri::command]
pub fn import_hermes_providers_from_live(state: State<'_, AppState>) -> Result<usize, String> {
    crate::services::provider::import_hermes_providers_from_live(state.inner())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_hermes_live_provider_ids() -> Result<Vec<String>, String> {
    hermes_config::get_providers()
        .map(|providers| providers.keys().cloned().collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_hermes_live_provider(
    #[allow(non_snake_case)] providerId: String,
) -> Result<Option<serde_json::Value>, String> {
    hermes_config::get_provider(&providerId).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_hermes_model_config() -> Result<Option<hermes_config::HermesModelConfig>, String> {
    hermes_config::get_model_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_hermes_memory(kind: hermes_config::MemoryKind) -> Result<String, String> {
    hermes_config::read_memory(kind).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_hermes_memory(kind: hermes_config::MemoryKind, content: String) -> Result<(), String> {
    hermes_config::write_memory(kind, &content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_hermes_memory_limits() -> Result<hermes_config::HermesMemoryLimits, String> {
    hermes_config::read_memory_limits().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_hermes_memory_enabled(
    kind: hermes_config::MemoryKind,
    enabled: bool,
) -> Result<hermes_config::HermesWriteOutcome, String> {
    hermes_config::set_memory_enabled(kind, enabled).map_err(|e| e.to_string())
}
