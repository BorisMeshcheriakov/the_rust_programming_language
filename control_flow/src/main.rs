fn main() {
    let number = 5;

    if number < 5 {
        println!("true")
    } else {
        println!("false")
    }

    // loop {
    //     println!("again")
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };

    println!("Result is {result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for el in a {
        println!("the value is: {el}")
    }
}
