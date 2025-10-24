use core::{f64};
use chrono::{Local, DateTime};
use super::AccessMethods;

struct DirectoryProperties {
    name: String,
    size: f64,
    location: String,
    date_created: Option<DateTime<Local>>,
    files: u32,
    folders: u32,
    access_permissions: Vec<AccessMethods>,
}
