use crate::switch::Object;

// 2D plane so we can add many shapes to a plane of stuff
pub struct TDPlane {
    pub plane: Vec<Object>
}

impl TDPlane {
    pub fn new() -> Self {
        Self {
            plane: Vec::new()
        }
    }

    pub fn from(vec: Vec<Object>) -> Self {
        Self {
            plane: vec
        }
    }
}