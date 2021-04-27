trait Machine {
    fn run(&self) -> i8;
}

struct Drone;
struct Car;
struct Factory;

struct Test(Box<Car>);

impl Test {
    fn new() -> Self {
        let a = Box::new(Car);
        Test { 0: a }
    }
}

impl Factory {
    fn new() -> Self {
        Factory
    }
    fn get_machine(&self, a: i8) -> Box<dyn Machine> {
        match a {
            1 => Box::new(Drone),
            _ => Box::new(Car),
        }
    }
}

impl Machine for Drone {
    fn run(&self) -> i8 {
        1
    }
}

impl Machine for Car {
    fn run(&self) -> i8 {
        2
    }
}

#[test]
fn test_factory() {
    let f = Factory::new();
    assert_eq!(f.get_machine(1).run(), 1);
    assert_eq!(f.get_machine(2).run(), 2)
}
fn main() {
    let a = Test::new();
    a.0.run();
}
