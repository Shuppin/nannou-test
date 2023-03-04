use nannou::{glam::Vec2, draw::mesh::vertex::Color};

pub struct Particle {
    pub pos: Vec2,
    pub old_pos: Vec2,
    pub force: Vec2,
    pub mass: f32,

    pub radius: f32,
    pub colour: Color,
    pub restitution: f32,

    id: u32,
}

impl Particle {
    pub fn new(pos: Vec2, mass: f32, radius: f32, colour: Color, id: u32) -> Self {
        Self {
            pos,
            old_pos: pos,
            force: Vec2::ZERO,
            mass,
            radius,
            colour,
            restitution: 0.85,
            id
        }
    }

    pub fn add_impulse(&mut self, amount_x: f32, amount_y: f32) {
        self.force += Vec2::new(amount_x, amount_y);
    } 
}

pub struct Stick {
    pub id_1: u32,
    pub id_2: u32,
    pub distance: f32
}

pub struct PhysicsWorld {
    objects: Vec<Particle>,
    connections: Vec<Stick>,
    gravity: f32,
    world_bounds: Vec2,
    scale: f32, // Meter to pixel ratio
    current_id: u32
}

impl PhysicsWorld {

    pub fn new(gravity: f32, world_bounds: Vec2, scale: f32) -> Self {
        Self {
            objects: vec![],
            connections: vec![],
            gravity,
            world_bounds,
            scale,
            current_id: 0,
        }
    }

    pub fn set_bounds(&mut self, new_bounds: Vec2) {
        if self.world_bounds != new_bounds {
            self.world_bounds = new_bounds
        }
    }

    pub fn next_id(&self) -> u32 {
        let current = self.current_id;
        self.current_id += 1;
        current
    }

    pub fn get_particle_by_id(&mut self, target_id: u32) -> Option<&mut Particle> {
        let mut left = 0;
        let mut right = self.objects.len() - 1;
    
        while left <= right {
            let mid = left + (right - left) / 2;
    
            if self.objects[mid].id == target_id {
                return Some(&mut self.objects[mid]);
            } else if self.objects[mid].id < target_id {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    
        None
    }

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.objects
    }

    pub fn add_object(&mut self, object: Particle) {
        self.objects.push(object);
    }

    pub fn get_sticks(&self) -> &Vec<Stick> {
        &self.connections
    }

    pub fn add_stick(&mut self, stick: Stick) {
        self.connections.push(stick)
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.connections.clear();
    }

    pub fn add_impulses(&mut self, amount_x: f32, amount_y: f32) {
        for particle in &mut self.objects {
            particle.add_impulse(amount_x, amount_y)
        }
    }

    // Update each particle's position based on elapsed time and acceleration
    pub fn step(&mut self, delta_time: f32) {
        for particle in &mut self.objects {
            // Compute new velocity
            let vel = particle.pos - particle.old_pos;

            // Current posiion becomes the old position
            particle.old_pos = particle.pos;

            // Compute acceleration using F = ma
            let mut acc = particle.force / particle.mass;

            acc.y += self.gravity;

            // Predict new position using Verlet integration
            particle.pos += vel + acc*self.scale * delta_time * delta_time;
            
            acc.y -= self.gravity;

            // Check if the particle has hit the bottom and top of the bounds
            if particle.pos.y - particle.radius < -self.world_bounds.y / 2.0 {
                particle.pos.y = -self.world_bounds.y / 2.0 + particle.radius;
                particle.old_pos.y = particle.pos.y + vel.y
            } else if particle.pos.y + particle.radius > self.world_bounds.y / 2.0 {
                particle.pos.y = self.world_bounds.y / 2.0 - particle.radius;
                particle.old_pos.y = particle.pos.y + vel.y
            }

            // Check for collision with the left and right walls of the bounds
            if particle.pos.x - particle.radius < -self.world_bounds.x / 2.0 {
                particle.pos.x = -self.world_bounds.x / 2.0 + particle.radius;
                particle.old_pos.x = particle.pos.x + vel.x
            } else if particle.pos.x + particle.radius > self.world_bounds.x / 2.0 {
                particle.pos.x = self.world_bounds.x / 2.0 - particle.radius;
                particle.old_pos.x = particle.pos.x + vel.x
            }

            particle.force = Vec2::ZERO
            
        }
    }
    
}

fn distance(p1: Vec2, p2: Vec2) -> f32 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.x;
    (dx*dx + dy*dy).sqrt()
}