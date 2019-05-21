use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};
use rand::Rng;

use crate::snake_eater::{Snake, BigBoss, ARENA_HEIGHT, ARENA_WIDTH};

pub struct EatSnakesSystem;

impl<'s> System<'s> for EatSnakesSystem {
    type SystemData = (
        WriteStorage<'s, Snake>,
        ReadStorage<'s, BigBoss>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut snakes, big_boss, transforms): Self::SystemData) {
        // Check for snake collisions
        for (snake, transform) in (&mut snakes, &transforms).join() {
            let snake_x = transform.translation().x;
            let snake_y = transform.translation().y;

            // Bounce at the top and the bottom
            if snake_y >= ARENA_HEIGHT - snake.height / 2.0 && snake.velocity[1] > 0.0 {
                snake.velocity[1] = -snake.velocity[1];
            } else if snake_y <= snake.height / 2.0 && snake.velocity[1] < 0.0 {
                snake.velocity[1] = -snake.velocity[1];
            }

            // Bounce at the left and right
            if snake_x >= ARENA_WIDTH - snake.width / 2.0 && snake.velocity[0] > 0.0 {
                snake.velocity[0] = -snake.velocity[0];
            } else if snake_x <= snake.width / 2.0 && snake.velocity[0] < 0.0 {
                snake.velocity[0] = -snake.velocity[0];
            }

            // Check if was eaten
            for (big_boss, big_boss_transform) in (&big_boss, &transforms).join() {
                let big_boss_x = big_boss_transform.translation().x - big_boss.width * 0.5;
                let big_boss_y = big_boss_transform.translation().y - big_boss.height * 0.5;

                if is_point_in_rect(
                    snake_x,
                    snake_y,
                    big_boss_x - snake.width / 2.0,
                    big_boss_y - snake.height / 2.0,
                    big_boss_x + big_boss.width + snake.width / 2.0,
                    big_boss_y + big_boss.height + snake.height / 2.0
                ) {
                    println!("should eat snake!");

                    let coordinates = get_random_place();
                    dbg!(coordinates);
                    // transform.set_xyz(coordinates[0], coordinates[1], coordinates[2]);
                }
            }
        }
    }
}


fn is_point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn get_random_place() -> [f32; 3] {
    let mut rng = rand::thread_rng();
    [
        rng.gen_range(0.0, ARENA_WIDTH),
        rng.gen_range(0.0, ARENA_HEIGHT),
        0.0
    ]
}