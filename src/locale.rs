use crate::error::Error;

use core::marker::Send;
use core::marker::Sync;
use core::result::Result as CoreResult;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    Afar,
    Abkhazian,
    Afrikaans,
    Akan,
    Albanian,
    Amharic,
    Arabic(ArabicVariant),
    Aragonese,
    Armenian,
    Assamese,
    Avaric,
    Avestan,
    Aymara,
    Azerbaijani,
    Bashkir,
    Bambara,
    Basque,
    Belarusian,
    Bengali,
    Bihari,
    Bislama,
    Tibetan,
    Bosnian,
    Breton,
    Bulgarian,
    Burmese,
    Catalan,
    Czech,
    Chamorro,
    Chechen,
    Chinese(ChineseVariant),
    ChurchSlavic,
    Chuvash,
    Cornish,
    Corsican,
    Cree,
    Welsh,
    Danish,
    German(GermanVariant),
    Divehi,
    Dutch(DutchVariant),
    Dzongkha,
    Greek,
    English(EnglishVariant),
    Esperanto,
    Estonian,
    Ewe,
    Faroese,
    Persian,
    Fijian,
    Finnish,
    French(FrenchVariant),
    WesternFrisian,
    Fulah,
    Georgian,
    Gaelic,
    Irish,
    Galician,
    Manx,
    Guarani,
    Gujarati,
    Haitian,
    Hausa,
    Hebrew,
    Herero,
    Hindi,
    HiriMotu,
    Croatian,
    Hungarian,
    Igbo,
    Icelandic,
    Ido,
    SichuanYi,
    Inuktitut,
    Interlingue,
    Indonesian,
    Inupiaq,
    Italian(ItalianVariant),
    Javanese,
    Japanese,
    Kalaallisut,
    Kannada,
    Kashmiri,
    Kanuri,
    Kazakh,
    CentralKhmer,
    Kikuyu,
    Kinyarwanda,
    Kirghiz,
    Komi,
    Kongo,
    Korean,
    Kuanyama,
    Kurdish,
    Lao,
    Latin,
    Latvian,
    Limburgan,
    Lingala,
    Lithuanian,
    Luxembourgish,
    LubaKatanga,
    Ganda,
    Macedonian,
    Marshallese,
    Malayalam,
    Maori,
    Marathi,
    Malay,
    Malagasy,
    Maltese,
    Mongolian,
    Nauru,
    Navajo,
    SouthernNdebele,
    NorthernNdebele,
    Ndonga,
    Nepali,
    NorwegianNynorsk,
    Norwegian,
    Chichewa,
    Occitan,
    Ojibwa,
    Oriya,
    Oromo,
    Ossetian,
    Panjabi,
    Pali,
    Polish,
    Portuguese(PortugueseVariant),
    Pushto,
    Quechua,
    Romansh,
    Romanian(RomanianVariant),
    Rundi,
    Russian(RussianVariant),
    Sango,
    Sanskrit,
    Sinhala,
    Slovak,
    Slovenian,
    NorthernSami,
    Samoan,
    Shona,
    Sindhi,
    Somali,
    SouthernSotho,
    Spanish(SpanishVariant),
    Sardinian,
    Serbian,
    Swati,
    Sundanese,
    Swahili,
    Swedish(SwedishVariant),
    Tahitian,
    Tamil,
    Tatar,
    Telugu,
    Tajik,
    Tagalog,
    Thai,
    Tigrinya,
    Tonga,
    Tswana,
    Tsonga,
    Turkmen,
    Turkish,
    Twi,
    Uighur,
    Ukrainian,
    Urdu,
    Uzbek,
    Venda,
    Vietnamese,
    Walloon,
    Wolof,
    Xhosa,
    Yiddish,
    Yoruba,
    Zhuang,
    Zulu,
}

impl Locale {
    /// Determine if the Locale has a specific variant (non-default)
    pub fn has_variant(&self) -> bool {
        match self {
            Locale::Arabic(variant) => *variant != ArabicVariant::Default,
            Locale::Chinese(variant) => *variant != ChineseVariant::Default,
            Locale::German(variant) => *variant != GermanVariant::Default,
            Locale::Dutch(variant) => *variant != DutchVariant::Default,
            Locale::English(variant) => *variant != EnglishVariant::Default,
            Locale::French(variant) => *variant != FrenchVariant::Default,
            Locale::Italian(variant) => *variant != ItalianVariant::Default,
            Locale::Portuguese(variant) => {
                *variant != PortugueseVariant::Default
            }
            Locale::Romanian(variant) => *variant != RomanianVariant::Default,
            Locale::Russian(variant) => *variant != RussianVariant::Default,
            Locale::Spanish(variant) => *variant != SpanishVariant::Default,
            Locale::Swedish(variant) => *variant != SwedishVariant::Default,
            _ => false,
        }
    }

    /// Return the locale with its default variant, if applicable
    pub fn with_default_variant(self) -> Self {
        match self {
            Locale::Arabic(_) => Locale::Arabic(ArabicVariant::Default),
            Locale::Chinese(_) => Locale::Chinese(ChineseVariant::Default),
            Locale::German(_) => Locale::German(GermanVariant::Default),
            Locale::Dutch(_) => Locale::Dutch(DutchVariant::Default),
            Locale::English(_) => Locale::English(EnglishVariant::Default),
            Locale::French(_) => Locale::French(FrenchVariant::Default),
            Locale::Italian(_) => Locale::Italian(ItalianVariant::Default),
            Locale::Portuguese(_) => {
                Locale::Portuguese(PortugueseVariant::Default)
            }
            Locale::Romanian(_) => Locale::Romanian(RomanianVariant::Default),
            Locale::Russian(_) => Locale::Russian(RussianVariant::Default),
            Locale::Spanish(_) => Locale::Spanish(SpanishVariant::Default),
            Locale::Swedish(_) => Locale::Swedish(SwedishVariant::Default),
            _ => self,
        }
    }
}

impl From<&Locale> for Locale {
    fn from(value: &Locale) -> Self {
        value.clone()
    }
}

impl TryFrom<String> for Locale {
    type Error = Error;

    fn try_from(value: String) -> CoreResult<Self, Self::Error> {
        let locale = &*value;

        locale.try_into()
    }
}

