use rand::seq::SliceRandom;
use rand::thread_rng;

include!(concat!(env!("OUT_DIR"), "/words.rs"));

fn main() {
    let rng = &mut thread_rng();
    println!(
        "{} {} {}",
        WORDS.choose(rng).unwrap(),
        WORDS.choose(rng).unwrap(),
        WORDS.choose(rng).unwrap(),
    );
}
