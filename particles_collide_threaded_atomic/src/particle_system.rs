mod particle;

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::vec2::{self, Vec2};
use crate::particle_system::particle::Particle;

pub struct ParticleSystem {
    particles : Vec<Particle>,

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
            let particle_pos = vec2::rand_vec2f();
            let particle_pos_scaled = Vec2 {
                x: particle_pos.x * bounds_range.x,
                y: particle_pos.y * bounds_range.y
            };

            let particle = Particle {
                pos: particle_pos_scaled + bounds_min,
                radius: 0.1
            };

            particles.push(particle);
        }

        return Self { particles , bounds_min, bounds_max }
    }
    
    pub fn move_particles(&mut self, max_speed : f32)
    {
        ParticleSystem::move_particle_chunk(self.particles.as_mut_slice(), max_speed, self.bounds_min, self.bounds_max);
    }

    pub fn move_particles_threaded(&mut self, max_speed : f32, pool : &mut scoped_threadpool::Pool)
    {
        let num_particles = self.particles.len();
        let num_threads = pool.thread_count() as usize;

        // rounding up to ensure no more than num_threads created
        let chunk_size = num_particles.div_ceil(num_threads);

        let bounds_min = self.bounds_min.clone();
        let bounds_max = self.bounds_max.clone();

        pool.scoped(|scope|
        {
            for slice in self.particles.chunks_mut(chunk_size) {
                scope.execute(move || ParticleSystem::move_particle_chunk(slice, max_speed, bounds_min.clone(), bounds_max.clone()));
            }
        });
    }

    pub fn test_collisions(&self, pool: &mut scoped_threadpool::Pool) -> usize
    {
        let num_particle_pairs = self.num_particle_pairs();
        let num_threads = pool.thread_count() as usize;
        let thread_job_count = num_particle_pairs.div_ceil(num_threads);

        let counter = &AtomicUsize::new(0);

        pool.scoped(|scope|
        {
            for i in 0 .. num_threads {
                let idx_start = i * thread_job_count;
                let idx_range = idx_start .. std::cmp::min(idx_start + thread_job_count, num_particle_pairs);

                scope.execute(||
                    test_particle_pair_collisions(self.particles.as_slice(), idx_range, counter)
                );
            }
        });

        return counter.load(Ordering::Acquire);
    }

    fn num_particle_pairs(&self) -> usize {
        let num_particles = self.particles.len();

        // prevent undefined behaviour if no particle pairs exist
        if num_particles < 2 {
            return 0;
        }

        return (num_particles * (num_particles - 1)) / 2;
    }

    // naieve approach, should be able to determine by id instead of explictly creating
    // full list of all jobs each call
    fn create_particle_collision_job_queue(&self) -> Vec<(&Particle, &Particle)>
    {
        let mut job_queue = vec!();

        for i in 0 .. (self.particles.len() - 1) {
            for j in (i + 1) .. self.particles.len() {
                job_queue.push((&self.particles[i], &self.particles[j]));
            }
        }

        return job_queue;
    }

    fn move_particle_chunk(slice: &mut [Particle], max_speed : f32, bounds_min: Vec2, bounds_max: Vec2) {
        for particle in slice
        {
            let direction = (vec2::rand_vec2f() * 2.) - Vec2 { x: 1., y: 1. };
            let distance: f32 = rand::random_range(0. ..= max_speed);
            let new_pos = particle.pos.clone() + (direction * distance);

            particle.pos = new_pos.clamp(bounds_min, bounds_max);
        }
    }

    fn test_particle_collision_chunk(slice: & [(&Particle, &Particle)]) {
        let mut collision_count: usize = 0;

        for c in slice {
            if c.0.collide(c.1) {
                collision_count += 1;
            }
        }

        println!("Collision count: {}", collision_count);
    }
}

// total pair-count un-needed, idx scales with total pair count
// the set of unique pairs in a smaller list exists within a larger list
fn get_particle_pair_idx(idx: usize) -> (usize, usize) {
    // quadratice formula
    let n = ((8 * idx + 1).isqrt() - 1) / 2;
    
    let i = idx - (n * (n + 1)) / 2;
    let j = n + 1;

    return (i, j);
}

fn test_particle_pair_collisions(particles: &[Particle], pair_range: std::ops::Range<usize>, counter: &AtomicUsize)
{
    for idx in pair_range {
        let (i, j) = get_particle_pair_idx(idx);

        if particles[i].collide(&particles[j]) {
            counter.fetch_add(1, Ordering::Relaxed);
        }
    }
}
