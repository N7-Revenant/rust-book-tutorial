const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn TypeOf<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    let mut x = 5;
    println!("The value of mutable x is: {}", x);
    x = 6;
    println!("The value of mutable x is: {}", x);

    let x = 5;

    let x = x +1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of shadowed x is: {x}");

    let spaces = "    ";
    print!("Type of spaces variable: ");
    TypeOf(&spaces);
    let spaces = spaces.len();
    print!("Type of spaces variable: ");
    TypeOf(&spaces);
}
