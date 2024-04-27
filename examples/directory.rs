use tarjama::context;
use tarjama::error::Error;
use tarjama::loader::toml;
use tarjama::locale;
use tarjama::Translator;

fn main() -> Result<(), Error> {
    let catalogue_bag = toml::load_sync("examples/translations")?;

    let mut translator = Translator::with_catalogue_bag(catalogue_bag);

    translator.set_fallback_locale(locale::Locale::English(
        locale::EnglishVariant::Default,
    ));

    let locales = vec![
        locale::Locale::English(locale::EnglishVariant::Default),
        locale::Locale::French(locale::FrenchVariant::Default),
        locale::Locale::Chinese(locale::ChineseVariant::Default),
        locale::Locale::Arabic(locale::ArabicVariant::Default),
    ];

    for locale in locales {
        let greeting = translator.trans(
            &locale,
            "messages",
            "greeting",
            context!(name = "Rust"),
        )?;
        println!("{}", greeting);

        for i in [0, 1, 4, 10] {
            let translation = translator.trans(
                &locale,
                "messages",
                "apple",
                context!(? = i),
            )?;
            println!("{}", translation);
        }
    }

    Ok(())
}
