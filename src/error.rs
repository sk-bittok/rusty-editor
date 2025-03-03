#[derive(Debug, Clone)]
pub enum Error {
    DialogueClosed,
    Generic(String),
    IO(std::io::ErrorKind),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DialogueClosed => "dialogue closed".to_string(),
                Self::Generic(err) => err.to_string(),
                Self::IO(err) => err.to_string(),
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err.kind())
    }
}
