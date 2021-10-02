pub struct Triangle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub sx: f32,
    pub sy: f32,
    pub sz: f32
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            sx: 0.0,
            sy: 0.0,
            sz: 0.0,
        }
    }

    // not rlly needed because it looks weird
    /*
    pub fn from() -> Self {

    }
    */

    pub fn verify(&mut self) -> bool {
        let mut bl = false;
        if self.sx <= (self.sy + self.sz) {
            bl = true;
        } else {
            bl = false;
            return bl;
        }
        if self.sy <= (self.sx + self.sz) {
            bl = true;
        }  else {
            bl = false;
            return bl;
        }
        if self.sz <= (self.sy + self.sx) {
            bl = true;
        } else {
            bl = false;
            return bl;
        }
        bl
    }
}