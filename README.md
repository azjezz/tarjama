# 🌍 Tarjama: Supercharge Your Rust App Internationalization 🚀

Welcome to Tarjama, your go-to library for making your Rust applications accessible to speakers of different languages around the globe! 🎉

[![Actions Status](https://github.com/azjezz/tarjama/workflows/ci/badge.svg)](https://github.com/azjezz/tarjama/actions)
[![Crates.io](https://img.shields.io/crates/v/tarjama.svg)](https://crates.io/crates/tarjama)
[![Docs](https://docs.rs/tarjama/badge.svg)](https://docs.rs/tarjama/latest/tarjama/)

## What's Tarjama? 🤔

Tarjama enables you to easily internationalize your Rust applications, supporting an extensive list of locales found in our `Locale` enum (`src/locale.rs`). With Tarjama, you can effortlessly switch between languages, catering to a global audience. 🌐

## Getting Started 🚀

Add Tarjama to your project using Cargo:

```shell
cargo add tarjama
```

### Sample Usage 🛠

**Basic Usage**:

```rust
use tarjama::Translator;
use tarjama::context;

// load translations from a directory of toml files
let catalogue_bag = toml::load("path/to/translations").await?;
let mut translator = Translator::from_catalogue_bag(catalogue_bag);
// set the fallback locale to English
translator.set_fallback_locale("en");

// translate the message 'greeting' in the 'messages' domain with the context 'name' set to 'World' in English.
let message_en = translator.trans("en", "messages", "greeting", context!(name = "World"))?;
// translate the message 'greeting' in the 'messages' domain with the context 'name' set to '世界' in Chinese,
// if the message is not found in Chinese, it will fallback to English, as it is set as the fallback locale.
let message_zh = translator.trans("zh", "messages", "greeting", context!(name = "世界"))?;

println!(message);
```

> [!NOTE]
> Each message file in the translation directory is expected to be named using the `{domain}.{locale}.toml` format.
> For example, `messages.en.toml` for the English locale.

**Manual Catalogue Creation**:

Tarjama allows you to create your catalogue bag manually, as shown below:

```rust
use std::collections::HashMap;
use tarjama::Translator;
use tarjama::context;
use tarjama::catalogue::{Catalogue, CatalogueBag};
use tarjama::locale::{Locale, EnglishVariant};

let catalogue_bag = CatalogueBag::from_catalogues(vec![
    Catalogue::from_messages(Locale::English(EnglishVariant::Default), HashMap::from([
        ("messages".to_owned(), HashMap::from([
          ("greeting".to_owned(), "Hello, {name}!".to_owned()),
        ]))
    ])),
]);

let translator = Translator::from_catalogue_bag(catalogue_bag);

let message = translator.trans("en", "messages", "greeting", context!(name = "World"))?;

println!(message);
```

> [!NOTE]
> If you plan on creating your catalogue bag manually, you could remove the `toml` feature from your `Cargo.toml`.
>
> ```bash
> cargo add tarjama --no-default-features
> ```
>
> This will reduce the size of your binary and remove the dependency on the `toml` crate.

### Actix Web Integration 🌐

Tarjama provides an Actix Web middleware for easy integration with Actix Web applications. To use the middleware, add the `actix-web` feature to your `Cargo.toml`:

```toml
tarjama = { version = "0.2", features = ["actix-web"] }
```

Then, add the middleware to your Actix Web application:

```rust
use actix_web::{web, App, HttpServer};
use tarjama::actix::TarjamaMiddleware;
use tarjama::Translator;
use tarjama::context;
use tarjama::locale::{Locale, EnglishVariant};

async fn example(translator: Translator, locale: Locale) -> Result<HttpResponse> {
    let content = translator.trans(locale, "messages", "greeting", context!(name = "World"))
            .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let translator = Translator::with_catalogue_bag(
        load("/path/to/translations/").await.expect("couldn't load translations"),
    );

    HttpServer::new(move || {
        App::new()
            .wrap(TranslatorMiddleware::new(translator.clone(), Locale::English(EnglishVariant::Default)))
            .route("/", web::get().to(example))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## Feature Flags 🚩</summary>
  - **actix-web**: For Actix Web middleware support, enabling this feature allows you to use the Tarjama middleware with Actix Web applications.
  - **file**: For basic file loader operations, this feature is useful for implementing custom loaders.
  - **toml**: For toml file support, enabling this feature allows you to load translations from toml files.
  - **default**: Includes `toml`.

## Licensing 📜

Tarjama is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

Choose the license that best suits your project's needs.

## Contribution 🤝

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Dive In 🏊

Ready to make your Rust app globally accessible? Integrate Tarjama today and let's break language barriers together!
