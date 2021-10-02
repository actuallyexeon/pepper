pub mod quad;

// regular ratio
pub struct Ratio {
    pub r1: f32,
    pub r2: f32
}

impl Ratio {
    pub fn find_ratio(&mut self, num: f32) -> (f32, f32) {
        let div = num / (self.r1 + self.r2);
        let a1 = div * self.r1;
        let a2 = div * self.r2;
        return (a1, a2);
    }
}