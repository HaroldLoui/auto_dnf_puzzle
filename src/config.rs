use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = {
        Config::new()
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    row_colors: Vec<String>,
    col_colors: Vec<String>,
    block_size: Vec<u32>,
    choose_top: Vec<u32>,
    choose_bottom: Vec<u32>,
    offset: Vec<u32>,
    loop_count: u8,
    image_position: Vec<u32>,
    start_game: Vec<u32>,
    confirm_game: Vec<u32>,
} 

impl Config {

    fn new() -> Self {
        let file = File::open("config.json").expect("找不到config.json文件");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("解析数据失败")
    }

    pub fn get_loop_count(&self) -> u8 {
        if self.loop_count < 1 {
            return 1;
        }
        self.loop_count
    }

    pub fn get_row_colors(&self) -> Option<Vec<&str>> {
        let row_colors: Vec<&str> = self.row_colors.iter().map(|s| s.as_str()).collect();
        Some(row_colors)
    }

    pub fn get_col_colors(&self) -> Option<Vec<&str>> {
        let col_colors: Vec<&str> = self.col_colors.iter().map(|s| s.as_str()).collect();
        Some(col_colors)
    }

    pub fn get_block_size(&self) -> Option<(u32, u32)> {
        let block_size = self.block_size.clone();
        self.vec_to_tuple(&block_size)
    }

    pub fn get_choose_top(&self) -> Option<(u32, u32)> {
        let choose_top = self.choose_top.clone();
        self.vec_to_tuple(&choose_top)
    }

    pub fn get_choose_bottom(&self) -> Option<(u32, u32)> {
        let choose_bottom = self.choose_bottom.clone();
        self.vec_to_tuple(&choose_bottom)
    }

    pub fn get_offset(&self) -> Option<(u32, u32)> {
        let offset = self.offset.clone();
        self.vec_to_tuple(&offset)
    }

    pub fn get_image_position(&self) -> Option<(u32, u32)> {
        let image_position = self.image_position.clone();
        self.vec_to_tuple(&image_position)
    }

    pub fn get_start_game(&self) -> Option<(u32, u32)> {
        let start_game = self.start_game.clone();
        self.vec_to_tuple(&start_game)
    }

    pub fn get_confirm_game(&self) -> Option<(u32, u32)> {
        let confirm_game = self.confirm_game.clone();
        self.vec_to_tuple(&confirm_game)
    }

    fn vec_to_tuple(&self, vec: &Vec<u32>) -> Option<(u32, u32)> {
        if vec.len() != 2 {
            return None;
        }
        Some((vec[0], vec[1]))
    }
}


#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_colors() {
        let row_colors = ["#000000","#332211","#664422","#996633","#cc8844","#ffaa55","#32cc66","#65ee77","#981088","#cb3299","#fe54aa","#3176bb"];
        let col_colors = ["#000000","#332211","#664422","#996633","#cc8844","#ffaa55","#32cc66","#65ee77","#981088","#cb3299","#fe54aa","#3176bb","#6498cc","#97badd","#cadcee","#fdfeff"];

        let config = Config::new();
        assert_eq!(Some(row_colors.to_vec()), config.get_row_colors());
        assert_eq!(Some(col_colors.to_vec()), config.get_col_colors());
    }

    #[test]
    fn test_config() {
        let config = Config::new();
        assert_eq!(Some((80, 60)), config.get_block_size());
        assert_eq!(Some((170, 970)), config.get_choose_top());
        assert_eq!(Some((170, 1000)), config.get_choose_bottom());
        assert_eq!(Some((70, 110)), config.get_offset());
        assert_eq!(Some((500, 250)), config.get_image_position());
        assert_eq!(Some((700, 1050)), config.get_start_game());
        assert_eq!(Some((700, 1100)), config.get_confirm_game());
    }

}