use macroquad::prelude::*;

pub struct Level {
    texture: Texture2D,
    pub lvl_num: i32,
}

impl Level {
    pub async fn new() -> Self {
        Self {
            texture: load_texture("assets/backgrounds/bg_1.png").await.unwrap(),
            lvl_num: 1,
        }
    }

    pub async fn increase_level(&mut self) {
        if self.lvl_num < 6 {
            self.lvl_num += 1;
        }
        
        let mut bg_path = String::from("assets/backgrounds/bg_");  
        let lvl_str = String::from(self.lvl_num.to_string());
        bg_path.push_str(&lvl_str);
        bg_path.push_str(".png");

        self.texture = load_texture(&bg_path).await.unwrap();
    }

    pub fn number(&mut self) -> i32 {
        return self.lvl_num
    }

    pub fn draw(&self) {
        draw_texture(self.texture, 1.0, 1.0, WHITE);
    }
}
