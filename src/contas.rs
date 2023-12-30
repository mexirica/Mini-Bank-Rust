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

    pub fn depositar(&mut self, valor: f32) -> () {
        self.saldo += valor;
        println!("Depósito bem sucedido, seu saldo é de {:?}", self.saldo)
    }

    pub fn saque(&mut self, valor: f32) -> () {
        if self.saldo < valor {
            println!("Valor maior que o saldo");
            return;
        }
        self.saldo -= valor;
        println!("Saque bem sucedido, seu saldo é de {:?}", self.saldo)
    }

    // CPF somente para ilustracao
    pub fn transferencia(&mut self, mut valor: f32, cpf_destino: String, moeda: Moeda) -> () {
        match self.moeda {
            Moeda::Real => {
                match moeda {
                    Moeda::Real => {}
                    Moeda::Dolar => { valor *= 5.4 }
                    Moeda::Euro => { valor *= 5.0 }
                }
            }
            Moeda::Dolar => {
                match moeda {
                    Moeda::Real => { valor *= 0.2 }
                    Moeda::Dolar => {}
                    Moeda::Euro => { valor *= 1.1 }
                }
            }
            Moeda::Euro => {
                match moeda {
                    Moeda::Real => { valor *= 0.18 }
                    Moeda::Dolar => { valor *= 0.9 }
                    Moeda::Euro => {}
                }
            }
        }
        println!("{:?}", valor);
        if self.saldo < valor {
            println!("Valor maior que o saldo");
            return;
        }
        self.saldo -= valor;
        println!("Transferencia bem sucedido, seu saldo é de {:?}", self.saldo)
    }
}