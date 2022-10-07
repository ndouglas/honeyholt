use std::fmt;

/// A short phrase.
pub trait HumanizedBriefDisplay {
  fn display_humanized_brief<'a>(&'a self) -> Box<dyn fmt::Display + 'a>;
}
