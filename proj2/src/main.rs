use std::io;

fn main() {
    println!("Welcome to the ultimate thing!");
    println!("please answer some random thing!");
    let mut poopoopapa = String::new();
    io::stdin().read_line(&mut poopoopapa).expect("sas");
    println!("The thingy you did or some stuf like that was this: {}",poopoopapa)

}



