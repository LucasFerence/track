use std::error::Error;

pub type ResErr = Box<dyn Error>;
pub type Res<T> = Result<T, Box<dyn Error>>;