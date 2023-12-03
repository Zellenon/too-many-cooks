use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

struct RayCast {
    max_distance: f32,
    angle: f32,
    
}

impl RayCast {
    fn new(angle: f32) -> Self {
        Self {
            max_distance: 1000.0,
            angle,
        }
    }
}

// contains a list of raycasts that are used to determine the distance of the engine to the nearest obstacle
#[derive(Component)]
pub struct EngineDistance {
    raycasts: Vec<RayCast>,
}

impl EngineDistance {
    pub fn new(num_raycasts: usize, spread_angle: f32) -> Self {
        let mut raycasts = Vec::new();
        for i in 0..num_raycasts {
            let angle = i as f32 * spread_angle - spread_angle * (num_raycasts as f32 / 2.0);
            raycasts.push(RayCast::new(angle));
        }
        Self { raycasts }
    }
    
    // pub fn get_closest_distance(&self) -> f32 {
    //     let mut closest_distance = f32::MAX;
    //     for raycast in &self.raycasts {
    //         if raycast.distance < closest_distance {
    //             closest_distance = raycast.distance;
    //         }
    //     }
    //     closest_distance
    // }
}