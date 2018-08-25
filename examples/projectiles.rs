extern crate ray_tracer;

//use std::io;

use ray_tracer::utils::tuple::Tuple;

struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

struct World {
    pub gravity: Tuple,
    pub wind: Tuple,
}

fn tick(world: &World, projectile: &mut Projectile, timestep: f64) {
    projectile.position = projectile.position + projectile.velocity * timestep;
    projectile.velocity = projectile.velocity + (world.gravity + world.wind) * timestep;
}

fn main() {
    // let buffer: String = String::new();
    // io::stdin().read_line(buffer).unwrap();
    // let
    let world = World {
        gravity: Tuple::new_vector(0, -9.8, 0),
        wind: Tuple::new_vector(1.0, 0, 0),
    };

    for timestep in [1.0_f64, 0.5, 0.2, 0.1, 0.01, 0.000001].into_iter() {
        let mut projectile = Projectile {
            position: Tuple::new_point(0, 1, 0),
            velocity: Tuple::new_vector(50, 100, 0),
        };
        while projectile.position.y() > 0.0 {
            tick(&world, &mut projectile, *timestep);
        }
        println!(
            "final: position: {:?}, velocity: {:?}",
            projectile.position, projectile.velocity
        );
    }
}
