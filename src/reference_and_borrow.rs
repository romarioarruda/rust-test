fn main() {
    let minha_string = String::from("Romário"); //aloca na memória heap

    meu_print(&minha_string); //Passo a referência da string, ao invés do ponteiro

    println!("Main print: {}", minha_string);
}

//função Borrow => pega um valor "emprestado" através da referência de memória
fn meu_print(string: &String) {
    println!("Meu print: {}", string);
}