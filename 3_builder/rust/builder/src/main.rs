fn main() {}
//简约版建造者模式

trait Build {
    fn build(arg1: String, arg2: String) -> Car;
    fn add(c: &mut Car, pattern: Pattern, arg: String) -> &mut Car;
}

enum Pattern {
    Tail(String),
    Sound(String),
}

struct CarBuild;

struct Car {
    engine: String,
    wheel: String,
    tail: String,
    sound: String,
}

impl Build for CarBuild {
    fn build(engine: String, wheel: String) -> Car {
        Car {
            engine,
            wheel,
            tail: String::new(),
            sound: String::new(),
        }
    }
    fn add(c: &mut Car, pattern: Pattern, arg: String) -> &mut Car {
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
