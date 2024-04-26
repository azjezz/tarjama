use crate::locale::Locale;

use core::marker::Send;
use core::marker::Sync;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CatalogueBag {
    catalogues: Vec<Catalogue>,
}

unsafe impl Sync for CatalogueBag {}
unsafe impl Send for CatalogueBag {}

impl CatalogueBag {
    /// Creates a new empty `CatalogueBag`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::ArabicVariant;
    /// use std::collections::HashMap;
    ///
    /// let catalogue = Catalogue::new(Locale::Arabic(ArabicVariant::Tunisia));
    ///
    /// assert_eq!(catalogue.locale(), &Locale::Arabic(ArabicVariant::Tunisia));
    /// ```
    pub fn new() -> Self {
        Self { catalogues: Vec::new() }
    }

    /// Creates a `CatalogueBag` containing the given catalogues.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::CatalogueBag;
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::ArabicVariant;
    /// use std::collections::HashMap;
    ///
    /// let bag = CatalogueBag::with_catalogues(vec![
    ///     Catalogue::with_messages(Locale::Arabic(ArabicVariant::Tunisia), HashMap::from([
    ///         ("messages".to_owned(), HashMap::from([
    ///             ("greeting".to_owned(), "{name} أهلا".to_owned())
    ///         ]))
    ///     ]))
    /// ]);
    ///
    /// assert_eq!(bag.get(&Locale::Arabic(ArabicVariant::Tunisia)).len(), 1);
    /// ```
    pub fn with_catalogues<T>(catalogues: T) -> Self
    where
        T: Into<Vec<Catalogue>>,
    {
        Self { catalogues: catalogues.into() }
    }

    /// Moves all the catalogues of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::CatalogueBag;
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::ArabicVariant;
    /// use tarjama::locale::EnglishVariant;
    /// use tarjama::locale::Locale;
    ///
    /// use std::collections::HashMap;
    ///
    /// let mut bag = CatalogueBag::with_catalogues(vec![Catalogue::with_messages(
    ///     Locale::Arabic(ArabicVariant::Tunisia),
    ///     HashMap::from([(
    ///         "messages".to_owned(),
    ///         HashMap::from([("greeting".to_owned(), "{name} أهلا".to_owned())]),
    ///     )]),
    /// )]);
    ///
    /// let mut other = CatalogueBag::with_catalogues(vec![Catalogue::with_messages(
    ///     Locale::English(EnglishVariant::Default),
    ///     HashMap::from([(
    ///         "messages".to_owned(),
    ///         HashMap::from([("greeting".to_owned(), "Hello, {name}!".to_owned())]),
    ///     )]),
    /// )]);
    ///
    /// bag.append(&mut other);
    ///
    /// assert_eq!(
    ///     bag.get(&Locale::English(EnglishVariant::Default)),
    ///     vec![&Catalogue::with_messages(
    ///         Locale::English(EnglishVariant::Default),
    ///         HashMap::from([(
    ///             "messages".to_owned(),
    ///             HashMap::from([("greeting".to_owned(), "Hello, {name}!".to_owned())])
    ///         )])
    ///     )]
    /// );
    ///
    /// assert!(other.is_empty());
    /// ```
    pub fn append(&mut self, other: &mut CatalogueBag) {
        self.catalogues.append(&mut other.catalogues);
    }

    /// Insert a catalogue into the bag.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::FrenchVariant;
    /// use tarjama::locale::EnglishVariant;
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::catalogue::CatalogueBag;
    ///
    /// let mut bag = CatalogueBag::new();
    ///
    /// bag.insert(Catalogue::new(Locale::English(EnglishVariant::Default)));
    /// bag.insert(Catalogue::new(Locale::French(FrenchVariant::Default)));
    ///
    /// assert_eq!(
    ///   bag.get(&Locale::English(EnglishVariant::Default)),
    ///   vec![
    ///     &Catalogue::new(Locale::English(EnglishVariant::Default))
    ///   ]
    /// );
    ///
    /// assert_eq!(
    ///   bag.get(&Locale::French(FrenchVariant::Default)),
    ///   vec![
    ///     &Catalogue::new(Locale::French(FrenchVariant::Default))
    ///   ]
    /// );
    /// ```
    pub fn insert(&mut self, catalogue: Catalogue) {
        self.catalogues.push(catalogue);
    }