/// Create a `Locale` from a string reference.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::ArabicVariant;
/// use tarjama::locale::ChineseVariant;
/// use tarjama::locale::GermanVariant;
/// use tarjama::locale::DutchVariant;
/// use tarjama::locale::EnglishVariant;
/// use tarjama::locale::FrenchVariant;
/// use tarjama::locale::ItalianVariant;
/// use tarjama::locale::PortugueseVariant;
/// use tarjama::locale::RomanianVariant;
/// use tarjama::locale::RussianVariant;
/// use tarjama::locale::SpanishVariant;
/// use tarjama::locale::SwedishVariant;
///
/// let locale: Locale = "ar".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar");
///
/// let locale: Locale = "ar_DZ".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_DZ");
///
/// let locale: Locale = "ar_BH".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_BH");
///
/// let locale: Locale = "ar_EG".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_EG");
///
/// let locale: Locale = "ar_IQ".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_IQ");
///
/// let locale: Locale = "ar_JO".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_JO");
///
/// let locale: Locale = "ar_KW".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_KW");
///
/// let locale: Locale = "ar_LB".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_LB");
///
/// let locale: Locale = "ar_LY".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_LY");
///
/// let locale: Locale = "ar_MA".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_MA");
///
/// let locale: Locale = "ar_OM".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_OM");
///
/// let locale: Locale = "ar_QA".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_QA");
///
/// let locale: Locale = "ar_SA".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_SA");
///
/// let locale: Locale = "ar_SY".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_SY");
///
/// let locale: Locale = "ar_TN".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_TN");
///
/// let locale: Locale = "ar_AE".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_AE");
///
/// let locale: Locale = "ar_YE".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ar_YE");
///
/// let locale: Locale = "zh".try_into().unwrap();
/// assert_eq!(locale.to_string(), "zh");
///
/// let locale: Locale = "zh_HK".try_into().unwrap();
/// assert_eq!(locale.to_string(), "zh_HK");
///
/// let locale: Locale = "zh_CN".try_into().unwrap();
/// assert_eq!(locale.to_string(), "zh_CN");
///
/// let locale: Locale = "zh_SG".try_into().unwrap();
/// assert_eq!(locale.to_string(), "zh_SG");
///
/// let locale: Locale = "zh_TW".try_into().unwrap();
/// assert_eq!(locale.to_string(), "zh_TW");
///
/// let locale: Locale = "de".try_into().unwrap();
/// assert_eq!(locale.to_string(), "de");
///
/// let locale: Locale = "de_AT".try_into().unwrap();
/// assert_eq!(locale.to_string(), "de_AT");
///
/// let locale: Locale = "de_LI".try_into().unwrap();
/// assert_eq!(locale.to_string(), "de_LI");
///
/// let locale: Locale = "de_LU".try_into().unwrap();
/// assert_eq!(locale.to_string(), "de_LU");
///
/// let locale: Locale = "de_CH".try_into().unwrap();
/// assert_eq!(locale.to_string(), "de_CH");
///
/// let locale: Locale = "nl".try_into().unwrap();
/// assert_eq!(locale.to_string(), "nl");
///
/// let locale: Locale = "nl_BE".try_into().unwrap();
/// assert_eq!(locale.to_string(), "nl_BE");
///
/// let locale: Locale = "en".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en");
///
/// let locale: Locale = "en_AU".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_AU");
///
/// let locale: Locale = "en_BZ".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_BZ");
///
/// let locale: Locale = "en_CA".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_CA");
///
/// let locale: Locale = "en_IE".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_IE");
///
/// let locale: Locale = "en_JM".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_JM");
///
/// let locale: Locale = "en_NZ".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_NZ");
///
/// let locale: Locale = "en_ZA".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_ZA");
///
/// let locale: Locale = "en_TT".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_TT");
///
/// let locale: Locale = "en_GB".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_GB");
///
/// let locale: Locale = "en_US".try_into().unwrap();
/// assert_eq!(locale.to_string(), "en_US");
///
/// let locale: Locale = "fr".try_into().unwrap();
/// assert_eq!(locale.to_string(), "fr");
///
/// let locale: Locale = "fr_FR".try_into().unwrap();
/// assert_eq!(locale.to_string(), "fr_FR");
///
/// let locale: Locale = "fr_BE".try_into().unwrap();
/// assert_eq!(locale.to_string(), "fr_BE");
///
/// let locale: Locale = "fr_CA".try_into().unwrap();
/// assert_eq!(locale.to_string(), "fr_CA");
///
/// let locale: Locale = "fr_LU".try_into().unwrap();
/// assert_eq!(locale.to_string(), "fr_LU");
///
/// let locale: Locale = "fr_CH".try_into().unwrap();
/// assert_eq!(locale.to_string(), "fr_CH");
///
/// let locale: Locale = "it".try_into().unwrap();
/// assert_eq!(locale.to_string(), "it");
///
/// let locale: Locale = "it_CH".try_into().unwrap();
/// assert_eq!(locale.to_string(), "it_CH");
///
/// let locale: Locale = "pt".try_into().unwrap();
/// assert_eq!(locale.to_string(), "pt");
///
/// let locale: Locale = "pt_BR".try_into().unwrap();
/// assert_eq!(locale.to_string(), "pt_BR");
///
/// let locale: Locale = "ro".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ro");
///
/// let locale: Locale = "ro_MD".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ro_MD");
///
/// let locale: Locale = "ru".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ru");
///
/// let locale: Locale = "ru_MD".try_into().unwrap();
/// assert_eq!(locale.to_string(), "ru_MD");
///
/// let locale: Locale = "es".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es");
///
/// let locale: Locale = "es_AR".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_AR");
///
/// let locale: Locale = "es_BO".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_BO");
///
/// let locale: Locale = "es_CL".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_CL");
///
/// let locale: Locale = "es_CO".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_CO");
///
/// let locale: Locale = "es_CR".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_CR");
///
/// let locale: Locale = "es_DO".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_DO");
///
/// let locale: Locale = "es_EC".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_EC");
///
/// let locale: Locale = "es_SV".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_SV");
///
/// let locale: Locale = "es_GT".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_GT");
///
/// let locale: Locale = "es_HN".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_HN");
///
/// let locale: Locale = "es_MX".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_MX");
///
/// let locale: Locale = "es_NI".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_NI");
///
/// let locale: Locale = "es_PA".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_PA");
///
/// let locale: Locale = "es_PY".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_PY");
///
/// let locale: Locale = "es_PE".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_PE");
///
/// let locale: Locale = "es_PR".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_PR");
///
/// let locale: Locale = "es_UY".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_UY");
///
/// let locale: Locale = "es_VE".try_into().unwrap();
/// assert_eq!(locale.to_string(), "es_VE");
///
/// let locale: Locale = "sv".try_into().unwrap();
/// assert_eq!(locale.to_string(), "sv");
///
/// let locale: Locale = "sv_FI".try_into().unwrap();
/// assert_eq!(locale.to_string(), "sv_FI");
///
/// let locale: Locale = "sv-FI".try_into().unwrap();
/// assert_eq!(locale.to_string(), "sv_FI");
/// ```
impl TryFrom<&str> for Locale {
    type Error = Error;

