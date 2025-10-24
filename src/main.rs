#![allow(unused)]
use anyhow::Result;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesCData, BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use regex::Regex;
use std::io::Cursor;
use std::{fs, path::Path, path::PathBuf};
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
    //"/Users/alan/Documents/Neopoligen/alanwsmith.com/svgs",
  );
  let extensions = vec!["svg"];
  let paths =
    make_copy_paths_list(&in_dir, &out_dir, &extensions).unwrap();
  paths.iter().for_each(|pair| {
    // dbg!(&pair);
    let scrubbed = scrub_svg(&pair.0).unwrap();
    write_file_with_mkdir(&pair.1, &scrubbed);
  });
  println!("Done");
}

pub fn scrub_svg(in_path: &PathBuf) -> Result<String> {
  let mut sizer = Sizer::new();
  let mut content = fs::read_to_string(in_path)?;
  let mut reader = Reader::from_str(&content);
  reader.config_mut().trim_text(true);
  let mut writer = Writer::new(Cursor::new(Vec::new()));

  let id = format!("svg-{}", Uuid::new_v4());
  let styles = include_str!("styles.css").replace("SVG_ID", &id);

  let mut remove_content = false;
  let mut styles_added = false;

  loop {
    match reader.read_event() {
      Ok(Event::Start(mut e)) if e.name().as_ref() == b"g" => {
        let mut r_start = BytesStart::new("rect");
        r_start.push_attribute(Attribute::from((
          "class",
          "svg-note-background",
        )));
        r_start.push_attribute(Attribute::from((
          "width",
          sizer.rect_width().as_str(),
        )));
        r_start.push_attribute(Attribute::from((
          "height",
          sizer.rect_height().as_str(),
        )));
        r_start.push_attribute(Attribute::from((
          "x",
          sizer.rect_x().as_str(),
        )));
        r_start.push_attribute(Attribute::from((
          "y",
          sizer.rect_y().as_str(),
        )));
        r_start.push_attribute(Attribute::from(("rx", "2%")));
        r_start.push_attribute(Attribute::from(("ry", "2%")));
        r_start.push_attribute(Attribute::from(("fill", "blue")));
        assert!(writer.write_event(Event::Start(r_start)).is_ok());
        let mut r_end = BytesEnd::new("rect");
        assert!(writer.write_event(Event::End(r_end)).is_ok());

        let mut elem = BytesStart::new("g");
        let to_move = ["transform"];
        e.attributes().for_each(|attr| {
          if let Ok(a) = attr {
            let check_it = String::from_utf8_lossy(a.key.0);
            if to_move.contains(&check_it.to_string().as_str()) {
              elem.push_attribute(a);
            }
          }
        });
        assert!(writer.write_event(Event::Start(elem)).is_ok());
      }

      Ok(Event::Start(mut e)) if e.name().as_ref() == b"svg" => {
        let mut elem = BytesStart::new("svg");
        let to_move = ["version", "xmlns"];

        e.attributes().for_each(|attr| {
          if let Ok(a) = attr {
            let check_it = String::from_utf8_lossy(a.key.0);
            let v = String::from_utf8_lossy(&a.value).to_string();
            if check_it == "width" {
              sizer.width = Some(v.clone());
            }
            if check_it == "height" {
              sizer.height = Some(v.clone());
            }
            if check_it == "viewBox" {
              sizer.view_box = Some(v.clone());
            }
            if to_move.contains(&check_it.to_string().as_str()) {
              elem.push_attribute(a);
            }
          }
        });

        dbg!(sizer.f_width());
        dbg!(sizer.f_height());
        elem.push_attribute(Attribute::from((
          "width",
          sizer.svg_width().as_str(),
        )));

        elem.push_attribute(Attribute::from((
          "height",
          sizer.svg_height().as_str(),
        )));

        elem.push_attribute(Attribute::from((
          "viewBox",
          sizer.view_box().as_str(),
        )));

        let id_attr = Attribute::from(("id", id.as_str()));
        elem.push_attribute(id_attr);
        assert!(writer.write_event(Event::Start(elem)).is_ok());
        if !styles_added {
          styles_added = true;
          let mut style_start = BytesStart::new("style");
          assert!(
            writer.write_event(Event::Start(style_start)).is_ok()
          );
          let mut cdata = BytesCData::new(&styles);
          assert!(writer.write_event(Event::CData(cdata)).is_ok());
          let mut style_end = BytesEnd::new("style");
          assert!(
            writer.write_event(Event::End(style_end)).is_ok()
          );
        }
      }

      Ok(Event::Empty(mut e)) if e.name().as_ref() == b"path" => {
        let mut elem = BytesStart::new("path");
        let to_move = ["stroke-width", "d", "stroke"];
        e.attributes().for_each(|attr| {
          if let Ok(a) = attr {
            let check_it = String::from_utf8_lossy(a.key.0);
            if to_move.contains(&check_it.to_string().as_str()) {
              elem.push_attribute(a);
            }
          }
        });
        assert!(writer.write_event(Event::Empty(elem)).is_ok());
      }

      Ok(Event::Start(mut e)) if e.name().as_ref() == b"path" => {
        let mut elem = BytesStart::new("path");
        let to_move = ["stroke-width", "d"];
        e.attributes().for_each(|attr| {
          if let Ok(a) = attr {
            let check_it = String::from_utf8_lossy(a.key.0);
            if to_move.contains(&check_it.to_string().as_str()) {
              elem.push_attribute(a);
            }
          }
        });
        assert!(writer.write_event(Event::Start(elem)).is_ok());
      }

      Ok(Event::Empty(e)) if e.name().as_ref() == b"title" => {}
      Ok(Event::Start(mut e)) if e.name().as_ref() == b"title" => {
        remove_content = true;
        e.clear_attributes();
        assert!(writer.write_event(Event::Start(e)).is_ok());
      }
      Ok(Event::End(e)) if e.name().as_ref() == b"title" => {
        remove_content = false;
        assert!(writer.write_event(Event::End(e)).is_ok());
      }

      Ok(Event::Empty(e)) if e.name().as_ref() == b"desc" => {}
      Ok(Event::Start(e)) if e.name().as_ref() == b"desc" => {
        remove_content = true;
      }
      Ok(Event::End(e)) if e.name().as_ref() == b"desc" => {
        remove_content = false;
      }

      Ok(Event::Empty(e)) if e.name().as_ref() == b"defs" => {}
      Ok(Event::Start(e)) if e.name().as_ref() == b"defs" => {
        remove_content = true;
      }
      Ok(Event::End(e)) if e.name().as_ref() == b"defs" => {
        remove_content = false;
      }

      //Ok(Event::Start(e)) if e.name().as_ref() == b"title" => {
      //   // let mut elem = BytesStart::new("my_elem");
      //   // elem.extend_attributes(
      //   //   e.attributes().map(|attr| attr.unwrap()),
      //   // );
      //   // elem.push_attribute(("my-key", "some value"));
      //   assert!(writer.write_event(Event::Start(e)).is_ok());
      // }

      // Ok(Event::Start(e)) if e.name().as_ref() == b"this_tag" => {
      //   let mut elem = BytesStart::new("my_elem");
      //   elem.extend_attributes(
      //     e.attributes().map(|attr| attr.unwrap()),
      //   );
      //   elem.push_attribute(("my-key", "some value"));
      //   assert!(writer.write_event(Event::Start(elem)).is_ok());
      // }
      Ok(Event::Comment(_)) => {}

      Ok(Event::Empty(e))
        if e.name().as_ref() == b"sodipodi:namedview" => {}

      Ok(Event::Eof) => break,

      Ok(e) => {
        if !remove_content {
          assert!(writer.write_event(e).is_ok())
        }
      }

      Err(e) => panic!(
        "Error at position {}: {:?}",
        reader.error_position(),
        e
      ),
    }
  }
  let result = writer.into_inner().into_inner();
  let mut output = String::from_utf8_lossy(&result).to_string();
  let updates = vec![(Regex::new(r#"<\?.*?\?>"#).unwrap(), "")];
  updates.iter().for_each(|update| {
    output = update.0.replace_all(&output, update.1).to_string();
  });
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
