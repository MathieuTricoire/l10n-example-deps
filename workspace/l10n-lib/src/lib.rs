use l10n::unic_langid::langid;
use l10n::{message, L10nMessage};

l10n::init!();

pub fn lib_fn() {
    let fr = &langid!("fr");
    let en = &langid!("en");

    let message = message!("lib", "Loc-lib");

    println!("library:");
    println!("- french: {}", message.translate(fr));
    println!("- english: {}", message.translate(en));
}
