use std::io;

fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    print!("Type of <guess> variable: ");
    type_of(&guess);

    let x = 2.0; // f64
    print!("Type of <x> variable: ");
    type_of(&x);

    let y: f32 = 3.0; // f64
    print!("Type of <y> variable: ");
    type_of(&y);

    // addition
    let sum = 5 + 10;
    print!("Result of addition: {}. Type: ", sum);
    type_of(&sum);

    // subtraction
    let difference = 95.5 - 4.3;
    print!("Result of subtraction: {}. Type: ", difference);
    type_of(&difference);

    // multiplication
    let product = 4 * 30;
    print!("Result of multiplication: {}. Type: ", product);
    type_of(&product);

    // division
    let quotient = 56.7 / 32.2;
    print!("Result of quotient division: {}. Type: ", quotient);
    type_of(&quotient);
    let truncated = -5 / 3; // Results in -1
    print!("Result of truncated division: {}. Type: ", truncated);
    type_of(&truncated);

    // remainder
    let remainder = 43 % 5;
    print!("Result of remainder: {}. Type: ", remainder);
    type_of(&remainder);

    let t = true;
    print!("Type of implicit <t> variable: ");
    type_of(&t);

    let f: bool = false; // with explicit type annotation
    print!("Type of explicit <f> variable: ");
    type_of(&f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Chars: {c}, {z}, {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    print!("Tuple contains: {:?}. Types of <tup> variable data: ", tup);
    type_of(&tup);

    let (_x, y, _z) = tup;
    println!("The value of <y> is: {}", y);
    println!("Elements of variable <tup>: first {}, second {}, third {}.", tup.0, tup.1, tup.2);

    let empty_tuple = ();
    print!("Type of <empty_tuple> variable: ");
    type_of(&empty_tuple);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array <a> contains: {:?}.", a);
    println!("Elements of array <a>: first {}, second {}, third {}, fourth {}, fifth {}",
                a[0], a[1], a[2], a[3], a[4]);
    let a1 = [3; 5];
    println!("Array <a1> contains: {:?}.", a1);

    println!("Please enter an array index (0-4).");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
