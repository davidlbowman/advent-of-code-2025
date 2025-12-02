fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The new value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours.");

    let shadow = 5;
    let shadow = shadow + 1;

    {
        let shadow = shadow * 2;
        println!("The value of x in the inner scope is: {shadow}");
    }

    println!("The avlue of x in the outer scope is: {shadow}");
}
