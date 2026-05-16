use dirs;
pub fn downloads_dir() -> anyhow::Result<PathBuf> {
    let path = dirs::download_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find Downloads folder"))?;
    Ok(path)
}