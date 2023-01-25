#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn unit(angle: f64) -> Vec2 {
        let adjusted = angle - 90.;
        Vec2 {
            x: adjusted.to_radians().cos(),
            y: adjusted.to_radians().sin(),
        }
    }
}

impl std::ops::Mul<f64> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
