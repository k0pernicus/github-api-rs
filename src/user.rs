///
/// Module to compose with `users`.
/// Github documentation available at https://developer.github.com/v3/users/.
///
use client::GithubClient;
use GetterAPI;
use hyper::method::Method;
use serde_json;

/// URL to access the Github API for myself
const USER_API_URL: &'static str = "user";
/// URL to access the Github API for other Github users
const USERS_API_URL: &'static str = "users";

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

    /// Update the current informations about the user, and returns a String that contains
    /// a message from the server if the request succeeds or an error message
    ///
    /// # Argument
    ///
    /// `new_infos` - A UserUpdateStructure that contains some informations to update
    pub fn update(&self, new_infos: &UserUpdateStructure) -> Result<String, String> {
        let infos_to_send = serde_json::to_string::<UserUpdateStructure>(new_infos);
        match infos_to_send {
            Ok(body) => self.github_client.process_request(Method::Patch, USER_API_URL, Some(body)),
            Err(error) => {
                Err(format!("Error converting the updated structure to string, due to {}",
                            error))
            }
        }
    }
}

impl<'a> GetterAPI for UserClient<'a> {
    type GetType = UserInfoStructure;
    /// Returns a Result type that contains a UserInfoStructure structure, or a String error.
    ///
    /// # Example
    ///
    /// `
    /// let github_client = GithubClient::new("k0pernicus", "myapikey0123456789");
    /// let user_client = github_client.get_myself_client();
    /// let user_infos = user_client.unwrap();
    /// `
    fn get(&self) -> Result<UserInfoStructure, String> {
        /// Check if the current user is the same for this client
        let url = if self.username == self.github_client.username {
            USER_API_URL.to_string()
        } else {
            format!("{}/{}", USERS_API_URL, self.username)
        };
        match self.github_client
            .process_request(Method::Get, &url, None) {
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
#[derive(Debug, Serialize, Deserialize)]
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
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hireable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}
