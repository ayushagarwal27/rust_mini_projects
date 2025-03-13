fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let mut x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    // x = 6;
    {
        let x = x * 2;
        println!("The value of inner scope of x: {x}")
    }
    println!("The value of x is: {x}");

    let tup = (500, 4.5, true);
    let (a, b, c) = tup;
    println!("{a} {b} {}", tup.2);

    let arr = [3; 5];

    println!("{:?}", arr);

    let z = {};

    // let x = (let y = 6);
}
