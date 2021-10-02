pub mod geometry;
pub mod switch;
pub mod ratios;

#[cfg(test)]
mod tests {
    use crate::geometry::triangle::Triangle;
    use crate::ratios::{quad::Quad, Ratio};
    #[test]
    fn triangle_verification() {
        let mut imm_triangle = Triangle {
            x: 45.0,
            y: 45.0,
            z: 90.0,
            sx: 1.0,
            sy: 1.0,
            sz: 3.0
        };
        println!("{:?}", imm_triangle.verify());
    }

    #[test]
    fn ratio_regular() {
        let mut ratio = Ratio {
            r1: 2.0,
            r2: 3.0
        };
        println!("{:?}", ratio.find_ratio(50.0));
    }

    #[test]
    fn ratio_quad() {
        let mut ratio = Quad {
            r1: 2.0,
            r2: 7.24797,
            r3: 2.45694567845678946545678657489745689734527891534927853469278345629378456,
            r4: 5.0,
        };
        println!("{:?}", ratio.find_ratio(50.0));
    }
}
