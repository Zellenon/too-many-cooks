use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct RayCastPlugin;

impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RayCastEvent>()
            .add_systems(Update, ray_cast_system);
    }
}

#[derive(Component)]
pub struct RayCast {
    pub max_distance: f32,
    pub angle: f32,
}

impl RayCast {
    pub fn new(angle: f32) -> Self {
        Self {
            max_distance: 1000.0,
            angle,
        }
    }
}

#[derive(Event, Debug)]
pub struct RayCastEvent {
    pub raycast_entity: Entity,
    pub collision_entity: Entity,
    pub intersection_point: Vec2,
    pub intersection_normal: Vec2,
    pub distance: f32,
    pub origin: Vec2,
    pub direction: Vec2,
}

fn ray_cast_system (
    raycast_info: Query<(Entity, &GlobalTransform, &RayCast)>,
    rapier_context: Res<RapierContext>,
    mut raycast_events: EventWriter<RayCastEvent>,
) {
    for (entity_id, global_transform, raycast) in raycast_info.iter().map(|(z, x, y)| (z, x.compute_transform(), y)) {
        let ray_pos = global_transform.translation.truncate();
        let ray_angle = global_transform.rotation.to_euler(EulerRot::ZYX).0 + raycast.angle;
        let ray_dir = Vec2::new(ray_angle.cos(), ray_angle.sin());
        let max_toi = raycast.max_distance;
        
        let solid = true;
        let filter = QueryFilter::default();

        rapier_context.intersections_with_ray(
            ray_pos, ray_dir, max_toi, solid, filter,
            |entity, intersection| {
                let event = RayCastEvent {
                    raycast_entity: entity_id,
                    collision_entity: entity,
                    intersection_point: intersection.point,
                    intersection_normal: intersection.normal,
                    distance: intersection.toi,
                    origin: ray_pos,
                    direction: ray_dir,
                };
                raycast_events.send(event);
                true // Return `false` instead if we want to stop searching for other hits.
            });
    }
}