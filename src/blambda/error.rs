use super::parse::Rule;

#[derive(Debug)]
pub struct BlambdaError {
    message: String,
}

impl Into<BlambdaError> for std::io::Error {
    fn into(self) -> BlambdaError {
        BlambdaError {
            message: format!("{}", self),
        }
    }
}

impl Into<BlambdaError> for pest::error::Error<Rule> {
    fn into(self) -> BlambdaError {
        BlambdaError {
            message: format!("{}", self),
        }
    }
}

impl std::fmt::Display for BlambdaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlambdaError: {}", self.message)
    }
}

impl std::error::Error for BlambdaError {
    fn description(&self) -> &str {
        &self.message
    }
}
