use std::{fs::{File, remove_dir}, path};
use rayon::prelude::*;


fn main() {


     let directories = ["data", "data_1","data_2", "data_3", "data-4", "data_14", "data_15", "data_16", "data_17", "data_18", "data_12", "data_13"];

     let _ = directories.par_iter()
        .map(|path| {
            println!("delete path {:?}", path);
            std::fs::remove_dir_all(path);
            std::fs::remove_dir(path)
        }).count();
        
}
