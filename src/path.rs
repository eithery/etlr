use std::io;
use std::path::{Path, PathBuf};
use crate::env;


const ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");
pub(crate) const CONFIG_DIR_NAME: &str = "config";


pub(crate) fn config_dir() -> io::Result<PathBuf> {
    Ok(Path::new(ROOT_DIR).join(CONFIG_DIR_NAME))
}


pub(crate) fn current_dir() -> io::Result<PathBuf> {
    std::env::current_dir()
}


pub(crate) fn current_config_dir() -> io::Result<PathBuf> {
    current_dir().map(|path| path.join(CONFIG_DIR_NAME))
}


pub(crate) fn home_dir() -> io::Result<PathBuf> {
    home::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not determine home directory.")
    })
}


pub(crate) fn config_env_dir() -> io::Result<PathBuf> {
    let config_home = env::config_home().map_err(|_| {
        let error_msg = format!("{} environment variable is not defined.", env::CONFIG_HOME_ENV_VAR);
        io::Error::new(io::ErrorKind::NotFound, error_msg)
    })?;
    Ok(PathBuf::from(config_home))
}
