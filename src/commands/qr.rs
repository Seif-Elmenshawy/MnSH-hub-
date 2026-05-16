use qrcode;
use image::Luma;
use std::path::PathBuf;
use dirs;
use colored::*;

fn downloads_dir() -> anyhow::Result<PathBuf> {
    let path = dirs::download_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find Downloads folder"))?;
    Ok(path)
}

pub fn qr_creator(link: String) -> anyhow::Result<()>{
    let mut outdir = downloads_dir()?;

    outdir.push("qr.png");
    
    let code = qrcode::QrCode::new(link.as_bytes()).unwrap();

    let image = code.render::<Luma<u8>>().build();

    image.save(&outdir)?;

    let string = code.render().light_color(" ").dark_color("#").build();

    println!("QR Code is generated and saved to {}", outdir.display().to_string().green());
    println!(
        "{}",
        code.render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build()
    );

    Ok(())
}