struct Points<T, U> {
    x: T,
    y: U,
}

enum Success<O, E> {
    ok(O),
    error(E),
}
impl<T, U> Points<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Points<f64, f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("This is from triat")
    }
}

pub struct Tweet {
    pub username: String,
    pub despcription: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("traits: {}", self.username)
    }
}
fn main() {
    let vec_num = vec![2, 8, 10, 300, 1, 4];
    let largest_num = find_largest(vec_num);
    println!("The largest num is: {}", largest_num);

    let vec_char = vec!['a', 'b', 'y', 'c', 'd', 'r'];
    let largest_char = find_largest(vec_char);
    println!("The largest char is: {}", largest_char);
    let p1 = Points { x: 10.4, y: 10.3 };
    p1.y();
}

fn find_largest<T: PartialOrd + Copy>(num_list: Vec<T>) -> T {
    let mut largest_num = num_list[0];
    for num in num_list {
        if num > largest_num {
            largest_num = num;
        }
    }
    largest_num
}
