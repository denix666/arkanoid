use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 700.0;
const NUM_OF_FRAMES: usize = 8;
const ANIMATION_SPEED: i32 = 4;
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
    paddle_normal: Texture2D,
    paddle_catch: Texture2D,
    paddle_laser: Texture2D,
    paddle_expanded: Texture2D,
    update_interval: i32,
    cur_frame: usize,
    pub kind: Kind,
    pub rect: Rect,
    anim_texture: Vec<Texture2D>,
    pub status: Status,
    pub animation_completed: bool,
}

impl Paddle {
    pub async fn new() -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=NUM_OF_FRAMES {
            let path = format!("assets/paddle/paddle_explode_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x: screen_width()/2.0, 
            y: screen_height() - 100.0, 
            paddle_normal: load_texture("assets/images/paddle.png").await.unwrap(),
            paddle_catch: load_texture("assets/images/paddle_catch.png").await.unwrap(),
            paddle_expanded: load_texture("assets/images/paddle_expanded.png").await.unwrap(),
            paddle_laser: load_texture("assets/images/paddle_laser.png").await.unwrap(),
            update_interval: 0,
            cur_frame: 0,
            kind: Kind::Normal,
            rect: Rect::new(screen_width()/2.0, screen_height() - 100.0, 76.0, 35.0),
            anim_texture: sprites,
            status: Status::Playing,
            animation_completed: false,
        }
    }

    pub fn center_x(&mut self) -> f32 {
        self.x + self.paddle_normal.width() / 2.0
    }

    pub fn width(&mut self) -> f32 {
        match self.kind {
            Kind::Normal => {
                self.paddle_normal.width()
            },
            Kind::Catch => {
                self.paddle_catch.width()
            },
            Kind::Expand => {
                self.paddle_expanded.width()
            },
            Kind::Laser => {
                self.paddle_laser.width()
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
                if self.x > screen_width() - 16.0 - self.paddle_normal.width() {
                    self.x = screen_width() - 16.0 - self.paddle_normal.width();
                    self.rect.w = self.paddle_normal.width();
                }
            },
            Kind::Catch => {
                if self.x > screen_width() - 16.0 - self.paddle_catch.width() {
                    self.x = screen_width() - 16.0 - self.paddle_catch.width();
                    self.rect.w = self.paddle_catch.width();
                }
            },
            Kind::Expand => {
                if self.x > screen_width() - 16.0 - self.paddle_expanded.width() {
                    self.x = screen_width() - 16.0 - self.paddle_expanded.width();
                    self.rect.w = self.paddle_expanded.width();
                }
            },
            Kind::Laser => {
                if self.x > screen_width() - 16.0 - self.paddle_laser.width() {
                    self.x = screen_width() - 16.0 - self.paddle_laser.width();
                    self.rect.w = self.paddle_laser.width();
                }
            },
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
        self.rect.h = 35.0;
    }

    pub fn show_die_animation(&mut self) {
        if !self.animation_completed {
            draw_texture(self.anim_texture[self.cur_frame], self.x, self.y, WHITE);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame += 1;
                if self.cur_frame == self.anim_texture.len() {
                    self.cur_frame = 0;
                    self.animation_completed = true;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        match self.status {
            Status::Playing => {
                match self.kind {
                    Kind::Normal => {
                        draw_texture(self.paddle_normal, self.x, self.y, WHITE);
                    },
                    Kind::Catch => {
                        draw_texture(self.paddle_catch, self.x, self.y, WHITE);
                    },
                    Kind::Expand => {
                        draw_texture(self.paddle_expanded, self.x, self.y, WHITE);
                    },
                    Kind::Laser => {
                        draw_texture(self.paddle_laser, self.x, self.y, WHITE);
                    },
                }
            },
            Status::Died => {
                self.show_die_animation();
            },
        }
    }
}
