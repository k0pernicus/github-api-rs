///
/// Module to compose with `users`.
/// Github documentation available at https://developer.github.com/v3/users/.
///
use client::GithubClient;

use hyper::method::Method;

use serde_json;

/// URL to access the Github API for myself
const USER_API: &'static str = "user";
/// URL to access the Github API for other Github users
const USERS_API: &'static str = "users";

/// A client to communicate with the Github API for Users.
/// You can create as much as UserClient structure as users you're looking for.
pub struct UserClient<'a> {
    /// The Github client
    github_client: &'a GithubClient,
    /// The username to build requests
    pub username: String,
}

impl<'a> UserClient<'a> {
    /// Returns a client to communicate with the Github API, specifically for a user
    ///
    /// # Arguments
    ///
    /// * `github_client` - The Github client that communicate with the Github API
    /// * `username` - The username of the user requested
    ///
    /// # Example
    ///
    /// `let user_client = UserClient::new(&github_client, "k0pernicus2");`
    pub fn new(github_client: &'a GithubClient, username: &'a str) -> Self {
        UserClient {
            github_client: github_client,
            username: username.to_owned(),
        }
    }

    /// Returns a Result type that contains a UserInfoStructure structure, or a String error.
    ///
    /// # Example
    ///
    /// `
    /// let github_client = GithubClient::new("k0pernicus", "myapikey0123456789");
    /// let user_client = github_client.get_myself_client();
    /// let user_infos = user_client.unwrap();
    /// `
    pub fn get(&self) -> Result<UserInfoStructure, String> {
        /// Check if the current user is the same for this client
        let url = if self.username == self.github_client.username {
            USER_API.to_string()
        } else {
            format!("{}/{}", USERS_API, self.username)
        };
        match self.github_client
            .process_request(Method::Get, &url) {
            Ok(response) => {
                match serde_json::from_str(&response) {
                    Ok(user_structure) => Ok(user_structure),
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

/// Interesting fields that represent globally a User on Github
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoStructure {
    #[serde(skip_serializing_if="Option::is_none")]
    login: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    id: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    avatar_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    gravatar_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    followers_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    following_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    gists_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    starred_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    subscriptions_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    organizations_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    repos_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    site_admin: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    public_repos: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    public_gists: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    followers: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    following: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    company: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    hireable: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    bio: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    total_private_repos: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    owned_private_repos: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    private_gists: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    disk_usage: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    collaborators: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    plan: Option<UserPlanStructure>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserPlanStructure {
    #[serde(skip_serializing_if="Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    space: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    private_repos: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    collaborators: Option<usize>,
}

/// Fields that can be modified using the Github API, for a given user
#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateStructure {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hireable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bio: Option<String>,
}
