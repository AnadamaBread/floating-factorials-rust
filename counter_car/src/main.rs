/* Rust: Floating Factorials
 * Written by Luis Baez
 * 
 * Takes an integer entered by the user.
 * Claims integer has to be less than 200.
 * Prints out the factorials, math by math. 
 *
 * */

use std::io;
use std::io::{stdin,stdout,Write};
use std::io::prelude::*;
use std::mem;
fn main() {
  print!("Enter a number, less than 200: ");
  io::stdout().flush();
  let mut numstring = String::new();
  stdin().read_line(&mut numstring);
  // TODO Error Checking
  let count: f64 = numstring.trim().parse().unwrap();
  factorial(count);
}

fn factorial(x: f64) {  
    println!("Number: {}", x);
    let countsize:f64 = x;
    let countint = countsize as i32;
    let mut j:f64 = 1.0;
    for i in 1..=countint {
        let mut i = i as f64 * 1.0;
        j = j * i;
        println!("{}", j);
    }
    println!();
}
