use colored::Colorize;
use image;

pub fn convert (path: String, output: String) -> Result<(), Box<dyn std::error::Error>>{
    let input = image::open(path)?;
    input.save(output);

    println!("{}", "Done".green());    
    Ok(())
}