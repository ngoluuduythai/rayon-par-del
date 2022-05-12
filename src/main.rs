use std::{fs::File, path};
use rayon::prelude::*;


fn main() {


     let directories = ["data", "data_2", "data_3", "data_4"];

     let _ = directories.par_iter()
        .map(|path| {
            println!("delete path {:?}", path);
            std::fs::remove_dir_all(path)
        }).count();
        
}
