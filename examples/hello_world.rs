#![allow(unused_imports)]
#![allow(unused_macros)]

use honeyholt::*;

struct Foo {
  pub foo: i32,
}

humanized_brief_display!(Foo, |var: &Foo| { var.foo.to_string() });

#[named]
fn main() {
  init_pretty_env_logger();
  trace_enter!();
  println!("Hello, world!");
  println!("{}", Foo { foo: 32 }.display_humanized_brief());
  trace_exit!();
}
