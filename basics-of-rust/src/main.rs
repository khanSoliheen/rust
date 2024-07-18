fn main() {
    println!("Hello, world!");
    let x: i32 = 10;
    println!("The value of X1 is: {}", x);
    let x: i32 = 15;
    println!("The value of X2 is: {}", x);
    let tup = ("Let's get rusty", 100_000);
    let (string, number): (&str, i32) = tup;
    println!(
        "The tup values are: {}, {}, index value: {}",
        string, number, tup.0
    );
    let array: [i32; 3] = [29, 39, 49];
    println!("The first index of array is: {}", array[0]);
    let sum = my_add_function(3, 6);
    if sum == 9 {
        println!("The sum is equal to: {}", sum)
    } else {
        println!("The sum is greater or lesser than: {}", sum)
    }
    println!("sum return value: {}", sum);
    loop {
        println!("I am from loop");
        break;
    }
    for item in array.iter() {
        println!("I am from 'for' loop: {}", item);
    }
    // while condtion {

    // }
    for number in 1..4 {
        println!("I am from series for loop: {}", number);
    }
    let x = 5;
    let _y = x; // copy

    let s1: String = String::from("Hello");
    // let s2 = s1 will not work s1 has been dropped
    let s2: String = s1.clone();
    println!("{}, world", s2);
    take_ownership(s1);
    // println!("s1 after lending: {}", s1); It has bee=n move to take_ownership
    let n = 5;
    make_copy(n);
    println!("make_copy integer: {}", n); // able to do because integers are copied.
    let some_string: String = gives_ownership();
    println!("{}", some_string);
    let take_give = "takes_and_gives_back_ownership";
    let string_take_give_back = takes_and_gives_back_ownership(take_give);
    println!("string_take_give_back: {}", string_take_give_back);
    let mut reference_string = String::from("references");
    references(&mut reference_string);
    let slice = "Hello text";
    let frst_word = first_word(slice);
    println!("{}", frst_word);
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[0..2];
}

fn my_add_function(x: i32, y: i32) -> i32 {
    x + y
}

fn take_ownership(s1: String) {
    println!("take_ownership: {}", s1)
}

fn make_copy(some_integer: i32) {
    println!("make copy integer: {}", some_integer)
}

fn gives_ownership() -> String {
    String::from("some_string_from_gives_ownership_function")
}

fn takes_and_gives_back_ownership(string: &str) -> &str {
    string
}

fn references(s: &mut String) {
    // s.len(); // by default references are immutable;
    println!("Hello from reference function");
    s.push_str("world")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
