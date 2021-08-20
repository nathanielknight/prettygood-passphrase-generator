//! PPG is a Pretty-good Pass-phrase Generator.
//!
//! When invoked from the command line it will print a five-word pass-phrase.
//!
//! For example:
//! ```shell
//! % ppg
//! saga junior polls store wrist
//! ```
//!
//! Words are drawn from the "formal" word list of
//! [Christopher Wellons's Pokerware](https://github.com/skeeto/pokerware)
//! using a random number generator from [rand]; the generator it uses is
//! intended to be cryptographically secure.

use rand::seq::SliceRandom;
use rand::thread_rng;

include!(concat!(env!("OUT_DIR"), "/words.rs"));

/// Generate a new pass-phrase.
///
/// This function generates a new five word pass-phrase using [rand]
/// to draw from the "formal" word list of Christopher Wellons's
/// [Pokerware](https://github.com/skeeto/pokerware).
pub fn generate() -> String {
    let rng = &mut thread_rng();
    format!(
        "{} {} {} {} {}",
        WORDS.choose(rng).expect("Word list was empty?"),
        WORDS.choose(rng).expect("Word list was empty?"),
        WORDS.choose(rng).expect("Word list was empty?"),
        WORDS.choose(rng).expect("Word list was empty?"),
        WORDS.choose(rng).expect("Word list was empty?"),
    )
    // The `.choose` method returns an option which is `None` if the sequence
    // it's called on is empty, but we know that the word list isn't empty, so
    // we can safely unwrap it.
}