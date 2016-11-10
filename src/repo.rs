///
/// Module to compose with `repos`.
/// Github documentation available at https://developer.github.com/v3/repos/.
///

use client::GithubClient;

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

    /// Returns a structure to get informations about a repository (RepoInfoStructure), owned by someone
    pub fn get(&self) -> Result<RepoInfoStructure, String> {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPermissionsStructure {
    admin: bool,
    push: bool,
    pull: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoInfoStructure {
    id: usize,
    owner: UserInfoStructure,
    name: String,
    description: String,
    private: bool,
    fork: bool,
    url: String,
    html_url: String,
    branches_url: String,
    collaborators_url: String,
    contributors_url: String,
    forks_url: String,
    languages_url: String,
    releases_url: String,
    stargazers_url: String,
    subscribers_url: String,
    subscription_url: String,
    homepage: String,
    language: String,
    forks_count: usize,
    stargazers_count: usize,
    watchers_count: usize,
    size: usize,
    open_issues_count: usize,
    permissions: RepoPermissionsStructure,
    subscribers_count: usize,
    #[serde(skip_serializing_if="Option::is_none")]
    parent: Option<Box<RepoInfoStructure>>,
    #[serde(skip_serializing_if="Option::is_none")]
    source: Option<Box<RepoInfoStructure>>,
}