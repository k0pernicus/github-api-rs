#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

pub mod client;
pub mod user;

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
        let user_client = UserClient::new(&github_client, "k0pernicus");
        match user_client.get() {
            Ok(value) => println!("{:?}", value),
            Err(error) => println!("{:?}", error),
        }
    }
}
