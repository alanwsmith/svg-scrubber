#![allow(unused)]
use crate::sizer::Sizer;
use anyhow::Result;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesCData, BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use regex::Regex;
use std::io::Cursor;
use std::{fs, path::Path, path::PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;

pub fn scrub_svg(in_path: &PathBuf) -> Result<String> {
  let mut content = fs::read_to_string(in_path)?;
  let mut reader = Reader::from_str(&content);
  reader.config_mut().trim_text(true);
  let mut writer = Writer::new(Cursor::new(Vec::new()));
  let mut remove_content = false;
  let mut styles_added = false;
  loop {
    match reader.read_event() {
      Ok(Event::Start(mut e)) if e.name().as_ref() == b"g" => {
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
        let to_move =
          ["version", "xmlns", "width", "height", "viewBox"];
        e.attributes().for_each(|attr| {
          if let Ok(a) = attr {
            let check_it = String::from_utf8_lossy(a.key.0);
            let v = String::from_utf8_lossy(&a.value).to_string();
            if to_move.contains(&check_it.to_string().as_str()) {
              elem.push_attribute(a);
            }
          }
        });
        assert!(writer.write_event(Event::Start(elem)).is_ok());
      }

      Ok(Event::Empty(mut e)) if e.name().as_ref() == b"path" => {
        let mut elem = BytesStart::new("path");
        let to_move = ["stroke-width", "d", "stroke", "fill"];
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
        let to_move = ["stroke-width", "d", "stroke", "fill"];
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
      }
      Ok(Event::End(e)) if e.name().as_ref() == b"title" => {
        remove_content = false;
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
  let updates = [(Regex::new(r#"<\?.*?\?>"#).unwrap(), "")];
  updates.iter().for_each(|update| {
    output = update.0.replace_all(&output, update.1).to_string();
  });
  Ok(output)
}
