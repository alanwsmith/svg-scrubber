#![allow(unused)]
use anyhow::Result;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesCData, BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use regex::Regex;
use std::io::Cursor;
use std::{fs, path::Path, path::PathBuf};
use svg_scrubber::prep_svg::*;
use svg_scrubber::scrub_svg::*;
use svg_scrubber::sizer::Sizer;
use uuid::Uuid;
use walkdir::WalkDir;

fn main() {
  println!("Starting...");
  let in_dir = PathBuf::from(
    "content/svgs/input",
    // "/Users/alan/Documents/Neopoligen/alanwsmith.com/svgs-raw-notes",
  );
  let out_dir = PathBuf::from(
    "content/svgs/output",
    // "/Users/alan/Documents/Neopoligen/alanwsmith.com/svgs",
  );
  let extensions = vec!["svg"];
  let paths =
    make_copy_paths_list(&in_dir, &out_dir, &extensions).unwrap();
  paths.iter().for_each(|pair| {
    // dbg!(&pair);
    let scrubbed = scrub_svg(&pair.0).unwrap();
    let prepped = prep_svg(&scrubbed).unwrap();
    write_file_with_mkdir(&pair.1, &prepped);
  });
  println!("Done");
}

pub fn make_copy_paths_list(
  in_dir: &PathBuf,
  out_dir: &Path,
  extensions: &Vec<&str>,
) -> Result<Vec<(PathBuf, PathBuf)>> {
  Ok(
    WalkDir::new(in_dir)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.path().is_file())
      .filter(|e| {
        !e.file_name()
          .to_str()
          .map(|s| s.starts_with("."))
          .unwrap_or(false)
      })
      .filter(|e| {
        if let Some(ext) = e.path().extension() {
          extensions.contains(&ext.to_str().unwrap())
        } else {
          false
        }
      })
      .map(|e| {
        let dest_path =
          out_dir.join(e.path().strip_prefix(in_dir).unwrap());
        (e.path().to_path_buf(), dest_path)
      })
      .collect(),
  )
}

pub fn write_file_with_mkdir(
  output_path: &PathBuf,
  content: &str,
) -> Result<()> {
  if let Some(parent_dir) = output_path.parent() {
    std::fs::create_dir_all(parent_dir)?;
  }
  std::fs::write(output_path, content)?;
  Ok(())
}
