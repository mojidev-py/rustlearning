fn main() {
    let mut loopsleft = 10;
    loop {
        println!("{} loops left",loopsleft);
        loopsleft -= 1;
        if loopsleft == 0 {
            break;
        }
    }   
}
