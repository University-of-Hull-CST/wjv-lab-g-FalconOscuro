use crate::vec2::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub pos : Vec2,
    pub radius : f32,
}

impl Particle {

    pub fn new(pos: Vec2, radius: f32) -> Self {
        return Self { pos, radius };
    }

    pub fn collide(&self, p1: &Particle) -> bool
    {
        let distance_sq = (self.pos - p1.pos).length_squared();

        return distance_sq <= (self.radius + p1.radius).powi(2)
    }
}
