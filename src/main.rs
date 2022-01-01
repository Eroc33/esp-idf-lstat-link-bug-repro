use std::path::PathBuf;

use esp_idf_sys as _;

fn main() {
    esp_idf_sys::link_patches();

    std::fs::symlink_metadata(PathBuf::from("")).unwrap();
}
