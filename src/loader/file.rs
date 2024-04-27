use crate::error::Error;
use crate::loader::error::Error as LoadingError;
use crate::locale::Locale;

use std::{collections::HashMap, path::Path};

/// A helper function for loading catalogues from directory.
///
/// You won't need this function unless you are creating your own file loader.
///
/// # Examples
///
/// ```
/// use tarjama::loader::file;
/// use tarjama::locale::Locale;
/// use tarjama::locale::EnglishVariant;
/// use tarjama::locale::FrenchVariant;
///
/// use std::collections::HashMap;
///
/// # async fn doc() {
/// let result = file::iterate("examples/translations", &vec!["toml".to_string()])
///     .await
///     .unwrap();
///
/// assert_eq!(
///     result,
///     HashMap::from([
///         (
///             Locale::English(EnglishVariant::Default),
///             HashMap::from([
///                 (
///                     "foo".to_string(),
///                     Vec::from(["examples/translations/foo.en.toml".to_string()])
///                 ),
///                 (
///                     "messages".to_string(),
///                     Vec::from(["examples/translations/messages.en.toml".to_string()])
///                 )
///             ])
///         ),
///         (
///             Locale::French(FrenchVariant::Default),
///             HashMap::from([
///                 (
///                     "foo".to_string(),
///                     Vec::from(["examples/translations/foo.fr.toml".to_string()])
///                 ),
///                 (
///                     "messages".to_string(),
///                     Vec::from(["examples/translations/messages.fr.toml".to_string()])
///                 )
///             ])
///         ),
///     ])
/// );
/// # }
/// ```
#[cfg(feature = "async")]
pub async fn iterate<T>(
    path: T,
    extensions: &[String],
) -> Result<HashMap<Locale, HashMap<String, Vec<String>>>, Error>
where
    T: AsRef<Path> + 'static,
{
    use tokio::fs;

    let path: &Path = path.as_ref();
    let mut entries = fs::read_dir(path).await.map_err(|e| {
        Error::LoadingError(LoadingError::FailedToReadDirectory(e))
    })?;

    let mut result: HashMap<Locale, HashMap<String, Vec<String>>> =
        HashMap::new();

    while let Some(entry) = entries.next_entry().await.map_err(|e| {
        Error::LoadingError(LoadingError::FailedToReadDirectory(e))
    })? {
        let entry_path_buf = entry.path();
        if !entry_path_buf.is_file() {
            continue;
        }
        if let Some(ext) = entry_path_buf.extension().and_then(|x| x.to_str())
        {
            if !extensions.contains(&ext.to_string()) {
                continue;
            }

            if let Some(stem) = entry_path_buf.file_stem().unwrap().to_str() {
                if let Some(pos) = stem.rfind('.') {
                    let domain = stem[..pos].to_string();
                    let locale_name = &stem[pos + 1..];
                    let locale: Locale = locale_name.try_into().map_err(|_| {
                        Error::LoadingError(LoadingError::InvalidFilenameFormat(format!(
                            "invalid filename: locale, expected valid locale code, found `{locale_name}` in `{}.{}`.",
                            stem,
                            ext
                        )))
                    })?;

                    let map = if !result.contains_key(&locale) {
                        result.insert(locale.clone(), HashMap::new());

                        result.get_mut(&locale).unwrap()
                    } else {
                        result.get_mut(&locale).unwrap()
                    };
                    if !map.contains_key(&domain) {
                        map.insert(domain.clone(), vec![]);
                    }

                    map.get_mut(&domain)
                        .unwrap()
                        .push(entry_path_buf.to_str().unwrap().to_string());
                } else {
                    return Err(Error::LoadingError(
                        LoadingError::InvalidFilenameFormat(format!("invalid filename: format, expected `{{domain}}.{{locale}}.{{ext}}` for `{}.{}`.", stem, ext))
                    ));
                }
            }
        }
    }

    Ok(result)
}

