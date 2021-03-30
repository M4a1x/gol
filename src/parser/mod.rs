use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

pub mod error;
pub mod life;
pub mod rules;

use crate::util::Cell;
use crate::util::Size;
pub use error::ParseError;
pub use rules::Rules;

#[derive(Debug, PartialEq)]
pub struct Pattern {
    size: Size,
    alive_list: Vec<Cell>,
    config: PatternConfig,
}

#[derive(Debug, PartialEq)]
pub struct PatternConfig {
    ruleset: Option<Rules>,
    description: Option<String>,
    author: Option<String>,
    wrap_edges: bool,
}

pub fn load_file(filepath: &Path) -> Result<Pattern, ParseError> {
    match filepath
        .extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.to_lowercase())
        .as_deref()
    {
        Some("life") | Some("lif") => life::parse(File::open(filepath)?),
        Some("rle") => panic!("Not yet implemented!"),
        Some("l") => panic!("Not yet implemented!"),
        Some("plf") => panic!("Not yet implemented!"),
        Some("mcl") => panic!("Not yet implemented!"),
        Some(all) => Err(ParseError::UnknownFileExtension(all.to_owned())),
        None => Err(ParseError::UnknownFileExtension(
            "file has no extension".to_owned(),
        )),
    }
}
