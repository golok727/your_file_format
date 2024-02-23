pub mod crab_error {
    use std::fmt;
    #[derive(Debug)]
    pub enum Error {
        IOError(String),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::IOError(msg) => write!(f, "Error Io: {}", msg),
            }
        }
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Self::IOError(format!("{}", value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_error() {
        let error = crab_error::Error::IOError("Something is not right".to_owned());

        assert_eq!(
            error.to_string(),
            "Error reading from file: Something is not right"
        )
    }
    #[test]
    fn convert() {
        let std_error = std::io::Error::from(std::io::ErrorKind::NotFound);

        let converted_error: crab_error::Error = std_error.into();

        assert_eq!(converted_error.to_string(), "Error Io: entity not found");
    }
}
