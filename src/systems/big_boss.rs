use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in snake eater.rs
use crate::snake_eater::{BigBoss, ARENA_HEIGHT, ARENA_WIDTH};

pub struct BigBossSystem;

impl<'s> System<'s> for BigBossSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, BigBoss>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut transforms, big_bosses, input): Self::SystemData) {
        for (big_boss, transform) in (&big_bosses, &mut transforms).join() {
            let x_mov = input.axis_value("horizontal");
            let y_mov = input.axis_value("vertical");

            let big_boss_x = transform.translation().x;
            let big_boss_y = transform.translation().y;

            if let Some(mv_amount) = x_mov {
                let scaled_amount = 2.0 * mv_amount as f32;
                // Prevent the player from walking off the left and right of the screen.
                if scaled_amount > 0.0 && big_boss_x <= ARENA_WIDTH - big_boss.width / 2.0 {
                    transform.translate_x(scaled_amount);
                }
                else if scaled_amount < 0.0 && big_boss_x >= 0.0 + big_boss.width / 2.0 {
                    transform.translate_x(scaled_amount);
                }
            }

            if let Some(mv_amount) = y_mov {
                let scaled_amount = 2.0 * mv_amount as f32;
                // Prevent the player from walking off the top or the bottom of the screen.
                if scaled_amount > 0.0 && big_boss_y <= ARENA_HEIGHT - big_boss.height / 2.0 {
                    transform.translate_y(scaled_amount);
                }
                else if scaled_amount < 0.0 && big_boss_y > 0.0 + big_boss.height / 2.0 {
                    transform.translate_y(scaled_amount);
                }
            }
        }
    }
}