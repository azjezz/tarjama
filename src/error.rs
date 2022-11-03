use crate::loader::error::Error as LoadingError;
use crate::locale::Locale;

#[derive(Debug)]
pub enum Error {
    MessageNotFound(Locale, String, String),
    InvalidLocale(String),
    FormattingError(String),
    LoadingError(LoadingError),
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::MessageNotFound(locale, domain, message) => write!(f, "message not found: message `'{message}'` could not be found in `'{domain}'` domain for `'{locale}'` locale."),
            Error::InvalidLocale(locale) => write!(f, "locale: invalid locale, expected a valid locale code but found `'{locale}'`."),
            Error::FormattingError(inner) => write!(f, "{inner}"),
            Error::LoadingError(inner) => write!(f, "{inner}"),
        }
    }
}

impl ::std::error::Error for Error {}
