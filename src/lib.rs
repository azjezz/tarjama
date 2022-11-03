pub mod catalogue;
pub mod context;
pub mod error;
pub mod formatter;
pub mod loader;
pub mod locale;
pub mod macros;

use std::fmt::Display;

use crate::catalogue::CatalogueBag;
use crate::context::Context;
use crate::error::Error;
use crate::formatter::Formatter;
use crate::locale::Locale;

/// Translator
///
/// # Example:
///
/// ```
/// use tarjama::error::Error;
/// use tarjama::locale::Locale;
/// use tarjama::locale::EnglishVariant;
/// use tarjama::Translator;
/// use tarjama::catalogue::Catalogue;
/// use tarjama::catalogue::CatalogueBag;
/// use tarjama::context;
///
/// use std::collections::HashMap;
///
/// let translator = Translator::with_catalogue_bag(CatalogueBag::with_catalogues(vec![
///     Catalogue::with_messages(Locale::English(EnglishVariant::Default), HashMap::from([
///         ("messages".to_owned(), HashMap::from([
///           ("greeting".to_owned(), "Hello, {name}!".to_owned()),
///         ]))
///     ]))
/// ]));
///
/// let message = translator.trans("en", "messages", "greeting", context!(name = "World")).unwrap();
///
/// assert_eq!(message, "Hello, World!".to_string());
/// ```
pub struct Translator {
    formatter: Box<dyn Formatter>,
    bag: CatalogueBag,
    fallback_locale: Option<Locale>,
}

unsafe impl Send for Translator {}
unsafe impl Sync for Translator {}

impl Translator {
    pub fn new(
        formatter: Box<dyn Formatter>,
        bag: CatalogueBag,
        fallback_locale: Option<Locale>,
    ) -> Self {
        Self { formatter, bag, fallback_locale }
    }

    pub fn with_catalogue_bag(bag: CatalogueBag) -> Self {
        Self { formatter: Default::default(), bag, fallback_locale: None }
    }

