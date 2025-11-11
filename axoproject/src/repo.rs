use crate::errors::*;
use crate::github::GithubRepo;
use crate::gitlab::GitlabRepo;

/// A generic representation of a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Repo {
    /// A GitHub repository.
    GitHub(GithubRepo),
    /// A GitLab repository.
    GitLab(GitlabRepo),
}

impl Repo {
    /// Constructs a new Repo from a URL string.
    pub fn from_url(repo_url: &str) -> Result<Self> {
        // We need to be a bit more intelligent than just checking for "github.com".
        // The github parsing logic will return a NotGitHubError if it's not a github
        // url, so we can use that to try parsing as other hosts.
        let gh_result = GithubRepo::from_url(repo_url);
        if let Err(e) = &gh_result {
            if !matches!(e, AxoprojectError::NotGitHubError { .. }) {
                return gh_result.map(Repo::GitHub);
            }
        } else {
            return gh_result.map(Repo::GitHub);
        }

        let gl_result = GitlabRepo::from_url(repo_url);
        if let Err(e) = &gl_result {
            if !matches!(e, AxoprojectError::NotGitLabError { .. }) {
                return gl_result.map(Repo::GitLab);
            }
        } else {
            return gl_result.map(Repo::GitLab);
        }

        // Return the github error by default, as it's the most common host.
        gh_result.map(Repo::GitHub)
    }

    /// Returns a URL suitable for web access to the repository.
    pub fn web_url(&self) -> String {
        match self {
            Repo::GitHub(repo) => repo.web_url(),
            Repo::GitLab(repo) => repo.web_url(),
        }
    }
}
