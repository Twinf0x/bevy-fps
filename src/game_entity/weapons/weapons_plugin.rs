use bevy::prelude::*;
use crate::game_entity::behaviour_components::*;
use crate::game_entity::player::player_components::*;

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_bullets.system());
    }
}

fn update_bullets(
    time: Res<Time>,
    commands: &mut Commands, 
    mut bullets: Query<(Entity, &mut GlobalTransform, &Bullet)>,
    mut targets: Query<(&GlobalTransform, &mut Destructable), Without<Player>>,
    walls: Query<&Obstacle>
) {
    for (bullet_entity, mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.velocity * time.delta_seconds();

        for (enemy_transform, mut enemy_health) in targets.iter_mut() {
            if transform.translation.distance(enemy_transform.translation) <= bullet.hit_range {
                enemy_health.current_health -= bullet.damage_on_hit;
                commands.despawn_recursive(bullet_entity);
                break;
            }
        }

        for wall in walls.iter() {
            if transform.translation.x < wall.bottom_left.x {
                continue;
            }

            if transform.translation.x > wall.top_right.x {
                continue;
            }

            if transform.translation.z < wall.bottom_left.y {
                continue;
            }

            if transform.translation.z < wall.top_right.y {
                continue;
            }

            commands.despawn_recursive(bullet_entity);
        }
    }
}