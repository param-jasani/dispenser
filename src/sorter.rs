use crate::{
    path_error_handler::unwrapper,
    properties::{
        dir_props::set_directory_properties,
        file_props::FileProperties,
        traits::{ChildItems, Dates},
    },
};
use clap::ValueEnum;
use std::{
    fs::{create_dir_all, rename},
    path::PathBuf,
};

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum SortFlags {
    SN,
    SE,
    SDM,
    SDA,
    SDC,
}

fn sort_selector(
    sort_method: &SortFlags,
    parent_path: &PathBuf,
    file: &FileProperties,
) -> Option<PathBuf> {
    match sort_method {
        SortFlags::SDA => Some(
            parent_path.join(
                file.date_accessed()
                    .expect("Err!! Can't retrive date accessed.")
                    .date_naive()
                    .to_string(),
            ),
        ),
        SortFlags::SDC => Some(
            parent_path.join(
                file.date_created()
                    .expect("Err!! Can't retrive date accessed.")
                    .date_naive()
                    .to_string(),
            ),
        ),
        SortFlags::SDM => Some(
            parent_path.join(
                file.date_modified()
                    .expect("Err!! Can't retrive date accessed.")
                    .date_naive()
                    .to_string(),
            ),
        ),
        SortFlags::SE => Some(parent_path.join(unwrapper(file, "Type"))),
        _ => None,
    }
}

pub fn sort_files(dir_paths: &Vec<PathBuf>, sort_method: SortFlags, rec_value: u32) {
    let dir_properties = set_directory_properties(&dir_paths, rec_value);
    for property in dir_properties {
        let files = property
            .retrive_child_files()
            .expect("No files are there in this directory.");
        println!("Sorting files...");
        for file in files {
            let parent_path = PathBuf::from(unwrapper(&file, "Path"));
            let sort_path = sort_selector(&sort_method, &parent_path, &file)
                .expect("Err!! Not able to retrive sort method");
            let file_as_path = PathBuf::from(unwrapper(&file, "Name"));
            if !sort_path.exists() {
                create_dir_all(&sort_path).expect("Err!! Failed to create directory.");
            }
            let _ = rename(
                parent_path.join(&file_as_path),
                sort_path.join(&file_as_path),
            );
        }
    }
    println!("Files have been sorted");
}
