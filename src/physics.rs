use nannou::{glam::Vec2, draw::mesh::vertex::Color};

// TODO: Implement this properly
const FRICTION: f32 = 0.97;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub colour: Color,
    pub restitution: f32
}

pub struct PhysicsWorld {
    objects: Vec<Particle>,
    gravity: f32,
    world_bounds: Vec2,
    scale: f32 // Meter to pixel ratio
}

impl PhysicsWorld {

    pub fn new(gravity: f32, world_bounds: Vec2, scale: f32) -> Self {
        Self {
            objects: vec![],
            gravity,
            world_bounds,
            scale
        }
    }

    pub fn set_bounds(&mut self, new_bounds: Vec2) {
        if self.world_bounds != new_bounds {
            self.world_bounds = new_bounds
        }
    }

    pub fn get_objects(&self) -> &Vec<Particle> {
        &self.objects
    }

    pub fn add_object(&mut self, object: Particle) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // Update each particle's position based on elapsed time and acceleration
    pub fn step(&mut self, delta_time: f32) {
        for particle in &mut self.objects {
            // Calculate new position and velocity
            particle.pos += particle.vel * delta_time + 0.5 * self.gravity*self.scale * delta_time.powi(2);
            particle.vel.y += self.gravity*self.scale * delta_time;

            // Check if the particle has hit the bottom of the bounds
            if particle.pos.y - particle.radius < -self.world_bounds.y / 2.0 {
                particle.pos.y = -self.world_bounds.y / 2.0 + particle.radius;
                particle.vel.y *= -particle.restitution;
                // Note: Weird bug occurs were ball start drifting to the left instead of coming to a stop
                particle.vel *= FRICTION
            }

            // Check for collision with the left and right walls of the window
            if particle.pos.x - particle.radius < -self.world_bounds.x / 2.0 {
                particle.pos.x = -self.world_bounds.x / 2.0 + particle.radius;
                particle.vel.x *= -particle.restitution;
            } else if particle.pos.x + particle.radius > self.world_bounds.x / 2.0 {
                particle.pos.x = self.world_bounds.x / 2.0 - particle.radius;
                particle.vel.x *= -particle.restitution;
            }
            
            // Check for collision with the top of the window
            if particle.pos.y + particle.radius > self.world_bounds.y / 2.0 {
                particle.pos.y = self.world_bounds.y / 2.0 - particle.radius;
                particle.vel.y *= -particle.restitution;
            }
        }
    }

}
