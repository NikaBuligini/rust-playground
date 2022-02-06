pub struct Binding {
  pub filename: String,
}

impl<'i> Binding {
  pub fn new(filename: String) -> Binding {
    Binding { filename }
  }

  pub fn parse(filename: String, code: &'i str) -> Result<Binding, std::io::Error> {
    Ok(Binding { filename })
  }
}
