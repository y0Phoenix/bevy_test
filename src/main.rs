use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, utils::HashMap};
use bevy_animations::{prelude::*, Handles};
use bevy_rapier2d::prelude::*;
use state::StatePlugin;

mod state;

pub const PLAYER_IDLE_FRAMES: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
pub const PLAYER_IDLE_TIMINGS: [f32; 7] = [0.075, 0.075, 0.075, 0.075, 0.075, 0.075, 0.075];
pub const PLAYER_WALKING_FRAMES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
pub const PLAYER_WALKING_METERS_PER_FRAME: f32 = 0.075;
pub const PLAYER_RUNNING_FRAMES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
pub const PLAYER_RUNNING_METERS_PER_FRAME: f32 = 0.0625;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Test".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .insert_resource(RapierConfiguration::new(10.))
        .add_plugins((
            RapierDebugRenderPlugin::default(),
            AnimationsPlugin {
                pixels_per_meter: 100.,
            },
            StatePlugin
        ))
        .init_resource::<AnimationTextures>()
        .add_systems(Startup, (load_assets, spawn_things, load_animations,).chain())
        .add_systems(Update, (move_player,))
        .run()
    ;
}

#[derive(Debug, Default, Resource)]
struct AnimationTextures(HashMap<String, Texture>);

#[derive(Debug, Default)]
struct Texture {
    pub sprite: SpriteBundle,
    pub layout: Handle<TextureAtlasLayout>
}

#[derive(Debug, Default, Component)]
struct Player;

fn move_player(
    mut query: Query<(&mut Transform, &mut TextureAtlas, &mut Handle<Image>, Entity, &mut Animator), With<Player>>,
    mut event: EventWriter<AnimationEvent>,
    input: Res<ButtonInput<KeyCode>>,
    textures: Res<AnimationTextures>,
    time: Res<Time>,
) {
    let (mut transform, _atlas, mut handle, player, mut animator) = query.single_mut();
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x = transform.translation.x - 100. * time.delta_seconds();
        animator.change_direction(AnimationDirection::Left);
        event.send(AnimationEvent("player_walking", player));
    }
    else if input.pressed(KeyCode::KeyD) {
        transform.translation.x = transform.translation.x + 100. * time.delta_seconds();
        animator.change_direction(AnimationDirection::Right);
        event.send(AnimationEvent("player_walking", player));
    }
    if input.just_pressed(KeyCode::KeyZ) {
        *handle = textures.0.get("Player2").expect("Should Exist").sprite.texture.clone();
    }
}

fn spawn_things(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    textures: Res<AnimationTextures>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMax { max_width: 1200., max_height: 675. },
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        ..Default::default()
    });
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(250., 75.))),
            material: materials.add(Color::linear_rgb(100., 0., 0.)),
            transform: Transform::from_translation(Vec3::new(0., -250., 0.)),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(125., 37.5),
    ));
    let player_texture = textures.0.get("Player1").expect("Should exist");
    commands.spawn((
        player_texture.sprite.clone(),
        TextureAtlas {
            layout: player_texture.layout.clone(),
            index: 0,
        },
        RigidBody::Dynamic,
        Player,
        Animator::default(),
    )).with_children(|parent| {
        parent.spawn((
            Collider::cuboid(12., 16.),
            TransformBundle::from(Transform::from_translation(Vec3::new(0., -22., 0.))) 
        ));
    });
}

fn load_assets(
    asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut textures: ResMut<AnimationTextures>,
) {
    let handle_1 = asset_server.load("NightBorne.png");
    let handle_2 = asset_server.load("player_sheet_1.png");
    let atlas = TextureAtlasLayout::from_grid(
        UVec2::new(96, 84),
        14,
        38,
        None,
        None,
    );
    let layout = texture_atlases.add(atlas);
    textures.0.insert("Player1".to_string(), Texture {
        sprite: SpriteBundle {
            texture: handle_1,
            transform: Transform::from_translation(Vec3::ZERO),
            ..Default::default()
        },
        layout: layout.clone()
    });
    textures.0.insert("Player2".to_string(), Texture {
        sprite: SpriteBundle {
            texture: handle_2,
            transform: Transform::from_translation(Vec3::ZERO),
            ..Default::default()
        },
        layout: layout.clone()
    });
}

fn load_animations(
    query: Query<Entity, (With<Player>, With<Animator>)>,
    mut animations: ResMut<Animations>,
    textures: Res<AnimationTextures>,
) {
    let player = query.single();
    let texture = textures.0.get("Player2").expect("Should exist");
    animations.insert_animation(NewAnimation { 
        handles: Handles::new(texture.sprite.texture.clone(), texture.layout.clone()),
        animation: AnimationType::Transform(
            TransformAnimation::new(
                Vec::from(PLAYER_WALKING_FRAMES),
                PLAYER_WALKING_METERS_PER_FRAME,
                Vec2::new(14., 38.),
                AnimationDirectionIndexes::FlipBased(FlipBasedDirection {
                    left_direction_is_flipped: true,
                    x_direction_index: 4,
                }),
                true,
            ),
            "player_walking",
        ),
    }, Some(player));
}
