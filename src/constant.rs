#[cfg(not(mock))]
pub const ENDPOINT: &str = "https://push.learningman.top";

#[cfg(mock)]
pub const ENDPOINT: &str = "http://127.0.0.1:14444";
