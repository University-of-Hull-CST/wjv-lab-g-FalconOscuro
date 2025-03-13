mod vec2;
mod particle_system;

use std::time::{Instant, Duration};

use clap::Parser;

use crate::vec2::Vec2;
use crate::particle_system::ParticleSystem;

const NUM_PARTICLES: usize = 100;
const BOUNDS_MIN: Vec2 = Vec2{ x:  0.0, y:  0.0};
const BOUNDS_MAX: Vec2 = Vec2{ x: 10.0, y: 10.0};
const MAX_SPEED: f32 = 2.0;

const RUN_TIME_MS: u64 = 10_000;

const NUM_THREADS: u32 = 4;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Number of particles created
    #[arg(short = 'p', long = "particles", default_value_t = NUM_PARTICLES)]
    num_particles: usize,

    /// Run time in milliseconds
    #[arg(long = "run-time", default_value_t = RUN_TIME_MS)]
    run_time_ms: u64,

    /// Number of threads used
    #[arg(short = 't', long = "threads", default_value_t = NUM_THREADS)]
    num_threads: u32,
}

fn main()
{
    let args = Args::parse();
    println!("Particles: {}", args.num_particles);
    println!("Run time: {}ms", args.run_time_ms);
    println!("Thread count: {}", args.num_threads);
    println!();

    println!("Creating particle system...");
    let mut particle_sys = ParticleSystem::new(args.num_particles, BOUNDS_MIN, BOUNDS_MAX);

    let run_duration = Duration::from_millis(args.run_time_ms);

    let mut pool = scoped_threadpool::Pool::new(args.num_threads);

    println!("Starting move loop for {} seconds.", run_duration.as_secs());

    let mut iterations: usize = 0;
    let start = Instant::now();
    let end_at = start + run_duration;

    while Instant::now() < end_at {
        particle_sys.move_particles_threaded(MAX_SPEED, &mut pool);

        iterations += 1;
    }

    let duration = Instant::now() - start;

    println!("Finished {} iterations in {} seconds.", iterations, duration.as_secs());
    println!("{} it/s", iterations as f64 / duration.as_secs_f64());
}
