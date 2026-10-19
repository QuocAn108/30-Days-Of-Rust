use std::fmt::Debug;

#[derive(Debug)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T: Debug, U: Debug> Pair<T, U> {
    pub fn show(&self) {
        println!("Pair contains: ({:?}, {:?})", self.first, self.second);
    }
}
