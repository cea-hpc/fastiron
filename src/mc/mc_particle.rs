use std::fmt::Display;

use num::{zero, Float, FromPrimitive};

use crate::{direction_cosine::DirectionCosine, tallies::MCTallyEvent};

use super::{
    mc_base_particle::{MCBaseParticle, Species},
    mc_location::MCLocation,
    mc_vector::MCVector,
};

/// Structure used to represent a particle.
#[derive(Debug, Default)]
pub struct MCParticle<T: Float> {
    /// Current position
    pub coordinate: MCVector<T>,
    /// Current velocity
    pub velocity: MCVector<T>,
    /// Direction of the particle
    pub direction_cosine: DirectionCosine<T>,
    /// Kinetic energy
    pub kinetic_energy: T,
    /// Weight
    pub weight: T,
    /// Time remaining before this particle hit census
    pub time_to_census: T,
    /// Cache-ing the current total cross section
    pub total_cross_section: T,
    /// Age
    pub age: T,
    /// Number of mean free paths to a collision (should be an integer?)
    pub num_mean_free_paths: T,
    /// Distance to a collision
    pub mean_free_path: T,
    /// Distance this particle travels in a segment
    pub segment_path_length: T,
    /// Random number seed for the rng for this particle
    pub random_number_seed: u64,
    /// Unique ID used to identify and track individual particles (should be usize?)
    pub identifier: u64,
    /// Last event this particle underwent
    pub last_event: MCTallyEvent,
    /// Number of collisions the particle underwent?
    pub num_collisions: u32,
    /// Number of segments the particle travelled?
    pub num_segments: T,
    /// Task working on (should be usize?)
    pub task: u32,
    /// Species of the particle
    pub species: Species,
    /// Breed of the particle, i.e. how it was produced (should be usize?)
    pub breed: u32,
    //// Current energy group the particle belong to (should be usize?)
    pub energy_group: usize,
    /// Current domain in the spatial grid (should be usize?)
    pub domain: u32,
    /// Current cell in the current domain (should be usize?)
    pub cell: u32,
    /// Facet to be crossed? (should be usize?)
    pub facet: u32,
    /// When crossing a facet, keep the surface normal dot product
    pub normal_dot: T,
}

impl<T: Float + FromPrimitive> MCParticle<T> {
    /// Constructor from a [MCBaseParticle] object.
    pub fn new(from_particle: &MCBaseParticle<T>) -> Self {
        let speed = from_particle.velocity.length();
        let d_cos = DirectionCosine {
            alpha: speed.recip() * from_particle.velocity.x,
            beta: speed.recip() * from_particle.velocity.y,
            gamma: speed.recip() * from_particle.velocity.z,
        };

        MCParticle {
            coordinate: from_particle.coordinate,
            velocity: from_particle.velocity,
            direction_cosine: d_cos,
            kinetic_energy: from_particle.kinetic_energy,
            weight: from_particle.weight,
            time_to_census: from_particle.time_to_census,
            total_cross_section: zero(),
            age: from_particle.age,
            num_mean_free_paths: from_particle.num_mean_free_paths,
            mean_free_path: zero(),
            segment_path_length: zero(),
            random_number_seed: from_particle.random_number_seed,
            identifier: from_particle.identifier,
            last_event: from_particle.last_event,
            num_collisions: from_particle.num_collisions,
            num_segments: from_particle.num_segments,
            task: 0,
            species: from_particle.species,
            breed: from_particle.breed,
            energy_group: 0,
            domain: from_particle.domain,
            cell: from_particle.cell,
            facet: 0,
            normal_dot: zero(),
        }
    }

    /// Returns the location of the particle as a [MCLocation] object.
    pub fn get_location(&self) -> MCLocation {
        MCLocation {
            domain: self.domain,
            cell: self.cell,
            facet: self.facet,
        }
    }

    /// Update the particle's field to model its movement along the specified
    /// direction and distance
    pub fn move_particle(&mut self, direction_cosine: &DirectionCosine<T>, distance: T) {
        self.coordinate.x = self.coordinate.x + direction_cosine.alpha * distance;
        self.coordinate.y = self.coordinate.y + direction_cosine.beta * distance;
        self.coordinate.z = self.coordinate.z + direction_cosine.gamma * distance;
    }
}

// replaces original method `PrintParticle`
impl<T: Float + Display> Display for MCParticle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "coordinate: {} {} {}",
            self.coordinate.x, self.coordinate.y, self.coordinate.z
        )?;
        writeln!(
            f,
            "velocity: {} {} {}",
            self.velocity.x, self.velocity.y, self.velocity.z
        )?;
        writeln!(
            f,
            "direction cosine: {} {} {}",
            self.direction_cosine.alpha, self.direction_cosine.beta, self.direction_cosine.gamma
        )?;
        writeln!(f, "kinetic energy: {}", self.kinetic_energy)?;
        writeln!(f, "weight: {}", self.weight)?;
        writeln!(f, "time to census: {}", self.time_to_census)?;
        writeln!(f, "total cross section: {}", self.total_cross_section)?;
        writeln!(f, "age: {}", self.age)?;
        writeln!(f, "num mean free paths: {}", self.num_mean_free_paths)?;
        writeln!(f, "mean free path: {}", self.mean_free_path)?;
        writeln!(f, "segment path length: {}", self.segment_path_length)?;
        writeln!(f, "random number seed: {}", self.random_number_seed)?;
        writeln!(f, "identifier: {}", self.identifier)?;
        writeln!(f, "last event: {:?}", self.last_event)?;
        writeln!(f, "num collisions: {}", self.num_collisions)?;
        writeln!(f, "num segments: {}", self.num_segments)?;
        writeln!(f, "task: {}", self.task)?;
        writeln!(f, "species: {:?}", self.species)?;
        writeln!(f, "breed: {}", self.breed)?;
        writeln!(f, "energy group: {}", self.energy_group)?;
        writeln!(f, "domain: {}", self.domain)?;
        writeln!(f, "cell: {}", self.cell)?;
        writeln!(f, "facet: {}", self.facet)?;
        writeln!(f, "normal dot: {}", self.normal_dot)
    }
}
