use std::path::PathBuf;

use crate::properties::traits::*;

pub fn path_checker_and_separator(
    user_input: &Vec<String>,
) -> (Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>) {
    let mut paths_buf: Vec<PathBuf> = Vec::new();
    let mut invalid_paths: Vec<PathBuf> = Vec::new();
    let mut metadata_na: Vec<PathBuf> = Vec::new();
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut dir_paths: Vec<PathBuf> = Vec::new();
    for path_str in user_input {
        let path_buf_check = PathBuf::from(&path_str.trim());
        if path_buf_check.exists() {
            paths_buf.push(path_buf_check);
        } else {
            invalid_paths.push(path_buf_check);
        }
    }
    for path in paths_buf {
        match path.metadata() {
            Ok(md) => {
                if md.is_file() {
                    file_paths.push(path);
                } else if md.is_dir() {
                    dir_paths.push(path);
                } else {
                    invalid_paths.push(path);
                }
            }
            Err(_) => {
                metadata_na.push(path);
                continue;
            }
        };
    }
    (file_paths, dir_paths, metadata_na, invalid_paths)
}

pub fn unwrapper<T>(property: &T, required_info: &str) -> String
where
    T: Info + Dates + Permission,
{
    match required_info {
        "Path" => match property.location() {
            Some(loc) => match property.name() {
                Some(name) => return loc.join(name).to_string_lossy().to_string(),
                None => return "Error occured while retrieving name.".to_string(),
            },
            None => return "Error occured while retrieving location.".to_string(),
        },
        "Name" => {
            match property.name() {
                Some(name) => return name,
                None => return "Error occured while retrieving name.".to_string(),
            };
        }
        "Location" => {
            match property.location() {
                Some(loc) => return loc.to_string_lossy().to_string(),
                None => return "Error occured while retrieving location.".to_string(),
            };
        }
        "Type" => {
            match property.extension() {
                Some(ext) => return ext.to_string(),
                None => return "Error occured while retrieving type.".to_string(),
            };
        }
        "Date Created" => {
            match property.date_created() {
                Some(dc) => {
                    return dc.to_string();
                }
                None => {
                    return "Error occured while retriving date and time of creation.".to_string();
                }
            };
        }
        "Date Accessed" => match property.date_accessed() {
            Some(da) => {
                return da.to_string();
            }
            None => {
                return "Error occured while retriving date and time of last access.".to_string();
            }
        },
        "Date Modified" => match property.date_modified() {
            Some(dm) => {
                return dm.to_string();
            }
            None => {
                return "Error occured while retriving date and time of last modification."
                    .to_string();
            }
        },
        _ => return "Error occured while unwrapping/extracting info".to_string(),
    }
}