    /// Set the fallback locale.
    ///
    /// A fallback locale will be used for translation if the message is not found using the given locale.
    ///
    /// # Example
    ///
    /// ```
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::catalogue::CatalogueBag;
    /// use tarjama::Translator;
    /// use tarjama::context;
    ///
    /// use std::collections::HashMap;
    ///
    /// let mut translator = Translator::with_catalogue_bag(CatalogueBag::with_catalogues(vec![
    ///     Catalogue::with_messages(Locale::English(EnglishVariant::Default), HashMap::from([
    ///         ("messages".to_owned(), HashMap::from([
    ///           ("greeting".to_owned(), "Hello, {name}!".to_owned()),
    ///         ]))
    ///     ]))
    /// ]));
    ///
    /// let result = translator.trans("fr", "messages", "greeting", context!(name = "World"));
    /// assert!(result.is_err());
    ///
    /// // set locale fallback to "en"
    /// translator.set_fallback_locale(Locale::English(EnglishVariant::Default));
    ///
    /// let result = translator.trans("fr", "messages", "greeting", context!(name = "World"));
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap(), "Hello, World!");
    ///
    /// // remove locale fallback
    /// translator.set_fallback_locale(None);
    ///
    /// let result = translator.trans("fr", "messages", "greeting", context!(name = "World"));
    /// assert!(result.is_err());
    /// ```
    ///
    pub fn set_fallback_locale<T>(&mut self, fallback_locale: T)
    where
        T: Into<Option<Locale>>,
    {
        self.fallback_locale = fallback_locale.into();
    }

    /// Translate a message.
    ///
    /// When the `count` field of `Context` is `Some(i)`, the message is parsed for plural forms, and
    /// a translation is chosen according to `i`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::catalogue::CatalogueBag;
    /// use tarjama::Translator;
    /// use tarjama::context;
    ///
    /// use std::collections::HashMap;
    ///
    /// let translator = Translator::with_catalogue_bag(CatalogueBag::with_catalogues(vec![
    ///     Catalogue::with_messages(Locale::English(EnglishVariant::Default), HashMap::from([
    ///         ("messages".to_owned(), HashMap::from([
    ///           ("apple".to_owned(), "{0} There are no apples | {1} There is one apple | There are {?} apples".to_owned()),
    ///         ]))
    ///     ]))
    /// ]));
    ///
    /// let message = translator.trans("en", "messages", "apple", context!(? = 4));
    /// assert_eq!(message.unwrap(), "There are 4 apples".to_string());
    /// ```
    pub fn trans<T, C>(
        &self,
        locale: T,
        domain: &str,
        id: &str,
        context: C,
    ) -> Result<String, Error>
    where
        T: TryInto<Locale> + Display,
        C: Into<Context>,
    {
        let context = context.into();
        let locale_string = locale.to_string();
        let locale = locale
            .try_into()
            .map_err(|_| Error::InvalidLocale(locale_string))?;
        let catalogues = self.bag.get(&locale);
        let mut message = None;
        for catalogue in catalogues.iter() {
            if let Some(msg) = catalogue.get(domain, id) {
                message = Some(msg);

                break;
            }
        }

        if let Some(message) = message {
            self.formatter.format(&locale, message, &context)
        } else {
            // fallback
            if let Some(fallback) = &self.fallback_locale {
                let catalogues = self.bag.get(fallback);
                for catalogue in catalogues.iter() {
                    if let Some(msg) = catalogue.get(domain, id) {
                        message = Some(msg);

                        break;
                    }
                }

                if let Some(message) = message {
                    return self.formatter.format(fallback, message, &context);
                }
            }

            Err(Error::MessageNotFound(
                locale,
                domain.to_string(),
                id.to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::catalogue::Catalogue;
    use crate::catalogue::CatalogueBag;
    use crate::context;
    use crate::locale::EnglishVariant;
    use crate::locale::FrenchVariant;
    use crate::locale::Locale;
    use crate::Translator;

    use std::collections::HashMap;

    macro_rules! assert_ok {
        ($result:expr, $expected:expr) => {
            assert!($result.is_ok());
            assert_eq!($result.unwrap(), $expected.to_string());
        };
    }

    macro_rules! assert_err {
        ($result:expr, $expected:expr) => {
            assert!($result.is_err());
            assert_eq!(
                $result.unwrap_err().to_string(),
                $expected.to_string()
            );
        };
    }

    #[test]
    fn translation() {
        let bag = CatalogueBag::with_catalogues(vec![
            Catalogue::with_messages(Locale::English(EnglishVariant::Default), HashMap::from([
                ("messages".to_owned(), HashMap::from([
                  ("greeting".to_owned(), "Hello, {name}!".to_owned()),
                  ("love".to_owned(), "I love rust!".to_owned()),
                  ("apple".to_owned(), "{0} There are no apples | {1} There is one apple | {2..4} There are few apples | There are {?} apples".to_owned()),
                ]))
            ])),
            Catalogue::with_messages(Locale::French(FrenchVariant::Default), HashMap::from([
                ("messages".to_owned(), HashMap::from([
                  ("greeting".to_owned(), "Bonjour, {name}!".to_owned()),
                  ("love".to_owned(), "J'aime rust!".to_owned()),
                ]))
            ]))
        ]);

        let translator = Translator::with_catalogue_bag(bag);

        assert_ok!(
            translator.trans(
                "en",
                "messages",
                "greeting",
                context!(name = "Saif")
            ),
            "Hello, Saif!"
        );
        assert_ok!(
            translator.trans(
                "fr",
                "messages",
                "greeting",
                context!(name = "Saif")
            ),
            "Bonjour, Saif!"
        );
        assert_ok!(
            translator.trans("en", "messages", "love", None),
            "I love rust!"
        );
        assert_ok!(
            translator.trans("fr", "messages", "love", None),
            "J'aime rust!"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 0)),
            "There are no apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 1)),
            "There is one apple"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 2)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 3)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 4)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 4)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 5)),
            "There are 5 apples"
        );

        assert_err!(
            translator.trans("fr", "messages", "bar", context!(? = 5)),
            "message not found: message `'bar'` could not be found in `'messages'` domain for `'fr'` locale."
        );
    }

    #[test]
    fn translation_fallback() {
        let bag = CatalogueBag::with_catalogues(vec![
            Catalogue::with_messages(Locale::English(EnglishVariant::Default), HashMap::from([
                ("messages".to_owned(), HashMap::from([
                  ("greeting".to_owned(), "Hello, {name}!".to_owned()),
                  ("apple".to_owned(), "{0} There are no apples | {1} There is one apple | {2..4} There are few apples | There are {?} apples".to_owned()),
                ]))
            ])),
            Catalogue::with_messages(Locale::French(FrenchVariant::Default), HashMap::from([
                ("messages".to_owned(), HashMap::from([
                  ("greeting".to_owned(), "Bonjour, {name}!".to_owned()),
                ]))
            ]))
        ]);

        let mut translator = Translator::with_catalogue_bag(bag);

        translator
            .set_fallback_locale(Locale::English(EnglishVariant::Default));

        assert_ok!(
            translator.trans(
                Locale::English(EnglishVariant::Default),
                "messages",
                "apple",
                context!(? = 0)
            ),
            "There are no apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 1)),
            "There is one apple"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 2)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 3)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 4)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("en", "messages", "apple", context!(? = 5)),
            "There are 5 apples"
        );
        assert_ok!(
            translator.trans(
                Locale::French(FrenchVariant::Default),
                "messages",
                "apple",
                context!(? = 0)
            ),
            "There are no apples"
        );
        assert_ok!(
            translator.trans("fr", "messages", "apple", context!(? = 1)),
            "There is one apple"
        );
        assert_ok!(
            translator.trans("fr", "messages", "apple", context!(? = 2)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("fr", "messages", "apple", context!(? = 3)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("fr", "messages", "apple", context!(? = 4)),
            "There are few apples"
        );
        assert_ok!(
            translator.trans("fr", "messages", "apple", context!(? = 5)),
            "There are 5 apples"
        );
    }

    #[test]
    fn trasnaltion_error() {
        let bag = CatalogueBag::with_catalogues(vec![
            Catalogue::with_messages(Locale::English(EnglishVariant::Default), HashMap::from([
                ("messages".to_owned(), HashMap::from([
                  ("greeting".to_owned(), "Hello, {name}!".to_owned()),
                  ("apple".to_owned(), "{0} There are no apples | {1} There is one apple | {2..4} There are few apples | There are {?} apples".to_owned()),
                ]))
            ])),
            Catalogue::with_messages(Locale::French(FrenchVariant::Default), HashMap::from([
                ("messages".to_owned(), HashMap::from([
                  ("greeting".to_owned(), "Bonjour, {name}!".to_owned()),
                ]))
            ]))
        ]);

        let translator = Translator::with_catalogue_bag(bag);

        assert_err!(
            translator.trans("foo", "messages", "bar", context!(? = 5)),
            "locale: invalid locale, expected a valid locale code but found `'foo'`."
        );
    }
}
