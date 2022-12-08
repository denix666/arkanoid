use macroquad::prelude::*;
use crate::{points::Point, bricks::Brick, resources::Resources};

pub fn draw_map(resources: &Resources) {
    draw_rectangle(0.0, 0.0, crate::resources::RES_WIDTH as f32, crate::resources::RES_HEIGHT as f32, DARKBROWN);
    draw_texture(resources.frame_left, 30.0, 0.0, WHITE);
    draw_texture(resources.frame_right, 648.0, 0.0, WHITE);
    
    draw_line(0.0, screen_height() - 50.0, screen_width() - 0.0, screen_height() - 50.0, 2.0, ORANGE);
}

pub fn draw_score(font: Font, score: &str) {
    draw_text_ex("SCORE: ", 440.0, 537.0, 
        TextParams {
            font,
            font_size: 50,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(score, 590.0, 537.0, 
        TextParams {
            font,
            font_size: 50,
            color: ORANGE,
            ..Default::default()
        },
    );
}

pub fn draw_level_number(font: Font, lvl_num: &str) {
    draw_text_ex("LEVEL: ", 40.0, 537.0, 
        TextParams {
            font,
            font_size: 50,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(lvl_num, 180.0, 537.0, 
        TextParams {
            font,
            font_size: 50,
            color: ORANGE,
            ..Default::default()
        },
    );
}

pub fn draw_lives(font: Font, lives: &str) {
    draw_text_ex("LIVES: ", 250.0, 537.0, 
        TextParams {
            font,
            font_size: 50,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(lives, 380.0, 537.0, 
        TextParams {
            font,
            font_size: 50,
            color: ORANGE,
            ..Default::default()
        },
    );
}

pub fn show_text(font: Font, header_text: &str, message_text: &str) {
    let header_dims = measure_text(header_text, Some(font), 70, 1.0);
    let message_dims = measure_text(message_text, Some(font), 30, 1.0);

    draw_text_ex(
        &header_text,
        screen_width() * 0.5 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 70,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        screen_width() * 0.5 - message_dims.width * 0.5,
        360.0,
        TextParams {
            font,
            font_size: 30,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub async fn load_bricks(points: &Vec<Point>) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();
    for point in points {
        match point.value.as_str() {
            "a" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "a").await,
                );
            },
            "b" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "b").await,
                );
            },
            "c" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "c").await,
                );
            },
            "f" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "f").await,
                );
            },
            "g" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "g").await,
                );
            },
            "m" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "m").await,
                );
            },
            "p" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "p").await,
                );
            },
            "r" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "r").await,
                );
            },
            "w" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "w").await,
                );
            },
            "y" => {
                bricks.push(
                    Brick::new(point.x as f32 * 50.0, point.y as f32 * 20.0, "y").await,
                );
            },
            _ => {},
        };
    }
    return bricks
}

pub fn make_map_array(lvl_num: i32) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    let map = match lvl_num {
        1 => vec![
            "              ",
            "              ",
            "              ",
            "              ",
            " mmmmmmmmmmmm ",
            " rrrrrrrrrrrr ",
            " yyyyyyyyyyyy ",
            " bbbbbbbbbbbb ",
            " gggggggggggg ",
            "              ",
            "              ",
            "              ",
            "              ",
            "              ",
            "              ",
            "              ",
            "              ",
            "              ",
            "              ",
        ],
        2 => vec![
            "              ",
            "              ",
            "              ",
            "              ",
            "              ",
            " m            ",
            " mw           ",
            " mwc          ",
            " mwcg         ",
            " mwcgr        ",
            " mwcgrb       ",
            " mwcgrbp      ",
            " mwcgrbpy     ",
            " mwcgrbpym    ",
            " mwcgrbpymw   ",
            " mwcgrbpymwc  ",
            " aaaaaaaaaaaa ",
            "              ",
            "              ",
        ],
        3 => vec![
            "              ",
            "              ",
            "              ",
            "              ",
            " gggggggggggg ",
            "              ",
            " aaawwwwwwwww ",
            "              ",
            " rrrrrrrrrrrr ",
            "              ",
            " wwwwwwwwwaaa ",
            "              ",
            " pppppppppppp ",
            "              ",
            " bbbwwwwwwwww ",
            "              ",
            " cccccccccccc ",
            "              ",
            " wwwwwwwwwccc ",
        ],
        4 => vec![
            "              ",
            "              ",
            "              ",
            "              ",
            " awcgm  yawcg ",
            " wcgmb  awcgm ",
            " cgmbp  wcgmb ",
            " gmbpy  cgmbp ",
            " mbpya  gmbpa ",
            " bpyaw  mbpay ",
            " pyawc  bpayw ",
            " yawcg  paywc ",
            " awcgm  aywcg ",
            " wcgmb  ywcgm ",
            " cgmbp  wcgmb ",
            " gmbpa  cgmbp ",
            " mbpay  gmbpy ",
            " bpayw  mbpya ",
            "              ",
        ],
        _ => panic!("no such level"),
    };

    let mut mx: i32 = 0;
    let mut my: i32 = 0;
    for line in map {
        for c in line.chars() {
            points.push(
                Point::new(mx,my,c.to_string()),
            );
            mx += 1;
        }
        my += 1;
        mx = 0;
    }

    return points
}