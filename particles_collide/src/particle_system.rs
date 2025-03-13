use crate::vec2::{self, Vec2};

pub struct ParticleSystem {
    particles : Vec<Vec2>,

    bounds_min : Vec2,
    bounds_max : Vec2,
}

impl ParticleSystem {

    pub fn new(num_particles : usize, bounds_min: Vec2, bounds_max: Vec2) -> Self
    {
        let mut particles = vec!();
        particles.reserve(num_particles);

        let bounds_range = bounds_max - bounds_min;

        for _ in 0 .. num_particles {
            let particle = vec2::rand_vec2f();
            let particle_scaled = Vec2 {
                x: particle.x * bounds_range.x,
                y: particle.y * bounds_range.y
            };

            particles.push(bounds_min + particle_scaled);
        }

        return Self { particles , bounds_min, bounds_max }
    }
    
    pub fn move_particles(&mut self, max_speed : f32)
    {
        ParticleSystem::move_particle_chunk(self.particles.as_mut_slice(), max_speed, (self.bounds_min, self.bounds_max));
    }

    pub fn move_particles_threaded(&mut self, max_speed : f32, pool : &mut scoped_threadpool::Pool)
    {
        let num_particles = self.particles.len();
        let num_threads = pool.thread_count() as usize;

        // rounding up to ensure no more than num_threads created
        let chunk_size = num_particles.div_ceil(num_threads);

        let bounds = (self.bounds_min, self.bounds_max);

        pool.scoped(|scope|
        {
            for slice in self.particles.chunks_mut(chunk_size) {
                scope.execute(move || ParticleSystem::move_particle_chunk(slice, max_speed, bounds));
            }
        });
    }

    fn move_particle_chunk(slice: &mut [Vec2], max_speed : f32, bounds: (Vec2, Vec2)) {
        for particle in slice
        {
            let direction = vec2::rand_vec2f();
            let distance = rand::random::<f32>() * max_speed;
            let new_pos = particle.clone() + (direction * distance);

            particle.clone_from(&new_pos.clamp(bounds.0, bounds.1));
        }
    }
}
