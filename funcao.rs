fn soma(a:i32, b:i32) -> i32 {
    return a + b;
}

fn subtracao(a:i32, b:i32) -> i32 {
    return a - b;
}

fn main() {
    println!("Soma: {}", soma(10, 10));
    println!("Subtração: {}", subtracao(3, 10));
}