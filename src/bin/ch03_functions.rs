fn main() {
    another_function(3);
    another_function(five());
    another_function(plus_one(five()));
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
