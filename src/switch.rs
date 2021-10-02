use crate::geometry::triangle::Triangle;

pub enum ObjectSwitch {
    Triangle(Triangle)
}

// object struct with a switch
pub struct Object {
    pub object: ObjectSwitch,
    pub x: f32, 
    pub y: f32
}