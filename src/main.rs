#[cfg(windows)]
#[link(name = "libadd.dll", kind = "dylib")]
extern {
  fn add(a: i32, b: i32) -> i32;
}

fn main() {
  println!("Hello, world from main.rs");
  let result = unsafe { add(2, 3) };
  println!("External 'add' from native/main.cpp compiled .dll result (2 + 3) = {}", result);
}
