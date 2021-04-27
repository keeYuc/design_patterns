trait Transform {
    fn we_move(&self) {
        println!("org move")
    }
}

struct Org;
struct Car {
    o: Box<dyn Transform>,
}
struct Ship {
    o: Box<dyn Transform>,
}
impl Transform for Org {}

impl Car {
    fn new(t: Box<dyn Transform>) -> Self {
        Car { o: t }
    }
    fn we_move(&self) {
        self.o.we_move();
        println!("im car")
    }
}

impl Ship {
    fn new(t: Box<dyn Transform>) -> Self {
        Ship { o: t }
    }
    fn we_move(&self) {
        self.o.we_move();
        println!("im ship")
    }
}

fn main() {
    Car::new(Box::new(Org))
}
