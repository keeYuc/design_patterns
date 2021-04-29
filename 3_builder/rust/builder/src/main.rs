fn main() {
    let cb = CarBuild::new();
    let mut lbjni = cb.build(String::from("engine"), String::from("wheel"));
    let mut fll = cb.build(String::from("engine"), String::from("wheel"));
    cb.add(&mut fll, Pattern::Sound, String::from("漫步者"));
    cb.add(&mut lbjni, Pattern::Sound, String::from("海森赛尔"));
    cb.add(&mut lbjni, Pattern::Tail, String::from("尾翼1号"));
    println!("{:?}", lbjni);
    println!("{:?}", fll)
}
//简约版建造者模式

trait Build {
    fn build(&self, arg1: String, arg2: String) -> Car;
    fn add<'a>(&self, c: &'a mut Car, p: Pattern, s: String);
}

enum Pattern {
    Tail,
    Sound,
}

struct CarBuild;

#[derive(Debug)]
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
    fn add<'a>(&self, c: &'a mut Car, pattern: Pattern, arg: String) {
        match pattern {
            Pattern::Sound => {
                c.sound = arg;
            }
            Pattern::Tail => {
                c.tail = arg;
            }
        }
    }
}
