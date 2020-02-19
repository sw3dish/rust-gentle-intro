fn modifies(x: &mut f64) {
  *x = 1.0
}

fn main() {
  let mut x = 0.0;
  modifies(&mut x);
  println!("x is {}", x);
}
