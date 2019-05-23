use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

use crate::snake_eater::{Snake, ARENA_HEIGHT, ARENA_WIDTH};

pub struct MoveSnakesSystem;

impl<'s> System<'s> for MoveSnakesSystem {
    type SystemData = (
        WriteStorage<'s, Snake>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut snakes, mut locals, time): Self::SystemData) {
        for (snake, local) in (&mut snakes, &mut locals).join() {
            let snake_x = local.translation().x;
            let snake_y = local.translation().y;

            // Prepare to bounce at the top and the bottom bounds of the screen.
            if snake_y >= ARENA_HEIGHT - snake.height / 2.0 && snake.velocity[1] > 0.0 {
                snake.velocity[1] = -snake.velocity[1];
            } else if snake_y <= snake.height / 2.0 && snake.velocity[1] < 0.0 {
                snake.velocity[1] = -snake.velocity[1];
            }

            // Prepare to bounce at the left and right bounds of the screen.
            if snake_x >= ARENA_WIDTH - snake.width / 2.0 && snake.velocity[0] > 0.0 {
                snake.velocity[0] = -snake.velocity[0];
            } else if snake_x <= snake.width / 2.0 && snake.velocity[0] < 0.0 {
                snake.velocity[0] = -snake.velocity[0];
            }

            // Move the snake one tick.
            local.translate_x(snake.velocity[0] * time.delta_seconds());
            local.translate_y(snake.velocity[1] * time.delta_seconds());
        }
    }
}