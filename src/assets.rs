
use std::path::{Path, PathBuf};
use config::Config;

pub struct Assets {

}

impl Assets {
    /**
     * path is a url path.
     * Suppose the url is: /assets/css/app-sdkj2ifak.css
     * In frontend development, we will remove the hash.
     */
    pub fn get_asset(path: &String, config: &Config) -> Result<String, _> {
        let search_path = config.search_path;
        let resource_path = Path::from(search_path).join(path);
    }
}

fn remove_hash