    fn try_from(value: &str) -> CoreResult<Self, Self::Error> {
        let value = value.replace("-", "_");
        match &*value {
            "aa" => Ok(Locale::Afar),
            "ab" => Ok(Locale::Abkhazian),
            "af" => Ok(Locale::Afrikaans),
            "ak" => Ok(Locale::Akan),
            "sq" => Ok(Locale::Albanian),
            "am" => Ok(Locale::Amharic),
            "ar" => Ok(Locale::Arabic(ArabicVariant::Default)),
            "ar_DZ" => Ok(Locale::Arabic(ArabicVariant::Algeria)),
            "ar_BH" => Ok(Locale::Arabic(ArabicVariant::Bahrain)),
            "ar_EG" => Ok(Locale::Arabic(ArabicVariant::Egypt)),
            "ar_IQ" => Ok(Locale::Arabic(ArabicVariant::Iraq)),
            "ar_JO" => Ok(Locale::Arabic(ArabicVariant::Jordan)),
            "ar_KW" => Ok(Locale::Arabic(ArabicVariant::Kuwait)),
            "ar_LB" => Ok(Locale::Arabic(ArabicVariant::Lebanon)),
            "ar_LY" => Ok(Locale::Arabic(ArabicVariant::Libya)),
            "ar_MA" => Ok(Locale::Arabic(ArabicVariant::Morocco)),
            "ar_OM" => Ok(Locale::Arabic(ArabicVariant::Oman)),
            "ar_QA" => Ok(Locale::Arabic(ArabicVariant::Qatar)),
            "ar_SA" => Ok(Locale::Arabic(ArabicVariant::SaudiArabia)),
            "ar_SY" => Ok(Locale::Arabic(ArabicVariant::Syria)),
            "ar_TN" => Ok(Locale::Arabic(ArabicVariant::Tunisia)),
            "ar_AE" => Ok(Locale::Arabic(ArabicVariant::UnitedArabEmirates)),
            "ar_YE" => Ok(Locale::Arabic(ArabicVariant::Yemen)),
            "an" => Ok(Locale::Aragonese),
            "hy" => Ok(Locale::Armenian),
            "as" => Ok(Locale::Assamese),
            "av" => Ok(Locale::Avaric),
            "ae" => Ok(Locale::Avestan),
            "ay" => Ok(Locale::Aymara),
            "az" => Ok(Locale::Azerbaijani),
            "ba" => Ok(Locale::Bashkir),
            "bm" => Ok(Locale::Bambara),
            "eu" => Ok(Locale::Basque),
            "be" => Ok(Locale::Belarusian),
            "bn" => Ok(Locale::Bengali),
            "bh" => Ok(Locale::Bihari),
            "bi" => Ok(Locale::Bislama),
            "bo" => Ok(Locale::Tibetan),
            "bs" => Ok(Locale::Bosnian),
            "br" => Ok(Locale::Breton),
            "bg" => Ok(Locale::Bulgarian),
            "my" => Ok(Locale::Burmese),
            "ca" => Ok(Locale::Catalan),
            "cs" => Ok(Locale::Czech),
            "ch" => Ok(Locale::Chamorro),
            "ce" => Ok(Locale::Chechen),
            "zh" => Ok(Locale::Chinese(ChineseVariant::Default)),
            "zh_HK" => Ok(Locale::Chinese(ChineseVariant::HongKong)),
            "zh_CN" => Ok(Locale::Chinese(ChineseVariant::China)),
            "zh_SG" => Ok(Locale::Chinese(ChineseVariant::Singapore)),
            "zh_TW" => Ok(Locale::Chinese(ChineseVariant::Taiwan)),
            "cu" => Ok(Locale::ChurchSlavic),
            "cv" => Ok(Locale::Chuvash),
            "kw" => Ok(Locale::Cornish),
            "co" => Ok(Locale::Corsican),
            "cr" => Ok(Locale::Cree),
            "cy" => Ok(Locale::Welsh),
            "da" => Ok(Locale::Danish),
            "de" => Ok(Locale::German(GermanVariant::Default)),
            "de_AT" => Ok(Locale::German(GermanVariant::Austria)),
            "de_LI" => Ok(Locale::German(GermanVariant::Liechtenstein)),
            "de_LU" => Ok(Locale::German(GermanVariant::Luxembourg)),
            "de_CH" => Ok(Locale::German(GermanVariant::Switzerland)),
            "dv" => Ok(Locale::Divehi),
            "nl" => Ok(Locale::Dutch(DutchVariant::Default)),
            "nl_BE" => Ok(Locale::Dutch(DutchVariant::Belgium)),
            "dz" => Ok(Locale::Dzongkha),
            "el" => Ok(Locale::Greek),
            "en" => Ok(Locale::English(EnglishVariant::Default)),
            "en_AU" => Ok(Locale::English(EnglishVariant::Australia)),
            "en_BZ" => Ok(Locale::English(EnglishVariant::Belize)),
            "en_CA" => Ok(Locale::English(EnglishVariant::Canada)),
            "en_IE" => Ok(Locale::English(EnglishVariant::Ireland)),
            "en_JM" => Ok(Locale::English(EnglishVariant::Jamaica)),
            "en_NZ" => Ok(Locale::English(EnglishVariant::NewZealand)),
            "en_ZA" => Ok(Locale::English(EnglishVariant::SouthAfrica)),
            "en_TT" => Ok(Locale::English(EnglishVariant::Trinidad)),
            "en_GB" => Ok(Locale::English(EnglishVariant::UnitedKingdom)),
            "en_US" => Ok(Locale::English(EnglishVariant::UnitedStates)),
            "eo" => Ok(Locale::Esperanto),
            "et" => Ok(Locale::Estonian),
            "ee" => Ok(Locale::Ewe),
            "fo" => Ok(Locale::Faroese),
            "fa" => Ok(Locale::Persian),
            "fj" => Ok(Locale::Fijian),
            "fi" => Ok(Locale::Finnish),
            "fr" => Ok(Locale::French(FrenchVariant::Default)),
            "fr_FR" => Ok(Locale::French(FrenchVariant::France)),
            "fr_BE" => Ok(Locale::French(FrenchVariant::Belgium)),
            "fr_CA" => Ok(Locale::French(FrenchVariant::Canada)),
            "fr_LU" => Ok(Locale::French(FrenchVariant::Luxembourg)),
            "fr_CH" => Ok(Locale::French(FrenchVariant::Switzerland)),
            "fy" => Ok(Locale::WesternFrisian),
            "ff" => Ok(Locale::Fulah),
            "ka" => Ok(Locale::Georgian),
            "gd" => Ok(Locale::Gaelic),
            "ga" => Ok(Locale::Irish),
            "gl" => Ok(Locale::Galician),
            "gv" => Ok(Locale::Manx),
            "gn" => Ok(Locale::Guarani),
            "gu" => Ok(Locale::Gujarati),
            "ht" => Ok(Locale::Haitian),
            "ha" => Ok(Locale::Hausa),
            "he" => Ok(Locale::Hebrew),
            "hz" => Ok(Locale::Herero),
            "hi" => Ok(Locale::Hindi),
            "ho" => Ok(Locale::HiriMotu),
            "hr" => Ok(Locale::Croatian),
            "hu" => Ok(Locale::Hungarian),
            "ig" => Ok(Locale::Igbo),
            "is" => Ok(Locale::Icelandic),
            "io" => Ok(Locale::Ido),
            "ii" => Ok(Locale::SichuanYi),
            "iu" => Ok(Locale::Inuktitut),
            "ie" => Ok(Locale::Interlingue),
            "id" => Ok(Locale::Indonesian),
            "ik" => Ok(Locale::Inupiaq),
            "it" => Ok(Locale::Italian(ItalianVariant::Default)),
            "it_CH" => Ok(Locale::Italian(ItalianVariant::Switzerland)),
            "jv" => Ok(Locale::Javanese),
            "ja" => Ok(Locale::Japanese),
            "kl" => Ok(Locale::Kalaallisut),
            "kn" => Ok(Locale::Kannada),
            "ks" => Ok(Locale::Kashmiri),
            "kr" => Ok(Locale::Kanuri),
            "kk" => Ok(Locale::Kazakh),
            "km" => Ok(Locale::CentralKhmer),
            "ki" => Ok(Locale::Kikuyu),
            "rw" => Ok(Locale::Kinyarwanda),
            "ky" => Ok(Locale::Kirghiz),
            "kv" => Ok(Locale::Komi),
            "kg" => Ok(Locale::Kongo),
            "ko" => Ok(Locale::Korean),
            "kj" => Ok(Locale::Kuanyama),
            "ku" => Ok(Locale::Kurdish),
            "lo" => Ok(Locale::Lao),
            "la" => Ok(Locale::Latin),
            "lv" => Ok(Locale::Latvian),
            "li" => Ok(Locale::Limburgan),
            "ln" => Ok(Locale::Lingala),
            "lt" => Ok(Locale::Lithuanian),
            "lb" => Ok(Locale::Luxembourgish),
            "lu" => Ok(Locale::LubaKatanga),
            "lg" => Ok(Locale::Ganda),
            "mk" => Ok(Locale::Macedonian),
            "mh" => Ok(Locale::Marshallese),
            "ml" => Ok(Locale::Malayalam),
            "mi" => Ok(Locale::Maori),
            "mr" => Ok(Locale::Marathi),
            "ms" => Ok(Locale::Malay),
            "mg" => Ok(Locale::Malagasy),
            "mt" => Ok(Locale::Maltese),
            "mn" => Ok(Locale::Mongolian),
            "na" => Ok(Locale::Nauru),
            "nv" => Ok(Locale::Navajo),
            "nr" => Ok(Locale::SouthernNdebele),
            "nd" => Ok(Locale::NorthernNdebele),
            "ng" => Ok(Locale::Ndonga),
            "ne" => Ok(Locale::Nepali),
            "nn" => Ok(Locale::NorwegianNynorsk),
            "no" => Ok(Locale::Norwegian),
            "ny" => Ok(Locale::Chichewa),
            "oc" => Ok(Locale::Occitan),
            "oj" => Ok(Locale::Ojibwa),
            "or" => Ok(Locale::Oriya),
            "om" => Ok(Locale::Oromo),
            "os" => Ok(Locale::Ossetian),
            "pa" => Ok(Locale::Panjabi),
            "pi" => Ok(Locale::Pali),
            "pl" => Ok(Locale::Polish),
            "pt" => Ok(Locale::Portuguese(PortugueseVariant::Default)),
            "pt_BR" => Ok(Locale::Portuguese(PortugueseVariant::Brazil)),
            "ps" => Ok(Locale::Pushto),
            "qu" => Ok(Locale::Quechua),
            "rm" => Ok(Locale::Romansh),
            "ro" => Ok(Locale::Romanian(RomanianVariant::Default)),
            "ro_MD" => Ok(Locale::Romanian(RomanianVariant::Moldova)),
            "rn" => Ok(Locale::Rundi),
            "ru" => Ok(Locale::Russian(RussianVariant::Default)),
            "ru_MD" => Ok(Locale::Russian(RussianVariant::Moldova)),
            "sg" => Ok(Locale::Sango),
            "sa" => Ok(Locale::Sanskrit),
            "si" => Ok(Locale::Sinhala),
            "sk" => Ok(Locale::Slovak),
            "sl" => Ok(Locale::Slovenian),
            "se" => Ok(Locale::NorthernSami),
            "sm" => Ok(Locale::Samoan),
            "sn" => Ok(Locale::Shona),
            "sd" => Ok(Locale::Sindhi),
            "so" => Ok(Locale::Somali),
            "st" => Ok(Locale::SouthernSotho),
            "es" => Ok(Locale::Spanish(SpanishVariant::Default)),
            "es_AR" => Ok(Locale::Spanish(SpanishVariant::Argentina)),
            "es_BO" => Ok(Locale::Spanish(SpanishVariant::Bolivia)),
            "es_CL" => Ok(Locale::Spanish(SpanishVariant::Chile)),
            "es_CO" => Ok(Locale::Spanish(SpanishVariant::Colombia)),
            "es_CR" => Ok(Locale::Spanish(SpanishVariant::CostaRica)),
            "es_DO" => Ok(Locale::Spanish(SpanishVariant::DominicanRepublic)),
            "es_EC" => Ok(Locale::Spanish(SpanishVariant::Ecuador)),
            "es_SV" => Ok(Locale::Spanish(SpanishVariant::ElSalvador)),
            "es_GT" => Ok(Locale::Spanish(SpanishVariant::Guatemala)),
            "es_HN" => Ok(Locale::Spanish(SpanishVariant::Honduras)),
            "es_MX" => Ok(Locale::Spanish(SpanishVariant::Mexico)),
            "es_NI" => Ok(Locale::Spanish(SpanishVariant::Nicaragua)),
            "es_PA" => Ok(Locale::Spanish(SpanishVariant::Panama)),
            "es_PY" => Ok(Locale::Spanish(SpanishVariant::Paraguay)),
            "es_PE" => Ok(Locale::Spanish(SpanishVariant::Peru)),
            "es_PR" => Ok(Locale::Spanish(SpanishVariant::PuertoRico)),
            "es_UY" => Ok(Locale::Spanish(SpanishVariant::Uruguay)),
            "es_VE" => Ok(Locale::Spanish(SpanishVariant::Venezuela)),
            "sc" => Ok(Locale::Sardinian),
            "sr" => Ok(Locale::Serbian),
            "ss" => Ok(Locale::Swati),
            "su" => Ok(Locale::Sundanese),
            "sw" => Ok(Locale::Swahili),
            "sv" => Ok(Locale::Swedish(SwedishVariant::Default)),
            "sv_FI" => Ok(Locale::Swedish(SwedishVariant::Finland)),
            "ty" => Ok(Locale::Tahitian),
            "ta" => Ok(Locale::Tamil),
            "tt" => Ok(Locale::Tatar),
            "te" => Ok(Locale::Telugu),
            "tg" => Ok(Locale::Tajik),
            "tl" => Ok(Locale::Tagalog),
            "th" => Ok(Locale::Thai),
            "ti" => Ok(Locale::Tigrinya),
            "to" => Ok(Locale::Tonga),
            "tn" => Ok(Locale::Tswana),
            "ts" => Ok(Locale::Tsonga),
            "tk" => Ok(Locale::Turkmen),
            "tr" => Ok(Locale::Turkish),
            "tw" => Ok(Locale::Twi),
            "ug" => Ok(Locale::Uighur),
            "uk" => Ok(Locale::Ukrainian),
            "ur" => Ok(Locale::Urdu),
            "uz" => Ok(Locale::Uzbek),
            "ve" => Ok(Locale::Venda),
            "vi" => Ok(Locale::Vietnamese),
            "wa" => Ok(Locale::Walloon),
            "wo" => Ok(Locale::Wolof),
            "xh" => Ok(Locale::Xhosa),
            "yi" => Ok(Locale::Yiddish),
            "yo" => Ok(Locale::Yoruba),
            "za" => Ok(Locale::Zhuang),
            "zu" => Ok(Locale::Zulu),
            _ => Err(Error::InvalidLocale(value.to_string())),
        }
    }
}

