///
/// Module to compose with `repos`.
/// Github documentation available at https://developer.github.com/v3/repos/.
///

use client::GithubClient;

use GetterAPI;

use hyper::method::Method;

use user::UserInfoStructure;

use serde_json;

const REPOS_API_URL: &'static str = "repos";

pub struct RepoClient<'a> {
    /// The Github client
    github_client: &'a GithubClient,
    /// Owner of the repository
    owner: String,
    /// Repository name
    reponame: String,
}

impl<'a> RepoClient<'a> {
    /// Returns a client to communicate with the Github API, specifically for a repository
    ///
    /// # Arguments
    ///
    /// * `github_client` - The Github client that communicate with the Github API
    /// * `owner` - The owner of the repository to get informations about
    /// * `reponame` - The repository name to get informations about
    ///
    /// # Example
    ///
    /// `let repo_client = RepoClient::new(&github_client, "k0pernicus", "github-api-rs");`
    pub fn new(github_client: &'a GithubClient, owner: &'a str, reponame: &'a str) -> Self {
        RepoClient {
            github_client: github_client,
            owner: owner.to_owned(),
            reponame: reponame.to_owned(),
        }
    }
}

impl<'a> GetterAPI for RepoClient<'a> {
    type GetType = RepoInfoStructure;
    /// Returns a structure to get informations about a repository (RepoInfoStructure), owned by someone
    fn get(&self) -> Result<RepoInfoStructure, String> {
        let url = format!("{}/{}/{}", REPOS_API_URL, self.owner, self.reponame);
        match self.github_client.process_request(Method::Get, &url, None) {
            Ok(response) => {
                match serde_json::from_str(&response) {
                    Ok(repo_structure) => Ok(repo_structure),
                    Err(error) => {
                        Err(format!("Error when converting the string request from Github to \
                                     JSON: {}, for response {}",
                                    error,
                                    response))
                    }
                }
            }
            Err(error) => Err(error),
        }
    }
}

/// Contains all necessary fields to define permissions, for a given repository
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPermissionsStructure {
    #[serde(skip_serializing_if="Option::is_none")]
    admin: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    push: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pull: Option<bool>,
}

/// Contains all necessary fields to define a repository
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoInfoStructure {
    #[serde(skip_serializing_if="Option::is_none")]
    id: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    owner: Option<UserInfoStructure>,
    #[serde(skip_serializing_if="Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    private: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    fork: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    branches_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    collaborators_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    contributors_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    forks_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    languages_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    releases_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    stargazers_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    subscribers_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    subscription_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    homepage: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    forks_count: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    stargazers_count: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    watchers_count: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    size: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    open_issues_count: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    permissions: Option<RepoPermissionsStructure>,
    #[serde(skip_serializing_if="Option::is_none")]
    subscribers_count: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    parent: Option<Box<RepoInfoStructure>>,
    #[serde(skip_serializing_if="Option::is_none")]
    source: Option<Box<RepoInfoStructure>>,
}