
use ruski::term::{Term};
use std::collections::HashMap;

pub struct Pool {
    // Multiset of combinatory logic expressions 
    p: HashMap<Term, u32>,
}

impl Pool {
    
    pub fn new() -> Self {
        Self {
            p: HashMap::new(),
        }
    }

    fn initialize(&mut self) {

    }

    fn insert(&mut self, term: Term) {
        *self.p.entry(term).or_insert(0) += 1;
    }

    fn remove(&mut self, term: &Term) {
        if let Some(count) = self.p.get_mut(term) {
            if *count == 1 {
                self.p.remove(term);
            } else {
                *count -= 1;
            }
        }
    }


}
