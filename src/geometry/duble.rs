use super::triangle::Triangle;
// special triangle that has two in one
// in the exact vertical alignment and horizontal is one particle
// could be used for car model

pub struct Duble {
    t1: Triangle,
    t2: Triangle
}

impl Duble {
    pub fn new() -> Self {
        Self {
            t1: Triangle::new(),
            t2: Triangle::new()
        }
    }
}