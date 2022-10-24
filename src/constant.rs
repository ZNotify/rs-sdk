#[cfg(not(feature = "mock"))]
pub const ENDPOINT: &str = "https://push.learningman.top";


#[cfg(feature = "mock")]
pub const ENDPOINT: &str = "http://localhost:14444";