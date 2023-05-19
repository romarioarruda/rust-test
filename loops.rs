fn tabuada_com_while(multiplicador: u8, mut contador: u8) {
    while contador < 10 {
        contador += 1;

        if contador == 5 {
            println!("Ignorando o {}", contador);
            continue;
        }

        let resultado = multiplicador * contador;

        println!("{} x {} = {}", contador, multiplicador, resultado);
    }
}

fn tabuada_com_for(adicao: u8) {
    for contador in 1..11 {
        if contador == 5 {
            println!("Ignorando o {}", contador);
            continue;
        }

        let resultado = adicao + contador;
        println!("{} + {} = {}", contador, adicao, resultado);
    }
}

fn main() {
    tabuada_com_while(5, 0);
    println!("====================");
    tabuada_com_for(1);
}