unsafe impl Sync for Locale {}
unsafe impl Send for Locale {}

/// Display a `Locale`.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
///
/// let locale = Locale::Afar;
/// assert_eq!(locale.to_string(), "aa");
///
/// let locale = Locale::Abkhazian;
/// assert_eq!(locale.to_string(), "ab");
///
/// let locale = Locale::Afrikaans;
/// assert_eq!(locale.to_string(), "af");
///
/// let locale = Locale::Akan;
/// assert_eq!(locale.to_string(), "ak");
///
/// let locale = Locale::Albanian;
/// assert_eq!(locale.to_string(), "sq");
///
/// let locale = Locale::Amharic;
/// assert_eq!(locale.to_string(), "am");
///
/// let locale = Locale::Aragonese;
/// assert_eq!(locale.to_string(), "an");
///
/// let locale = Locale::Armenian;
/// assert_eq!(locale.to_string(), "hy");
///
/// let locale = Locale::Assamese;
/// assert_eq!(locale.to_string(), "as");
///
/// let locale = Locale::Avaric;
/// assert_eq!(locale.to_string(), "av");
///
/// let locale = Locale::Avestan;
/// assert_eq!(locale.to_string(), "ae");
///
/// let locale = Locale::Aymara;
/// assert_eq!(locale.to_string(), "ay");
///
/// let locale = Locale::Azerbaijani;
/// assert_eq!(locale.to_string(), "az");
///
/// let locale = Locale::Bashkir;
/// assert_eq!(locale.to_string(), "ba");
///
/// let locale = Locale::Bambara;
/// assert_eq!(locale.to_string(), "bm");
///
/// let locale = Locale::Basque;
/// assert_eq!(locale.to_string(), "eu");
///
/// let locale = Locale::Belarusian;
/// assert_eq!(locale.to_string(), "be");
///
/// let locale = Locale::Bengali;
/// assert_eq!(locale.to_string(), "bn");
///
/// let locale = Locale::Bihari;
/// assert_eq!(locale.to_string(), "bh");
///
/// let locale = Locale::Bislama;
/// assert_eq!(locale.to_string(), "bi");
///
/// let locale = Locale::Tibetan;
/// assert_eq!(locale.to_string(), "bo");
///
/// let locale = Locale::Bosnian;
/// assert_eq!(locale.to_string(), "bs");
///
/// let locale = Locale::Breton;
/// assert_eq!(locale.to_string(), "br");
///
/// let locale = Locale::Bulgarian;
/// assert_eq!(locale.to_string(), "bg");
///
/// let locale = Locale::Burmese;
/// assert_eq!(locale.to_string(), "my");
///
/// let locale = Locale::Catalan;
/// assert_eq!(locale.to_string(), "ca");
///
/// let locale = Locale::Czech;
/// assert_eq!(locale.to_string(), "cs");
///
/// let locale = Locale::Chamorro;
/// assert_eq!(locale.to_string(), "ch");
///
/// let locale = Locale::Chechen;
/// assert_eq!(locale.to_string(), "ce");
///
/// let locale = Locale::ChurchSlavic;
/// assert_eq!(locale.to_string(), "cu");
///
/// let locale = Locale::Chuvash;
/// assert_eq!(locale.to_string(), "cv");
///
/// let locale = Locale::Cornish;
/// assert_eq!(locale.to_string(), "kw");
///
/// let locale = Locale::Corsican;
/// assert_eq!(locale.to_string(), "co");
///
/// let locale = Locale::Cree;
/// assert_eq!(locale.to_string(), "cr");
///
/// let locale = Locale::Welsh;
/// assert_eq!(locale.to_string(), "cy");
///
/// let locale = Locale::Danish;
/// assert_eq!(locale.to_string(), "da");
///
/// let locale = Locale::Divehi;
/// assert_eq!(locale.to_string(), "dv");
///
/// let locale = Locale::Dzongkha;
/// assert_eq!(locale.to_string(), "dz");
///
/// let locale = Locale::Greek;
/// assert_eq!(locale.to_string(), "el");
///
/// let locale = Locale::Esperanto;
/// assert_eq!(locale.to_string(), "eo");
///
/// let locale = Locale::Estonian;
/// assert_eq!(locale.to_string(), "et");
///
/// let locale = Locale::Ewe;
/// assert_eq!(locale.to_string(), "ee");
///
/// let locale = Locale::Faroese;
/// assert_eq!(locale.to_string(), "fo");
///
/// let locale = Locale::Persian;
/// assert_eq!(locale.to_string(), "fa");
///
/// let locale = Locale::Fijian;
/// assert_eq!(locale.to_string(), "fj");
///
/// let locale = Locale::Finnish;
/// assert_eq!(locale.to_string(), "fi");
///
/// let locale = Locale::WesternFrisian;
/// assert_eq!(locale.to_string(), "fy");
///
/// let locale = Locale::Fulah;
/// assert_eq!(locale.to_string(), "ff");
///
/// let locale = Locale::Georgian;
/// assert_eq!(locale.to_string(), "ka");
///
/// let locale = Locale::Gaelic;
/// assert_eq!(locale.to_string(), "gd");
///
/// let locale = Locale::Irish;
/// assert_eq!(locale.to_string(), "ga");
///
/// let locale = Locale::Galician;
/// assert_eq!(locale.to_string(), "gl");
///
/// let locale = Locale::Manx;
/// assert_eq!(locale.to_string(), "gv");
///
/// let locale = Locale::Guarani;
/// assert_eq!(locale.to_string(), "gn");
///
/// let locale = Locale::Gujarati;
/// assert_eq!(locale.to_string(), "gu");
///
/// let locale = Locale::Haitian;
/// assert_eq!(locale.to_string(), "ht");
///
/// let locale = Locale::Hausa;
/// assert_eq!(locale.to_string(), "ha");
///
/// let locale = Locale::Hebrew;
/// assert_eq!(locale.to_string(), "he");
///
/// let locale = Locale::Herero;
/// assert_eq!(locale.to_string(), "hz");
///
/// let locale = Locale::Hindi;
/// assert_eq!(locale.to_string(), "hi");
///
/// let locale = Locale::HiriMotu;
/// assert_eq!(locale.to_string(), "ho");
///
/// let locale = Locale::Croatian;
/// assert_eq!(locale.to_string(), "hr");
///
/// let locale = Locale::Hungarian;
/// assert_eq!(locale.to_string(), "hu");
///
/// let locale = Locale::Igbo;
/// assert_eq!(locale.to_string(), "ig");
///
/// let locale = Locale::Icelandic;
/// assert_eq!(locale.to_string(), "is");
///
/// let locale = Locale::Ido;
/// assert_eq!(locale.to_string(), "io");
///
/// let locale = Locale::SichuanYi;
/// assert_eq!(locale.to_string(), "ii");
///
/// let locale = Locale::Inuktitut;
/// assert_eq!(locale.to_string(), "iu");
///
/// let locale = Locale::Interlingue;
/// assert_eq!(locale.to_string(), "ie");
///
/// let locale = Locale::Indonesian;
/// assert_eq!(locale.to_string(), "id");
///
/// let locale = Locale::Inupiaq;
/// assert_eq!(locale.to_string(), "ik");
///
/// let locale = Locale::Javanese;
/// assert_eq!(locale.to_string(), "jv");
///
/// let locale = Locale::Japanese;
/// assert_eq!(locale.to_string(), "ja");
///
/// let locale = Locale::Kalaallisut;
/// assert_eq!(locale.to_string(), "kl");
///
/// let locale = Locale::Kannada;
/// assert_eq!(locale.to_string(), "kn");
///
/// let locale = Locale::Kashmiri;
/// assert_eq!(locale.to_string(), "ks");
///
/// let locale = Locale::Kanuri;
/// assert_eq!(locale.to_string(), "kr");
///
/// let locale = Locale::Kazakh;
/// assert_eq!(locale.to_string(), "kk");
///
/// let locale = Locale::CentralKhmer;
/// assert_eq!(locale.to_string(), "km");
///
/// let locale = Locale::Kikuyu;
/// assert_eq!(locale.to_string(), "ki");
///
/// let locale = Locale::Kinyarwanda;
/// assert_eq!(locale.to_string(), "rw");
///
/// let locale = Locale::Kirghiz;
/// assert_eq!(locale.to_string(), "ky");
///
/// let locale = Locale::Komi;
/// assert_eq!(locale.to_string(), "kv");
///
/// let locale = Locale::Kongo;
/// assert_eq!(locale.to_string(), "kg");
///
/// let locale = Locale::Korean;
/// assert_eq!(locale.to_string(), "ko");
///
/// let locale = Locale::Kuanyama;
/// assert_eq!(locale.to_string(), "kj");
///
/// let locale = Locale::Kurdish;
/// assert_eq!(locale.to_string(), "ku");
///
/// let locale = Locale::Lao;
/// assert_eq!(locale.to_string(), "lo");
///
/// let locale = Locale::Latin;
/// assert_eq!(locale.to_string(), "la");
///
/// let locale = Locale::Latvian;
/// assert_eq!(locale.to_string(), "lv");
///
/// let locale = Locale::Limburgan;
/// assert_eq!(locale.to_string(), "li");
///
/// let locale = Locale::Lingala;
/// assert_eq!(locale.to_string(), "ln");
///
/// let locale = Locale::Lithuanian;
/// assert_eq!(locale.to_string(), "lt");
///
/// let locale = Locale::Luxembourgish;
/// assert_eq!(locale.to_string(), "lb");
///
/// let locale = Locale::LubaKatanga;
/// assert_eq!(locale.to_string(), "lu");
///
/// let locale = Locale::Ganda;
/// assert_eq!(locale.to_string(), "lg");
///
/// let locale = Locale::Macedonian;
/// assert_eq!(locale.to_string(), "mk");
///
/// let locale = Locale::Marshallese;
/// assert_eq!(locale.to_string(), "mh");
///
/// let locale = Locale::Malayalam;
/// assert_eq!(locale.to_string(), "ml");
///
/// let locale = Locale::Maori;
/// assert_eq!(locale.to_string(), "mi");
///
/// let locale = Locale::Marathi;
/// assert_eq!(locale.to_string(), "mr");
///
/// let locale = Locale::Malay;
/// assert_eq!(locale.to_string(), "ms");
///
/// let locale = Locale::Malagasy;
/// assert_eq!(locale.to_string(), "mg");
///
/// let locale = Locale::Maltese;
/// assert_eq!(locale.to_string(), "mt");
///
/// let locale = Locale::Mongolian;
/// assert_eq!(locale.to_string(), "mn");
///
/// let locale = Locale::Nauru;
/// assert_eq!(locale.to_string(), "na");
///
/// let locale = Locale::Navajo;
/// assert_eq!(locale.to_string(), "nv");
///
/// let locale = Locale::SouthernNdebele;
/// assert_eq!(locale.to_string(), "nr");
///
/// let locale = Locale::NorthernNdebele;
/// assert_eq!(locale.to_string(), "nd");
///
/// let locale = Locale::Ndonga;
/// assert_eq!(locale.to_string(), "ng");
///
/// let locale = Locale::Nepali;
/// assert_eq!(locale.to_string(), "ne");
///
/// let locale = Locale::NorwegianNynorsk;
/// assert_eq!(locale.to_string(), "nn");
///
/// let locale = Locale::Norwegian;
/// assert_eq!(locale.to_string(), "no");
///
/// let locale = Locale::Chichewa;
/// assert_eq!(locale.to_string(), "ny");
///
/// let locale = Locale::Occitan;
/// assert_eq!(locale.to_string(), "oc");
///
/// let locale = Locale::Ojibwa;
/// assert_eq!(locale.to_string(), "oj");
///
/// let locale = Locale::Oriya;
/// assert_eq!(locale.to_string(), "or");
///
/// let locale = Locale::Oromo;
/// assert_eq!(locale.to_string(), "om");
///
/// let locale = Locale::Ossetian;
/// assert_eq!(locale.to_string(), "os");
///
/// let locale = Locale::Panjabi;
/// assert_eq!(locale.to_string(), "pa");
///
/// let locale = Locale::Pali;
/// assert_eq!(locale.to_string(), "pi");
///
/// let locale = Locale::Polish;
/// assert_eq!(locale.to_string(), "pl");
///
/// let locale = Locale::Pushto;
/// assert_eq!(locale.to_string(), "ps");
///
/// let locale = Locale::Quechua;
/// assert_eq!(locale.to_string(), "qu");
///
/// let locale = Locale::Romansh;
/// assert_eq!(locale.to_string(), "rm");
///
/// let locale = Locale::Rundi;
/// assert_eq!(locale.to_string(), "rn");
///
/// let locale = Locale::Sango;
/// assert_eq!(locale.to_string(), "sg");
///
/// let locale = Locale::Sanskrit;
/// assert_eq!(locale.to_string(), "sa");
///
/// let locale = Locale::Sinhala;
/// assert_eq!(locale.to_string(), "si");
///
/// let locale = Locale::Slovak;
/// assert_eq!(locale.to_string(), "sk");
///
/// let locale = Locale::Slovenian;
/// assert_eq!(locale.to_string(), "sl");
///
/// let locale = Locale::NorthernSami;
/// assert_eq!(locale.to_string(), "se");
///
/// let locale = Locale::Samoan;
/// assert_eq!(locale.to_string(), "sm");
///
/// let locale = Locale::Shona;
/// assert_eq!(locale.to_string(), "sn");
///
/// let locale = Locale::Sindhi;
/// assert_eq!(locale.to_string(), "sd");
///
/// let locale = Locale::Somali;
/// assert_eq!(locale.to_string(), "so");
///
/// let locale = Locale::SouthernSotho;
/// assert_eq!(locale.to_string(), "st");
///
/// let locale = Locale::Sardinian;
/// assert_eq!(locale.to_string(), "sc");
///
/// let locale = Locale::Serbian;
/// assert_eq!(locale.to_string(), "sr");
///
/// let locale = Locale::Swati;
/// assert_eq!(locale.to_string(), "ss");
///
/// let locale = Locale::Sundanese;
/// assert_eq!(locale.to_string(), "su");
///
/// let locale = Locale::Swahili;
/// assert_eq!(locale.to_string(), "sw");
///
/// let locale = Locale::Tahitian;
/// assert_eq!(locale.to_string(), "ty");
///
/// let locale = Locale::Tamil;
/// assert_eq!(locale.to_string(), "ta");
///
/// let locale = Locale::Tatar;
/// assert_eq!(locale.to_string(), "tt");
///
/// let locale = Locale::Telugu;
/// assert_eq!(locale.to_string(), "te");
///
/// let locale = Locale::Tajik;
/// assert_eq!(locale.to_string(), "tg");
///
/// let locale = Locale::Tagalog;
/// assert_eq!(locale.to_string(), "tl");
///
/// let locale = Locale::Thai;
/// assert_eq!(locale.to_string(), "th");
///
/// let locale = Locale::Tigrinya;
/// assert_eq!(locale.to_string(), "ti");
///
/// let locale = Locale::Tonga;
/// assert_eq!(locale.to_string(), "to");
///
/// let locale = Locale::Tswana;
/// assert_eq!(locale.to_string(), "tn");
///
/// let locale = Locale::Tsonga;
/// assert_eq!(locale.to_string(), "ts");
///
/// let locale = Locale::Turkmen;
/// assert_eq!(locale.to_string(), "tk");
///
/// let locale = Locale::Turkish;
/// assert_eq!(locale.to_string(), "tr");
///
/// let locale = Locale::Twi;
/// assert_eq!(locale.to_string(), "tw");
///
/// let locale = Locale::Uighur;
/// assert_eq!(locale.to_string(), "ug");
///
/// let locale = Locale::Ukrainian;
/// assert_eq!(locale.to_string(), "uk");
///
/// let locale = Locale::Urdu;
/// assert_eq!(locale.to_string(), "ur");
///
/// let locale = Locale::Uzbek;
/// assert_eq!(locale.to_string(), "uz");
///
/// let locale = Locale::Venda;
/// assert_eq!(locale.to_string(), "ve");
///
/// let locale = Locale::Vietnamese;
/// assert_eq!(locale.to_string(), "vi");
///
/// let locale = Locale::Walloon;
/// assert_eq!(locale.to_string(), "wa");
///
/// let locale = Locale::Wolof;
/// assert_eq!(locale.to_string(), "wo");
///
/// let locale = Locale::Xhosa;
/// assert_eq!(locale.to_string(), "xh");
///
/// let locale = Locale::Yiddish;
/// assert_eq!(locale.to_string(), "yi");
///
/// let locale = Locale::Yoruba;
/// assert_eq!(locale.to_string(), "yo");
///
/// let locale = Locale::Zhuang;
/// assert_eq!(locale.to_string(), "za");
///
/// let locale = Locale::Zulu;
/// assert_eq!(locale.to_string(), "zu");
/// ```
impl Display for Locale {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let code = match self {
            Locale::Afar => "aa",
            Locale::Abkhazian => "ab",
            Locale::Afrikaans => "af",
            Locale::Akan => "ak",
            Locale::Albanian => "sq",
            Locale::Amharic => "am",
            Locale::Arabic(ArabicVariant::Default) => "ar",
            Locale::Arabic(ArabicVariant::Algeria) => "ar_DZ",
            Locale::Arabic(ArabicVariant::Bahrain) => "ar_BH",
            Locale::Arabic(ArabicVariant::Egypt) => "ar_EG",
            Locale::Arabic(ArabicVariant::Iraq) => "ar_IQ",
            Locale::Arabic(ArabicVariant::Jordan) => "ar_JO",
            Locale::Arabic(ArabicVariant::Kuwait) => "ar_KW",
            Locale::Arabic(ArabicVariant::Lebanon) => "ar_LB",
            Locale::Arabic(ArabicVariant::Libya) => "ar_LY",
            Locale::Arabic(ArabicVariant::Morocco) => "ar_MA",
            Locale::Arabic(ArabicVariant::Oman) => "ar_OM",
            Locale::Arabic(ArabicVariant::Qatar) => "ar_QA",
            Locale::Arabic(ArabicVariant::SaudiArabia) => "ar_SA",
            Locale::Arabic(ArabicVariant::Syria) => "ar_SY",
            Locale::Arabic(ArabicVariant::Tunisia) => "ar_TN",
            Locale::Arabic(ArabicVariant::UnitedArabEmirates) => "ar_AE",
            Locale::Arabic(ArabicVariant::Yemen) => "ar_YE",
            Locale::Aragonese => "an",
            Locale::Armenian => "hy",
            Locale::Assamese => "as",
            Locale::Avaric => "av",
            Locale::Avestan => "ae",
            Locale::Aymara => "ay",
            Locale::Azerbaijani => "az",
            Locale::Bashkir => "ba",
            Locale::Bambara => "bm",
            Locale::Basque => "eu",
            Locale::Belarusian => "be",
            Locale::Bengali => "bn",
            Locale::Bihari => "bh",
            Locale::Bislama => "bi",
            Locale::Tibetan => "bo",
            Locale::Bosnian => "bs",
            Locale::Breton => "br",
            Locale::Bulgarian => "bg",
            Locale::Burmese => "my",
            Locale::Catalan => "ca",
            Locale::Czech => "cs",
            Locale::Chamorro => "ch",
            Locale::Chechen => "ce",
            Locale::Chinese(ChineseVariant::Default) => "zh",
            Locale::Chinese(ChineseVariant::HongKong) => "zh_HK",
            Locale::Chinese(ChineseVariant::China) => "zh_CN",
            Locale::Chinese(ChineseVariant::Singapore) => "zh_SG",
            Locale::Chinese(ChineseVariant::Taiwan) => "zh_TW",
            Locale::ChurchSlavic => "cu",
            Locale::Chuvash => "cv",
            Locale::Cornish => "kw",
            Locale::Corsican => "co",
            Locale::Cree => "cr",
            Locale::Welsh => "cy",
            Locale::Danish => "da",
            Locale::German(GermanVariant::Default) => "de",
            Locale::German(GermanVariant::Austria) => "de_AT",
            Locale::German(GermanVariant::Liechtenstein) => "de_LI",
            Locale::German(GermanVariant::Luxembourg) => "de_LU",
            Locale::German(GermanVariant::Switzerland) => "de_CH",
            Locale::Divehi => "dv",
            Locale::Dutch(DutchVariant::Default) => "nl",
            Locale::Dutch(DutchVariant::Belgium) => "nl_BE",
            Locale::Dzongkha => "dz",
            Locale::Greek => "el",
            Locale::English(EnglishVariant::Default) => "en",
            Locale::English(EnglishVariant::Australia) => "en_AU",
            Locale::English(EnglishVariant::Belize) => "en_BZ",
            Locale::English(EnglishVariant::Canada) => "en_CA",
            Locale::English(EnglishVariant::Ireland) => "en_IE",
            Locale::English(EnglishVariant::Jamaica) => "en_JM",
            Locale::English(EnglishVariant::NewZealand) => "en_NZ",
            Locale::English(EnglishVariant::SouthAfrica) => "en_ZA",
            Locale::English(EnglishVariant::Trinidad) => "en_TT",
            Locale::English(EnglishVariant::UnitedKingdom) => "en_GB",
            Locale::English(EnglishVariant::UnitedStates) => "en_US",
            Locale::Esperanto => "eo",
            Locale::Estonian => "et",
            Locale::Ewe => "ee",
            Locale::Faroese => "fo",
            Locale::Persian => "fa",
            Locale::Fijian => "fj",
            Locale::Finnish => "fi",
            Locale::French(FrenchVariant::Default) => "fr",
            Locale::French(FrenchVariant::France) => "fr_FR",
            Locale::French(FrenchVariant::Belgium) => "fr_BE",
            Locale::French(FrenchVariant::Canada) => "fr_CA",
            Locale::French(FrenchVariant::Luxembourg) => "fr_LU",
            Locale::French(FrenchVariant::Switzerland) => "fr_CH",
            Locale::WesternFrisian => "fy",
            Locale::Fulah => "ff",
            Locale::Georgian => "ka",
            Locale::Gaelic => "gd",
            Locale::Irish => "ga",
            Locale::Galician => "gl",
            Locale::Manx => "gv",
            Locale::Guarani => "gn",
            Locale::Gujarati => "gu",
            Locale::Haitian => "ht",
            Locale::Hausa => "ha",
            Locale::Hebrew => "he",
            Locale::Herero => "hz",
            Locale::Hindi => "hi",
            Locale::HiriMotu => "ho",
            Locale::Croatian => "hr",
            Locale::Hungarian => "hu",
            Locale::Igbo => "ig",
            Locale::Icelandic => "is",
            Locale::Ido => "io",
            Locale::SichuanYi => "ii",
            Locale::Inuktitut => "iu",
            Locale::Interlingue => "ie",
            Locale::Indonesian => "id",
            Locale::Inupiaq => "ik",
            Locale::Italian(ItalianVariant::Default) => "it",
            Locale::Italian(ItalianVariant::Switzerland) => "it_CH",
            Locale::Javanese => "jv",
            Locale::Japanese => "ja",
            Locale::Kalaallisut => "kl",
            Locale::Kannada => "kn",
            Locale::Kashmiri => "ks",
            Locale::Kanuri => "kr",
            Locale::Kazakh => "kk",
            Locale::CentralKhmer => "km",
            Locale::Kikuyu => "ki",
            Locale::Kinyarwanda => "rw",
            Locale::Kirghiz => "ky",
            Locale::Komi => "kv",
            Locale::Kongo => "kg",
            Locale::Korean => "ko",
            Locale::Kuanyama => "kj",
            Locale::Kurdish => "ku",
            Locale::Lao => "lo",
            Locale::Latin => "la",
            Locale::Latvian => "lv",
            Locale::Limburgan => "li",
            Locale::Lingala => "ln",
            Locale::Lithuanian => "lt",
            Locale::Luxembourgish => "lb",
            Locale::LubaKatanga => "lu",
            Locale::Ganda => "lg",
            Locale::Macedonian => "mk",
            Locale::Marshallese => "mh",
            Locale::Malayalam => "ml",
            Locale::Maori => "mi",
            Locale::Marathi => "mr",
            Locale::Malay => "ms",
            Locale::Malagasy => "mg",
            Locale::Maltese => "mt",
            Locale::Mongolian => "mn",
            Locale::Nauru => "na",
            Locale::Navajo => "nv",
            Locale::SouthernNdebele => "nr",
            Locale::NorthernNdebele => "nd",
            Locale::Ndonga => "ng",
            Locale::Nepali => "ne",
            Locale::NorwegianNynorsk => "nn",
            Locale::Norwegian => "no",
            Locale::Chichewa => "ny",
            Locale::Occitan => "oc",
            Locale::Ojibwa => "oj",
            Locale::Oriya => "or",
            Locale::Oromo => "om",
            Locale::Ossetian => "os",
            Locale::Panjabi => "pa",
            Locale::Pali => "pi",
            Locale::Polish => "pl",
            Locale::Portuguese(PortugueseVariant::Default) => "pt",
            Locale::Portuguese(PortugueseVariant::Brazil) => "pt_BR",
            Locale::Pushto => "ps",
            Locale::Quechua => "qu",
            Locale::Romansh => "rm",
            Locale::Romanian(RomanianVariant::Default) => "ro",
            Locale::Romanian(RomanianVariant::Moldova) => "ro_MD",
            Locale::Rundi => "rn",
            Locale::Russian(RussianVariant::Default) => "ru",
            Locale::Russian(RussianVariant::Moldova) => "ru_MD",
            Locale::Sango => "sg",
            Locale::Sanskrit => "sa",
            Locale::Sinhala => "si",
            Locale::Slovak => "sk",
            Locale::Slovenian => "sl",
            Locale::NorthernSami => "se",
            Locale::Samoan => "sm",
            Locale::Shona => "sn",
            Locale::Sindhi => "sd",
            Locale::Somali => "so",
            Locale::SouthernSotho => "st",
            Locale::Spanish(SpanishVariant::Default) => "es",
            Locale::Spanish(SpanishVariant::Argentina) => "es_AR",
            Locale::Spanish(SpanishVariant::Bolivia) => "es_BO",
            Locale::Spanish(SpanishVariant::Chile) => "es_CL",
            Locale::Spanish(SpanishVariant::Colombia) => "es_CO",
            Locale::Spanish(SpanishVariant::CostaRica) => "es_CR",
            Locale::Spanish(SpanishVariant::DominicanRepublic) => "es_DO",
            Locale::Spanish(SpanishVariant::Ecuador) => "es_EC",
            Locale::Spanish(SpanishVariant::ElSalvador) => "es_SV",
            Locale::Spanish(SpanishVariant::Guatemala) => "es_GT",
            Locale::Spanish(SpanishVariant::Honduras) => "es_HN",
            Locale::Spanish(SpanishVariant::Mexico) => "es_MX",
            Locale::Spanish(SpanishVariant::Nicaragua) => "es_NI",
            Locale::Spanish(SpanishVariant::Panama) => "es_PA",
            Locale::Spanish(SpanishVariant::Paraguay) => "es_PY",
            Locale::Spanish(SpanishVariant::Peru) => "es_PE",
            Locale::Spanish(SpanishVariant::PuertoRico) => "es_PR",
            Locale::Spanish(SpanishVariant::Uruguay) => "es_UY",
            Locale::Spanish(SpanishVariant::Venezuela) => "es_VE",
            Locale::Sardinian => "sc",
            Locale::Serbian => "sr",
            Locale::Swati => "ss",
            Locale::Sundanese => "su",
            Locale::Swahili => "sw",
            Locale::Swedish(SwedishVariant::Default) => "sv",
            Locale::Swedish(SwedishVariant::Finland) => "sv_FI",
            Locale::Tahitian => "ty",
            Locale::Tamil => "ta",
            Locale::Tatar => "tt",
            Locale::Telugu => "te",
            Locale::Tajik => "tg",
            Locale::Tagalog => "tl",
            Locale::Thai => "th",
            Locale::Tigrinya => "ti",
            Locale::Tonga => "to",
            Locale::Tswana => "tn",
            Locale::Tsonga => "ts",
            Locale::Turkmen => "tk",
            Locale::Turkish => "tr",
            Locale::Twi => "tw",
            Locale::Uighur => "ug",
            Locale::Ukrainian => "uk",
            Locale::Urdu => "ur",
            Locale::Uzbek => "uz",
            Locale::Venda => "ve",
            Locale::Vietnamese => "vi",
            Locale::Walloon => "wa",
            Locale::Wolof => "wo",
            Locale::Xhosa => "xh",
            Locale::Yiddish => "yi",
            Locale::Yoruba => "yo",
            Locale::Zhuang => "za",
            Locale::Zulu => "zu",
        };

