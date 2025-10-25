use std::path::PathBuf;
use chrono::{Local, DateTime};
use super::{AccessMethods, file_props::FileProperties, dir_props::DirectoryProperties};

pub trait Info {
    fn name(&self) -> Option<String>;
    fn extension(&self) -> Option<String>;
    fn size(&self) -> f64;
    fn location(&self) -> Option<PathBuf>;
}

pub trait Dates {
    fn date_created(&self) -> Option<DateTime<Local>>;
    fn date_modified(&self) -> Option<DateTime<Local>>;
    fn date_accessed(&self) -> Option<DateTime<Local>>;
}

pub trait Permission {
    fn access_configs(&self) -> Vec<AccessMethods>;
}

pub trait Hash {
    fn hash(&self) -> Option<String>;
}

pub trait child_items{
    fn contains(&self) -> (u32, u32);
    fn retrive_child_files(&self) -> Option<Vec<FileProperties>>;
    fn retrive_sub_folders(&self) -> Option<Vec<DirectoryProperties>>;
}