fn main() {
    let lista: [f32; 5] = [8.0, 7.0, 9.3, 8.5, 10.0];

    println!("Tamanho da Lista: {} itens", lista.len());

    for nota in lista {
        println!("Nota: {}", nota)
    }
}