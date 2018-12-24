/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate byte_unit;
extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate walkdir;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use byte_unit::Byte;
use docopt::Docopt;
use walkdir::WalkDir;

const USAGE: &str = "
DR purge.

Usage:
  drpurge -l <drproject> -s <source>
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_l: bool,
    arg_drproject: String,
    flag_s: bool,
    arg_source: String,
}

const MEDIA_SUBDIR: &str = "MediaFiles";

fn list_media_assets(library: &Path) -> Vec<PathBuf> {
    let mut media_files = PathBuf::from(library);
    media_files.push(MEDIA_SUBDIR);

    list_source_assets(&media_files)
}

fn list_source_assets(source: &Path) -> Vec<PathBuf> {
    let assets: Vec<PathBuf> = WalkDir::new(source)
        .into_iter()
        .filter_map(|e| {
            if let Ok(e) = e {
                let path = e.path();
                if path.is_file() {
                    return Some(PathBuf::from(path));
                }
            }
            None
        })
        .collect();

    assets
}

fn files_equal(asset: &PathBuf, source: &PathBuf) -> bool {
    let source_attr = fs::metadata(source);
    let asset_attr = fs::metadata(asset);
    if source_attr.is_err() || asset_attr.is_err() {
        return false;
    }
    let source_attr = source_attr.unwrap();
    let asset_attr = asset_attr.unwrap();
    source_attr.len() == asset_attr.len()
}

fn canonicalize_filename(path: &PathBuf) -> Option<String> {
    if let Some(filename) = path.file_name() {
        return Some(filename.to_string_lossy().to_string());
    }
    None
}

fn intersect(set: Vec<PathBuf>, source: Vec<PathBuf>) -> Vec<PathBuf> {
    let lib_content: HashMap<String, PathBuf> = set.into_iter()
        .filter_map(|asset| {
            if let Some(file_name) = asset.file_name() {
                return Some((file_name.to_string_lossy().to_string(), asset.clone()));
            }
            None
        })
        .collect();

    source.into_iter()
        .filter_map(|source_media| {
            if let Some(file_name) = canonicalize_filename(&source_media) {
                if let Some(p) = lib_content.get(&file_name) {
                    if files_equal(p, &source_media) {
                        return Some(source_media.clone());
                    }
                }
            }
            None
        })
        .collect()
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let library = Path::new(&args.arg_drproject);
    let assets = list_media_assets(library);

    let source = Path::new(&args.arg_source);
    let source_assets = list_source_assets(source);

    let dupes = intersect(assets, source_assets);

    let saved = dupes.into_iter().fold(0, |acc, f| {
        println!("{}", f.to_string_lossy());
        if let Ok(size) = fs::metadata(f) {
            acc + size.len()
        } else {
            acc
        }
    });

    let byte = Byte::from_bytes(saved as u128);
    let adjusted_byte = byte.get_appropriate_unit(false);

    println!("Will save {} ({} bytes)", adjusted_byte, saved);
}
