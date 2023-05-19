const PI: f32 = 3.14;

fn printnumber() {
    let number = 300;
    println!("Number: {number}, size: {}", std::mem::size_of_val(&number));
}

fn main() {
    printnumber();

    let guess: u32 = "111".parse().expect("Not a number!");
    println!("Guess: {guess}, size: {}", std::mem::size_of_val(&guess));

    println!("PI: {PI}");
}