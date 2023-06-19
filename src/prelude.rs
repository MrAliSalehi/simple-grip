use std::error::Error;

pub type MyResult = Result<(), Box<dyn Error + Send + Sync>>;
