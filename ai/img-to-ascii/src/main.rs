/*
cargo run R.png
Rust和OpenCV打造ASCII图片
*/

use opencv::core;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <image>", args[0]);
        return;
    }

    // 读取图像文件
    let image = imgcodecs::imread(args[1].as_str(), imgcodecs::IMREAD_COLOR).unwrap();

    // 转换为灰度
    let mut gray_image = Mat::default();
    imgproc::cvt_color(&image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0).unwrap();


        // 设置所需宽度或高度
    let desired_width = 150;
    let desired_height = 150;

    // 在保持长宽比的同时计算比例因子
    let scale_w = desired_width as f64 / image.cols() as f64;
    let scale_h = desired_height as f64 / image.rows() as f64;
    let scale = scale_w.min(scale_h);

    let new_width = (image.cols() as f64 * scale) as i32;
    let new_height = (image.rows() as f64 * scale) as i32;

    // 调整图像大小
    let mut resized = Mat::default();

    imgproc::resize(
        &gray_image,
        &mut resized,
        core::Size::new(new_width, new_height),
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )
    .unwrap();


    
    // ASCII转换
    // 更多的字符表示更精细的细节
    // <$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. >
    let ascii_chars = " .,:;i1tfLCG08@";

    let mut ascii_art = String::new();

    for i in 0..resized.rows() {
        for j in 0..resized.cols() {
            let pixel = resized.at_2d::<u8>(i, j).unwrap();
            let ascii_index = *pixel as usize * ascii_chars.len() / 256;
            ascii_art.push(ascii_chars.chars().nth(ascii_index).unwrap_or(' '));
        }
        ascii_art.push('\n');
    }

    
    // 保存ASCII图像文件
    let mut file = File::create(format!(
        "{}.txt",
        args[1].split('.').collect::<Vec<&str>>()[0]
    ))
    .unwrap();
    file.write_all(ascii_art.as_bytes()).unwrap();

    // 打印ASCII图像到控制台
    println!("{}", ascii_art);

    println!("Done!");
}