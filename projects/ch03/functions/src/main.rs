fn five() -> i32 {
    5
}

fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = five();
        plus_one(x)
    };

    println!("The value of <y> is: {y}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}