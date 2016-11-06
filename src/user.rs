use client::GithubClient;
use hyper::method::Method;

/// URL to access the Github API for User
const USER_API: &'static str = "users";

/// A client to communicate with the Github API for Users.
/// You can create as much as UserClient structure as users you're looking for.
pub struct UserClient<'a> {
    /// The Github client
    github_client: &'a GithubClient,
    /// The username to build requests
    username: String,
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

    /// Returns a String that represents the body of the request for a user, or an error
    pub fn get(&self) -> Result<String, String> {
        self.github_client.process_request(Method::Get, &format!("{}/{}", USER_API, self.username))
    }
}

/// Interesting fields that represent globally a User on Github
#[derive(Serialize, Deserialize)]
struct UserStructure {
    login: String,
    id: usize,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    site_admin: bool,
    name: String,
    company: String,
    location: String,
    email: String,
    hireable: bool,
    bio: String,
    public_repos: usize,
    public_gists: usize,
    followers: usize,
    following: usize,
}
