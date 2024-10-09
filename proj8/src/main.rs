
struct Shape {
    organic: bool,
    sides: i64,
    width: i64,
    height: i64
}

impl Shape {
    fn area(&self) -> i64 {
        self.width * self.height
    }
    fn organicize(&mut self) {
        self.organic = false; 
    }
}

fn main() {
    let mut square: Shape = Shape {
        organic: true,
        sides: 33,
        width: 33,
        height: 33
    };
    println!("{} - Not an organic shape",square.organic);
    println!("{} units",square.height);
    println!("{} units",square.width);
    println!("{} sides",square.sides);
    let area: i64 = square.area();
    println!("{} units squared",area);
    square.organicize();
    println!("{} - Whoops, is an organic shape!",square.organic)
}