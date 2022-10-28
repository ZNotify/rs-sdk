#[cfg(not(mock))]
pub const ENDPOINT: &str = "https://push.learningman.top";


#[cfg(mock)]
pub const ENDPOINT: &str = "http://localhost:14444";