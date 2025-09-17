use super::*;

/// Common parameter constructors for Kubernetes API calls
impl Kubeapi {
    /// Default parameters for `GET` operations
    pub fn get_params(&self) -> api::GetParams {
        api::GetParams::default()
    }

    /// Default parameters for `LIST` operations
    pub fn list_params(&self) -> api::ListParams {
        api::ListParams::default()
    }

    /// Default parameters for `POST` operations
    pub fn post_params(&self) -> api::PostParams {
        api::PostParams::default()
    }

    /// Default parameters for `DELETE` operations
    pub fn delete_params(&self, cascade: Cascade, dry_run: DryRun) -> api::DeleteParams {
        let dp = match cascade {
            Cascade::Background => api::DeleteParams::background(),
            Cascade::Foreground => api::DeleteParams::foreground(),
            Cascade::Orphan => api::DeleteParams::orphan(),
        };

        match dry_run {
            DryRun::Server => dp.dry_run(),
            DryRun::None | DryRun::Client => dp,
        }
    }

    /// Parameters for `POST` operations with a specified field manager
    pub fn post_params_with_manager(&self, manager: &str) -> api::PostParams {
        api::PostParams {
            field_manager: Some(manager.to_string()),
            ..default()
        }
    }
}
