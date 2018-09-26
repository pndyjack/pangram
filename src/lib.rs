use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
  let mut not_occured: HashSet<char> = vec![
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
  ].into_iter()
    .collect();
  26 == sentence
    .to_lowercase()
    .chars()
    .into_iter()
    .map(|letter| {
      if not_occured.contains(&letter) {
        not_occured.remove(&letter);
        return 1;
      }
      0
    })
    .sum()
}
