use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, Serialize)]
pub struct CipherResult {
  pub phrase: String,
  pub cipher: String,
  pub key: BTreeMap<char, char>,
}

pub fn make_cipher(phrase: &str, mapping: &HashMap<char, char>) -> CipherResult {
  let cipher = phrase
    .chars()
    .map(|c| mapping.get(&c).copied().unwrap_or(c))
    .collect::<String>();

  let mut key = BTreeMap::new();
  for c in phrase.chars() {
    if let Some(&mapped_letter) = mapping.get(&c) {
      key.insert(mapped_letter, c);
    }
  }

  CipherResult {
    phrase: phrase.to_string(),
    cipher,
    key,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_mapping() -> HashMap<char, char> {
    HashMap::from([('a', 'b'), ('b', 'c'), ('c', 'a')])
  }

  #[test]
  fn scrambles_letters() {
    let mapping = test_mapping();

    let result = make_cipher("abbc", &mapping);
    assert_eq!(result.cipher, "bcca");
    assert_eq!(result.phrase, "abbc");
  }

  #[test]
  fn keeps_spaces() {
    let mapping = test_mapping();

    let result = make_cipher("ab bc", &mapping);
    assert_eq!(result.cipher, "bc ca");
  }

  #[test]
  fn key_maps_puzzle_to_real() {
    let mapping = test_mapping();

    let result = make_cipher("abbc", &mapping);
    assert_eq!(
      result.key,
      BTreeMap::from([('b', 'a'), ('c', 'b'), ('a', 'c')])
    );
  }

  #[test]
  fn key_only_has_used_letters() {
    let mapping = test_mapping();

    let result = make_cipher("ab ba", &mapping);
    assert_eq!(result.key.get(&'a'), None);
    assert_eq!(result.key.get(&'b'), Some(&'a'));
    assert_eq!(result.key.get(&'c'), Some(&'b'));
  }

  #[test]
  fn key_is_serialised() {
    let mapping = test_mapping();

    let result = make_cipher("ab", &mapping);
    let json = serde_json::to_string(&result).expect("serialises");
    assert_eq!(
      json,
      r#"{"phrase":"ab","cipher":"bc","key":{"b":"a","c":"b"}}"#
    );
  }
}
