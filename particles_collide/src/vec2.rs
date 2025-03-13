use rand::random_range;
use std::{ops, cmp};

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x : f32,
    pub y : f32,
}

impl Vec2 {

    pub fn zero() -> Self {
        return Self { x : 0.0, y : 0.0 }
    }

    pub fn new(x : f32, y : f32) -> Self {
        return Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        return f32::powf(self.x.powf(2.0) + self.y.powf(2.0), 0.5);
    }

    pub fn normalized(&self) -> Vec2 {
        return *self / self.magnitude();
    }
    
    pub fn clamp(&self, min : Vec2, max : Vec2) -> Vec2 {
        return Vec2 {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y)
        };
    }
}

impl cmp::PartialEq<Vec2> for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        return Vec2 {
            x : self.x + rhs.x,
            y : self.y + rhs.y,
        };
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    
    fn sub(self, rhs: Vec2) -> Self::Output {
        return Vec2 {
            x : self.x - rhs.x,
            y : self.y - rhs.y,
        };
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2
    {
        return Vec2 {
            x : self.x * rhs,
            y : self.y * rhs,
        };
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Vec2
    {
        return Vec2 {
            x : self.x / rhs,
            y : self.y / rhs,
        };
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

pub fn vec_linear_interpolate(from: Vec2, to: Vec2, step: f32) -> Vec2 {
    return from + ((to - from) * step);
}

pub fn rand_vec2f() -> Vec2 {
    return Vec2 {
        x : random_range(0.0 ..= 1.0), 
        y : random_range(0.0 ..= 1.0)
    };
}

