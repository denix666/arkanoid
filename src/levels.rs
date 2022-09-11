use macroquad::prelude::*;

pub const LVL_1:[[&str; 13]; 19] = [
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["m","m","m","m","m","m","m","m","m","m","m","m","m"],
    ["r","r","r","r","r","r","r","r","r","r","r","r","r"],
    ["y","y","y","y","y","y","y","y","y","y","y","y","y"],
    ["b","b","b","b","b","b","b","b","b","b","b","b","b"],
    ["g","g","g","g","g","g","g","g","g","g","g","g","g"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
];

pub const LVL_2:[[&str; 13]; 19] = [
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["m"," "," "," "," "," "," "," "," "," "," "," "," "],
    ["m","w"," "," "," "," "," "," "," "," "," "," "," "],
    ["m","w","c"," "," "," "," "," "," "," "," "," "," "],
    ["m","w","c","g"," "," "," "," "," "," "," "," "," "],
    ["m","w","c","g","r"," "," "," "," "," "," "," "," "],
    ["m","w","c","g","r","b"," "," "," "," "," "," "," "],
    ["m","w","c","g","r","b","p"," "," "," "," "," "," "],
    ["m","w","c","g","r","b","p","y"," "," "," "," "," "],
    ["m","w","c","g","r","b","p","y","m"," "," "," "," "],
    ["m","w","c","g","r","b","p","y","m","w"," "," "," "],
    ["m","w","c","g","r","b","p","y","m","w","c"," "," "],
    ["m","w","c","g","r","b","p","y","m","w","c","g"," "],
    ["a","a","a","a","a","a","a","a","a","a","a","a","r"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
];

pub const LVL_3:[[&str; 13]; 19] = [
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["g","g","g","g","g","g","g","g","g","g","g","g","g"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["a","a","a","w","w","w","w","w","w","w","w","w","w"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["r","r","r","r","r","r","r","r","r","r","r","r","r"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["w","w","w","w","w","w","w","w","w","w","a","a","a"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["p","p","p","p","p","p","p","p","p","p","p","p","p"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["b","b","b","w","w","w","w","w","w","w","w","w","w"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["c","c","c","c","c","c","c","c","c","c","c","c","c"],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    ["w","w","w","w","w","w","w","w","w","w","c","c","c"],
];

pub const LVL_4:[[&str; 13]; 19] = [
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" ","w","c","g","m","b"," ","y","a","w","c","g"," "],
    [" ","c","g","m","b","p"," ","a","w","c","g","m"," "],
    [" ","g","m","b","p","y"," ","w","c","g","m","b"," "],
    [" ","m","b","p","y","a"," ","c","g","m","b","p"," "],
    [" ","b","p","y","a","w"," ","g","m","b","p","a"," "],
    [" ","p","y","a","w","c"," ","m","b","p","a","y"," "],
    [" ","y","a","w","c","g"," ","b","p","a","y","w"," "],
    [" ","a","w","c","g","m"," ","p","a","y","w","c"," "],
    [" ","w","c","g","m","b"," ","a","y","w","c","g"," "],
    [" ","c","g","m","b","p"," ","y","w","c","g","m"," "],
    [" ","g","m","b","p","a"," ","w","c","g","m","b"," "],
    [" ","m","b","p","a","y"," ","c","g","m","b","p"," "],
    [" ","b","p","a","y","w"," ","g","m","b","p","y"," "],
    [" ","p","a","y","w","c"," ","m","b","p","y","a"," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
];

pub const LVL_5:[[&str; 13]; 19] = [
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," ","w"," "," "," ","w"," "," "," "," "],
    [" "," "," "," "," ","w"," ","w"," "," "," "," "," "],
    [" "," "," "," "," ","w"," ","w"," "," "," "," "," "],
    [" "," "," "," ","a","a","a","a","a"," "," "," "," "],
    [" "," "," "," ","a","a","a","a","a"," "," "," "," "],
    [" "," "," ","a","a","r","a","r","a","a"," "," "," "],
    [" "," "," ","a","a","r","a","r","a","a"," "," "," "],
    [" "," ","a","a","a","a","a","a","a","a","a"," "," "],
    [" "," ","a","a","a","a","a","a","a","a","a"," "," "],
    [" "," ","a"," ","a","a","a","a","a"," ","a"," "," "],
    [" "," ","a"," ","a"," "," "," ","a"," ","a"," "," "],
    [" "," ","a"," ","a"," "," "," ","a"," ","a"," "," "],
    [" "," "," "," "," ","a"," ","a"," "," "," "," "," "],
    [" "," "," "," "," ","a"," ","a"," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
];

pub const LVL_6:[[&str; 13]; 19] = [
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," ","y","g","g","g","g","g","g","g","y"," "," "],
    [" "," ","g","y","c","c","c","c","c","y","g"," "," "],
    [" "," ","g","c","y","f","f","f","y","c","g"," "," "],
    [" "," ","g","c","f","y","p","y","f","c","g"," "," "],
    [" "," ","g","c","f","p","y","p","f","c","g"," "," "],
    [" "," ","g","c","f","y","p","y","f","c","g"," "," "],
    [" "," ","g","c","y","f","f","f","y","c","g"," "," "],
    [" "," ","g","y","c","c","c","c","c","y","g"," "," "],
    [" "," ","y","g","g","g","g","g","g","g","y"," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
    [" "," "," "," "," "," "," "," "," "," "," "," "," "],
];

pub struct Level {
    texture: Texture2D,
    pub lvl_num: i32,
    pub bricks_amount: i32,
}

impl Level {
    pub async fn new() -> Self {
        Self {
            texture: load_texture("assets/backgrounds/bg_1.png").await.unwrap(),
            lvl_num: 1,
            bricks_amount: 0,
        }
    }

    pub async fn set_level(&mut self, level: i32) {
        self.lvl_num = level;

        let path = format!("assets/backgrounds/bg_{}.png",self.lvl_num);
        self.texture = load_texture(&path).await.unwrap();
    }

    pub fn number(&mut self) -> i32 {
        return self.lvl_num
    }

    pub fn draw(&self) {
        draw_texture(self.texture, 1.0, 1.0, WHITE);
    }
}
