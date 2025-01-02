use std:: io;
use std::cmp::Ordering;

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

        let geuss: u32 = geuss.trim().parse().expect("please type a num");

    println!("you guessed : {}",geuss);

    

    
   match geuss.cmp(&secnum){

    Ordering::Equal=> println!("you got the correct"),
    Ordering::Less =>println!("less please up"),
    Ordering::Greater=>println!("high please down")
   }

            

}