        write!(f, "{}", code)
    }
}

/// A `ArabicVariant` enum for `Arabic` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::ArabicVariant;
///
/// let locale = Locale::Arabic(ArabicVariant::Default);
/// assert_eq!(locale.to_string(), "ar");
///
/// let locale = Locale::Arabic(ArabicVariant::Algeria);
/// assert_eq!(locale.to_string(), "ar_DZ");
///
/// let locale = Locale::Arabic(ArabicVariant::Bahrain);
/// assert_eq!(locale.to_string(), "ar_BH");
///
/// let locale = Locale::Arabic(ArabicVariant::Egypt);
/// assert_eq!(locale.to_string(), "ar_EG");
///
/// let locale = Locale::Arabic(ArabicVariant::Iraq);
/// assert_eq!(locale.to_string(), "ar_IQ");
///
/// let locale = Locale::Arabic(ArabicVariant::Jordan);
/// assert_eq!(locale.to_string(), "ar_JO");
///
/// let locale = Locale::Arabic(ArabicVariant::Kuwait);
/// assert_eq!(locale.to_string(), "ar_KW");
///
/// let locale = Locale::Arabic(ArabicVariant::Lebanon);
/// assert_eq!(locale.to_string(), "ar_LB");
///
/// let locale = Locale::Arabic(ArabicVariant::Libya);
/// assert_eq!(locale.to_string(), "ar_LY");
///
/// let locale = Locale::Arabic(ArabicVariant::Morocco);
/// assert_eq!(locale.to_string(), "ar_MA");
///
/// let locale = Locale::Arabic(ArabicVariant::Oman);
/// assert_eq!(locale.to_string(), "ar_OM");
///
/// let locale = Locale::Arabic(ArabicVariant::Qatar);
/// assert_eq!(locale.to_string(), "ar_QA");
///
/// let locale = Locale::Arabic(ArabicVariant::SaudiArabia);
/// assert_eq!(locale.to_string(), "ar_SA");
///
/// let locale = Locale::Arabic(ArabicVariant::Syria);
/// assert_eq!(locale.to_string(), "ar_SY");
///
/// let locale = Locale::Arabic(ArabicVariant::Tunisia);
/// assert_eq!(locale.to_string(), "ar_TN");
///
/// let locale = Locale::Arabic(ArabicVariant::UnitedArabEmirates);
/// assert_eq!(locale.to_string(), "ar_AE");
///
/// let locale = Locale::Arabic(ArabicVariant::Yemen);
/// assert_eq!(locale.to_string(), "ar_YE");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArabicVariant {
    Default,
    Algeria,
    Bahrain,
    Egypt,
    Iraq,
    Jordan,
    Kuwait,
    Lebanon,
    Libya,
    Morocco,
    Oman,
    Qatar,
    SaudiArabia,
    Syria,
    Tunisia,
    UnitedArabEmirates,
    Yemen,
}

