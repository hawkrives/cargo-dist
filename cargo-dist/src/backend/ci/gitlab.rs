//! GitLab CI script generation

use camino::Utf8PathBuf;
use serde::Serialize;

use crate::{
    DistGraph, DistResult,
};

#[cfg(not(windows))]
const GITLAB_CI_DIR: &str = ".gitlab/workflows/";
#[cfg(windows)]
const GITLAB_CI_DIR: &str = r".gitlab\workflows\";
const GITLAB_CI_FILE: &str = "dist.yml";

/// Info about running dist in GitLab CI
#[derive(Debug, Serialize)]
pub struct GitlabCiInfo {
    /// Cached path to GitLab CI workflows directory
    #[serde(skip_serializing)]
    pub gitlab_ci_dir: Utf8PathBuf,
    /// Whether to fail-fast
    pub fail_fast: bool,
    /// Whether to cache builds
    pub cache_builds: bool,
    /// Whether to include builtin local artifacts tasks
    pub build_local_artifacts: bool,
    /// Whether to make CI get dispatched manually instead of by tag
    pub dispatch_releases: bool,
    /// Trigger releases on pushes to this branch instead of ci
    pub release_branch: Option<String>,
    /// What kind of job to run on pull request
    pub pr_run_mode: cargo_dist_schema::PrRunMode,
    /// whether to prefix gitlab-ci.yml and the tag pattern
    pub tag_namespace: Option<String>,
}

impl GitlabCiInfo {
    /// Compute the GitLab CI stuff
    pub fn new(dist: &DistGraph, ci_config: &crate::config::v1::ci::gitlab::GitlabCiConfig) -> DistResult<GitlabCiInfo> {
        let gitlab_ci_dir = dist.repo_dir.join(GITLAB_CI_DIR);
        let fail_fast = ci_config.fail_fast;
        let cache_builds = ci_config.cache_builds.unwrap_or(false);
        let build_local_artifacts = ci_config.build_local_artifacts;
        let dispatch_releases = ci_config.dispatch_releases;
        let release_branch = ci_config.release_branch.clone();
        let tag_namespace = ci_config.tag_namespace.clone();
        let pr_run_mode = ci_config.pr_run_mode;

        Ok(GitlabCiInfo {
            gitlab_ci_dir,
            fail_fast,
            cache_builds,
            build_local_artifacts,
            dispatch_releases,
            release_branch,
            pr_run_mode,
            tag_namespace,
        })
    }

    /// Write the GitLab CI script to disk
    pub fn write_to_disk(&self, dist: &DistGraph) -> DistResult<()> {
        use axoasset::LocalAsset;
        
        let ci_file = self.ci_file_path();
        let rendered = self.generate_ci(dist)?;

        LocalAsset::write_new_all(&rendered, &ci_file)?;
        eprintln!("generated GitLab CI to {}", ci_file);

        Ok(())
    }

    /// Check whether the new configuration differs from the config on disk
    /// without actually writing the result.
    pub fn check(&self, dist: &DistGraph) -> DistResult<()> {
        use crate::backend::diff_files;
        
        let ci_file = self.ci_file_path();
        let rendered = self.generate_ci(dist)?;
        diff_files(&ci_file, &rendered)
    }

    /// Generate the GitLab CI script
    pub fn generate_ci(&self, _dist: &DistGraph) -> DistResult<String> {
        // For now, return a basic GitLab CI template
        // This will be expanded with proper template rendering in the future
        let yaml = format!(
            r#"# GitLab CI configuration for cargo-dist
# This is a basic template - full implementation coming soon

variables:
  DIST_VERSION: "{}"

stages:
  - plan
  - build
  - publish

plan:
  stage: plan
  script:
    - echo "Planning release"
  only:
    - tags

build:
  stage: build
  script:
    - echo "Building artifacts"
  needs: [plan]
  only:
    - tags

publish:
  stage: publish
  script:
    - echo "Publishing release"
  needs: [build]
  only:
    - tags
"#,
            env!("CARGO_PKG_VERSION")
        );

        Ok(yaml)
    }

    /// Get the path to the GitLab CI file
    pub fn ci_file_path(&self) -> Utf8PathBuf {
        self.gitlab_ci_dir.join(GITLAB_CI_FILE)
    }
}
