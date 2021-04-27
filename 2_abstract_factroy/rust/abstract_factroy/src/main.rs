trait Factory {
    fn produces(&self) -> Box<dyn Goods> {
        Box::new(Gpu)
    }
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

impl Factory for Amd {}

impl Amd {
    fn new() -> Self {
        Amd
    }
}

fn produce(f: impl Factory) -> Box<dyn Goods> {
    f.produces()
}

fn main() {
    produce(Amd::new()).buy();
}
