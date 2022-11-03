use std::collections::HashMap;

use tarjama::catalogue::{Catalogue, CatalogueBag};
use tarjama::error::Error;
use tarjama::locale::{EnglishVariant, Locale};
use tarjama::{context, Translator};

fn main() -> Result<(), Error> {
    let catalogue_bag = CatalogueBag::with_catalogues(vec![
        Catalogue::with_messages(Locale::English(EnglishVariant::Default), HashMap::from([
            ("messages".to_owned(), HashMap::from([
              ("apple".to_owned(), "{0} There are no apples | {1} There is one apple | {2..4} There are few apples | There are {?} apples".to_owned()),
            ]))
        ])),
    ]);

    let translator = Translator::with_catalogue_bag(catalogue_bag);

    for i in [0, 1, 4, 10] {
        println!(
            "{}",
            translator.trans("en", "messages", "apple", context!(? = i))?
        );
    }

    Ok(())
}
