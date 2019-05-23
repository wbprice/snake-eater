use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in snake eater.rs
use crate::snake_eater::{BigBoss, ARENA_HEIGHT, BIG_BOSS_HEIGHT};

pub struct BigBossSystem;

impl<'s> System<'s> for BigBossSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, BigBoss>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut transforms, big_bosses, input): Self::SystemData) {
        for (_big_boss, transform) in (&big_bosses, &mut transforms).join() {
            let x_mov = input.axis_value("horizontal");
            let y_mov = input.axis_value("vertical");

            if let Some(mv_amount) = x_mov {
                let scaled_amount = 2.0 * mv_amount as f32;
                transform.translate_x(scaled_amount);
            }

            if let Some(mv_amount) = y_mov {
                let scaled_amount = 2.0 * mv_amount as f32;
                transform.translate_y(scaled_amount);
            }
        }
    }
}