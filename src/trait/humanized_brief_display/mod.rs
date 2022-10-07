use std::fmt;

/// A short phrase.
pub trait HumanizedBriefDisplay {
  fn humanized_brief_display<'a>(&'a self) -> Box<dyn fmt::Display + 'a>;
}