unsafe impl Sync for ArabicVariant {}
unsafe impl Send for ArabicVariant {}

/// A `ChineseVariant` enum for `Chinese` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::ChineseVariant;
///
/// let locale = Locale::Chinese(ChineseVariant::Default);
/// assert_eq!(locale.to_string(), "zh");
///
/// let locale = Locale::Chinese(ChineseVariant::HongKong);
/// assert_eq!(locale.to_string(), "zh_HK");
///
/// let locale = Locale::Chinese(ChineseVariant::China);
/// assert_eq!(locale.to_string(), "zh_CN");
///
/// let locale = Locale::Chinese(ChineseVariant::Singapore);
/// assert_eq!(locale.to_string(), "zh_SG");
///
/// let locale = Locale::Chinese(ChineseVariant::Taiwan);
/// assert_eq!(locale.to_string(), "zh_TW");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChineseVariant {
    Default,
    HongKong,
    China,
    Singapore,
    Taiwan,
}

unsafe impl Sync for ChineseVariant {}
unsafe impl Send for ChineseVariant {}

/// A `GermanVariant` enum for `German` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::GermanVariant;
///
/// let locale = Locale::German(GermanVariant::Default);
/// assert_eq!(locale.to_string(), "de");
///
/// let locale = Locale::German(GermanVariant::Austria);
/// assert_eq!(locale.to_string(), "de_AT");
///
/// let locale = Locale::German(GermanVariant::Liechtenstein);
/// assert_eq!(locale.to_string(), "de_LI");
///
/// let locale = Locale::German(GermanVariant::Luxembourg);
/// assert_eq!(locale.to_string(), "de_LU");
///
/// let locale = Locale::German(GermanVariant::Switzerland);
/// assert_eq!(locale.to_string(), "de_CH");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GermanVariant {
    Default,
    Austria,
    Liechtenstein,
    Luxembourg,
    Switzerland,
}

