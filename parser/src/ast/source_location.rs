use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct SourceLocation {
  pub start: usize,
  pub end: usize,
}

impl SourceLocation {
  pub fn new(start: usize, end: usize) -> Self {
    debug_assert!(start <= end);
    Self { start, end }
  }

  pub fn extend(&mut self, other: SourceLocation) {
    self.start = self.start.min(other.start);
    self.end = self.end.max(other.end);
  }

  pub fn clamp_start(&self) -> SourceLocation {
    Self::new(self.start, self.start)
  }

  pub fn clamp_end(&self) -> SourceLocation {
    Self::new(self.end, self.end)
  }

  pub fn decr_end(&self) -> SourceLocation {
    Self::new(self.start, self.end - 1)
  }

  pub fn incr_end(&self) -> SourceLocation {
    Self::new(self.start, self.end + 1)
  }
}

impl From<usize> for SourceLocation {
  fn from(offset: usize) -> Self {
    Self::new(offset, offset)
  }
}

impl fmt::Debug for SourceLocation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}..{}", self.start, self.end)
  }
}
