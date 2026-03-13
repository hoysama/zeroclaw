pub mod wizard;

// Re-exported for CLI and external use
#[allow(unused_imports)]
pub use wizard::{
    get_provider_model_catalog, recommended_default_model_for_provider,
    resolve_provider_selection_context, run_channels_repair_wizard, run_models_list,
    run_models_refresh, run_models_refresh_all, run_models_set, run_models_status,
    run_quick_setup, run_wizard, ProviderModelCatalog, ProviderModelOption,
    ProviderResolutionContext,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_reexport_exists<F>(_value: F) {}

    #[test]
    fn wizard_functions_are_reexported() {
        assert_reexport_exists(run_wizard);
        assert_reexport_exists(run_channels_repair_wizard);
        assert_reexport_exists(run_quick_setup);
        assert_reexport_exists(run_models_refresh);
        assert_reexport_exists(run_models_list);
        assert_reexport_exists(run_models_set);
        assert_reexport_exists(run_models_status);
        assert_reexport_exists(run_models_refresh_all);
        assert_reexport_exists(resolve_provider_selection_context);
        assert_reexport_exists(recommended_default_model_for_provider);
        assert_reexport_exists(get_provider_model_catalog);
    }
}
