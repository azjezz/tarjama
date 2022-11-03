use tarjama::error::Error;
use tarjama::loader::toml;
use tarjama::{context, Translator};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let bag = toml::load("examples/translations").await?;

    let translator = Translator::with_catalogue_bag(bag);

    let english = translator.trans(
        "en",
        "messages",
        "greeting",
        context!(name = "Rust"),
    )?;
    let french = translator.trans(
        "fr",
        "messages",
        "greeting",
        context!(name = "Rust"),
    )?;

    println!("en: {english} / fr: {french}");

    Ok(())
}
