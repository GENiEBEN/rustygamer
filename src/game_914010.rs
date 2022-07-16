use directories::ProjectDirs;
use exitcode::OK;
use std::io;

pub(crate) fn get_save_dir() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Foo Corp",  "Bar App") {
        println!("Pro {:?}", proj_dirs.cache_dir());

    }
    
    
}