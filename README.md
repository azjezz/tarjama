# Tarjama

Tarjama helps internationalize your rust applications.

[![Actions Status](https://github.com/azjezz/tarjama/workflows/ci/badge.svg)](https://github.com/azjezz/tarjama/actions)
[![Crates.io](https://img.shields.io/crates/v/tarjama.svg)](https://crates.io/crates/tarjama)
[![Docs](https://docs.rs/tarjama/badge.svg)](https://docs.rs/tarjama/latest/tarjama/)

## Usage

To bring this crate into your repository, either add `tarjama` to your `Cargo.toml`, or run `cargo add tarjama`.

Here's a simple example that loads translations from `translations/` directory, translates and prints `greeting` message:

```rust
use tarjama::Translator;

use tarjama::loader::toml;
use tarjama::context;

let catalogue_bag = toml::load("/path/to/translations/").await?;
let translator = Translator::from_catalogue_bag(catalogue_bag);

let message = translator.trans("en", "messages", "greeting", context!(name = "World"))?;

println!("{message}");
```

> Note: Each message file in the translation directory is expected to be named using the `{domain}.{locale}.toml` format.

Plurazation is also supported:

```rust
use tarjama::catalogue::{Catalogue, CatalogueBag};
use tarjama::locale::{Locale, EnglishVariant};
use tarjama::Translator;

use tarjama::context;
use std::collections::HashMap;

let catalogue_bag = CatalogueBag::from_catalogues(vec![
    Catalogue::from_messages(Locale::English(EnglishVariant::Default), HashMap::from([
        ("messages".to_owned(), HashMap::from([
          ("apple".to_owned(), "{0} There are no apples | {1} There is one apple | {2..4} There are few apples | There are {?} apples".to_owned()),
        ]))
    ])),
]);

let translator = Translator::from_catalogue_bag(catalogue_bag);

println!("{}", translator.trans("en", "messages", "apple", context!(count = 0 ))?);
println!("{}", translator.trans("en", "messages", "apple", context!(count = 1 ))?);
println!("{}", translator.trans("en", "messages", "apple", context!(count = 3 ))?);
println!("{}", translator.trans("en", "messages", "apple", context!(count = 10))?);
```

> This example outputs:
>
> ```
> There are no apples
> There is one apple
> There are few apples
> There are 10 apples
> ```

## Crate features

### Default, and Toml

The `default` feature for `tarjama` includes `toml` feature, which allows loading `CatalogueBag` from a directory containing
toml translation message files.

If you wish to create the `CatalogueBag` manually, you could opt-out of the `toml` feature as follows:

```toml
tarjama = { version = "0.1.0", default-features = false}
```

### File

If you wish to create your own `CatalogueBag` file loader that uses a format other than `toml`, this feature would be of great help.

Here's an example of how to do so:

```rust
use tarjama::catalogue::{Catalogue, CatalogueBag};
use tarjama::loader::file;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn parse(content: String) -> HashMap<String, String> {
    todo!("parse messages");
}

async fn load<T>(directory: T) -> Result<CatalogueBag, Box<dyn Error>>
where
    T: AsRef<Path> + 'static,
{
    let mut bag = CatalogueBag::new();
    let data = file::iterate(directory, &["json".to_string()]).await?;

    for (locale, domains) in data {
        let mut catalogue = Catalogue::new(locale);
        for (domain, files) in domains {
            for file in files {
                let content = fs::read_to_string(file)?;

                for (id, message) in parse(content) {
                    catalogue.insert(&domain, &id, &message);
                }
            }
        }

        bag.insert(catalogue);
    }

    Ok(bag)
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
