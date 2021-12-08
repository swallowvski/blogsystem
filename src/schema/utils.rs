use std::{fs, io, path::Path};

pub fn check_create_dir(path: &str) -> io::Result<()> {
    let p = Path::new(path);

    if !p.exists() {
        fs::create_dir(p)?;
    };
    Ok(())
}
