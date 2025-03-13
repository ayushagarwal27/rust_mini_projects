use core::num;

fn main() {
    // loop {
    //     println!("again")
    // }

    //   --------------------------

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    //   --------------------------

    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count = {count}");

    //   --------------------------

    let mut number: i32 = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    //   --------------------------

    let arr = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < arr.len() {
    //     println!("the value is: {}", arr[index]);
    //     index += 1;
    // }

    for el in arr {
        println!("the value is: {el}")
    }
    //   --------------------------

    for el in (1..4).rev(){
        println!("{el}");
    }
    println!("LIFTOFF!");
}
