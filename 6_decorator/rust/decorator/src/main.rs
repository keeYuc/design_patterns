trait Transform {
    fn we_move(&self) {
        println!("move")
    }
}

struct Org;
struct Car {
    o: Org,
}
struct Ship {
    o: Org,
}
impl Transform for Org {}

fn main() {}
