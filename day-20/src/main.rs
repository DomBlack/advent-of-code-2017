#[macro_use]
extern crate lazy_static;
extern crate regex;

mod vectors;
mod particle;

use particle::Particle;

#[cfg(not(test))]
fn main() {
    let vectors = parse(include_str!("input.txt"));

    println!("Part 1: {}", slowest_particle(&vectors));
    println!("Part 2: {}", particles_left_after_collisions(&vectors));
}

fn parse(input: &str) -> Vec<Particle> {
    input.trim().lines().map( | l | l.parse().unwrap()).collect()
}

/// Finds the index of the slowest particle
fn slowest_particle(particles: &Vec<Particle>) -> usize {
    particles.iter()
        .enumerate()
        .min_by_key( | &(_, particle) | particle.acceleration.manhattan() )
        .unwrap()
        .0
}

/// Finds how many particles are left after any collisions which destory them (whole time steps only)
fn particles_left_after_collisions(particles: &Vec<Particle>) -> usize {
    let mut particles = particles.clone();

    let mut count_since_last_collision = 0;

    // We keep looping until we're relativity sure no more collisions will occur
    while count_since_last_collision < 100 {
        count_since_last_collision = count_since_last_collision + 1;

        // Move all particles
        particles.iter_mut().for_each(| particle | particle.step());

        // Sort them by position to make the next step quicker
        particles.sort_by_key( | p | p.position.clone());

        // Loop over all particles
        for i in 0 .. particles.len() -1 {
            if i >= particles.len() - 1 {
                break;
            }

            // Remove any duplicates
            if particles[i].position == particles[i + 1].position {
                while i + 1 < particles.len() && particles[i].position == particles[i + 1].position {
                    particles.remove(i + 1);
                }
                particles.remove(i);
                count_since_last_collision = 0;
            }
        }

    }

    particles.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use vectors::V3;

    const TWO_PARTICLES_INPUT: &str = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
                         p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";

    #[test]
    fn test_parse() {
        assert_eq!(
            "< 4,3,-1>".parse::<V3>(),
            Ok(V3::new(4, 3, -1)),
            "V3 Parse"
        );

        assert_eq!(
            parse(TWO_PARTICLES_INPUT),
            vec![
                Particle { position: V3::new(3, 0, 0), velocity: V3::new(2, 0, 0), acceleration: V3::new(-1, 0, 0) },
                Particle { position: V3::new(4, 0, 0), velocity: V3::new(0, 0, 0), acceleration: V3::new(-2, 0, 0) },
            ],
            "Partial Parse"
        );
    }

    #[test]
    fn test_slowest_particle() {
        assert_eq!(
            slowest_particle(&parse(TWO_PARTICLES_INPUT)),
            0
        );
    }

    #[test]
    fn test_particle_collisions() {
        const INPUT: &str = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
                             p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
                             p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
                             p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

        assert_eq!(
            particles_left_after_collisions(&parse(INPUT)),
            1
        );
    }
}
