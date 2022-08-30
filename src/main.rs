mod paddle;
use paddle::Paddle;

mod levels;
use levels::*;

mod game;
use game::Game;

mod labels;
use labels::Label;

mod brick;
use brick::Brick;

mod ball;
use ball::Ball;

mod resources;
use resources::Resources;

mod functions;
use functions::*;

use macroquad::prelude::*;

const FRAME_INDENT:f32 = 25.0;

pub enum GameState {
    Game,
    Pause,
    Intro,
    LevelFail,
    GameOver,
    InitLevel,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Arkanoid"
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: 700,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let mut level = Level::new().await;
    let mut paddle = Paddle::new().await;
    let mut game = Game::new().await;
    let mut ball = Ball::new(paddle.center_x(), paddle.y - 16.0).await;

    let mut bricks: Vec<Brick> = Vec::new();

    let level_label = Label::new("Level - ".to_string(), 40.0, 787.0, 50).await;
    let lives_label = Label::new("Lives - ".to_string(), 240.0, 787.0, 50).await;
    let score_label = Label::new("Score - ".to_string(), 440.0, 787.0, 50).await;
    
    let resources = Resources::new().await;

    let mut lvl;

    loop {
        clear_background(BLACK);
        
        match game_state {
            GameState::Intro => {
                draw_texture(resources.intro_texture, 0.0, 0.0, WHITE);

                if is_key_pressed(KeyCode::Space) {
                    game.update_game(0, 2);
                    game_state = GameState::InitLevel;
                }
            }

            GameState::InitLevel => {
                println!("loading...");
                let mut brick_x: f32 = FRAME_INDENT;
                let mut brick_y: f32 = 0.0;

                match level.lvl_num {
                    1 => {
                        lvl = LVL_1;
                    }
                    2 => {
                        lvl = LVL_2;
                    }
                    3 => {
                        lvl = LVL_3;
                    }
                    _ => {
                        panic!("no such level!")
                    }
                }

                for i in 0..lvl.len() {
                    brick_y = brick_y + 20.0;
                    for j in lvl[i] {
                        if j != 0 {
                            bricks.push(
                                Brick::new(brick_x, brick_y, j).await,
                            );
                        }
                        brick_x = brick_x + 50.0;
                    }
                    brick_x = FRAME_INDENT;
                }
                game_state = GameState::Game;
            }

            GameState::LevelFail => {
                level.draw();
                draw_texture(resources.frame_texture, 0.0, 0.0, WHITE);
                draw_level_failed_text(resources.font);
                
                if is_key_pressed(KeyCode::Space) {
                    if game.lives() > 0 {
                        let score = game.score();
                        let lives = game.lives();
                        game.update_game(score, lives-1);
                        game_state = GameState::Game;
                        ball.vertical_dir = ball::VerticalDir::Up;
                        paddle.x = screen_width()/2.0;
                        ball.x = paddle.center_x();
                        ball.y = paddle.y - 16.0;
                    } else {
                        game_state = GameState::GameOver;
                    }
                }

                level_label.draw_label(level.number());
                lives_label.draw_label(game.lives());
                score_label.draw_label(game.score());
            }

            GameState::GameOver => {
                level.draw();
                draw_game_over_text(resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game.update_game(0, 2);
                    level.lvl_num = 1;
                    game_state = GameState::Game;
                    bricks.clear();
                    ball.vertical_dir = ball::VerticalDir::Up;
                    paddle.x = screen_width()/2.0;
                    ball.x = paddle.center_x();
                    ball.y = paddle.y - 16.0;
                }
            }

            GameState::Pause => {
                level.draw();
                draw_paused_text(resources.font);

                if is_key_pressed(KeyCode::Space) | is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Game;
                }
            }

            GameState::Game => {
                paddle.update(get_frame_time());
                ball.update(get_frame_time());

                level.draw();
                draw_texture(resources.frame_texture, 0.0, 0.0, WHITE);
                paddle.draw();
                ball.draw();
                level_label.draw_label(level.number());
                lives_label.draw_label(game.lives());
                score_label.draw_label(game.score());

                for brick in &mut bricks {
                    brick.draw();
                    //brick.update(get_frame_time());
                }
                
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Pause;
                }
                
                // For debug
                if is_key_pressed(KeyCode::C) {
                    paddle.kind = paddle::Kind::Catch;
                }
                if is_key_pressed(KeyCode::N) {
                    paddle.kind = paddle::Kind::Normal;
                }
                if is_key_pressed(KeyCode::L) {
                    paddle.kind = paddle::Kind::Laser;
                }
                if is_key_pressed(KeyCode::E) {
                    paddle.kind = paddle::Kind::Expanded;
                }
                if is_key_pressed(KeyCode::Q) {
                    bricks.clear();
                    level.increase_level().await;
                    game_state = GameState::InitLevel;
                }

                match ball.horizontal_dir { 
                    ball::HorizontalDir::Left => {
                        if ball.x < 16.0 {
                            ball.horizontal_dir = ball::HorizontalDir::Right;
                        }
                    },
                    ball::HorizontalDir::Right => {
                        if ball.x > screen_width() - 16.0 - ball.ball_width() {
                            ball.horizontal_dir = ball::HorizontalDir::Left;
                        }
                    },
                }

                match ball.vertical_dir {
                    ball::VerticalDir::Up => {
                        if ball.y < 16.0 {
                            ball.vertical_dir = ball::VerticalDir::Down;
                        }
                    },
                    ball::VerticalDir::Down => {
                        if ball.center_x() < paddle.x + paddle.width() && ball.center_x() > paddle.x && ball.y > paddle.y {
                            ball.vertical_dir = ball::VerticalDir::Up;
                        }

                        if ball.y + ball.ball_height() > resources.frame_texture.height() {
                            game_state = GameState::LevelFail;
                        }
                    },
                }
            }
        }

        next_frame().await
    }
}
