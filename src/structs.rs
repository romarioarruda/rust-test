struct Titular {
    nome: String
}

struct Conta {
    titutal: Titular,
    saldo: f64
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor
    }

    fn depositar(&mut self, valor: f64) {
        self.saldo += valor
    }
}

fn main() {
    let mut conta: Conta = Conta{
        titutal: Titular { 
            nome: String::from("Romário Arruda")
        },
        saldo: 3000000.0
    };

    conta.sacar(1000.0);
    conta.depositar(500.0);

    println!(
        "Titular: {}, \nSaldo: {}",
        conta.titutal.nome, conta.saldo
    );
}