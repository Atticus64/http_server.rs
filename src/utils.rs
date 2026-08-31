use std::cell::Cell;
pub struct IDManager {
  next_id: Cell<i64>
}

impl IDManager {
  pub fn new() -> IDManager {
    IDManager { next_id: Cell::new(1) }
  }

  pub fn get_id(&self) -> i64 {
    let ans = self.next_id.get();
    self.next_id.set(ans + 1);
    ans
  }
}


