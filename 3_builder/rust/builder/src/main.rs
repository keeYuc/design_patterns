fn main() {
    let cb = CarBuild::new();
    cb.build(String::from("engine"), String::from("wheel"));
    // .add(Pattern::Tail {});
}
//简约版建造者模式

trait Build {
    fn build(&self, arg1: String, arg2: String) -> Car;
    fn add(c: Car, pattern: Pattern, arg: String) -> Car;
}

enum Pattern {
    Tail,
    Sound,
}

struct CarBuild;

struct Car {
    engine: String,
    wheel: String,
    tail: String,
    sound: String,
}

impl CarBuild {
    fn new() -> Self {
        CarBuild
    }
}

impl Build for CarBuild {
    fn build(&self, engine: String, wheel: String) -> Car {
        Car {
            engine,
            wheel,
            tail: String::new(),
            sound: String::new(),
        }
    }
    fn add(mut c: Car, pattern: Pattern, arg: String) -> Car {
        match pattern {
            Pattern::Sound(_) => {
                c.sound = arg;
                return c;
            }
            Pattern::Tail(_) => {
                return c;
            }
        }
    }
}
