enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn main() {
    println!("É fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Segunda));
    println!("É fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Terca));
    println!("É fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Quarta));
    println!("É fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Quinta));
    println!("É fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Sexta));
    println!("É fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Sabado));
    println!("É fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Domingo));
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}