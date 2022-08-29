use macroquad::prelude::*;

pub struct Resources {
    pub intro_texture: Texture2D,
    pub frame_texture: Texture2D,
    pub font: Font,
    pub icon: Texture2D,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            frame_texture: load_texture("assets/images/frame.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/iomanoid.ttf").await.unwrap(),
            icon: load_texture("assets/images/icon.png").await.unwrap(),
        }
    }
}
