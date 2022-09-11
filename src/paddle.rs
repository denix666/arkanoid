use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 700.0;
const ANIMATION_SPEED: i32 = 4;
const PULSATION_SPEED: i32 = 6;
pub enum Kind {
    Normal,
    Catch,
    Expand,
    Laser,
}

pub enum Status {
    Playing,
    Died,
}

pub struct Paddle {
    pub x: f32,
    pub y: f32, 
    paddle_catch: Texture2D,
    update_interval: i32,
    cur_frame: usize,
    pub kind: Kind,
    pub rect: Rect,
    explode_animation_texture: Vec<Texture2D>,
    paddle_wide: Vec<Texture2D>,
    paddle_wide_cur_frame: usize,
    paddle_wide_update_interval: i32,
    paddle_laser: Vec<Texture2D>,
    paddle_laser_cur_frame: usize,
    paddle_laser_update_interval: i32,
    paddle_normal: Vec<Texture2D>,
    paddle_normal_cur_frame: usize,
    paddle_normal_update_interval: i32,
    pub status: Status,
    pub explode_animation_completed: bool,
}

impl Paddle {
    pub async fn new() -> Self {
        let mut explode_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=8 { // Number of sprites in animation
            let path = format!("assets/paddle/explode/paddle_explode_{}.png", i);
            explode_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut paddle_wide_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=4 { // Number of sprites in animation
            let path = format!("assets/paddle/wide/paddle_wide_{}.png", i);
            paddle_wide_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut paddle_laser_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=4 { // Number of sprites in animation
            let path = format!("assets/paddle/laser/paddle_laser_{}.png", i);
            paddle_laser_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut paddle_normal_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=4 { // Number of sprites in animation
            let path = format!("assets/paddle/normal/paddle_{}.png", i);
            paddle_normal_sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x: screen_width()/2.0, 
            y: screen_height() - 90.0, 
            paddle_catch: load_texture("assets/paddle/paddle_catch.png").await.unwrap(),
            update_interval: 0,
            paddle_wide: paddle_wide_sprites,
            paddle_wide_update_interval: 0,
            paddle_wide_cur_frame: 0,
            paddle_laser: paddle_laser_sprites,
            paddle_laser_update_interval: 0,
            paddle_laser_cur_frame: 0,
            paddle_normal: paddle_normal_sprites,
            paddle_normal_update_interval: 0,
            paddle_normal_cur_frame: 0,
            cur_frame: 0,
            kind: Kind::Normal,
            rect: Rect::new(0.0, 0.0, 0.0,0.0),
            explode_animation_texture: explode_sprites,
            status: Status::Playing,
            explode_animation_completed: false,
        }
    }

    pub fn center_x(&mut self) -> f32 {
        match self.kind {
            Kind::Normal => {
                self.x + self.paddle_normal[0].width() / 2.0
            },
            Kind::Catch => {
                self.x + self.paddle_catch.width() / 2.0
            },
            Kind::Expand => {
                self.x + self.paddle_wide[0].width() / 2.0
            },
            Kind::Laser => {
                self.x + self.paddle_laser[0].width() / 2.0
            },
        }
    }

    pub fn width(&mut self) -> f32 {
        match self.kind {
            Kind::Normal => {
                self.paddle_normal[0].width()
            },
            Kind::Catch => {
                self.paddle_catch.width()
            },
            Kind::Expand => {
                self.paddle_wide[0].width()
            },
            Kind::Laser => {
                self.paddle_laser[0].width()
            },
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move = 0.0;

        if is_key_down(KeyCode::Left) {
            x_move -= 1.0;
        }

        if is_key_down(KeyCode::Right) {
            x_move += 1.0;
        }

        self.x += x_move * dt * PLAYER_SPEED;

        if self.x < 16.0 {
            self.x = 16.0;
        }

        match self.kind {
            Kind::Normal => {
                if self.x > screen_width() - 16.0 - self.paddle_normal[0].width() {
                    self.x = screen_width() - 16.0 - self.paddle_normal[0].width();
                }
                self.rect.w = self.paddle_normal[0].width();
                self.rect.h = self.paddle_normal[0].height();
            },
            Kind::Catch => {
                if self.x > screen_width() - 16.0 - self.paddle_catch.width() {
                    self.x = screen_width() - 16.0 - self.paddle_catch.width();
                }
                self.rect.w = self.paddle_catch.width();
                self.rect.h = self.paddle_catch.height();
            },
            Kind::Expand => {
                if self.x > screen_width() - 16.0 - self.paddle_wide[0].width() {
                    self.x = screen_width() - 16.0 - self.paddle_wide[0].width();
                }
                self.rect.w = self.paddle_wide[0].width();
                self.rect.h = self.paddle_wide[0].height();
            },
            Kind::Laser => {
                if self.x > screen_width() - 16.0 - self.paddle_laser[0].width() {
                    self.x = screen_width() - 16.0 - self.paddle_laser[0].width();
                }
                self.rect.w = self.paddle_laser[0].width();
                self.rect.h = self.paddle_laser[0].height();
            },
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn paddle_wide_draw(&mut self) {
        draw_texture(self.paddle_wide[self.paddle_wide_cur_frame], self.x, self.y, WHITE);
        self.paddle_wide_update_interval += 1;
        if self.paddle_wide_update_interval > PULSATION_SPEED { // Animation speed for pulsation
            self.paddle_wide_update_interval = 0;
            self.paddle_wide_cur_frame += 1;
            if self.paddle_wide_cur_frame == self.paddle_wide.len() {
                self.paddle_wide_cur_frame = 0;
            }
        }
    }

    pub fn paddle_normal_draw(&mut self) {
        draw_texture(self.paddle_normal[self.paddle_normal_cur_frame], self.x, self.y, WHITE);
        self.paddle_normal_update_interval += 1;
        if self.paddle_normal_update_interval > PULSATION_SPEED { // Animation speed for pulsation
            self.paddle_normal_update_interval = 0;
            self.paddle_normal_cur_frame += 1;
            if self.paddle_normal_cur_frame == self.paddle_normal.len() {
                self.paddle_normal_cur_frame = 0;
            }
        }
    }

    pub fn paddle_laser_draw(&mut self) {
        draw_texture(self.paddle_laser[self.paddle_laser_cur_frame], self.x, self.y, WHITE);
        self.paddle_laser_update_interval += 1;
        if self.paddle_laser_update_interval > PULSATION_SPEED { // Animation speed for pulsation
            self.paddle_laser_update_interval = 0;
            self.paddle_laser_cur_frame += 1;
            if self.paddle_laser_cur_frame == self.paddle_laser.len() {
                self.paddle_laser_cur_frame = 0;
            }
        }
    }

    pub fn show_explode_animation(&mut self) {
        let mut diff_in_size_width = 0.0;
        let mut diff_in_size_height = 0.0;
        if !self.explode_animation_completed {
            // center position of animation because of diffrent sprites sizes
            if self.cur_frame > 1 {
                diff_in_size_width = self.explode_animation_texture[self.cur_frame].width() - self.explode_animation_texture[self.cur_frame-1].width();
                diff_in_size_height = self.explode_animation_texture[self.cur_frame].height() - self.explode_animation_texture[self.cur_frame-1].height();
            }
            draw_texture(self.explode_animation_texture[self.cur_frame], self.x - diff_in_size_width, self.y - diff_in_size_height, WHITE);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED { // Animation speed
                self.update_interval = 0;
                self.cur_frame += 1;
                if self.cur_frame == self.explode_animation_texture.len() {
                    self.cur_frame = 0;
                    self.explode_animation_completed = true;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        match self.status {
            Status::Playing => {
                match self.kind {
                    Kind::Normal => {
                        self.paddle_normal_draw();
                    },
                    Kind::Catch => {
                        draw_texture(self.paddle_catch, self.x, self.y, WHITE);
                    },
                    Kind::Expand => {
                        self.paddle_wide_draw();
                    },
                    Kind::Laser => {
                        self.paddle_laser_draw();
                    },
                }
            },
            Status::Died => {
                self.show_explode_animation();
            },
        }
    }
}
