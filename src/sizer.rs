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

  pub fn width_adjust(&self) -> f32 {
    100.0
  }
  pub fn height_adjust(&self) -> f32 {
    80.0
  }

  pub fn svg_width(&self) -> String {
    format!("{}pt", self.f_width() + self.width_adjust())
    // format!("{}pt", self.f_width())
  }

  pub fn svg_height(&self) -> String {
    format!("{}pt", self.f_height() + self.height_adjust())
    // format!("{}pt", self.f_height())
  }

  pub fn rect_width(&self) -> String {
    format!("{}", self.f_width() + self.width_adjust())
    // format!("{}", self.f_width())
  }

  pub fn rect_height(&self) -> String {
    format!("{}", self.f_height() + self.height_adjust())
    // format!("{}", self.f_height())
  }

  pub fn rect_x(&self) -> String {
    //format!("{}", self.f_vb_min_x())
    format!("{}", self.f_vb_min_x() - (self.width_adjust() / 2.0))
  }

  pub fn rect_y(&self) -> String {
    // format!("{}", self.f_vb_min_y())
    format!("{}", self.f_vb_min_y() - (self.height_adjust() / 2.0))
  }

  pub fn view_box(&self) -> String {
    format!(
      "{} {} {} {}",
      self.f_vb_min_x() - (self.width_adjust() / 2.0),
      self.f_vb_min_y() - (self.height_adjust() / 2.0),
      self.f_vb_width() + self.width_adjust(),
      self.f_vb_height() + self.height_adjust(),
      // self.f_vb_min_x(),
      // self.f_vb_min_y(),
      // self.f_vb_width(),
      // self.f_vb_height(),
    )
  }

  pub fn f_width(&self) -> f32 {
    self.float_value(self.width.as_ref().unwrap())
  }

  pub fn f_height(&self) -> f32 {
    self.float_value(self.height.as_ref().unwrap())
  }

  pub fn vb_parts(&self) -> Vec<f32> {
    self
      .view_box
      .as_ref()
      .unwrap()
      .split(" ")
      .map(|n| n.parse::<f32>().unwrap())
      .collect::<Vec<f32>>()
  }

  pub fn f_vb_min_x(&self) -> f32 {
    self.vb_parts()[0]
  }

  pub fn f_vb_min_y(&self) -> f32 {
    self.vb_parts()[1]
  }

  pub fn f_vb_width(&self) -> f32 {
    self.vb_parts()[2]
  }

  pub fn f_vb_height(&self) -> f32 {
    self.vb_parts()[3]
  }

  pub fn float_value(
    &self,
    input: &str,
  ) -> f32 {
    input.replace("pt", "").parse::<f32>().unwrap()
  }
}
