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

use macroquad::{prelude::*, audio::{play_sound, PlaySoundParams}};

const FRAME_INDENT:f32 = 25.0;

pub enum GameState {
    Game,
    Pause,
    Intro,
    LevelFail,
    LevelCompleted,
    GameOver,
    GameCompleted,
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

            GameState::GameCompleted => {
                level.draw();
                draw_game_completed_text(resources.font);

                if is_key_pressed(KeyCode::Space) {
                    level.lvl_num = 1;
                    game_state = GameState::Intro;
                }
            }

            GameState::LevelCompleted => {
                level.draw();
                draw_level_completed_text(resources.font);
                if is_key_pressed(KeyCode::Space) {
                    level.lvl_num = level.lvl_num + 1;
                    level.set_level(level.lvl_num).await;
                    ball.vertical_dir = ball::VerticalDir::Up;
                    paddle.x = screen_width()/2.0;
                    ball.x = paddle.center_x();
                    ball.y = paddle.y - 16.0;
                    game_state = GameState::InitLevel;
                }
            }

            GameState::InitLevel => {
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
                    4 => {
                        lvl = LVL_4;
                    }
                    5 => {
                        lvl = LVL_5;
                    }
                    6 => {
                        lvl = LVL_6;
                    }
                    _ => {
                        panic!("no such level!")
                    }
                }

                level.set_level(level.lvl_num).await;
                level.bricks_amount = 0;
                for i in 0..lvl.len() {
                    brick_y = brick_y + 20.0;
                    for j in lvl[i] {
                        if j != " " {
                            bricks.push(
                                Brick::new(brick_x, brick_y, j).await,
                            );
                            level.bricks_amount = level.bricks_amount + 1;
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
                    game_state = GameState::InitLevel;
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

                let score = game.score();
                let lives = game.lives();

                for brick in &mut bricks {
                    brick.draw();

                    if !brick.destroyed {
                        if let Some(_i) = ball.rect.intersect(brick.left_side) {
                            brick.destroyed = true;
                            level.bricks_amount = level.bricks_amount - 1;
                            game.update_game(score+1, lives);
                            ball.horizontal_dir = ball::HorizontalDir::Left;
                            play_sound(resources.destroyed_block, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        } else 
                        if let Some(_i) = ball.rect.intersect(brick.right_side) {
                            brick.destroyed = true;
                            level.bricks_amount = level.bricks_amount - 1;
                            game.update_game(score+1, lives);
                            ball.horizontal_dir = ball::HorizontalDir::Right;
                            play_sound(resources.destroyed_block, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        } else
                        if let Some(_i) = ball.rect.intersect(brick.up_side) {
                            brick.destroyed = true;
                            level.bricks_amount = level.bricks_amount - 1;
                            game.update_game(score+1, lives);
                            ball.vertical_dir = ball::VerticalDir::Up;
                            play_sound(resources.destroyed_block, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        } else
                        if let Some(_i) = ball.rect.intersect(brick.down_side) {
                            brick.destroyed = true;
                            level.bricks_amount = level.bricks_amount - 1;
                            game.update_game(score+1, lives);
                            ball.vertical_dir = ball::VerticalDir::Down;
                            play_sound(resources.destroyed_block, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        }
                    }
                }

                if level.bricks_amount < 1 {
                    if level.lvl_num == 6 {
                        game_state = GameState::GameCompleted;
                    } else {
                        game_state = GameState::LevelCompleted;
                    }
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
                    level.lvl_num = level.lvl_num + 1;
                    level.set_level(level.lvl_num).await;
                    game_state = GameState::InitLevel;
                }

                match ball.horizontal_dir { 
                    ball::HorizontalDir::Left => {
                        if ball.x < 16.0 {
                            ball.horizontal_dir = ball::HorizontalDir::Right;
                            play_sound(resources.ball_hit, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        }
                    },
                    ball::HorizontalDir::Right => {
                        if ball.x > screen_width() - 16.0 - ball.ball_width() {
                            ball.horizontal_dir = ball::HorizontalDir::Left;
                            play_sound(resources.ball_hit, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        }
                    },
                }

                match ball.vertical_dir {
                    ball::VerticalDir::Up => {
                        if ball.y < 16.0 {
                            ball.vertical_dir = ball::VerticalDir::Down;
                            play_sound(resources.ball_hit, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        }
                    },
                    ball::VerticalDir::Down => {
                        if ball.center_x() < paddle.x + paddle.width() && ball.center_x() > paddle.x && ball.y > paddle.y {
                            ball.vertical_dir = ball::VerticalDir::Up;
                            play_sound(resources.paddle_hit, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        }

                        if ball.y + ball.ball_height() > resources.frame_texture.height() {
                            game_state = GameState::LevelFail;
                            play_sound(resources.ball_lost, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                        }
                    },
                }
            }
        }

        next_frame().await
    }
}
