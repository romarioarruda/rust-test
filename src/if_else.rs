fn main() {
    let idade = 12;
    let eh_maior = idade >= 18;

    let mensagem = if eh_maior { "Tem permissão" } else { "Não tem permissão" };

    println!("{}", mensagem)
}