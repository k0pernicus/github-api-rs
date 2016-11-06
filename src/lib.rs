#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate serde_json;

pub mod client;
pub mod user;

// Custom headers
header! { (XRateLimitLimit, "X-RateLimit-Limit") => [usize] }
header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [usize] }

#[cfg(test)]
mod tests {
    use super::*;
    use client::GithubClient;
    use user::UserClient;

    const DEFAULT_API_KEY: &'static str = "HelloWorld12345";

    #[test]
    fn test_client() {
        let github_client = GithubClient::new("k0pernicus", DEFAULT_API_KEY);
        assert!(github_client.username == "k0pernicus".to_string());
        assert!(github_client.api_key == DEFAULT_API_KEY.to_string());
    }

    #[test]
    fn test_user() {
        let github_client = GithubClient::new("k0pernicus", DEFAULT_API_KEY);
        let myself_client = github_client.get_myself_client();
        let another_user_client = github_client.get_user_client("k0pernicus");
        assert!(myself_client.username == another_user_client.username);
        match myself_client.get() {
            Ok(value) => println!("{:?}", value),
            Err(error) => println!("{:?}", error),
        }
    }
}
