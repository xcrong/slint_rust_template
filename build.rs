use std::{env, error::Error, fs::File, io::BufWriter};

use ico::{IconDir, IconDirEntry, IconImage};
use image::GenericImageView;
use winresource::WindowsResource;

fn convert_png_to_ico(png_path: &str, icon_path: &str) -> Result<(), Box<dyn Error>> {
    // 读取PNG图片和尺寸
    let png = image::open(png_path)?;
    let (width, height) = png.dimensions();

    // 将PNG图片转换为4通道 RGBA数据
    let rgba_data = png.to_rgba8();

    // 创建一个ico图标目录并将ico图标数据放入其中
    let ico_image = IconImage::from_rgba_data(width, height, rgba_data.into_raw());
    // IconDir 相当于一个容器，可以容纳图标数据
    let mut icon_dir = IconDir::new(ico::ResourceType::Icon);
    icon_dir.add_entry(IconDirEntry::encode(&ico_image)?);

    // 把 icon_dir 中存储的数据写入 ico 文件
    let file = File::create(icon_path)?;
    let mut writer = BufWriter::new(file);
    icon_dir.write(&mut writer)?;

    Ok(())
}

fn main() {
    slint_build::compile("ui/app.slint").expect("Slint build failed");

    // 将 png 转换成 ico
    convert_png_to_ico("./ui/res/logo.png", "./ui/res/logo.ico").unwrap();

    // 将 ico 图标嵌入 二进制文件
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("./ui/res/logo.ico")
            .compile()
            .unwrap();
    }
}
