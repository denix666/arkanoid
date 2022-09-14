mod paddle;
use paddle::Paddle;

mod levels;
use levels::*;

mod game;
use game::Game;

mod labels;
use labels::Label;

mod bullet;
use bullet::Bullet;

mod brick;
use brick::Brick;

mod doors;
use doors::Door;

mod powers;
use powers::Power;

mod enemy;
use enemy::Enemy;

mod ball;
use ball::Ball;

mod resources;
use resources::Resources;

mod functions;
use functions::*;

use macroquad::{prelude::*, audio::{play_sound, PlaySoundParams, stop_sound}};

extern crate rand;
use rand::{Rng};

const FRAME_INDENT:f32 = 28.0;
const NUMBER_OF_BONUSES:i32 = 9;
const INIT_BALL_SPEED:f32 = 120.0;

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
    let mut ball = Ball::new(paddle.center_x(), paddle.y-16.0).await;
    let mut bricks: Vec<Brick> = Vec::new();
    let mut powers: Vec<Power> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let level_label = Label::new("Level - ".to_string(), 40.0, 787.0, 50).await;
    let lives_label = Label::new("Lives - ".to_string(), 240.0, 787.0, 50).await;
    let score_label = Label::new("Score - ".to_string(), 440.0, 787.0, 50).await;
    let mut door = Door::new().await;
    
    let resources = Resources::new().await;

    let mut lvl;

    play_sound(resources.intro_music, PlaySoundParams {
        looped: true,
        volume: 3.0,
    });

    loop {
        clear_background(BLACK);
        
        match game_state {
            GameState::Intro => {
                draw_texture(resources.intro_texture, 0.0, 0.0, WHITE);
                
                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game.lives = 2;
                    game_state = GameState::InitLevel;
                    stop_sound(resources.intro_music);
                }
            }

            GameState::GameCompleted => {
                level.draw();
                show_text(resources.font, "Game Completed!", "Press 'space' to restart game...");

                if is_key_pressed(KeyCode::Space) {
                    level.lvl_num = 1;
                    game_state = GameState::Intro;
                }
                powers.clear();
                bullets.clear();
                enemies.clear();
            }

            GameState::LevelCompleted => {
                level.draw();
                show_text(resources.font, "Level Completed!", "Press 'space' to continue...");
                if is_key_pressed(KeyCode::Space) {
                    level.lvl_num = level.lvl_num + 1;
                    level.set_level(level.lvl_num).await;
                    ball.vertical_dir = ball::VerticalDir::Up;
                    paddle.x = screen_width()/2.0;
                    ball.x = paddle.center_x();
                    ball.y = paddle.y-16.0;
                    paddle.kind = paddle::Kind::Normal;
                    game_state = GameState::InitLevel;
                }
                powers.clear();
                bullets.clear();
                enemies.clear();
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
                paddle.kind = paddle::Kind::Normal;
                level.bricks_amount = 0;
                bricks.clear();
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

                powers.clear();
                bullets.clear();
                enemies.clear();
                game.time_in_seconds = get_time();
                game.enemy_amount_now = 0;
                ball.speed = INIT_BALL_SPEED;
                game.last_enemy_burn_time = get_time();

                // Set random bonuses
                for _ in 0..=NUMBER_OF_BONUSES {
                    let index = rand::random::<usize>() % bricks.len();
                    bricks[index].brick_with_bonus = true;
                }

                play_sound(resources.level_start, PlaySoundParams {
                    looped: false,
                    volume: 3.0,
                });
                ball.last_ball_time = get_time();
                ball.released = false;
                game_state = GameState::Game;
                level.destroyed_bricks_amount = 0;
                ball.x = paddle.center_x();
                ball.y = paddle.y-16.0;
            }

            GameState::LevelFail => {
                level.draw();
                paddle.status = paddle::Status::Died;
                paddle.draw();
                show_text(resources.font, "Level fail", "Press 'space' to continue...");
                
                if is_key_pressed(KeyCode::Space) {
                    if game.lives > 0 {
                        game.lives -= 1;
                        game_state = GameState::Game;
                        game.enemy_amount_now = 0;
                        ball.vertical_dir = ball::VerticalDir::Up;
                        paddle.x = screen_width()/2.0;
                        ball.x = paddle.center_x();
                        ball.y = paddle.y-16.0;
                        paddle.kind = paddle::Kind::Normal;
                        paddle.status = paddle::Status::Playing;
                        paddle.explode_animation_completed = false;
                        ball.last_ball_time = get_time();
                        ball.speed = INIT_BALL_SPEED;
                        ball.released = false;
                        powers.clear();
                        bullets.clear();
                        enemies.clear();
                        game.time_in_seconds = get_time();
                    } else {
                        game_state = GameState::GameOver;
                        play_sound(resources.game_over, PlaySoundParams {
                            looped: false,
                            volume: 3.0,
                        });
                    }
                }

                level_label.draw_label(level.number());
                lives_label.draw_label(game.lives);
                score_label.draw_label(game.score);
            }

            GameState::GameOver => {
                level.draw();
                show_text(resources.font, "Game over", "Press 'space' to start new game...");

                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game.lives = 2;
                    level.lvl_num = 1;
                    game_state = GameState::InitLevel;
                    bricks.clear();
                    ball.vertical_dir = ball::VerticalDir::Up;
                    paddle.x = screen_width()/2.0;
                    paddle.kind = paddle::Kind::Normal;
                    paddle.status = paddle::Status::Playing;
                    paddle.explode_animation_completed = false;
                    ball.x = paddle.center_x();
                    ball.y = paddle.y-16.0;
                }
            }

            GameState::Pause => {
                level.draw();
                show_text(resources.font, "PAUSED", "Press 'space' to continue...");

                if is_key_pressed(KeyCode::Space) | is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Game;
                }
            }

            GameState::Game => {
                paddle.update(get_frame_time());

                if get_time() - game.time_in_seconds >= 10.0 {
                    game.time_in_seconds = get_time();
                    ball.speed += 20.0;
                }
                
                if get_time() - ball.last_ball_time >= ball.idle_time || is_key_pressed(KeyCode::Space) {
                    ball.released = true;
                }

                if get_time() - door.last_door_time >= 2.0 {
                    match door.status {
                        doors::Status::RightWait=>{door.status=doors::Status::RightClose;},
                        doors::Status::LeftWait=>{door.status=doors::Status::LeftClose;}
                        _ => {}, 
                    };
                }

                if ball.released {
                    ball.update(get_frame_time());
                } else {
                    ball.x = paddle.center_x();
                }

                level.draw();
                draw_texture(resources.frame_left, 0.0, 0.0, WHITE);
                draw_texture(resources.frame_right, 678.0, 0.0, WHITE);
                paddle.draw();
                ball.draw();
                door.draw();
                level_label.draw_label(level.number());
                lives_label.draw_label(game.lives);
                score_label.draw_label(game.score);

                for brick in &mut bricks {
                    brick.draw();

                    if !brick.destroyed {
                        if let Some(_i) = ball.rect.intersect(brick.left_side) {
                            brick.destroyed = true;
                            ball.horizontal_dir = ball::HorizontalDir::Left;
                        } else 
                        if let Some(_i) = ball.rect.intersect(brick.right_side) {
                            brick.destroyed = true;
                            ball.horizontal_dir = ball::HorizontalDir::Right;
                        } else
                        if let Some(_i) = ball.rect.intersect(brick.up_side) {
                            brick.destroyed = true;
                            ball.vertical_dir = ball::VerticalDir::Up;
                        } else
                        if let Some(_i) = ball.rect.intersect(brick.down_side) {
                            brick.destroyed = true;
                            ball.vertical_dir = ball::VerticalDir::Down;
                        }

                        if brick.destroyed {
                            level.bricks_amount -= 1;
                            level.destroyed_bricks_amount += 1;
                            game.score += 1;
                            play_sound(resources.destroyed_block, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                            if brick.brick_with_bonus {
                                powers.push(
                                    Power::new(brick.x, brick.y).await,
                                );
                            }
                        }
                    }
                }

                for power in &mut powers {
                    power.draw();

                    if power.y > resources.frame_left.height() - FRAME_INDENT {
                        power.actual = false;
                    }

                    if power.actual {
                        if let Some(_i) = paddle.rect.intersect(power.rect) {
                            power.actual = false;

                            if power.kind == "laser" {
                                paddle.kind = paddle::Kind::Laser;
                                game.score += 5;
                            } else
                            if power.kind == "catch" {
                                paddle.kind = paddle::Kind::Catch;
                                game.score += 5;
                            } else
                            if power.kind == "expand" {
                                paddle.kind = paddle::Kind::Expand;
                                game.score += 5;
                            } else
                            if power.kind == "life" {
                                paddle.kind = paddle::Kind::Normal;
                                game.score += 5;
                                game.lives += 1;
                            }
                            else
                            if power.kind == "slow" {
                                game.score += 5;
                                if ball.speed > 60.0 {
                                    ball.speed -= 20.0;
                                }
                            }

                            play_sound(resources.bonus, PlaySoundParams {
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

                for bullet in &mut bullets {
                    bullet.draw();

                    if bullet.y < 0.0 + FRAME_INDENT {
                        bullet.actual = false;
                    }

                    if bullet.actual {
                        for brick in &mut bricks {
                            if !brick.destroyed {
                                if let Some(_i) = bullet.rect.intersect(brick.down_side) {
                                    bullet.actual = false;
                                    brick.destroyed = true;
                                    level.bricks_amount = level.bricks_amount - 1;
                                    if brick.brick_with_bonus {
                                        powers.push(
                                            Power::new(brick.x, brick.y).await,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Pause;
                }

                if is_key_pressed(KeyCode::Space) {
                    match paddle.kind {
                        paddle::Kind::Laser => {
                            bullets.push(
                                Bullet::new(paddle.x+5.0, paddle.y).await,
                            );
                            bullets.push(
                                Bullet::new(paddle.x+paddle.width()-11.0, paddle.y).await,
                            );
                        },
                        _ => {},
                    }
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
                    paddle.kind = paddle::Kind::Expand;
                }
                if is_key_pressed(KeyCode::Q) {
                    bricks.clear();
                    powers.clear();
                    bullets.clear();
                    level.lvl_num = level.lvl_num + 1;
                    level.set_level(level.lvl_num).await;
                    game_state = GameState::InitLevel;
                }
                /////////////////////////////////////////////////////

                // Enemy
                if game.enemy_amount_now < 6 && level.destroyed_bricks_amount > 15 {
                    if get_time() - game.last_enemy_burn_time >= 5.0 {
                        door.last_door_time = get_time();
                        game.last_enemy_burn_time = get_time();
                        match rand::thread_rng().gen_range(0..=1) { 
                            0 => {
                                door.status = doors::Status::LeftOpen;
                                enemies.push(
                                    Enemy::new(180.0, -20.0).await,
                                );
                            },
                            _ => {
                                door.status = doors::Status::RightOpen;
                                enemies.push(
                                    Enemy::new(520.0, -20.0).await,
                                );
                            },
                        };
                    }
                }
                for enemy in &mut enemies {
                    if get_time() - enemy.burn_time >= 1.0 {
                        if enemy.y < paddle.y {
                            enemy.y += 1.0;
                        }
                        if enemy.x < paddle.x {
                            enemy.x += 1.0;
                        } else 
                        if enemy.x > paddle.x {
                            enemy.x -= 1.0;
                        }
                        if let Some(_i) = enemy.rect.intersect(paddle.rect) {
                            enemy.destroyed = true;
                        }
                        if let Some(_i) = enemy.rect.intersect(ball.rect) {
                            enemy.destroyed = true;
                        }
                        enemy.draw();
                    }
                }

                // Ball
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
                        if ball.center_x() < paddle.x + paddle.width() && ball.center_x() > paddle.x && ball.y > paddle.y-16.0 {
                            ball.vertical_dir = ball::VerticalDir::Up;
                            // Left part of paddle
                            if ball.center_x() >= paddle.x && ball.center_x() <= paddle.x + paddle.width() / 3.0 {
                                ball.ball_step_move_x = 2.0;
                                ball.horizontal_dir = ball::HorizontalDir::Left;
                            }
                            // Right part of paddle
                            if ball.center_x() >= paddle.x + paddle.width() / 3.0 * 2.0 && ball.center_x() <= paddle.x + paddle.width() {
                                ball.ball_step_move_x = 2.0;
                                ball.horizontal_dir = ball::HorizontalDir::Right;
                            }
                            // Center part of paddle
                            if ball.center_x() >= paddle.x + paddle.width() / 3.0 && ball.center_x() <= paddle.x + paddle.width() - paddle.width() / 3.0 {
                                ball.ball_step_move_x = 1.0;
                            }
                            play_sound(resources.paddle_hit, PlaySoundParams {
                                looped: false,
                                volume: 3.0,
                            });
                            match paddle.kind {
                                paddle::Kind::Catch => {
                                    ball.last_ball_time=get_time();
                                    ball.released=false;
                                    ball.y = paddle.y-16.0;
                                }
                                _ => {},
                            }
                        }

                        if ball.y + ball.ball_height() > resources.frame_left.height() {
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
