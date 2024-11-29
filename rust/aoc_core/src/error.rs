use std::fmt;

#[derive(Debug)]
struct Error {
    message: Option<String>,
}

impl Error {
    pub fn blank() -> Self {
        Self { message: None }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(message) = &self.message {
            f.write_str(message)
        } else {
            Ok(())
        }
    }
}

impl<S: AsRef<str>> From<S> for Error {
    fn from(s: S) -> Self {
        Self {
            message: Some(String::from(s.as_ref())),
        }
    }
}

pub fn err<S: AsRef<str>>(message: S) -> anyhow::Error {
    Error::from(message).into()
}

pub fn err_blank() -> anyhow::Error {
    Error::blank().into()
}
