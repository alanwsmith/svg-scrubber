#![allow(unused)]
use anyhow::Result;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use regex::Regex;
use std::io::Cursor;
use std::{fs, path::Path, path::PathBuf};
use walkdir::WalkDir;

fn main() {
  println!("Hello, world!");
  let in_dir = PathBuf::from(
    "/Users/alan/Documents/Neopoligen/alanwsmith.com/svgs-raw-notes",
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
  let mut reader = Reader::from_str(&content);
  reader.config_mut().trim_text(true);
  let mut writer = Writer::new(Cursor::new(Vec::new()));

  let mut editTitle = false;
  let mut editDesc = false;

  loop {
    match reader.read_event() {
      Ok(Event::Start(mut e)) if e.name().as_ref() == b"svg" => {
        let mut elem = BytesStart::new("svg");
        let to_move =
          vec!["width", "height", "version", "viewBox", "xmlns"];
        e.attributes().into_iter().for_each(|attr| {
          if let Ok(a) = attr {
            let check_it = String::from_utf8_lossy(a.key.0);
            if to_move.contains(&check_it.to_string().as_str()) {
              elem.push_attribute(a);
            } else {
              dbg!(check_it);
            }
          }
          ()
        });
        assert!(writer.write_event(Event::Start(elem)).is_ok());
      }

      Ok(Event::Start(mut e)) if e.name().as_ref() == b"title" => {
        editTitle = true;
        e.clear_attributes();
        assert!(writer.write_event(Event::Start(e)).is_ok());
      }

      Ok(Event::Text(mut e)) if editTitle => {
        // TODO: Update th title here
        editTitle = false;
      }

      Ok(Event::Start(e)) if e.name().as_ref() == b"desc" => {
        editDesc = true;
      }
      Ok(Event::Text(mut e)) if editDesc => {
        editDesc = false;
      }
      Ok(Event::End(e)) if e.name().as_ref() == b"desc" => {}

      // Ok(Event::Start(e)) if e.name().as_ref() == b"desc" => {}
      // Ok(Event::End(e)) if e.name().as_ref() == b"desc" => {}

      //Ok(Event::Start(e)) if e.name().as_ref() == b"title" => {
      //   // let mut elem = BytesStart::new("my_elem");
      //   // elem.extend_attributes(
      //   //   e.attributes().map(|attr| attr.unwrap()),
      //   // );
      //   // elem.push_attribute(("my-key", "some value"));
      //   assert!(writer.write_event(Event::Start(e)).is_ok());
      // }
      Ok(Event::Start(e)) if e.name().as_ref() == b"this_tag" => {
        let mut elem = BytesStart::new("my_elem");
        elem.extend_attributes(
          e.attributes().map(|attr| attr.unwrap()),
        );
        elem.push_attribute(("my-key", "some value"));
        assert!(writer.write_event(Event::Start(elem)).is_ok());
      }
      Ok(Event::End(e)) if e.name().as_ref() == b"this_tag" => {
        assert!(
          writer
            .write_event(Event::End(BytesEnd::new("my_elem")))
            .is_ok()
        );
      }
      Ok(Event::Comment(_)) => {}
      Ok(Event::Empty(e))
        if e.name().as_ref() == b"sodipodi:namedview" => {}
      // Ok(Event::Empty(e)) => {
      //   dbg!(e);
      // }
      Ok(Event::Eof) => break,
      // Ok(Event::Start(e)) if e.name().as_ref() != b"path" => {
      //   dbg!(&e);
      // }
      Ok(e) => assert!(writer.write_event(e).is_ok()),
      Err(e) => panic!(
        "Error at position {}: {:?}",
        reader.error_position(),
        e
      ),
    }
  }
  let result = writer.into_inner().into_inner();
  let output = String::from_utf8_lossy(&result).to_string();

  // let updates = vec![(Regex::new(r#"<\?.*?\?>"#).unwrap(), "")];
  // updates.iter().for_each(|update| {
  //   update.0.replace_all(&content, update.1);
  // });
  Ok(output)
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
