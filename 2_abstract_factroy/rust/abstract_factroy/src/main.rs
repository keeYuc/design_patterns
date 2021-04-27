trait Factory {
    fn produce() -> Box<dyn Goods>;
}

trait Goods {
    fn buy(&self);
}

struct Amd;
struct Gpu;

impl Goods for Gpu {
    fn buy(&self) {
        println!("buy buy buy")
    }
}

impl Factory for Amd {
    fn produce() -> Box<dyn Goods> {
        Box::new(Gpu)
    }
}

impl Amd {
    fn new() -> Self {
        Amd
    }
}

fn main() {
    let a = Amd::new();
    a.produce();
}
