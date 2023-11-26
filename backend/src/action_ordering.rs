pub struct ActionOrdering {
  actions: [(u64, u32); 7],
  size: usize,
}

impl ActionOrdering {
  pub fn new() -> Self {
      Self {
          actions: [(0, 0); 7],
          size: 0,
      }
  }

  pub fn push(&mut self, action: u64, score: u32) {
      let mut i = self.size;
      while i > 0 && score < self.actions[i - 1].1 {
          self.actions[i] = self.actions[i - 1];
          i -= 1;
      }

      self.actions[i] = (action, score);
      self.size += 1;
  }
}

impl Iterator for ActionOrdering {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
      if self.size == 0 { return None; }

      self.size -= 1;
      Some(self.actions[self.size].0)
  }
}
