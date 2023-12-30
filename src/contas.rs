pub enum Moeda {
    Real,
    Dolar,
    Euro,
}

pub struct Conta {
    titular: String,
    cpf: String,
    moeda: Moeda,
    saldo: f32,
}
impl Conta {
    pub fn new(titular: String, cpf: String, mut moeda: Moeda, saldo: f32) -> Conta {
        Conta { titular, cpf, moeda, saldo }
    }

    pub fn depositar(&mut self, valor: f32) {
        self.saldo += valor;
        println!("Depósito bem sucedido, seu saldo é de {:?}", self.saldo)
    }

    pub fn saque(&mut self, valor: f32) {
        if self.saldo < valor {
            println!("Valor maior que o saldo");
            return;
        }
        self.saldo -= valor;
        println!("Saque bem sucedido, seu saldo é de {:?}", self.saldo)
    }

    // CPF somente para ilustracao
    pub fn transferencia(&mut self, valor: f32, cpf_destino: String, moeda_destino: Moeda) {
        let taxa = self.calcula_taxa(moeda_destino);
        let valor_convertido = valor * taxa;

        if self.saldo < valor_convertido {
            println!("Valor maior que o saldo");
            return;
        }

        self.saldo -= valor_convertido;
        println!("Transferência bem-sucedida, seu saldo é de {:?}", self.saldo);
    }

    fn calcula_taxa(&self, moeda_destino: Moeda) -> f32 {
        match (&self.moeda, moeda_destino) {
            (Moeda::Real, Moeda::Dolar) => 5.4,
            (Moeda::Real, Moeda::Euro) => 5.0,
            (Moeda::Dolar, Moeda::Real) => 0.2,
            (Moeda::Dolar, Moeda::Euro) => 1.1,
            (Moeda::Euro, Moeda::Real) => 0.18,
            (Moeda::Euro, Moeda::Dolar) => 0.9,
            _ => 1.0, // Mesma moeda, taxa 1.0
        }
    }
}