// special ratio with 4 fields
// for example 2:2:3:3
// could be used for car weight

pub struct Quad {
    pub r1: f32,
    pub r2: f32, 
    pub r3: f32,
    pub r4: f32
}

impl Quad {
    // return matches ratio struct order
    pub fn find_ratio(&mut self, num: f32) -> (f32, f32, f32, f32) {
        let div = num / (self.r1 + self.r2 + self.r3 + self.r4);
        let a1 = div * self.r1;
        let a2 = div * self.r2;
        let a3 = div * self.r3;
        let a4 = div * self.r4;
        return (a1, a2, a3, a4);
    }
}