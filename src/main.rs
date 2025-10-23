#![allow(unused)]
use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
  println!("Hello, world!");
  let in_dir = PathBuf::from(
    "/Users/alan/Documents/Neopoligen/alanwsmith.com/svgs-raw",
  );
  let out_dir = PathBuf::from(
    "/Users/alan/Documents/Neopoligen/alanwsmith.com/svgs",
  );
  let extensions = vec!["svg"];
  let paths =
    make_copy_paths_list(&in_dir, &out_dir, &extensions).unwrap();
  dbg!(paths);
}

pub fn make_copy_paths_list(
  in_dir: &PathBuf,
  out_dir: &PathBuf,
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
