use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 4;

pub enum Status {
    LeftClose,
    LeftOpen,
    RightClose,
    RightOpen,
    Idle,
    RightWait,
    LeftWait,
}

pub struct Door {
    right_open_animation_texture: Vec<Texture2D>,
    left_open_animation_texture: Vec<Texture2D>,
    right_close_animation_texture: Vec<Texture2D>,
    left_close_animation_texture: Vec<Texture2D>,
    frame_idle_texture: Texture2D,
    x: f32,
    y: f32,
    cur_frame: usize,
    update_interval: i32,
    pub status: Status,
    pub last_door_time: f64,
}

impl Door {
    pub async fn new() -> Self {
        let mut right_open_animation_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=9 { // Number of sprites in animation
            let path = format!("assets/images/doors/right_{}.png", i);
            right_open_animation_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut left_open_animation_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=9 { // Number of sprites in animation
            let path = format!("assets/images/doors/left_{}.png", i);
            left_open_animation_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut right_close_animation_sprites:Vec<Texture2D> = Vec::new();
        let mut j = 10;
        for _ in 1..=9 { // Number of sprites in animation
            j -= 1;
            let path = format!("assets/images/doors/right_{}.png", j);
            right_close_animation_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut left_close_animation_sprites:Vec<Texture2D> = Vec::new();
        let mut j = 10;
        for _ in 1..=9 { // Number of sprites in animation
            j -= 1;
            let path = format!("assets/images/doors/left_{}.png", j);
            left_close_animation_sprites.push(load_texture(&path).await.unwrap());
        }
        Self {
            x: 52.0,
            y: 0.0,
            cur_frame: 0,
            right_open_animation_texture: right_open_animation_sprites,
            left_open_animation_texture: left_open_animation_sprites,
            status: Status::Idle,
            update_interval: 0,
            frame_idle_texture: load_texture("assets/images/frame_top.png").await.unwrap(),
            right_close_animation_texture: right_close_animation_sprites,
            left_close_animation_texture: left_close_animation_sprites,
            last_door_time: 0.0,
        }
    }

    pub fn left_open_animation(&mut self) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.left_open_animation_texture.len() {
                self.frame_idle_texture = self.left_open_animation_texture[self.left_open_animation_texture.len()-1];
                self.status = Status::LeftWait;
                self.cur_frame = 0;
            }
        }
    }

    pub fn left_close_animation(&mut self) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.left_close_animation_texture.len() {
                self.frame_idle_texture = self.left_close_animation_texture[self.left_close_animation_texture.len()-1];
                self.status = Status::Idle;
                self.cur_frame = 0;
            }
        }
    }

    pub fn right_close_animation(&mut self) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.right_close_animation_texture.len() {
                self.frame_idle_texture = self.right_close_animation_texture[self.right_close_animation_texture.len()-1];
                self.status = Status::Idle;
                self.cur_frame = 0;
            }
        }
    }

    pub fn right_open_animation(&mut self) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.right_open_animation_texture.len() {
                self.frame_idle_texture = self.right_open_animation_texture[self.right_open_animation_texture.len()-1];
                self.status = Status::RightWait;
                self.cur_frame = 0;
            }
        }
    }

    pub fn draw(&mut self) {
        match self.status {
            Status::LeftClose => {
                draw_texture(self.left_close_animation_texture[self.cur_frame], self.x, self.y, WHITE);
                self.left_close_animation();
            },
            Status::LeftOpen => {
                draw_texture(self.left_open_animation_texture[self.cur_frame], self.x, self.y, WHITE);
                self.left_open_animation();
            },
            Status::RightClose => {
                draw_texture(self.right_close_animation_texture[self.cur_frame], self.x, self.y, WHITE);
                self.right_close_animation();
            },
            Status::RightOpen => {
                draw_texture(self.right_open_animation_texture[self.cur_frame], self.x, self.y, WHITE);
                self.right_open_animation();
            },
            _ => {
                draw_texture(self.frame_idle_texture, self.x, self.y, WHITE);
            },
        }
    }
}
