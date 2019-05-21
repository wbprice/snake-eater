use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use SnakeEater::Snake;

pub struct MoveSnakesSystem;

impl<'s> System<'s> for MoveSnakesSystem {
    type SystemData = {
        ReadStorage<'s, Snake>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    }

    fn run(&mut self, (snakes, mut locals, time): Self::SystemData) {
        for (snake, local) in (&snakes, &mut locals).join() {
            local.translate_x(snake.velocity[0] * time.delta_seconds());
            local.translate_y(snake.velocity[1] * time.delta_seconds());
        }
    }
}