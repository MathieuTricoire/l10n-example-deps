use l10n_lib::lib_fn;

use l10n::unic_langid::langid;
use l10n::{message, L10nMessage};

l10n::init!();

fn main() {
    let fr = &langid!("fr");
    let en = &langid!("en");

    let message = message!("bin", "Loc-bin");

    println!("binary:");
    println!("- french: {}", message.translate(fr));
    println!("- english: {}", message.translate(en));

    println!();
    lib_fn();
}
