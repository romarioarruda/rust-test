fn find_proposito(linguagem: &str) -> &str {
    return match linguagem {
        "PHP" | "php" => "Web development",
        "Python" => "Machine Learning",
        _ => "Desconhecido"
    };
}

fn main() {
    println!("PHP => {}", find_proposito("PHP"));
    println!("php => {}", find_proposito("php"));
    println!("Python => {}", find_proposito("Python"));
    println!("Unknown => {}", find_proposito(""));
}