unsafe impl Sync for GermanVariant {}
unsafe impl Send for GermanVariant {}

/// A `DutchVariant` enum for `Dutch` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::DutchVariant;
///
/// let locale = Locale::Dutch(DutchVariant::Default);
/// assert_eq!(locale.to_string(), "nl");
///
/// let locale = Locale::Dutch(DutchVariant::Belgium);
/// assert_eq!(locale.to_string(), "nl_BE");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DutchVariant {
    Default,
    Belgium,
}

unsafe impl Sync for DutchVariant {}
unsafe impl Send for DutchVariant {}

/// A `EnglishVariant` enum for `English` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::EnglishVariant;
///
/// let locale = Locale::English(EnglishVariant::Default);
/// assert_eq!(locale.to_string(), "en");
///
/// let locale = Locale::English(EnglishVariant::Australia);
/// assert_eq!(locale.to_string(), "en_AU");
///
/// let locale = Locale::English(EnglishVariant::Belize);
/// assert_eq!(locale.to_string(), "en_BZ");
///
/// let locale = Locale::English(EnglishVariant::Canada);
/// assert_eq!(locale.to_string(), "en_CA");
///
/// let locale = Locale::English(EnglishVariant::Ireland);
/// assert_eq!(locale.to_string(), "en_IE");
///
/// let locale = Locale::English(EnglishVariant::Jamaica);
/// assert_eq!(locale.to_string(), "en_JM");
///
/// let locale = Locale::English(EnglishVariant::NewZealand);
/// assert_eq!(locale.to_string(), "en_NZ");
///
/// let locale = Locale::English(EnglishVariant::SouthAfrica);
/// assert_eq!(locale.to_string(), "en_ZA");
///
/// let locale = Locale::English(EnglishVariant::Trinidad);
/// assert_eq!(locale.to_string(), "en_TT");
///
/// let locale = Locale::English(EnglishVariant::UnitedKingdom);
/// assert_eq!(locale.to_string(), "en_GB");
///
/// let locale = Locale::English(EnglishVariant::UnitedStates);
/// assert_eq!(locale.to_string(), "en_US");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnglishVariant {
    Default,
    Australia,
    Belize,
    Canada,
    Ireland,
    Jamaica,
    NewZealand,
    SouthAfrica,
    Trinidad,
    UnitedKingdom,
    UnitedStates,
}

