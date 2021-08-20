use rand::seq::SliceRandom;
use rand::thread_rng;

include!(concat!(env!("OUT_DIR"), "/words.rs"));

pub fn generate() -> Result<String, &'static str> {
    let rng = &mut thread_rng();
    let s = format!(
        "{} {} {} {} {}",
        WORDS.choose(rng).ok_or("error while choosing word")?,
        WORDS.choose(rng).ok_or("error while choosing word")?,
        WORDS.choose(rng).ok_or("error while choosing word")?,
        WORDS.choose(rng).ok_or("error while choosing word")?,
        WORDS.choose(rng).ok_or("error while choosing word")?,
    );
    Ok(s)
}