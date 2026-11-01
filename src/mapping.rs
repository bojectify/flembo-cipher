use rand::{Rng, seq::SliceRandom};
use std::collections::HashMap;

fn no_letter_maps_to_itself(mapping: &HashMap<char, char>) -> bool {
  for c in 'a'..='z' {
    if let Some(&mapped) = mapping.get(&c)
      && c == mapped
    {
      return false;
    }
  }
  true
}

pub fn build_mapping(rng: &mut impl Rng) -> HashMap<char, char> {
  let letters_vec = ('a'..='z').collect::<Vec<char>>();
  let mut shuffled = letters_vec.clone();
  shuffled.shuffle(rng);
  let mapping = letters_vec.into_iter().zip(shuffled).collect();
  if no_letter_maps_to_itself(&mapping) {
    mapping
  } else {
    build_mapping(rng)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use rand::{SeedableRng, rngs::StdRng};
  use std::collections::HashSet;

  #[test]
  fn maps_every_letter() {
    let mut rng = StdRng::seed_from_u64(42);
    let mapping = build_mapping(&mut rng);
    assert_eq!(mapping.len(), 26);
    for c in 'a'..='z' {
      assert!(mapping.contains_key(&c));
    }
  }

  #[test]
  fn mapping_has_no_letter_that_maps_to_itself() {
    for seed in 0..1000 {
      let mut rng = StdRng::seed_from_u64(seed);
      let mapping = build_mapping(&mut rng);
      assert!(no_letter_maps_to_itself(&mapping));
    }
  }

  #[test]
  fn checker_rejects_self_mapped_letter() {
    let mut mapping = HashMap::new();
    mapping.insert('a', 'b');
    mapping.insert('b', 'c');
    mapping.insert('c', 'a');
    assert!(no_letter_maps_to_itself(&mapping));

    mapping.insert('d', 'd');
    assert!(!no_letter_maps_to_itself(&mapping));
  }

  #[test]
  fn every_letter_is_used_once() {
    let mut rng = StdRng::seed_from_u64(42);
    let mapping = build_mapping(&mut rng);

    let unique: HashSet<&char> = mapping.values().collect();
    assert_eq!(unique.len(), 26);
  }

  #[test]
  fn the_mapping_is_deterministic_given_a_seed() {
    let mut rng1 = StdRng::seed_from_u64(42);
    let mapping1 = build_mapping(&mut rng1);

    let mut rng2 = StdRng::seed_from_u64(42);
    let mapping2 = build_mapping(&mut rng2);

    assert_eq!(mapping1, mapping2);
  }
}
