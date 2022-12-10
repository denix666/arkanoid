use macroquad::{prelude::*, audio::{load_sound, Sound}};

pub const NUMBER_OF_LEVELS: i32 = 11;
pub const RES_WIDTH: i32 = 700;
pub const RES_HEIGHT: i32 = 550;
pub const FRAME_INDENT: f32 = 50.0;
pub const NUMBER_OF_BONUSES: i32 = 9;
pub const INIT_BALLS_SPEED: f32 = 100.0;

pub struct Resources {
    pub intro_texture: Texture2D,
    pub frame_right: Texture2D,
    pub frame_left: Texture2D,
    pub font: Font,
    pub brick_hit: Sound,
    pub paddle_hit: Sound,
    pub explode: Sound,
    pub bonus: Sound,
    pub fail: Sound,
    pub laser: Sound,
    pub intro_music: Sound,
    pub level_start: Sound,
    pub game_over: Sound,
    pub freeze: Sound,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            frame_right: load_texture("assets/images/frame_right.png").await.unwrap(),
            frame_left: load_texture("assets/images/frame_left.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/iomanoid.ttf").await.unwrap(),
            brick_hit: load_sound("assets/sounds/brick_hit.ogg").await.unwrap(),
            paddle_hit: load_sound("assets/sounds/paddle_hit.ogg").await.unwrap(),
            explode: load_sound("assets/sounds/explode.ogg").await.unwrap(),
            bonus: load_sound("assets/sounds/bonus.ogg").await.unwrap(),
            fail: load_sound("assets/sounds/fail.ogg").await.unwrap(),
            laser: load_sound("assets/sounds/laser.ogg").await.unwrap(),
            intro_music: load_sound("assets/sounds/intro.ogg").await.unwrap(),
            level_start: load_sound("assets/sounds/level_start.ogg").await.unwrap(),
            game_over: load_sound("assets/sounds/game_over.ogg").await.unwrap(),
            freeze: load_sound("assets/sounds/freeze.ogg").await.unwrap(),
        }
    }
}