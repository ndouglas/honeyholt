/// Implements a human-friendly verbose display for a type.
#[macro_export]
macro_rules! humanized_verbose_display {
  ($struct_name: ident, $impl: expr) => {
    use honeyholt::r#trait::humanized_verbose_display::HumanizedVerboseDisplay;
    use std::fmt;
    pub impl HumanizedVerboseDisplay for $struct_name {
      fn humanized_verbose_display<'a>(&'a self) -> Box<dyn fmt::Display + 'a> {
        struct MyDisplay<'a>(pub &'a $struct_name);
        impl<'a> fmt::Display for MyDisplay<'a> {
          fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", ($impl)(self.0))
          }
        }
        Box::new(MyDisplay(self))
      }
    }
  }
}
