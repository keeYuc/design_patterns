trait Factory {
    fn produce() -> Box<dyn Goods>;
}

trait Goods {
    fn buy();
}

struct Amd;

impl Factory for Amd {
    fn new() -> Self {
        Amd
    }
    fn produce() -> Box<dyn Goods> {}
}

fn main() {}
