pub mod traits;
pub mod file_props;
mod dir_props;

#[derive(Clone)]
pub enum AccessMethods {
    Read,
    Write,
    Execute,
}

