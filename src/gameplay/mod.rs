pub mod arena;
pub mod player;
pub mod enemy;

use arena::setup_arena;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use enemy::{execute_enemy_behavior, setup_enemies, spawn_enemies, start_enemy_dying, EnemyReceiveHitEvent};
use player::{execute_bullets_lifetime, execute_player_behavior, on_bullet_collided, on_player_damaged, setup_player, BulletCollided, PlayerDamage, PlayerInfo};

use crate::{animation::play_animations, control::handle_camera_zoom, menu::in_game_menu::{setup_game_ui, update_game_ui}, GameState};

const GRP_PLAYER: Group = Group::GROUP_1;
const GRP_ENEMY: Group = Group::GROUP_2;
const GRP_ENVIRONMENT: Group = Group::GROUP_3;
const GRP_PLAYER_BULLET: Group = Group::GROUP_4;

#[derive(Component)]
pub struct LevelComponents;

#[derive(Resource)]
pub struct GameScore(pub u32);

#[derive(Component,Debug)]
pub enum CollidingObj {
    Player,
    Bullet,
    Enemy { dmg: u32 },
    Environment,
}

pub struct MyGameplayPlugin;

impl Plugin for MyGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), 
            (
                setup_arena,
                setup_player,
                setup_enemies,
                setup_game_ui,
            ).chain()
        );

        app
        .add_event::<PlayerDamage>()
        .add_event::<EnemyReceiveHitEvent>()
        .add_event::<BulletCollided>();
        app.add_systems(Update, 
            (
                execute_player_behavior,
                execute_enemy_behavior,
                execute_bullets_lifetime,
                spawn_enemies,
                play_animations,
                handle_camera_zoom,
                update_game_ui,
            ).run_if(in_state(GameState::InGame))
        )
        .add_systems(FixedUpdate, 
            (
                on_bullet_collided,
                start_enemy_dying,
                on_player_damaged,
                handle_game_over,
            ).run_if(in_state(GameState::InGame))
        )
        .add_systems(FixedUpdate, handle_collision.run_if(in_state(GameState::InGame)));
    }
}

pub fn handle_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_damage_writer: EventWriter<PlayerDamage>,
    mut enemy_gamage_writer: EventWriter<EnemyReceiveHitEvent>,
    mut bullet_collided: EventWriter<BulletCollided>,
    query: Query<&CollidingObj>,
    mut game_score: ResMut<GameScore>,
) {
    for collision_event in collision_events.read() {
        if let &CollisionEvent::Started(c1, c2 , _) = collision_event {
            let obj1 = query.get(c1).ok();
            let obj2 = query.get(c2).ok();
            use CollidingObj::*;
            match ((c1,obj1), (c2,obj2)) {
                ((c2,Some(Enemy {dmg})), (_,Some(Player))) | ((_,Some(Player)), (c2,Some(Enemy {dmg}))) => {
                    player_damage_writer.send(PlayerDamage(*dmg));
                    enemy_gamage_writer.send(EnemyReceiveHitEvent(c2));
                },
                ((c2,Some(Enemy {dmg:_})), (c1,Some(Bullet))) | ((c1,Some(Bullet)), (c2,Some(Enemy {dmg:_}))) => {
                    bullet_collided.send(BulletCollided(c1));
                    enemy_gamage_writer.send(EnemyReceiveHitEvent(c2));

                    // TODO: move from here
                    game_score.0 += 1;
                },
                ((_,Some(Environment)), (c1,Some(Bullet))) | ((c1,Some(Bullet)), (_,Some(Environment))) => {
                    bullet_collided.send(BulletCollided(c1));
                },
                _ => (),
            }
        }
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    player_info: Single<&PlayerInfo>,
) {
    if player_info.health <= 0 {
        commands.set_state(GameState::End);
    }
}