unsafe impl Sync for EnglishVariant {}
unsafe impl Send for EnglishVariant {}

/// A `FrenchVariant` enum for `French` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::FrenchVariant;
///
/// let locale = Locale::French(FrenchVariant::Default);
/// assert_eq!(locale.to_string(), "fr");
///
/// let locale = Locale::French(FrenchVariant::France);
/// assert_eq!(locale.to_string(), "fr_FR");
///
/// let locale = Locale::French(FrenchVariant::Belgium);
/// assert_eq!(locale.to_string(), "fr_BE");
///
/// let locale = Locale::French(FrenchVariant::Canada);
/// assert_eq!(locale.to_string(), "fr_CA");
///
/// let locale = Locale::French(FrenchVariant::Luxembourg);
/// assert_eq!(locale.to_string(), "fr_LU");
///
/// let locale = Locale::French(FrenchVariant::Switzerland);
/// assert_eq!(locale.to_string(), "fr_CH");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrenchVariant {
    Default,
    France,
    Belgium,
    Canada,
    Luxembourg,
    Switzerland,
}

unsafe impl Sync for FrenchVariant {}
unsafe impl Send for FrenchVariant {}

/// A `ItalianVariant` enum for `Italian` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::ItalianVariant;
///
/// let locale = Locale::Italian(ItalianVariant::Default);
/// assert_eq!(locale.to_string(), "it");
///
/// let locale = Locale::Italian(ItalianVariant::Switzerland);
/// assert_eq!(locale.to_string(), "it_CH");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItalianVariant {
    Default,
    Switzerland,
}

unsafe impl Sync for ItalianVariant {}
unsafe impl Send for ItalianVariant {}

/// A `PortugueseVariant` enum for `Portuguese` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::PortugueseVariant;
///
/// let locale = Locale::Portuguese(PortugueseVariant::Default);
/// assert_eq!(locale.to_string(), "pt");
///
/// let locale = Locale::Portuguese(PortugueseVariant::Brazil);
/// assert_eq!(locale.to_string(), "pt_BR");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortugueseVariant {
    Default,
    Brazil,
}

unsafe impl Sync for PortugueseVariant {}
unsafe impl Send for PortugueseVariant {}

/// A `RomanianVariant` enum for `Romanian` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::RomanianVariant;
///
/// let locale = Locale::Romanian(RomanianVariant::Default);
/// assert_eq!(locale.to_string(), "ro");
///
/// let locale = Locale::Romanian(RomanianVariant::Moldova);
/// assert_eq!(locale.to_string(), "ro_MD");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RomanianVariant {
    Default,
    Moldova,
}

unsafe impl Sync for RomanianVariant {}
unsafe impl Send for RomanianVariant {}

/// A `RussianVariant` enum for `Russian` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::RussianVariant;
///
/// let locale = Locale::Russian(RussianVariant::Default);
/// assert_eq!(locale.to_string(), "ru");
///
/// let locale = Locale::Russian(RussianVariant::Moldova);
/// assert_eq!(locale.to_string(), "ru_MD");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RussianVariant {
    Default,
    Moldova,
}

unsafe impl Sync for RussianVariant {}
unsafe impl Send for RussianVariant {}

/// A `SpanishVariant` enum for `Spanish` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::SpanishVariant;
///
/// let locale = Locale::Spanish(SpanishVariant::Default);
/// assert_eq!(locale.to_string(), "es");
///
/// let locale = Locale::Spanish(SpanishVariant::Argentina);
/// assert_eq!(locale.to_string(), "es_AR");
///
/// let locale = Locale::Spanish(SpanishVariant::Bolivia);
/// assert_eq!(locale.to_string(), "es_BO");
///
/// let locale = Locale::Spanish(SpanishVariant::Chile);
/// assert_eq!(locale.to_string(), "es_CL");
///
/// let locale = Locale::Spanish(SpanishVariant::Colombia);
/// assert_eq!(locale.to_string(), "es_CO");
///
/// let locale = Locale::Spanish(SpanishVariant::CostaRica);
/// assert_eq!(locale.to_string(), "es_CR");
///
/// let locale = Locale::Spanish(SpanishVariant::DominicanRepublic);
/// assert_eq!(locale.to_string(), "es_DO");
///
/// let locale = Locale::Spanish(SpanishVariant::Ecuador);
/// assert_eq!(locale.to_string(), "es_EC");
///
/// let locale = Locale::Spanish(SpanishVariant::ElSalvador);
/// assert_eq!(locale.to_string(), "es_SV");
///
/// let locale = Locale::Spanish(SpanishVariant::Guatemala);
/// assert_eq!(locale.to_string(), "es_GT");
///
/// let locale = Locale::Spanish(SpanishVariant::Honduras);
/// assert_eq!(locale.to_string(), "es_HN");
///
/// let locale = Locale::Spanish(SpanishVariant::Mexico);
/// assert_eq!(locale.to_string(), "es_MX");
///
/// let locale = Locale::Spanish(SpanishVariant::Nicaragua);
/// assert_eq!(locale.to_string(), "es_NI");
///
/// let locale = Locale::Spanish(SpanishVariant::Panama);
/// assert_eq!(locale.to_string(), "es_PA");
///
/// let locale = Locale::Spanish(SpanishVariant::Paraguay);
/// assert_eq!(locale.to_string(), "es_PY");
///
/// let locale = Locale::Spanish(SpanishVariant::Peru);
/// assert_eq!(locale.to_string(), "es_PE");
///
/// let locale = Locale::Spanish(SpanishVariant::PuertoRico);
/// assert_eq!(locale.to_string(), "es_PR");
///
/// let locale = Locale::Spanish(SpanishVariant::Uruguay);
/// assert_eq!(locale.to_string(), "es_UY");
///
/// let locale = Locale::Spanish(SpanishVariant::Venezuela);
/// assert_eq!(locale.to_string(), "es_VE");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpanishVariant {
    Default,
    Argentina,
    Bolivia,
    Chile,
    Colombia,
    CostaRica,
    DominicanRepublic,
    Ecuador,
    ElSalvador,
    Guatemala,
    Honduras,
    Mexico,
    Nicaragua,
    Panama,
    Paraguay,
    Peru,
    PuertoRico,
    Uruguay,
    Venezuela,
}

unsafe impl Sync for SpanishVariant {}
unsafe impl Send for SpanishVariant {}

/// A `SwedishVariant` enum for `Swedish` locale.
///
/// # Examples
///
/// ```
/// use tarjama::locale::Locale;
/// use tarjama::locale::SwedishVariant;
///
/// let locale = Locale::Swedish(SwedishVariant::Default);
/// assert_eq!(locale.to_string(), "sv");
///
/// let locale = Locale::Swedish(SwedishVariant::Finland);
/// assert_eq!(locale.to_string(), "sv_FI");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwedishVariant {
    Default,
    Finland,
}

unsafe impl Sync for SwedishVariant {}
unsafe impl Send for SwedishVariant {}
