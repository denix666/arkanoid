use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 4;

pub struct DieAnimation {
    pub x: f32,
    pub y: f32, 
    update_interval: i32,
    cur_frame: usize,
    textures: Vec<Texture2D>,
    pub destroyed: bool,
}

impl DieAnimation {
    pub async fn new(x: f32, y: f32)  -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=8 {
            let path = format!("assets/images/paddle/explode/paddle_explode_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x,
            y,
            update_interval: 0,
            cur_frame: 0,
            textures: sprites,
            destroyed: false,
        }
    }

    pub fn draw(&mut self) {
        let mut diff_in_size_width = 0.0;
        let mut diff_in_size_height = 0.0;

        if !self.destroyed {
            if self.cur_frame > 1 {
                diff_in_size_width = self.textures[self.cur_frame].width() - self.textures[self.cur_frame-1].width();
                diff_in_size_height = self.textures[self.cur_frame].height() - self.textures[self.cur_frame-1].height();
            }
            draw_texture(self.textures[self.cur_frame], self.x - diff_in_size_width, self.y - diff_in_size_height, WHITE);
            self.update_interval += 1;
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame += 1;
                if self.cur_frame == self.textures.len() {
                    self.destroyed = true;
                }
            }
        }
    }
}
