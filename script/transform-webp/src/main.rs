
/*
图像大小和格式对web性能的影响很大，大的图片文件会显著降低网站速度，影响用户体验。

我们可以通过使用更好的图像格式来提高性能，如AVIF、WebP、JPEG 2000。其中，AVIF和WebP是目前网络浏览器支持最多的格式。

我们选择将网站图像转换为WebP格式，之所以选择这种格式，是因为它很有效，而且在不同的浏览器中得到了广泛的支持。
*/

use image::DynamicImage;
use webp::Encoder;

/// 将' DynamicImage '编码为webp格式的字节
pub fn encode_webp(image: &DynamicImage) -> Result<Vec<u8>, String> {
    let encoder =
        Encoder::from_image(image).map_err(|e| format!("Failed to create WebP encoder: {}", e))?;
    let webp_data = encoder.encode(100.0);
    Ok(webp_data.to_vec())
}


fn convert_image(input_path: &Path, output_dir: &Option<String>) -> Result<(), String> {
    // 打开并解码图像
    let image_render =
        Reader::open(input_path).map_err(|e| format!("Failed to open image: {}", e))?;
    let image = image_render
        .decode()
        .map_err(|e| format!("Failed to decode image: {}\n", e))?;

    // 将图像编码为WebP
    let webp_data = encode_webp(&image)?;

    // 确定输出路径
    let output_path = if let Some(output_dir) = output_dir {
        Path::new(output_dir)
            .join(input_path.file_stem().unwrap())
            .with_extension("webp")
    } else {
        input_path.with_extension("webp")
    };

    // 将WebP图像写入输出路径
    fs::write(output_path.clone(), webp_data)
        .map_err(|e| format!("Failed to write WebP file: {}", e))?;

    println!("Generated: {}", output_path.display());

    Ok(())
}

fn main() {
    let dir = ["./input/5613.png", "./input/coffee.jpg"];
    let output = Some("./output".to_string());

    for file in dir.iter() {
        if let Err(e) = convert_image(Path::new(&file), &output) {
            eprintln!("Error: {}", e);
        }
    }
}