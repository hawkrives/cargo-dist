use std::fmt;

use crate::errors::*;

use url::Url;

/// Represents a GitLab repository that we can query things about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitlabRepo {
    /// The repository owner.
    pub owner: String,
    /// The repository name.
    pub name: String,
}

impl GitlabRepo {
    /// Returns the domain. At the moment this is hardcoded to gitlab.com, but
    /// it may support more options in the future.
    pub fn domain(&self) -> String {
        "https://gitlab.com".to_owned()
    }

    /// Path component. Used with `domain` to construct `web_url`.
    pub fn web_path(&self) -> String {
        format!("/{}/{}", self.owner, self.name)
    }

    /// Returns a URL suitable for web access to the repository.
    pub fn web_url(&self) -> String {
        format!("{}{}", self.domain(), self.web_path())
    }

    /// Constructs a new Gitlab repository from a "owner/name" string. Notably, this does not check
    /// whether the repo actually exists.
    pub fn from_url(repo_url: &str) -> Result<Self> {
        GitlabRepoInput::new(repo_url.to_string())?.parse()
    }
}

impl fmt::Display for GitlabRepo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}/{})", self.owner, self.name)
    }
}

/// An intermediary type to hold a repository URL.
#[derive(Debug)]
pub enum GitlabRepoInput {
    /// An HTTPS URL.
    Url(String),
    /// An SSH URL.
    Ssh(String),
}

impl GitlabRepoInput {
    /// Creates a new GitlabRepoInput from a string.
    pub fn new(repo_string: String) -> Result<Self> {
        // Handle git+https- just the same as https
        if repo_string.starts_with("https") || repo_string.starts_with("git+https") {
            Ok(Self::Url(repo_string))
        } else if repo_string.starts_with("git@") {
            Ok(Self::Ssh(repo_string))
        } else {
            let err = AxoprojectError::UnknownRepoStyle { url: repo_string };
            Err(err)
        }
    }

    /// Parses the input into a GitlabRepo.
    pub fn parse(self) -> Result<GitlabRepo> {
        match self {
            Self::Url(s) => Ok(Self::parse_url(s)?),
            Self::Ssh(s) => Ok(Self::parse_ssh(s)?),
        }
    }

    fn parse_url(repo_string: String) -> Result<GitlabRepo> {
        let parsed = Url::parse(&repo_string)?;
        if parsed.domain() != Some("gitlab.com") {
            return Err(AxoprojectError::NotGitLabError { url: repo_string });
        }
        let segment_list = parsed.path_segments().map(|c| c.collect::<Vec<_>>());
        if let Some(segments) = segment_list {
            if segments.len() >= 2 {
                let owner = segments[0..segments.len() - 1].join("/");
                let name = Self::remove_git_suffix(segments.last().unwrap().to_string());
                return Ok(GitlabRepo { owner, name });
            }
        }
        Err(AxoprojectError::RepoParseError { repo: repo_string })
    }

    fn parse_ssh(repo_string: String) -> Result<GitlabRepo> {
        let core = Self::remove_git_suffix(Self::remove_git_prefix(repo_string.clone())?);
        let segments: Vec<&str> = core.split('/').collect();
        if !segments.is_empty() && segments.len() >= 2 {
            let owner = segments[0..segments.len() - 1].join("/");
            let name = Self::remove_git_suffix(segments.last().unwrap().to_string());
            return Ok(GitlabRepo { owner, name });
        }
        Err(AxoprojectError::RepoParseError { repo: repo_string })
    }

    fn remove_git_prefix(s: String) -> Result<String> {
        let prefix = "git@gitlab.com:";
        if let Some(stripped) = s.strip_prefix(prefix) {
            Ok(stripped.to_string())
        } else {
            Err(AxoprojectError::NotGitLabError { url: s })
        }
    }

    fn remove_git_suffix(s: String) -> String {
        if let Some(chomped) = s.strip_suffix(".git") {
            chomped.to_string()
        } else {
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_an_https_repo_string() {
        let input = "https://gitlab.com/axodotdev/oranda";
        let actual_owner = "axodotdev";
        let actual_name = "oranda";
        let parsed = GitlabRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }

    #[test]
    fn it_parses_an_https_repo_string_with_dot_git() {
        let input = "https://gitlab.com/axodotdev/oranda.git";
        let actual_owner = "axodotdev";
        let actual_name = "oranda";
        let parsed = GitlabRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }

    #[test]
    fn it_parses_an_ssh_repo_string() {
        let input = "git@gitlab.com:axodotdev/oranda.git";
        let actual_owner = "axodotdev";
        let actual_name = "oranda";
        let parsed = GitlabRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }

    #[test]
    fn it_parses_a_nested_https_repo_string() {
        let input = "https://gitlab.com/axodotdev/oranda/nested";
        let actual_owner = "axodotdev/oranda";
        let actual_name = "nested";
        let parsed = GitlabRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }

    #[test]
    fn it_parses_a_nested_https_repo_string_with_dot_git() {
        let input = "https://gitlab.com/axodotdev/oranda/nested.git";
        let actual_owner = "axodotdev/oranda";
        let actual_name = "nested";
        let parsed = GitlabRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }

    #[test]
    fn it_parses_a_nested_ssh_repo_string() {
        let input = "git@gitlab.com:axodotdev/oranda/nested.git";
        let actual_owner = "axodotdev/oranda";
        let actual_name = "nested";
        let parsed = GitlabRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }
}
