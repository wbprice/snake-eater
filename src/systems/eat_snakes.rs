use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, Write, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiText
};
use rand::Rng;

use crate::snake_eater::{Snake, BigBoss, ARENA_HEIGHT, ARENA_WIDTH, ScoreText, Scoreboard};

pub struct EatSnakesSystem;

impl<'s> System<'s> for EatSnakesSystem {
    type SystemData = (
        ReadStorage<'s, BigBoss>,
        ReadStorage<'s, Snake>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, Scoreboard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (big_bosses, snakes, mut transforms, mut ui_text, mut scores, score_text): Self::SystemData) {
        // Get position of Big Boss
        let (mut big_boss_x, mut big_boss_y, mut big_boss_height, mut big_boss_width) = (0.0, 0.0, 0.0, 0.0);
        for (big_boss, transform) in (&big_bosses, &transforms).join() {
            big_boss_x = transform.translation().x;
            big_boss_y = transform.translation().y;
            big_boss_width = big_boss.width;
            big_boss_height = big_boss.height;
        }

        // Get position of each snake so we can check for collisions
        for (snake, transform) in (&snakes, &mut transforms).join() {
            let snake_x = transform.translation().x;
            let snake_y = transform.translation().y;

            if is_point_in_rect(
                snake_x,
                snake_y,
                big_boss_x - snake.width / 2.0,
                big_boss_y - snake.height / 2.0,
                big_boss_x + big_boss_width + snake.width / 2.0,
                big_boss_y + big_boss_height + snake.height / 2.0
            ) {
                let coordinates = get_random_place();
                transform.set_xyz(coordinates[0], coordinates[1], coordinates[2]);
                // update health text
                scores.health = (scores.health + 1).min(999);
                // update score
                if let Some(text) = ui_text.get_mut(score_text.health_score) {
                    text.text = scores.health.to_string();
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