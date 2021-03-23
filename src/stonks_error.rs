use std::fmt;
use std::error::Error;

// Error strategy
#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(err: std::io::Error) -> RuntimeError {
        eprintln!("{:?}", err);
        RuntimeError { message: "io error".to_string() }
    }
}

impl From<hyper::Error> for RuntimeError {
    fn from(err: hyper::Error) -> RuntimeError {
        eprintln!("{:?}", err);
        RuntimeError { message: "hyper request error".to_string() }
    }
}

impl From<hyper::http::Error> for RuntimeError {
    fn from(err: hyper::http::Error) -> RuntimeError {
        eprintln!("{:?}", err);
        RuntimeError { message: "hyper http error".to_string() }
    }
}

impl From<serde_yaml::Error> for RuntimeError {
    fn from(err: serde_yaml::Error) -> RuntimeError {
        eprintln!("{:?}", err);
        RuntimeError { message: "Yaml error".to_string() }
    }
}
