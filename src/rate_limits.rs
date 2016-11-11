use client::GithubClient;

use GetterAPI;

use hyper::method::Method;

use serde_json;

const RATELIMITS_API_URL: &'static str = "rate_limit";

pub struct RateLimits<'a> {
    github_client: &'a GithubClient,
}

impl<'a> RateLimits<'a> {
    pub fn new(github_client: &'a GithubClient) -> Self {
        RateLimits { github_client: github_client }
    }
}

impl<'a> GetterAPI for RateLimits<'a> {
    type GetType = Limits;
    fn get(&self) -> Result<Limits, String> {
        match self.github_client.process_request(Method::Get, RATELIMITS_API_URL, None) {
            Ok(response) => {
                match serde_json::from_str(&response) {
                    Ok(rate_limits) => Ok(rate_limits),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Rate {
    #[serde(skip_serializing_if="Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    remaining: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    reset: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesLimit {
    #[serde(skip_serializing_if="Option::is_none")]
    core: Option<Rate>,
    #[serde(skip_serializing_if="Option::is_none")]
    graphql: Option<Rate>,
    #[serde(skip_serializing_if="Option::is_none")]
    search: Option<Rate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limits {
    #[serde(skip_serializing_if="Option::is_none")]
    resources: Option<ResourcesLimit>,
    #[serde(skip_serializing_if="Option::is_none")]
    rate: Option<Rate>,
}