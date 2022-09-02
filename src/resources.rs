use macroquad::{prelude::*, audio::{Sound, load_sound}};

pub struct Resources {
    pub intro_texture: Texture2D,
    pub frame_texture: Texture2D,
    pub font: Font,
    pub icon: Texture2D,
    pub ball_hit: Sound,
    pub paddle_hit: Sound,
    pub ball_lost: Sound,
    pub destroyed_block: Sound,
    pub level_start: Sound,
    pub intro_music: Sound,
    pub game_over: Sound,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            frame_texture: load_texture("assets/images/frame.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/iomanoid.ttf").await.unwrap(),
            icon: load_texture("assets/images/icon.png").await.unwrap(),
            ball_hit: load_sound("assets/sounds/ball_hit.ogg").await.unwrap(),
            paddle_hit: load_sound("assets/sounds/paddle_hit.ogg").await.unwrap(),
            ball_lost: load_sound("assets/sounds/ball_lost.ogg").await.unwrap(),
            destroyed_block: load_sound("assets/sounds/destroyed_block.ogg").await.unwrap(),
            level_start: load_sound("assets/sounds/level_start.ogg").await.unwrap(),
            intro_music: load_sound("assets/sounds/intro.ogg").await.unwrap(),
            game_over: load_sound("assets/sounds/game_over.ogg").await.unwrap(),
        }
    }
}
