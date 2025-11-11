//! gitlab ci config

use super::*;

/// gitlab ci config (final)
#[derive(Debug, Default, Clone)]
pub struct GitlabCiConfig {
    /// Whether we should try to merge otherwise-parallelizable tasks onto the same machine,
    pub merge_tasks: bool,

    /// Whether failing tasks should make us give up on all other tasks
    pub fail_fast: bool,

    /// Whether CI tasks should have build caches enabled.
    pub cache_builds: Option<bool>,

    /// Whether CI should include logic to build local artifacts (default true)
    pub build_local_artifacts: bool,

    /// Whether CI should trigger releases by dispatch instead of tag push (default false)
    pub dispatch_releases: bool,

    /// Instead of triggering releases on tags, trigger on pushing to a specific branch
    pub release_branch: Option<String>,

    /// Which actions to run on pull requests.
    pub pr_run_mode: cargo_dist_schema::PrRunMode,

    /// a prefix to add to the gitlab-ci.yml and tag pattern
    pub tag_namespace: Option<String>,

    /// Plan jobs to run in CI
    pub plan_jobs: Vec<JobStyle>,

    /// Local artifacts jobs to run in CI
    pub build_local_jobs: Vec<JobStyle>,

    /// Global artifacts jobs to run in CI
    pub build_global_jobs: Vec<JobStyle>,

    /// Host jobs to run in CI
    pub host_jobs: Vec<JobStyle>,

    /// Publish jobs to run in CI
    pub publish_jobs: Vec<JobStyle>,

    /// Post-announce jobs to run in CI
    pub post_announce_jobs: Vec<JobStyle>,
}

/// gitlab ci config (inheritance not yet folded)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitlabCiLayer {
    // For now, GitLab CI will use the same common config as GitHub
    // In the future, GitLab-specific options can be added here
}

impl ApplyLayer for GitlabCiLayer {
    type Layer = GitlabCiLayer;
    fn apply_layer(&mut self, _layer: Self::Layer) {
        // GitLab-specific layer application will go here when needed
    }
}

impl GitlabCiConfig {
    /// get defaults for workspace config
    pub fn defaults_for_workspace(_workspaces: &WorkspaceGraph, common: &CommonCiConfig) -> Self {
        Self {
            merge_tasks: common.merge_tasks,
            fail_fast: common.fail_fast,
            cache_builds: common.cache_builds,
            build_local_artifacts: common.build_local_artifacts,
            dispatch_releases: common.dispatch_releases,
            release_branch: common.release_branch.clone(),
            pr_run_mode: common.pr_run_mode,
            tag_namespace: common.tag_namespace.clone(),
            plan_jobs: common.plan_jobs.clone(),
            build_local_jobs: common.build_local_jobs.clone(),
            build_global_jobs: common.build_global_jobs.clone(),
            host_jobs: common.host_jobs.clone(),
            publish_jobs: common.publish_jobs.clone(),
            post_announce_jobs: common.post_announce_jobs.clone(),
        }
    }
}

impl ApplyLayer for GitlabCiConfig {
    type Layer = GitlabCiLayer;
    fn apply_layer(&mut self, _layer: Self::Layer) {
        // GitLab-specific layer application will go here when needed
    }
}
