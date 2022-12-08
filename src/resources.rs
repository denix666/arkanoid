use macroquad::{prelude::*, audio::{load_sound, Sound}};

pub const NUMBER_OF_LEVELS: i32 = 4;
pub const RES_WIDTH: i32 = 700;
pub const RES_HEIGHT: i32 = 550;
pub const FRAME_INDENT: f32 = 50.0;
pub const NUMBER_OF_BONUSES: i32 = 9;

pub struct Resources {
    //pub intro_texture: Texture2D,
    pub frame_right: Texture2D,
    pub frame_left: Texture2D,
    pub font: Font,
    pub brick_hit: Sound,
    pub paddle_hit: Sound,
    pub explode: Sound,
    pub bonus: Sound,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            //intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            frame_right: load_texture("assets/images/frame_right.png").await.unwrap(),
            frame_left: load_texture("assets/images/frame_left.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/iomanoid.ttf").await.unwrap(),
            brick_hit: load_sound("assets/sounds/brick_hit.ogg").await.unwrap(),
            paddle_hit: load_sound("assets/sounds/paddle_hit.ogg").await.unwrap(),
            explode: load_sound("assets/sounds/explode.ogg").await.unwrap(),
            bonus: load_sound("assets/sounds/bonus.ogg").await.unwrap(),
        }
    }
}