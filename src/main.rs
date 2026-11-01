fn main() {
  println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum PhraseError {
  InvalidChar(char),
  NoWords,
  TooLong { length: usize, max: usize },
}

fn clean(phrase: &str, max_length: usize) -> Result<String, PhraseError> {
  if let Some(bad) = phrase
    .chars()
    .find(|c| c.is_alphanumeric() && !c.is_ascii_alphabetic())
  {
    return Err(PhraseError::InvalidChar(bad));
  }

  let cleaned = phrase
    .to_ascii_lowercase()
    .chars()
    .map(|c: char| {
      if matches!(c, '\u{2010}'..='\u{2015}' | '\u{2212}' | '-') {
        ' '
      } else {
        c
      }
    })
    .filter(|c: &char| *c == ' ' || c.is_ascii_alphabetic())
    .collect::<String>()
    .split_whitespace()
    .collect::<Vec<&str>>()
    .join(" ");

  let length = cleaned.len();

  if length > max_length {
    return Err(PhraseError::TooLong {
      length,
      max: max_length,
    });
  }

  if cleaned.is_empty() {
    return Err(PhraseError::NoWords);
  }

  Ok(cleaned)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_to_lowercase() {
    assert_eq!(clean("INPUT", 150), Ok("input".to_string()));
  }
  #[test]
  fn removes_leading_trailing_whitespace() {
    assert_eq!(clean("  INPUT  ", 150), Ok("input".to_string()));
  }

  #[test]
  fn removes_punctuation() {
    assert_eq!(clean("input!", 150), Ok("input".to_string()));
    assert_eq!(clean("don't", 150), Ok("dont".to_string()));
  }

  #[test]
  fn turns_hyphens_into_spaces() {
    assert_eq!(clean("well-known", 150), Ok("well known".to_string()));
    assert_eq!(clean("a - b", 150), Ok("a b".to_string()));
    assert_eq!(
      clean("well\u{2010}known", 150),
      Ok("well known".to_string())
    );
    // en dash
    assert_eq!(
      clean("well\u{2013}known", 150),
      Ok("well known".to_string())
    );

    // em dash
    assert_eq!(
      clean("well\u{2014}known", 150),
      Ok("well known".to_string())
    );

    // minus sign
    assert_eq!(
      clean("well\u{2212}known", 150),
      Ok("well known".to_string())
    );
    assert_eq!(clean("well\u{AD}known", 150), Ok("wellknown".to_string())); // soft hyphen is removed
  }

  #[test]
  fn removes_multiple_spaces() {
    assert_eq!(clean("a  b", 150), Ok("a b".to_string()));
  }

  #[test]
  fn errors_on_invalid_characters() {
    assert_eq!(clean("input 4", 150), Err(PhraseError::InvalidChar('4')));
    assert_eq!(clean("café", 150), Err(PhraseError::InvalidChar('é')));
  }

  #[test]
  fn errors_on_no_words() {
    assert_eq!(clean("!!!", 150), Err(PhraseError::NoWords));
    assert_eq!(clean("", 150), Err(PhraseError::NoWords));
  }

  #[test]
  fn errors_on_too_long() {
    assert_eq!(
      clean("Dave", 3),
      Err(PhraseError::TooLong { length: 4, max: 3 })
    );
  }

  #[test]
  fn succeeds_on_too_long_post_clean() {
    assert_eq!(clean("  abc!  ", 3), Ok("abc".to_string()));
  }
  #[test]
  fn succeeds_on_the_correct_length() {
    assert_eq!(clean("abc", 3), Ok("abc".to_string()));
  }
}
