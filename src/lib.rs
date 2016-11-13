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

/// A trait for structures that send a GET request
pub trait GetterAPI {
    type GetType: std::fmt::Debug + serde::Serialize + serde::Deserialize;
    fn get(&self) -> Result<Self::GetType, String>;
}

/// A trait for structures that send a PATCH request
pub trait UpdaterAPI {
    type PatchType: std::fmt::Debug + serde::Serialize + serde::Deserialize;
    fn patch(&self, &Self::PatchType) -> Result<String, String>;
}