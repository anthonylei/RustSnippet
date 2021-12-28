fn main() {
    let mut x = 5;
    let y = 9;
    println!("The value of x is: {}, {}", x, y);
    x = 6;
    let y = "Six"; // 
    println!("The value of x is: {}, {}", x, y);

    // const SUVSCRIBER_COUNT: u32 = 100_000;

    // integers
    let a = 98_200; // Decimal
    let b = 0xff; // hex
    let c = 0o77; // Octal
    let d = 0b11111_0000;  // binary
    let e = b'A';   // Byte (u8 only)

    println!("{},{},{},{},{}",a,b,c,d,e);

    // Float-point numbers
    let f = 2.0;
    let g: f32 = 3.0;
    println!("{},{}",f, g);

    // addition
    let sum = 5 + 10;
    // substraction
    let diff = 86.2 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    println!("{},{},{},{},{}",sum, diff, product, quotient, remainder);
    // Booleans
    let t = true;
    let f: bool = false;

    println!("{},{}",t, f);
    // Character - some other unicode char
    let c = 'z';

    println!("{}", c);
    // Compond Types
    // Tuple
    let tup = ("Lets get rusty!", 100_000);
    let (channel, sub_count) = tup; 
    let sub_count2 = tup.1; // assign sub_count with first tup 

    // let error_codes = [200, 404, 500];
    // let not_found = error_codes[1];
    // let byte = [0, 8];

    println!("{}", channel);
    println!("{}", sub_count2);
    println!("{}", sub_count);
    // println!("{}", error_codes);
    // println!("{}", not_found);
    // println!("{}", byte);

    my_function(1, 2);
}

// Rust use snake cases
fn my_function(x: i32, y: i32) -> i32 {
    let sum = x + y; 
    
    if sum < 10 {
        println!("first condition was true");
    } else if sum < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    // Forever loop unless break
    loop {
        println!("again!");
        break;
    }

    // Loop can return things
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("{}", result);

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("While Loop Done!");

    // For loop
    let a_list = [10, 20, 30, 40, 50];

    for element in a_list.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }

    /*
     tarnary operation
    */
    let number = if true {5} else {6};
    println!("{}", number);
    sum // when return as the last line, we omit the ;
    // return sum;
}