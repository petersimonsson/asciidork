use std::ops::{Deref, DerefMut};

use crate::internal::*;

#[derive(Debug)]
pub struct ListStack(Vec<ListMarker>);

impl ListStack {
  pub fn starts_nested_list(&self, next_marker: ListMarker) -> bool {
    if self.is_empty() {
      return false;
    }
    match next_marker {
      ListMarker::Digits(_) => self
        .iter()
        .all(|marker| !matches!(marker, ListMarker::Digits(_))),
      marker => !self.contains(&marker),
    }
  }

  pub fn continues_current_list(&self, next: ListMarker) -> bool {
    self.last().map_or(false, |last| match (last, next) {
      (ListMarker::Digits(_), ListMarker::Digits(_)) => true,
      (last, next) => *last == next,
    })
  }
}

impl Default for ListStack {
  fn default() -> Self {
    Self(Vec::with_capacity(6))
  }
}

impl Deref for ListStack {
  type Target = Vec<ListMarker>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for ListStack {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

// tests

#[test]
fn test_continues_current_list() {
  use ListMarker::*;
  let cases: Vec<(ListMarker, &[ListMarker], bool)> = vec![
    (Star(1), &[Star(1)], true),
    (Star(2), &[Star(1)], false),
    (Star(1), &[Star(2)], false),
    (Dot(1), &[Star(1), Star(2)], false),
    (Digits(2), &[Digits(1)], true),
  ];
  for (next, markers, expected) in cases {
    let mut stack = ListStack::default();
    for marker in markers {
      stack.push(*marker);
    }
    assert_eq!(stack.continues_current_list(next), expected);
  }
}

#[test]
fn test_starts_nested_list() {
  use ListMarker::*;
  let cases: Vec<(ListMarker, &[ListMarker], bool)> = vec![
    (Star(1), &[Star(1)], false),
    (Star(2), &[Star(1)], true),
    (Star(1), &[Star(2)], true),
    (Dot(1), &[Star(2), Star(1)], true),
    (Digits(2), &[Digits(1)], false),
  ];
  for (next, markers, expected) in cases {
    let mut stack = ListStack::default();
    for marker in markers {
      stack.push(*marker);
    }
    assert_eq!(stack.starts_nested_list(next), expected);
  }
}
