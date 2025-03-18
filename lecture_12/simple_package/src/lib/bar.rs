use crate::foo;

pub fn say_something() {
  println!("Bar");
  foo::say_something();
}
