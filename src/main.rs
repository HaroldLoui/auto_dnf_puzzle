#[macro_use]
extern crate lazy_static;

mod color;
mod config;
mod enigo_utils;
mod focus_win;

use color::{Color, ColorIndex, COLOR_INDEX_MAP};
use config::GLOBAL_CONFIG;
use enigo_utils::EnigoUtils;
use image::{ImageBuffer, Pixel, Rgba};
use std::time::Duration;
use win_screenshot::prelude::*;
use focus_win::focus_dnf_window;

fn main() {
    // 聚焦到dnf窗口
    match focus_dnf_window() {
        Ok(_) => {
            auto_puzzle();
        }
        Err(s) => {
            println!("{}", s);
        }
    };
}

fn auto_puzzle() {
    // 单个图块大小
    let block_size = GLOBAL_CONFIG.get_block_size().unwrap();
    // 选中图块上下两部分点击位置
    let choose_index_top = GLOBAL_CONFIG.get_choose_top().unwrap();
    let choose_index_bottom = GLOBAL_CONFIG.get_choose_bottom().unwrap();
    // 游戏区域左上角偏移量
    let delta = GLOBAL_CONFIG.get_offset().unwrap();
    // 多少轮游戏次数
    let count = GLOBAL_CONFIG.get_loop_count();
    // 游戏结束后的确认按钮位置
    let confirm_position = GLOBAL_CONFIG.get_confirm_game().unwrap();
    // 选择拼图的位置
    let image_position = GLOBAL_CONFIG.get_image_position().unwrap();
    // 确认开始游戏的位置
    let start_position = GLOBAL_CONFIG.get_start_game().unwrap();
    let mut enigo_utils = EnigoUtils::new();
    for cnt in 0..count {
        println!("开始游戏，当前轮次：{}", cnt);
        // 选择拼图
        enigo_utils.move_mouse(image_position);
        enigo_utils.left_click();
        // 点击确认按钮开始游戏
        enigo_utils.move_mouse(start_position);
        enigo_utils.left_click();
        // 暂停200毫秒进入游戏界面
        sleep(200);
        // 本轮游戏是否成功完成
        let mut is_success = true;
        // 开始循环
        for _ in 0..192 {
            let buf = capture_display().expect("无法捕获当前屏幕");
            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(buf.width, buf.height, buf.pixels)
                .expect("无法创建图像缓冲区");
            // 获取选取区域图块的上下两部分颜色
            let pixel_top = buffer
                .get_pixel(choose_index_top.0, choose_index_top.1)
                .to_rgb();
            let pixel_bottom = buffer
                .get_pixel(choose_index_bottom.0, choose_index_bottom.1)
                .to_rgb();
            let color_index =
                ColorIndex::new(Color::from_rgb(pixel_top), Color::from_rgb(pixel_bottom));
            if let Some(point) = COLOR_INDEX_MAP.get(&color_index) {
                // 计算放置位置
                let x = point.0 * block_size.1 + block_size.1 / 2 + delta.0;
                let y = point.1 * block_size.0 + block_size.0 / 2 + delta.1;
                // 先移动到选取图块位置
                enigo_utils.move_mouse(choose_index_top);
                // 选取
                enigo_utils.left_click();
                // 移动到对应位置
                enigo_utils.move_mouse((x, y));
                // 放下
                enigo_utils.left_click();
            } else {
                is_success = false;
                println!("无法根据拼图色块解析坐标，将直接结束本轮游戏！");
                break;
            }
        }
        // 游戏成功才进入下一轮
        if is_success {
            println!("当前轮次已结束，即将进行下一轮游戏...");
            // 结束一轮后暂停1.5s等待游戏动画结束
            sleep(2000);
            // 鼠标点击开始下一轮
            enigo_utils.move_mouse(confirm_position);
            enigo_utils.left_click();
        } else {
            println!("数据解析失败，程序即将退出，请确认参数是否正确！");
            break;
        }
    }
}

fn sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
