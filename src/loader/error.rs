#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "file")]
    FailedToReadDirectory(::std::io::Error),

    #[cfg(feature = "file")]
    FailedToReadFile(String, ::std::io::Error),

    #[cfg(feature = "file")]
    InvalidFilenameFormat(String),

    #[cfg(feature = "toml")]
    FailedToParseFile(::toml::de::Error),

    Custom(String),
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            #[cfg(feature = "file")]
            Error::FailedToReadDirectory(inner) => write!(
                f,
                "unreadable node: directory, {}.",
                inner.to_string().to_lowercase()
            ),
            #[cfg(feature = "file")]
            Error::FailedToReadFile(node, inner) => write!(
                f,
                "unreadable node: file `{node}`, {}.",
                inner.to_string().to_lowercase(),
            ),
            #[cfg(feature = "file")]
            Error::FailedToParseFile(inner) => write!(f, "{inner}."),
            #[cfg(feature = "toml")]
            Error::InvalidFilenameFormat(inner) => write!(f, "{inner}"),
            Error::Custom(inner) => write!(f, "{inner}"),
        }
    }
}

impl ::std::error::Error for Error {}
