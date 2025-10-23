#![allow(unused)]
use anyhow::Result;
use regex::Regex;
use std::{fs, path::Path, path::PathBuf};
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

  paths.iter().for_each(|pair| {
    let scrubbed = scrub_svg(&pair.0).unwrap();
    write_file_with_mkdir(&pair.1, &scrubbed);
  });
}

pub fn scrub_svg(in_path: &PathBuf) -> Result<String> {
  let mut content = fs::read_to_string(in_path)?;
  // let updates = vec![(Regex::new(r#"<\?.*?\?>"#).unwrap(), "")];
  // updates.iter().for_each(|update| {
  //   update.0.replace_all(&content, update.1);
  // });
  Ok(content)
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
