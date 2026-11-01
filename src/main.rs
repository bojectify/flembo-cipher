mod cipher;
mod mapping;
mod phrase;

use clap::Parser;
use rand::{SeedableRng, rngs::StdRng};

use crate::phrase::clean;

fn main() {
  let args = Args::parse();

  let phrase = match clean(&args.phrase, args.max_length) {
    Ok(cleaned) => cleaned,
    Err(error) => {
      eprintln!("error: {error}");
      std::process::exit(1);
    }
  };

  let seed = args.seed.unwrap_or_else(rand::random);
  let mut rng = StdRng::seed_from_u64(seed);
  let mapping = mapping::build_mapping(&mut rng);
  let cipher_result = cipher::make_cipher(&phrase, &mapping);
  let json = serde_json::to_string(&cipher_result).expect("serialises");
  println!("{json}");
}

/// Turn a phrase into a Flembo cipher code.
///
/// The phrase is lowercased and stripped of punctuation, then every letter is
/// swapped for a different one. Prints JSON with the cleaned phrase, the cipher,
/// and the key (cipher letter -> real letter). Errors go to stderr with exit code 1.
#[derive(Parser)]
#[command(version)]
struct Args {
  /// The phrase to turn into a code
  phrase: String,

  /// Seed for repeatable output (random if omitted)
  #[arg(long)]
  seed: Option<u64>,

  /// Maximum length of the cleaned phrase, in characters
  #[arg(long, default_value_t = 150)]
  max_length: usize,
}
