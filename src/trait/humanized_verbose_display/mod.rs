use std::fmt;

/// A short phrase.
pub trait HumanizedVerboseDisplay {
  fn humanized_verbose_display<'a>(&'a self) -> Box<dyn fmt::Display + 'a>;
}
