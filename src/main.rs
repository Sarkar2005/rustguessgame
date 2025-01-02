use std::{any::TypeId, io};
use rand::Rng;

fn main() {
    println!("geuss the number");

    let secnum  = rand::thread_rng() . gen_range(0, 100);

     println!("the secrate num is :{}",secnum);

    println! ("please input your geuss");

    let mut geuss = String::new();
    
    io::stdin()
         .read_line(&mut  geuss)
         .expect("failed to read line");

    println!("you guessed : {}",geuss);

    // if geuss = secnum {
    //     println!("you gotta right geuss");
    // } 

    if TypeId::of()::geuss 

            

}
