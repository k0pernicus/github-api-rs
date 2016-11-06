use hyper::Client;
use hyper::client::response::Response;
use hyper::Error;
use hyper::header::UserAgent;
use hyper::method::Method;
use hyper::status::StatusCode;

use serde_json;

use std::io::Read;

use user::UserClient;

/// URL to access the Github API
const GITHUB_API_URL: &'static str = "https://api.github.com";
const USER_AGENT: &'static str = "[Github API] k0pernicus/github-api-rs";

/// A client to communicate with the Github API is represented here
pub struct GithubClient {
    /// The username of the user
    pub username: String,
    /// The personal API key of the user
    pub api_key: String,
    /// The Hyper client
    client: Client,
}

/// A private structure to get and process errors from Github
#[derive(Debug, Deserialize)]
struct GitHubErrorResult {
    /// The global error message
    message: String,
    /// All detected errors
    errors: Vec<GitHubErrorPart>,
}

/// A private structure to parse an error from Github
#[derive(Debug, Deserialize)]
struct GitHubErrorPart {
    /// The field that contains the error
    field: String,
    /// The code that contains the error
    code: String,
    /// The message to understand the error
    message: String,
}

impl GithubClient {
    /// Returns a Github client to communicate with the Github API
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice that holds the username of the user
    /// * `api_key` - A string slice that holds the Github API key
    ///
    /// # Example
    ///
    /// `let github_client = GithubClient::new("k0pernicus", "myapikey0123456789")`
    pub fn new(username: &str, api_key: &str) -> Self {
        GithubClient {
            username: username.to_owned(),
            api_key: api_key.to_owned(),
            client: Client::new(),
        }
    }

    /// Returns a User client to communicate with the Github API about a User
    ///
    /// # Argument
    ///
    /// * `username` - A string slice that holds the username of a Github user
    ///
    /// # Example
    ///
    /// `let user_client = GithubClient::get_user("k0pernicus")`
    pub fn get_user_client<'a>(&'a self, username: &'a str) -> UserClient {
        UserClient::new(&self, username)
    }

    /// Returns a User client that corresponding to the current user
    pub fn get_myself_client<'a>(&'a self) -> UserClient {
        UserClient::new(&self, &self.username)
    }

    /// Process a request, using an HTTP/HTTPS request method and a URL.
    /// This method will send a request from Hyper, and check/process the response from this one.
    ///
    /// # Arguments
    ///
    /// * `method` - An HTTP/HTTPS request method
    /// * `url` - A string slice that represent the URL to send the request
    ///
    /// # Example
    ///
    /// `
    /// match process_request(Method::Get, "https://api.github.com/users/k0pernicus") {
    ///     Ok(body) => println!("The response body is {:?}", body),
    ///     Err(error) => println!("Oops, an error has occured: {:?}", error)
    /// }
    /// `
    pub fn process_request(&self, http_method: Method, url: &str) -> Result<String, String> {
        match self.send_request(http_method, url) {
            Ok(mut value) => self.get_result_from_request(&mut value),
            Err(error) => Err(format!("Error processing the request: {}", error)),
        }
    }

    /// Send an HTTP/HTTPS request over Hyper, to a given URL.
    /// This method returns a Response (a given message from the server) or an Error.
    ///
    /// # Arguments
    ///
    /// * `method` - An HTTP/HTTPS request method
    /// * `url` - The URL to send the request
    fn send_request(&self, method: Method, url: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", GITHUB_API_URL, url);
        self.client
            .request(method, &url)
            // TODO: Add Authorization
            .header(UserAgent(USER_AGENT.to_owned()))
            .send()
    }

    /// Return a Result type that contains the body of the request response, or an error.
    ///
    /// # Argument
    ///
    /// * `response` - A reference from a mutable request Response type
    fn get_result_from_request(&self, response: &mut Response) -> Result<String, String> {
        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Ok(_) => {}
            Err(error) => return Err(format!("Error processing the response request: {}", error)),
        }
        if response.status.class().default_code() == StatusCode::Ok {
            Ok(body)
        } else {
            match serde_json::from_str::<GitHubErrorResult>(&body) {
                Ok(error) => Err(error.message),
                Err(error) => {
                    Err(format!("Error processing Github error: {}\nBody: {}", error, body))
                }
            }
        }
    }
}
