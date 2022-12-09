use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound, stop_sound}};
extern crate rand;
use rand::Rng;

mod resources;
use resources::Resources;

mod die_animation;
use die_animation::DieAnimation;

mod explode_animation;
use explode_animation::ExplodeAnimation;

mod bricks;
use bricks::Brick;

mod doors;
use doors::Door;

mod ball;
use ball::Ball;

mod powers;
use powers::Power;

mod enemy;
use enemy::Enemy;

mod game;
use game::Game;

mod paddle;
use paddle::{Paddle, PaddleType};

mod bullet;
use bullet::Bullet;

mod levels;
use levels::*;

mod points;
use points::Point;

fn window_conf() -> Conf {
    let mut title = String::from("Arkanoid v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: resources::RES_WIDTH,
        window_height: resources::RES_HEIGHT,
        ..Default::default()
    }
}

enum GameState {
    Intro,
    InitLevel,
    Game,
    Pause,
    LevelCompleted,
    LevelFailed,
    GameOver,
}

fn draw_info(game: &Game, resources: &Resources) {
    draw_level_number(resources.font, game.lvl_num.to_string().as_str());
    draw_lives(resources.font, game.lives.to_string().as_str());
    draw_score(resources.font, game.score.to_string().as_str());
}

fn get_percentage_rounded(x: f32, y: f32) -> i32 {
    let result = (x * 100.0) / y;
    return result.round() as i32
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let mut game = Game::new().await;
    let mut paddle = Paddle::new().await;
    let mut points: Vec<Point> = make_map_array(1);
    let mut bricks: Vec<Brick> = Vec::new();
    let resources = Resources::new().await;
    let mut door = Door::new().await;
    let mut die_animations: Vec<DieAnimation> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut explode_animations: Vec<ExplodeAnimation> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut powers: Vec<Power> = Vec::new();
    let mut balls: Vec<Ball> = Vec::new();

    play_sound(resources.intro_music, PlaySoundParams {
        looped: true,
        volume: 0.3,
    });
    
    loop {
        clear_background(BLACK);
        
        match game_state {
            GameState::Intro => {
                draw_texture(resources.intro_texture, 0.0, 0.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game.lvl_num = 1;
                    game.lives = 2;
                    game_state = GameState::InitLevel;
                    stop_sound(resources.intro_music);
                }
            },
            GameState::InitLevel => {
                points.clear();
                enemies.clear();
                bullets.clear();
                powers.clear();
                balls.clear();
                explode_animations.clear();
                points = make_map_array(game.lvl_num);
                game_state = GameState::Game;
                paddle.x = screen_width() / 2.0 - paddle.width() / 2.0;
                paddle.paddle_type = paddle::PaddleType::Normal;

                // Load bricks
                bricks = load_bricks(&points).await;

                // Set random bonuses
                for _ in 0..=resources::NUMBER_OF_BONUSES {
                    let index = rand::random::<usize>() % bricks.len();
                    bricks[index].brick_with_bonus = true;
                }

                balls.push(Ball::new(paddle.center_x(), paddle.y - 16.0).await);
                balls[0].vertical_dir = ball::VerticalDir::Up;
                balls[0].last_ball_time = get_time();
                game.last_speed_increased = get_time();
                game.balls_speed = resources::INIT_BALLS_SPEED;
                play_sound(resources.level_start, PlaySoundParams {
                    looped: false,
                    volume: 0.3,
                });
            },
            GameState::Game => {
                // Increate speed every 10 seconds
                if get_time() - game.last_speed_increased >= 10.0 {
                    game.balls_speed += 10.0;
                    game.last_speed_increased = get_time();
                }
                
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Pause;
                }

                set_cursor_grab(true);
                show_mouse(false);

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    match paddle.paddle_type {
                        paddle::PaddleType::Laser => {
                            bullets.push(
                                Bullet::new(paddle.x+5.0, paddle.y).await,
                            );
                            bullets.push(
                                Bullet::new(paddle.x+paddle.width()-11.0, paddle.y).await,
                            );
                            play_sound(resources.laser, PlaySoundParams {
                                looped: false,
                                volume: 0.2,
                            });
                        },
                        _ => {},
                    }
                }

                draw_map(&resources);
                paddle.update(get_frame_time());
                paddle.draw();
                for brick in &mut bricks {
                    brick.draw();
                }

                // Close open doors
                if get_time() - door.last_door_time >= 2.0 {
                    match door.status {
                        doors::Status::RightWait=>{door.status=doors::Status::RightClose;},
                        doors::Status::LeftWait=>{door.status=doors::Status::LeftClose;}
                        _ => {}, 
                    };
                }
                door.draw();

                for ball in &mut balls {
                    ball.speed = game.balls_speed;
                    if ball.released {
                        ball.update(get_frame_time(), &resources);
                    } else {
                        ball.x = paddle.center_x();
                        ball.y = paddle.y - 16.0;
                        if get_time() - ball.last_ball_time >= ball.idle_time  ||
                           is_key_pressed(KeyCode::Space) || 
                           is_mouse_button_pressed(MouseButton::Left) {
                            ball.released = true;
                        }
                    }
                    ball.draw();

                    if let Some(_i) = ball.rect.intersect(paddle.rect) {
                        ball.vertical_dir = ball::VerticalDir::Up;
                        
                        match paddle.paddle_type {
                            PaddleType::Catch => {
                                ball.last_ball_time = get_time();
                                ball.released = false;
                            },
                            _ => {
                                ball.released = true;
                                play_sound(resources.paddle_hit, PlaySoundParams {
                                    looped: false,
                                    volume: 0.3,
                                });
                            }
                        }

                        match get_percentage_rounded(ball.center_x() - paddle.x, paddle.width()) {
                            -20..=20 => {
                                ball.ball_step_move_x = 3.0;
                                ball.horizontal_dir = ball::HorizontalDir::Left;
                            },
                            21..=40 => {ball.ball_step_move_x = 2.0},
                            41..=70 => {ball.ball_step_move_x = 1.0},
                            71..=80 => {ball.ball_step_move_x = 2.0},
                            81..=120 => {
                                ball.ball_step_move_x = 3.0;
                                ball.horizontal_dir = ball::HorizontalDir::Right;
                            },
                            _ => {}    
                        };
                        break;
                    }

                    for brick in &mut bricks {
                        if let Some(_i) = ball.rect.intersect(brick.rect) {
                            if brick.power != 0 {
                                brick.power -= 1;
                            } else {
                                brick.destroyed = true;
                                if brick.brick_with_bonus {
                                    powers.push(
                                        Power::new(brick.x, brick.y).await,
                                    );
                                }
                            }
                            game.score += 10;
                            play_sound(resources.brick_hit, PlaySoundParams {
                                looped: false,
                                volume: 0.3,
                            });
                            match ball.vertical_dir {
                                ball::VerticalDir::Up => {
                                    ball.vertical_dir = ball::VerticalDir::Down;
                                },
                                ball::VerticalDir::Down => {
                                    ball.vertical_dir = ball::VerticalDir::Up;
                                },
                            };
                        }
                    }

                    for enemy in &mut enemies {
                        if let Some(_i) = ball.rect.intersect(enemy.rect) {
                            enemy.destroyed = true;
                            explode_animations.push(
                                ExplodeAnimation::new(enemy.x, enemy.y).await,
                            );
                            play_sound(resources.explode, PlaySoundParams {
                                looped: false,
                                volume: 0.3,
                            });
                            match ball.horizontal_dir {
                                ball::HorizontalDir::Left => {
                                    ball.horizontal_dir = ball::HorizontalDir::Right;
                                },
                                ball::HorizontalDir::Right => {
                                    ball.horizontal_dir = ball::HorizontalDir::Left;
                                },
                            };
                            match ball.vertical_dir {
                                ball::VerticalDir::Up => {
                                    ball.vertical_dir = ball::VerticalDir::Down;
                                },
                                ball::VerticalDir::Down => {
                                    ball.vertical_dir = ball::VerticalDir::Up;
                                },
                            };
                        }
                    }
                }

                for animation in &mut die_animations {
                    animation.draw();
                }

                // Enemy
                if enemies.len() < 5 {
                    if get_time() - game.last_enemy_burn_time >= 8.0 {
                        door.last_door_time = get_time();
                        game.last_enemy_burn_time = get_time();
                        match rand::thread_rng().gen_range(0..=1) { 
                            0 => {
                                door.status = doors::Status::LeftOpen;
                                enemies.push(
                                    Enemy::new(180.0, -40.0).await,
                                );
                            },
                            _ => {
                                door.status = doors::Status::RightOpen;
                                enemies.push(
                                    Enemy::new(500.0, -40.0).await,
                                );
                            },
                        };
                    }
                }

                for enemy in &mut enemies {
                    if get_time() - enemy.burn_time >= 1.0 {
                        enemy.update(paddle.x, paddle.y);
                    }
                    if let Some(_i) = enemy.rect.intersect(paddle.rect) {
                        enemy.destroyed = true;
                        explode_animations.push(
                            ExplodeAnimation::new(enemy.x, enemy.y).await,
                        );
                        play_sound(resources.explode, PlaySoundParams {
                            looped: false,
                            volume: 0.3,
                        });
                    }
                    enemy.draw();
                }

                for animation in &mut explode_animations {
                    animation.draw();
                }

                for power in &mut powers {
                    power.update(get_frame_time());
                    power.draw();

                    if let Some(_i) = paddle.rect.intersect(power.rect) {
                        power.destroyed = true;
                        game.score += 5;
                        play_sound(resources.bonus, PlaySoundParams {
                            looped: false,
                            volume: 0.2,
                        });
                        match power.power_type.to_string().as_str() {
                            "laser" => {paddle.paddle_type = paddle::PaddleType::Laser},
                            "catch" => {paddle.paddle_type = paddle::PaddleType::Catch},
                            "expand" => {paddle.paddle_type = paddle::PaddleType::Expand},
                            "slow" => {
                                paddle.paddle_type = paddle::PaddleType::Normal;
                                if game.balls_speed >= 60.0 {
                                    game.balls_speed -= 40.0;
                                }
                            },
                            "duplicate" => {
                                paddle.paddle_type = paddle::PaddleType::Normal;
                                balls.push(
                                    Ball::new(balls[0].x, balls[0].y).await,
                                );
                                balls[0].vertical_dir = ball::VerticalDir::Up;
                            },
                            "life" => {
                                paddle.paddle_type = paddle::PaddleType::Normal;
                                game.lives += 1;
                            },
                            _ => {},
                        }
                        break;
                    }
                }

                for bullet in &mut bullets {
                    bullet.update(get_frame_time());
                    bullet.draw();

                    for enemy in &mut enemies {
                        if let Some(_i) = bullet.rect.intersect(enemy.rect) {
                            bullet.destroyed = true;
                            enemy.destroyed = true;
                            game.score += 30;
                            explode_animations.push(
                                ExplodeAnimation::new(enemy.x, enemy.y).await,
                            );
                            play_sound(resources.explode, PlaySoundParams {
                                looped: false,
                                volume: 0.3,
                            });
                            break;
                        }
                    }

                    for brick in &mut bricks {
                        if let Some(_i) = bullet.rect.intersect(brick.rect) {
                            if brick.power != 0 {
                                brick.power -= 1;
                            } else {
                                brick.destroyed = true;
                                if brick.brick_with_bonus {
                                    powers.push(
                                        Power::new(brick.x, brick.y).await,
                                    );
                                }
                            }
                            bullet.destroyed = true;
                            game.score += 10;
                        }
                    }
                }

                if bricks.len() == 0 {
                    game_state = GameState::LevelCompleted;
                }

                if balls.len() == 0 {
                    die_animations.push(
                        DieAnimation::new(paddle.x, paddle.y).await,
                    );
                    play_sound(resources.fail, PlaySoundParams {
                        looped: false,
                        volume: 0.3,
                    });
                    if game.lives > 0 {
                        game_state = GameState::LevelFailed;
                    } else {
                        game_state = GameState::GameOver;
                    }
                }

                draw_info(&game, &resources);
            },
            GameState::Pause => {
                set_cursor_grab(false);
                show_mouse(true);
                draw_map(&resources);
                draw_info(&game, &resources);
                door.draw();
                show_text(resources.font, "PAUSED", "Press 'space' to continue...");

                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Game;
                }
            },
            GameState::LevelCompleted => {
                set_cursor_grab(false);
                show_mouse(true);
                draw_map(&resources);
                draw_info(&game, &resources);
                door.draw();
                show_text(resources.font, "Level completed!", "Press 'space' to continue...");

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    if game.lvl_num == resources::NUMBER_OF_LEVELS {
                        game.lvl_num = 1;
                    } else {
                        game.lvl_num += 1;
                    }
                    game_state = GameState::InitLevel;
                }
            },
            GameState::LevelFailed => {
                set_cursor_grab(false);
                show_mouse(true);
                draw_map(&resources);
                draw_info(&game, &resources);
                door.draw();
                enemies.clear();
                powers.clear();
                bullets.clear();
                balls.clear();
                for brick in &mut bricks {
                    brick.draw();
                }
                for animation in &mut die_animations {
                    animation.draw();
                }
                show_text(resources.font, "Level failed", "Press 'space' to continue...");

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    paddle.x = screen_width() / 2.0 - paddle.width() / 2.0;
                    paddle.paddle_type = paddle::PaddleType::Normal;
                    game.lives -= 1;
                    balls.push(Ball::new(paddle.center_x(), paddle.y - 16.0).await);
                    balls[0].last_ball_time = get_time();
                    game.balls_speed = resources::INIT_BALLS_SPEED;
                    game_state = GameState::Game;
                    play_sound(resources.level_start, PlaySoundParams {
                        looped: false,
                        volume: 0.3,
                    });
                }
            },
            GameState::GameOver => {
                set_cursor_grab(false);
                show_mouse(true);
                draw_map(&resources);
                draw_info(&game, &resources);
                door.draw();
                enemies.clear();
                powers.clear();
                bullets.clear();
                balls.clear();
                for brick in &mut bricks {
                    brick.draw();
                }
                for animation in &mut die_animations {
                    animation.draw();
                }
                show_text(resources.font, "Game Over", "Press 'space' to continue...");
                if is_key_pressed(KeyCode::Space)  || is_mouse_button_pressed(MouseButton::Left) {
                    game.score = 0;
                    game.lvl_num = 1;
                    game.lives = 2;
                    game_state = GameState::InitLevel;
                }
            },
        };

        // GC
        match enemies.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                enemies.remove(idx);
            },
            None => {},
        };

        match bullets.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                bullets.remove(idx);
            },
            None => {},
        };

        match bricks.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                bricks.remove(idx);
            },
            None => {},
        };

        match die_animations.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                die_animations.remove(idx);
            },
            None => {},
        };

        match balls.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                balls.remove(idx);
            },
            None => {},
        };

        match explode_animations.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                explode_animations.remove(idx);
            },
            None => {},
        };

        match powers.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                powers.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}
