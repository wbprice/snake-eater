use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub const ARENA_HEIGHT: f32 = 480.0;
pub const ARENA_WIDTH: f32 = 480.0;
const BIG_BOSS_HEIGHT: f32 = 32.0;
const BIG_BOSS_WIDTH: f32 = 64.0;

pub struct SnakeEater;

impl SimpleState for SnakeEater {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
            let world = data.world;
            world.register::<BigBoss>();

            // Load the spritesheet necessary to render the graphics.
            let sprite_sheet_handle = load_sprite_sheet(world);

            initialise_camera(world);
            initialise_big_boss(world, sprite_sheet_handle);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_big_boss(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut big_boss_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    big_boss_transform.set_xyz(BIG_BOSS_WIDTH * 0.5, y, 0.0);

    // Assign the sprite for Big Boss
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    world
        .create_entity()
        .with(BigBoss::new())
        .with(sprite_render.clone())
        .with(big_boss_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/snake_eater_proto_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/snake_eater_proto_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

pub struct BigBoss {
    width: f32,
    height: f32,
    health: f32
}

impl BigBoss {
    fn new() -> BigBoss {
        BigBoss {
            width: BIG_BOSS_WIDTH,
            height: BIG_BOSS_HEIGHT,
            health: 1.0
        }
    }
}

impl Component for BigBoss {
    type Storage = DenseVecStorage<Self>;
}

