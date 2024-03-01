use bevy::sprite::MaterialMesh2dBundle;
use leafwing_input_manager::action_state;

use self::{
    input::GalaxyInputPlugin,
    planet::{Planet, PlanetNormal},
};
use crate::{physics::*, prelude::*};

mod input;

use self::input::*;

/// This should hold all the information required for players. How they move,
/// interact with the world, are controlled, ect.
pub struct GalaxyPlayerPlugin;

impl Plugin for GalaxyPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GalaxyInputPlugin)
            .add_event::<PlayerMovement>()
            .add_event::<PlayerJump>()
            .add_systems(OnEnter(EngineState::InGame), Self::setup)
            .add_systems(
                FixedUpdate,
                ((player_movement_reciever, player_jump_reciever)
                    .after(input::GalaxyInputPlugin::player_input_sender))
                .run_if(in_state(EngineState::InGame)),
            )
            .add_systems(OnExit(EngineState::InGame), teardown::<Loaded>);
    }
}

#[derive(Component, Debug)]
pub struct Player;

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

        let collider = Collider::capsule(50., 25.);

        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vec2::ONE * 0.99, 10);

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
            collider,
            RigidBody::Dynamic,
            Loaded,
            LinearDamping(0.5),
            AngularDamping(0.5),
            ShapeCaster::new(caster_shape, Vec2::ZERO, 0.0, Direction2d::NEG_Y)
                .with_max_time_of_impact(10.0),
            // RayCaster::new(Vec2::ZERO, Direction2d::NEG_Y).with_max_hits(1)
        ));
    }
}

#[derive(Event, Debug)]
pub struct PlayerJump;

#[derive(Event, Debug)]
pub struct PlayerMovement {
    pub direction: Direction2d,
}

fn player_movement_reciever(
    mut events: EventReader<PlayerMovement>,
    mut query: Query<(&mut LinearVelocity, Has<Grounded>), With<Player>>,
    time: Res<Time>,
) {
    for movement in events.read() {
        for (mut linear_velocity, is_grounded) in &mut query {
            **linear_velocity += *movement.direction * Vec2::splat(500. * time.delta_seconds());
        }
    }
}

fn player_jump_reciever(
    mut commands: Commands,
    mut events: EventReader<PlayerJump>,
    mut query: Query<
        (&Rotation, &mut LinearVelocity, &PlanetNormal),
        (With<Player>, With<Grounded>),
    >,
    mut planet_query: Query<&GlobalTransform, With<Planet>>,
    time: Res<Time>,
) {
    for jump in events.read() {
        for (rotation, mut linear_velocity, local_down) in &mut query {
            **linear_velocity -= ***local_down * 25.;
        }
    }
}
