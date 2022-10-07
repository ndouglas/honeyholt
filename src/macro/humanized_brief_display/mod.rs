/// Implements a human-friendly brief display for a type.
#[macro_export]
macro_rules! humanized_brief_display {
  ($struct_name: ident, $impl: expr) => {
    use honeyholt::r#trait::humanized_brief_display::HumanizedBriefDisplay;
    use std::fmt;
    impl HumanizedBriefDisplay for $struct_name {
      fn display_humanized_brief<'a>(&'a self) -> Box<dyn fmt::Display + 'a> {
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
