use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::snake_eater::{Snake, ARENA_HEIGHT, ARENA_WIDTH};

pub struct MoveSnakesSystem;

impl<'s> System<'s> for MoveSnakesSystem {
    type SystemData = (
        ReadStorage<'s, Snake>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut snakes, mut transforms, time): Self::SystemData) {
        for (snake, transform) in (&mut snakes, &mut transforms).join() {
            transform.translate_x(snake.velocity[0] * time.delta_seconds());
            transform.translate_y(snake.velocity[1] * time.delta_seconds());
            
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
        }
    }
}