    /// Returns a reference to the catalogues corresponding to the locale.
    ///
    /// # Examples
    /// ```
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::FrenchVariant;
    /// use tarjama::locale::EnglishVariant;
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::catalogue::CatalogueBag;
    ///
    /// let mut bag = CatalogueBag::new();
    ///
    /// let mut bag = CatalogueBag::with_catalogues(vec![
    ///   Catalogue::new(Locale::English(EnglishVariant::Default)),
    /// ]);
    ///
    /// assert_eq!(
    ///   bag.get(&Locale::English(EnglishVariant::Default)),
    ///   vec![
    ///     &Catalogue::new(Locale::English(EnglishVariant::Default)),
    ///   ]
    /// );
    ///
    /// let mut bag = CatalogueBag::with_catalogues(vec![
    ///   Catalogue::new(Locale::French(FrenchVariant::Default)),
    ///   Catalogue::new(Locale::English(EnglishVariant::Default)),
    ///   Catalogue::new(Locale::English(EnglishVariant::Default)),
    /// ]);
    ///
    /// assert_eq!(
    ///   bag.get(&Locale::English(EnglishVariant::Default)),
    ///   vec![
    ///     &Catalogue::new(Locale::English(EnglishVariant::Default)),
    ///     &Catalogue::new(Locale::English(EnglishVariant::Default))
    ///   ]
    /// );
    ///
    /// assert_eq!(
    ///   bag.get(&Locale::French(FrenchVariant::Default)),
    ///   vec![
    ///     &Catalogue::new(Locale::French(FrenchVariant::Default)),
    ///   ]
    /// );
    /// ```
    pub fn get(&self, locale: &Locale) -> Vec<&Catalogue> {
        self.catalogues
            .iter()
            .filter(|c| c.locale() == locale)
            .collect::<Vec<&Catalogue>>()
    }

    /// Returns `true` if the bag contains no catalogues.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::catalogue::CatalogueBag;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    ///
    /// let mut b = CatalogueBag::new();
    /// assert!(b.is_empty());
    ///
    /// b.insert(Catalogue::new(Locale::English(EnglishVariant::Default)));
    ///
    /// assert!(!b.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.catalogues.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Catalogue {
    locale: Locale,
    messages: HashMap<String, HashMap<String, String>>,
}

unsafe impl Sync for Catalogue {}
unsafe impl Send for Catalogue {}

/// A message catalogue for a specific locale.
///
/// # Examples
///
/// ```
/// use tarjama::catalogue::Catalogue;
/// use tarjama::locale::Locale;
/// use tarjama::locale::EnglishVariant;
///
/// let mut catalogue = Catalogue::new(Locale::English(EnglishVariant::Default));
///
/// assert_eq!(catalogue.locale(), &Locale::English(EnglishVariant::Default));
///
/// catalogue.insert("messages", "greeting", "Hello, {name}!");
///
/// assert_eq!(catalogue.domains(), vec![&"messages"]);
/// assert_eq!(catalogue.get("messages", "greeting"), Some(&"Hello, {name}!".to_string()));
/// ```
impl Catalogue {
    /// Creates an empty `Catalogue`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::ArabicVariant;
    ///
    /// let mut catalogue = Catalogue::new(Locale::Arabic(ArabicVariant::Tunisia));
    ///
    /// assert!(catalogue.domains().is_empty());
    /// ```
    pub fn new(locale: Locale) -> Self {
        Catalogue { locale, messages: HashMap::new() }
    }

    /// Creates a `Catalogue` containing the given messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::ArabicVariant;
    /// use std::collections::HashMap;
    ///
    /// let catalogue = Catalogue::with_messages(Locale::Arabic(ArabicVariant::Tunisia), HashMap::from([
    ///     ("messages".to_owned(), HashMap::from([
    ///         ("greeting".to_owned(), "{name} أهلا".to_owned())
    ///     ]))
    /// ]));
    ///
    /// assert_eq!(catalogue.locale(), &Locale::Arabic(ArabicVariant::Tunisia));
    /// assert_eq!(catalogue.domains(), vec![&"messages"]);
    /// ```
    pub fn with_messages(
        locale: Locale,
        messages: HashMap<String, HashMap<String, String>>,
    ) -> Self {
        Catalogue { locale, messages }
    }

    /// Returns a reference to the locale of the catalogue.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    ///
    /// let mut catalogue = Catalogue::new(Locale::English(EnglishVariant::Default));
    ///
    /// assert_eq!(catalogue.locale(), &Locale::English(EnglishVariant::Default));
    /// ```
    pub fn locale(&self) -> &Locale {
        &self.locale
    }

    /// Returns a reference to the available domains list.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    ///
    /// let mut catalogue = Catalogue::new(Locale::English(EnglishVariant::Default));
    ///
    /// assert!(catalogue.domains().is_empty());
    ///
    /// catalogue.insert("messages", "greeting", "Hello, {name}!");
    ///
    /// assert_eq!(catalogue.domains().len(), 1);
    /// assert_eq!(catalogue.domains(), vec![&"messages"]);
    ///
    /// catalogue.insert("errors", "email.invalid", "Your email address ( {email} ) is invalid.");
    ///
    /// assert_eq!(catalogue.domains().len(), 2);
    /// assert_eq!(catalogue.domains(), vec![&"errors".to_string(), &"messages".to_string()]);
    /// ```
    pub fn domains(&self) -> Vec<&String> {
        let mut domains: Vec<&String> =
            self.messages.keys().into_iter().collect();
        domains.sort();

        domains
    }

