pub struct Sizer {
  pub width: Option<String>,
  pub height: Option<String>,
  pub view_box: Option<String>,
}

impl Sizer {
  pub fn new() -> Sizer {
    Sizer {
      width: None,
      height: None,
      view_box: None,
    }
  }

  pub fn width(&self) -> String {
    "1203pt".to_string()
  }

  pub fn height(&self) -> String {
    "281pt".to_string()
  }

  pub fn view_box(&self) -> String {
    "-292 558 603 481".to_string()
  }

  pub fn f_width(&self) -> f32 {
    self.float_value(&self.width.as_ref().unwrap())
  }

  pub fn f_height(&self) -> f32 {
    self.float_value(&self.height.as_ref().unwrap())
  }

  pub fn float_value(
    &self,
    input: &String,
  ) -> f32 {
    input.replace("pt", "").parse::<f32>().unwrap()
  }
}
