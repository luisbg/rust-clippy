#![feature(plugin)]
#![plugin(clippy)]
#![deny(clone_from)]
#[allow(unused_variables)]

// the easiest case
fn clone_from() {
    let a = String::new();
    let b = a.clone();  //~ERROR Consider using b.clone_from(&a)
}

// need a main anyway, use it get rid of unused warnings too
fn main() {
    clone_from();
}
