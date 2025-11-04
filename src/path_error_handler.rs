use std::{path::PathBuf};

pub fn path_checker_and_separator(user_input: &Vec<String>) -> (Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>){
    let mut paths_buf: Vec<PathBuf> = Vec::new();
    let mut invalid_paths: Vec<PathBuf> = Vec::new();
    let mut metadata_na: Vec<PathBuf> = Vec::new();
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut dir_paths: Vec<PathBuf> = Vec::new();
    for path_str in user_input {
        let path_buf_check = PathBuf::from(&path_str.trim());
        if path_buf_check.exists(){
            paths_buf.push(path_buf_check);
        } 
        else {
            invalid_paths.push(path_buf_check);
        }
    }
    for path in paths_buf {
        match path.metadata() {
            Ok(md) => { 
                if md.is_file(){
                    file_paths.push(path);
                }
                else if md.is_dir() {
                    dir_paths.push(path);
                }
                else{
                    invalid_paths.push(path);
                }
            },
            Err(_) => { 
                    metadata_na.push(path);
                    continue;
            },
        };
    }
    (file_paths, dir_paths, metadata_na, invalid_paths)
}
