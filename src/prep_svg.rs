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

pub fn prep_svg(content: &str) -> Result<String> {
  let mut sizer = Sizer::new();
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
        let mut g2_start = BytesStart::new("g");
        g2_start.push_attribute(
          e.attributes()
            .find(|a| a.as_ref().unwrap().key.0 == b"transform")
            .unwrap()
            .unwrap(),
        );

        let mut g2_end = BytesEnd::new("g");
        assert!(writer.write_event(Event::Start(g2_start)).is_ok());
        assert!(writer.write_event(Event::End(g2_end)).is_ok());

        /*
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
                r_start.push_attribute(Attribute::from(("rx", "1%")));
                r_start.push_attribute(Attribute::from(("ry", "1%")));
                r_start.push_attribute(Attribute::from((
                  "fill",
                  "rebeccapurple",
                )));
                assert!(writer.write_event(Event::Start(r_start)).is_ok());
                let mut r_end = BytesEnd::new("rect");
                assert!(writer.write_event(Event::End(r_end)).is_ok());
        */

        e.push_attribute(Attribute::from(("fill", "none")));
        e.push_attribute(Attribute::from(("stroke", "black")));
        assert!(writer.write_event(Event::Start(e)).is_ok());
      }
      Ok(Event::Start(mut e)) if e.name().as_ref() == b"svg" => {
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
          }
        });
        let mut el = BytesStart::new("svg");
        let id_attr = Attribute::from(("id", id.as_str()));
        el.push_attribute(id_attr);
        el.push_attribute(Attribute::from((
          "width",
          format!("{}", sizer.f_width()).as_str(),
        )));
        el.push_attribute(Attribute::from((
          "height",
          format!("{}", sizer.f_height()).as_str(),
        )));
        el.push_attribute(Attribute::from((
          "viewBox",
          format!("{}", sizer.view_box()).as_str(),
        )));
        el.push_attribute(Attribute::from((
          "xmlns",
          "http://www.w3.org/2000/svg",
        )));
        assert!(writer.write_event(Event::Start(el)).is_ok());
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
