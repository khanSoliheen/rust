pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}
pub trait Pilot {
    fn fly() {}
}
pub trait Wizard {
    fn fly() {}
}

struct Human {}

impl Pilot for Human {
    fn fly() {
        println!("This is your Pilot speaking");
    }
}

impl Wizard for Human {
    fn fly() {
        println!("up")
    }
}

impl Human {
    fn fly() {
        println!("Waiving arms");
    }
}
impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}
fn main() {
    <Human as Wizard>::fly();
}