pub fn iterate_sync<T>(
    path: T,
    extensions: &[String],
) -> Result<HashMap<Locale, HashMap<String, Vec<String>>>, Error>
where
    T: AsRef<Path>,
{
    use std::fs;

    let path: &Path = path.as_ref();
    // Use std::fs to read the directory synchronously
    let entries = fs::read_dir(path).map_err(|e| {
        Error::LoadingError(LoadingError::FailedToReadDirectory(e))
    })?;

    let mut result: HashMap<Locale, HashMap<String, Vec<String>>> =
        HashMap::new();

    for entry in entries {
        let entry = entry.map_err(|e| {
            Error::LoadingError(LoadingError::FailedToReadDirectory(e))
        })?;

        let entry_path_buf = entry.path();
        if !entry_path_buf.is_file() {
            continue;
        }
        if let Some(ext) = entry_path_buf.extension().and_then(|x| x.to_str())
        {
            if !extensions.contains(&ext.to_string()) {
                continue;
            }

            if let Some(stem) = entry_path_buf.file_stem().unwrap().to_str() {
                if let Some(pos) = stem.rfind('.') {
                    let domain = stem[..pos].to_string();
                    let locale_name = &stem[pos + 1..];
                    let locale: Locale = locale_name.try_into().map_err(|_| {
                        Error::LoadingError(LoadingError::InvalidFilenameFormat(format!(
                            "invalid filename: locale, expected valid locale code, found `{locale_name}` in `{}.{}`.",
                            stem,
                            ext
                        )))
                    })?;

                    let map = if !result.contains_key(&locale) {
                        result.insert(locale.clone(), HashMap::new());

                        result.get_mut(&locale).unwrap()
                    } else {
                        result.get_mut(&locale).unwrap()
                    };
                    if !map.contains_key(&domain) {
                        map.insert(domain.clone(), vec![]);
                    }

                    map.get_mut(&domain)
                        .unwrap()
                        .push(entry_path_buf.to_str().unwrap().to_string());
                } else {
                    return Err(Error::LoadingError(
                        LoadingError::InvalidFilenameFormat(format!("invalid filename: format, expected `{{domain}}.{{locale}}.{{ext}}` but got `{}.{}`.", stem, ext))
                    ));
                }
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::loader::file;
    use crate::locale::ArabicVariant;
    use crate::locale::ChineseVariant;
    use crate::locale::EnglishVariant;
    use crate::locale::FrenchVariant;
    use crate::locale::Locale;

    use std::collections::HashMap;

    #[tokio::test]
    async fn iterate_test() {
        let result =
            file::iterate("examples/translations", &["toml".to_string()])
                .await
                .unwrap();

        assert_eq!(
            result,
            HashMap::from([
                (
                    Locale::English(EnglishVariant::Default),
                    HashMap::from([
                        (
                            "foo".to_string(),
                            Vec::from(["examples/translations/foo.en.toml"
                                .to_string()])
                        ),
                        (
                            "messages".to_string(),
                            Vec::from([
                                "examples/translations/messages.en.toml"
                                    .to_string()
                            ])
                        )
                    ])
                ),
                (
                    Locale::French(FrenchVariant::Default),
                    HashMap::from([
                        (
                            "foo".to_string(),
                            Vec::from(["examples/translations/foo.fr.toml"
                                .to_string()])
                        ),
                        (
                            "messages".to_string(),
                            Vec::from([
                                "examples/translations/messages.fr.toml"
                                    .to_string()
                            ])
                        )
                    ])
                ),
                (
                    Locale::Chinese(ChineseVariant::Default),
                    HashMap::from([(
                        "messages".to_string(),
                        Vec::from(["examples/translations/messages.zh.toml"
                            .to_string()])
                    )])
                ),
                (
                    Locale::Arabic(ArabicVariant::Default),
                    HashMap::from([(
                        "messages".to_string(),
                        Vec::from(["examples/translations/messages.ar.toml"
                            .to_string()])
                    )])
                )
            ])
        );
    }

    #[tokio::test]
    async fn iterate_sync_test() {
        let result =
            file::iterate_sync("examples/translations", &["toml".to_string()])
                .unwrap();

        assert_eq!(
            result,
            HashMap::from([
                (
                    Locale::English(EnglishVariant::Default),
                    HashMap::from([
                        (
                            "foo".to_string(),
                            Vec::from(["examples/translations/foo.en.toml"
                                .to_string()])
                        ),
                        (
                            "messages".to_string(),
                            Vec::from([
                                "examples/translations/messages.en.toml"
                                    .to_string()
                            ])
                        )
                    ])
                ),
                (
                    Locale::French(FrenchVariant::Default),
                    HashMap::from([
                        (
                            "foo".to_string(),
                            Vec::from(["examples/translations/foo.fr.toml"
                                .to_string()])
                        ),
                        (
                            "messages".to_string(),
                            Vec::from([
                                "examples/translations/messages.fr.toml"
                                    .to_string()
                            ])
                        )
                    ])
                ),
                (
                    Locale::Chinese(ChineseVariant::Default),
                    HashMap::from([(
                        "messages".to_string(),
                        Vec::from(["examples/translations/messages.zh.toml"
                            .to_string()])
                    )])
                ),
                (
                    Locale::Arabic(ArabicVariant::Default),
                    HashMap::from([(
                        "messages".to_string(),
                        Vec::from(["examples/translations/messages.ar.toml"
                            .to_string()])
                    )])
                )
            ])
        );
    }
}