    /// Returns a reference to the message corresponding to the message id and domain.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    ///
    /// let mut catalogue = Catalogue::new(Locale::English(EnglishVariant::Default));
    ///
    /// catalogue.insert("messages", "greeting", "Hello, {name}!");
    ///
    /// assert_eq!(catalogue.get("messages", "greeting"), Some(&"Hello, {name}!".to_string()));
    /// assert_eq!(catalogue.get("messages", "foo"), None);
    /// assert_eq!(catalogue.get("bar", "foo"), None);
    /// ```
    pub fn get(&self, domain: &str, id: &str) -> Option<&String> {
        match self.messages.get(domain) {
            Some(messages) => messages.get(id),
            None => None,
        }
    }

    /// Returns a reference to the messages corresponding to the domain.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    /// use std::collections::HashMap;
    ///
    /// let mut catalogue = Catalogue::new(Locale::English(EnglishVariant::Default));
    ///
    /// assert_eq!(catalogue.get_all("messages"), None);
    ///
    /// catalogue.insert("messages", "greeting", "Hello, {name}!");
    ///
    /// assert_eq!(catalogue.get_all("messages"), Some(&HashMap::from([
    ///   ("greeting".to_string(), "Hello, {name}!".to_string())
    /// ])));
    /// ```
    pub fn get_all(&self, domain: &str) -> Option<&HashMap<String, String>> {
        self.messages.get(domain)
    }

    /// Inserts a message into the catalogue.
    ///
    /// If the catalogue did not have this message id present, [`None`] is returned.
    ///
    /// If the catalogue did have this message id present, the message is updated, and the old
    /// message is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    ///
    /// let mut catalogue = Catalogue::new(Locale::English(EnglishVariant::Default));
    ///
    /// assert_eq!(catalogue.insert("messages", "greeting", "Hello, {name}!"), None);
    /// assert_eq!(catalogue.insert("messages", "greeting", "Welcome, {name}!"), Some("Hello, {name}!".to_string()));
    ///
    /// assert_eq!(catalogue.get("messages", "greeting"), Some(&"Welcome, {name}!".to_string()));
    /// ```
    pub fn insert(
        &mut self,
        domain: &str,
        id: &str,
        message: &str,
    ) -> Option<String> {
        match self.messages.entry(domain.to_owned()) {
            Entry::Occupied(mut o) => {
                o.get_mut().insert(id.to_owned(), message.to_owned())
            }
            Entry::Vacant(v) => {
                v.insert(HashMap::from([(id.to_owned(), message.to_owned())]));

                None
            }
        }
    }

    /// Removes a message id from the catalogue, returning the message at the message id if it
    /// was previously in the catalogue.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    ///
    /// let mut catalogue = Catalogue::new(Locale::English(EnglishVariant::Default));
    ///
    /// catalogue.insert("messages", "greeting", "Hello, {name}!");
    ///
    /// assert_eq!(catalogue.remove("messages", "greeting"), Some("Hello, {name}!".to_string()));
    /// assert_eq!(catalogue.remove("messages", "greeting"), None);
    /// ```
    pub fn remove(&mut self, domain: &str, id: &str) -> Option<String> {
        match self.messages.entry(domain.to_owned()) {
            Entry::Occupied(mut o) => o.get_mut().remove(id),
            Entry::Vacant(_) => None,
        }
    }

    /// Removes a domain from the catalogue, returning the messages at the domain if it
    /// was previously in the catalogue.
    ///
    /// # Examples
    ///
    /// ```
    /// use tarjama::catalogue::Catalogue;
    /// use tarjama::locale::Locale;
    /// use tarjama::locale::EnglishVariant;
    /// use std::collections::HashMap;
    ///
    /// let mut catalogue = Catalogue::new(Locale::English(EnglishVariant::Default));
    ///
    /// catalogue.insert("messages", "hello", "Hello, {name}!");
    /// catalogue.insert("messages", "welcome", "Welcome, {name}!");
    ///
    /// assert_eq!(catalogue.remove_all("messages"), Some(HashMap::from([
    ///   ("hello".to_string(), "Hello, {name}!".to_string()),
    ///   ("welcome".to_string(), "Welcome, {name}!".to_string())
    /// ])));
    ///
    /// assert_eq!(catalogue.remove_all("messages"), None);
    /// ```
    pub fn remove_all(
        &mut self,
        domain: &str,
    ) -> Option<HashMap<String, String>> {
        match self.messages.entry(domain.to_owned()) {
            Entry::Occupied(o) => Some(o.remove()),
            Entry::Vacant(_) => None,
        }
    }
}
