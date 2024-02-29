use bevy::sprite::MaterialMesh2dBundle;
use leafwing_input_manager::action_state;

use self::input::GalaxyInputPlugin;
use crate::{physics::*, prelude::*};

mod input;

use self::input::*;

/// This should hold all the information required for players. How they move,
/// interact with the world, are controlled, ect.
pub struct GalaxyPlayerPlugin;

impl Plugin for GalaxyPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GalaxyInputPlugin)
            .add_systems(OnEnter(EngineState::InGame), Self::setup)
            .add_systems(
                FixedUpdate,
                (Self::player_movement_sender, player_movement_reciever)
                    .chain()
                    .run_if(in_state(EngineState::InGame)),
            )
            .add_event::<PlayerMovement>()
            .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}

impl GalaxyPlayerPlugin {
    fn setup(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let input_map = InputMap::new([
            (Action::Left, KeyCode::KeyA),
            (Action::Right, KeyCode::KeyD),
            (Action::Down, KeyCode::KeyS),
            (Action::Up, KeyCode::KeyW),
            (Action::Jump, KeyCode::Space),
        ]);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Capsule2d::new(25., 50.)).into(),
                material: materials.add(Color::RED),
                transform: Transform::from_xyz(0., 100. + 50., 100.),
                ..default()
            },
            Name::new("Player"),
            InputManagerBundle::with_map(input_map),
            Player,
            Collider::capsule(50., 25.),
            RigidBody::Dynamic,
            Loaded,
            LinearDamping(0.5),
            AngularDamping(0.5),
        ));
    }

    fn player_movement_sender(
        mut commands: Commands,
        mut actions: Query<&ActionState<Action>, With<Player>>,
        mut event_writer: EventWriter<PlayerMovement>,
    ) {
        let action_state = actions.single();

        let mut overall_direction = Vec2::ZERO;

        Action::DIRECTIONS.into_iter().for_each(|direction_action| {
            if action_state.pressed(&direction_action) {
                if let Some(direction) = direction_action.direction() {
                    overall_direction += *direction;
                }
            }
        });

        let overall_direction = Direction2d::new(overall_direction);

        if let Ok(direction) = overall_direction {
            event_writer.send(PlayerMovement { direction });
        }
    }

    fn player_jump_sender(
        mut commands: Commands,
        mut actions: Query<&ActionState<Action>, With<Player>>,
        mut event_writer: EventWriter<PlayerJump>,
    ) {
        let action_state = actions.single();

        if action_state.pressed(&Action::Jump) {
            event_writer.send(PlayerJump);
        }
    }
}

#[derive(Event, Debug)]
struct PlayerJump;

fn player_movement_reciever(
    mut events: EventReader<PlayerMovement>,
    mut query: Query<(&mut LinearVelocity, Has<Grounded>), With<Player>>,
    time: Res<Time>,
) {
    for movement in events.read() {
        for (mut linear_velocity, is_grounded) in &mut query {
            **linear_velocity += *movement.direction * Vec2::splat(1000. * time.delta_seconds());
        }
    }
}

fn player_jump_reciever(
    mut commands: Commands,
    mut events: EventReader<PlayerJump>,
    mut query: Query<(Has<Grounded>, &Rotation, Entity), With<Player>>,
    time: Res<Time>,
) {
    for jump in events.read() {
        info!("{jump:?}");

        for (is_grounded, rotation, player) in &mut query {
            if is_grounded {
                // commands.entity(player).insert(ExternalImpulse::new());
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Event, Debug)]
pub struct PlayerMovement {
    pub direction: Direction2d,
}
