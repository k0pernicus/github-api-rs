#![feature(custom_derive, custom_attribute, proc_macro)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod user;
pub mod rate_limits;
pub mod repo;

// Custom headers
header! { (XRateLimitLimit, "X-RateLimit-Limit") => [usize] }
header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [usize] }

trait GetterAPI {
    type GetType: std::fmt::Debug + serde::Serialize + serde::Deserialize;
    fn get(&self) -> Result<Self::GetType, String>;
}

#[cfg(test)]
mod tests {
    use client::GithubClient;
    use GetterAPI;
    use rate_limits::RateLimits;
    use repo::RepoClient;
    use std::env;
    use user::UserUpdateStructure;

    const DEFAULT_API_KEY: &'static str = "HelloWorld12345";
    const DEFAULT_GITHUB_PROFILE: &'static str = "username";

    #[test]
    fn test_client() {
        let github_client = GithubClient::new(DEFAULT_GITHUB_PROFILE, DEFAULT_API_KEY);
        assert!(github_client.username == DEFAULT_GITHUB_PROFILE.to_string());
        assert!(github_client.api_key == DEFAULT_API_KEY.to_string());
    }

    #[test]
    fn test_user() {
        let api_key = "GITHUB_API_RS";
        let github_api_key = match env::var(api_key) {
            Ok(val) => val,
            Err(_) => DEFAULT_API_KEY.to_string(),
        };
        let github_client = GithubClient::new(DEFAULT_GITHUB_PROFILE, &github_api_key);
        let myself = github_client.get_myself_client();
        match myself.get() {
            Ok(value) => println!("[test_user] GET SUCCESS: {:?}", value),
            Err(error) => println!("[test_user] GET ERROR: {:?}", error),
        }
    }

    #[test]
    fn test_repo() {
        let api_key = "GITHUB_API_RS";
        let github_api_key = match env::var(api_key) {
            Ok(val) => val,
            Err(_) => DEFAULT_API_KEY.to_string(),
        };
        let github_client = GithubClient::new(DEFAULT_GITHUB_PROFILE, &github_api_key);
        let current_repo_api = RepoClient::new(&github_client, "k0pernicus", "github-api-rs");
        match current_repo_api.get() {
            Ok(value) => println!("[test_repo] GET SUCCESS: {:?}", value),
            Err(error) => println!("[test_repo] GET ERROR: {:?}", error),
        }
    }

    #[test]
    fn test_rate_limits() {
        let api_key = "GITHUB_API_RS";
        let github_api_key = match env::var(api_key) {
            Ok(val) => val,
            Err(_) => DEFAULT_API_KEY.to_string(),
        };
        let github_client = GithubClient::new(DEFAULT_GITHUB_PROFILE, &github_api_key);
        let current_limits = RateLimits::new(&github_client);
        match current_limits.get() {
            Ok(value) => println!("[test_rate_limits] GET SUCCESS: {:?}", value),
            Err(error) => println!("[test_rate_limits] GET ERROR: {:?}", error),
        }
    }

    #[test]
    fn modify_user() {
        let key = "GITHUB_API_RS";
        let github_api_key = match env::var(key) {
            Ok(val) => val,
            Err(_) => DEFAULT_API_KEY.to_string(),
        };
        let github_client = GithubClient::new(DEFAULT_GITHUB_PROFILE, &github_api_key);
        let myself_client = github_client.get_myself_client();
        match myself_client.get() {
            Ok(value) => println!("[modify_user] GET SUCCESS: {:?}", value),
            Err(error) => println!("[modify_user] GET ERROR: {:?}", error),
        }
        let new_infos = UserUpdateStructure {
            name: None,
            email: None,
            blog: None,
            company: Some(String::from("DernierCri")),
            location: Some(String::from("Lille, France")),
            hireable: Some(false),
            bio: None,
        };
        match myself_client.update(&new_infos) {
            Ok(value) => println!("[modify_user] UPDATE SUCCESS: {:?}", value),
            Err(error) => println!("[modify_user] UPDATE ERROR: {:?}", error),
        }
    }

}
