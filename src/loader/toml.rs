use crate::catalogue::Catalogue;
use crate::catalogue::CatalogueBag;
use crate::error::Error;
use crate::loader::error::Error as LoadingError;
use crate::loader::file::iterate;

use futures_util::future::join_all;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use toml;

/// Load a catalogue bag from a directory containing toml files.
///
/// Files within the directory should be named in the following format:  `{domain}.{locale}.toml`.
///
/// # Examples
///
/// ```
/// # async fn doc() {
/// use tarjama::loader::toml::load;
///
/// let catalogue_bag = load("examples/translations").await.expect("Failed to load catalogue bag");
/// # }
/// ```
pub async fn load<T>(directory: T) -> Result<CatalogueBag, Error>
where
    T: AsRef<Path> + 'static,
{
    let data = iterate(directory, &["toml".to_string()]).await?;

    let mut bag = CatalogueBag::new();
    for (locale, domain_files) in data {
        let mut catalogue = Catalogue::new(locale);
        for (domain, message_files) in domain_files {
            let message_groups =
                join_all(message_files.iter().map(|path| async move {
                    fs::read_to_string(path)
                        .await
                        .map_err(|e| -> Error {
                            Error::LoadingError(
                                LoadingError::FailedToReadFile(
                                    path.to_string(),
                                    e,
                                ),
                            )
                        })
                        .and_then(|content| {
                            toml::from_str::<HashMap<String, String>>(&content)
                                .map_err(|e| -> Error {
                                    Error::LoadingError(
                                        LoadingError::FailedToParseFile(e),
                                    )
                                })
                        })
                }))
                .await;

            for messages in message_groups {
                for (id, message) in messages? {
                    catalogue.insert(&domain, &id, &message);
                }
            }
        }

        bag.insert(catalogue);
    }

    Ok(bag)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::locale::EnglishVariant;
    use crate::locale::FrenchVariant;
    use crate::locale::Locale;
    use tokio;

    #[tokio::test]
    async fn load_test() {
        let bag = load("examples/translations").await.unwrap();

        let catalogue = bag.get(&Locale::French(FrenchVariant::Default))[0];
        assert_eq!(
            catalogue.get("messages", "greeting").unwrap(),
            "Bonjour, {name}!"
        );
        assert_eq!(
            catalogue.get("foo", "foo").unwrap(),
            "foo! {foo} {} {bar} {} {baz} 🥐"
        );
        assert_eq!(
            catalogue.get("foo", "bar").unwrap(),
            "bar! {} {foo} {} {bar} {} 🥐"
        );

        let catalogue = bag.get(&Locale::English(EnglishVariant::Default))[0];
        assert_eq!(
            catalogue.get("messages", "greeting").unwrap(),
            "Hello, {name}!"
        );
        assert_eq!(
            catalogue.get("foo", "foo").unwrap(),
            "foo! {foo} {} {bar} {} {baz} 🍔"
        );
        assert_eq!(
            catalogue.get("foo", "bar").unwrap(),
            "bar! {} {foo} {} {bar} {} 🍔"
        );
    }

    macro_rules! test_loading_error {
        ($filename:literal, $error:literal) => {
            let error = load($filename).await.unwrap_err();

            assert_eq!(error.to_string(), $error);
        };
    }

    #[tokio::test]
    async fn load_error_test() {
        test_loading_error!(
            "examples/translations/invalid/parse",
            "invalid type: sequence, expected a string for key `foo` at line 1 column 1."
        );

        test_loading_error!(
            "examples/translations/invalid/unreadable-file",
            "unreadable node: file `examples/translations/invalid/unreadable-file/messages.en.toml`, permission denied (os error 13)."
        );

        test_loading_error!(
            "examples/translations/invalid/unreadable-dir",
            "unreadable node: directory, permission denied (os error 13)."
        );

        test_loading_error!(
            "examples/translations/invalid/filename/format",
            "invalid filename: format, expected `{domain}.{locale}.{ext}` for `filename.toml`."
        );

        test_loading_error!(
            "examples/translations/invalid/filename/locale",
            "invalid filename: locale, expected valid locale code, found `xx` in `messages.xx.toml`."
        );
    }
}
