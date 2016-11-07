#![feature(custom_derive, custom_attribute, proc_macro)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod user;

// Custom headers
header! { (XRateLimitLimit, "X-RateLimit-Limit") => [usize] }
header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [usize] }

#[cfg(test)]
mod tests {
    use client::GithubClient;
    use std::env;
    use user::UserUpdateStructure;

    const DEFAULT_API_KEY: &'static str = "HelloWorld12345";

    #[test]
    fn test_client() {
        let github_client = GithubClient::new("k0pernicus", DEFAULT_API_KEY);
        assert!(github_client.username == "k0pernicus".to_string());
        assert!(github_client.api_key == DEFAULT_API_KEY.to_string());
    }

    #[test]
    fn test_user() {
        let api_key = "GITHUB_API_RS";
        let GITHUB_API_KEY = match env::var(api_key) {
            Ok(val) => val,
            Err(e) => DEFAULT_API_KEY.to_string(),
        };
        let github_client = GithubClient::new("k0pernicus", &GITHUB_API_KEY);
        let myself = github_client.get_myself_client();
        match myself.get() {
            Ok(value) => println!("SUCCESS: {:?}", value),
            Err(error) => println!("ERROR: {:?}", error),
        }
    }

    // #[test]
    // fn modify_user() {
    //     let key = "GITHUB_API_RS";
    //     let GITHUB_API_KEY = match env::var(key) {
    //         Ok(val) => val,
    //         Err(e) => DEFAULT_API_KEY.to_string(),
    //     };
    //     let github_client = GithubClient::new("k0pernicus", &GITHUB_API_KEY);
    //     let myself_client = github_client.get_myself_client();
    //     match myself_client.get() {
    //         Ok(value) => println!("GET SUCCESS: {:?}", value),
    //         Err(error) => println!("GET ERROR: {:?}", error),
    //     }
    //     let new_infos = UserUpdateStructure {
    //         name: None,
    //         email: None,
    //         blog: None,
    //         company: Some(String::from("DernierCri")),
    //         location: Some(String::from("Lille, France")),
    //         hireable: Some(false),
    //         bio: None,
    //     };
    //     match myself_client.update(&new_infos) {
    //         Ok(value) => println!("UPDATE SUCCESS: {:?}", value),
    //         Err(error) => println!("UPDATE ERROR: {:?}", error),
    //     }
    // }

}
