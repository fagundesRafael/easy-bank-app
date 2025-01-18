use std::io;

fn secret_option() {
    println!("\nOpção secreta!\nParabéns você descobriu a opção secreta!\nVocê é gay! xD");
    std::process::exit(0);
}

fn insert_value(user_bank_balance: &mut f64) {
    println!("\nInforme o valor do depósito (somente números):");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Falha ao ler o valor");

    let value: f64 = loop {
        match value.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("\nValor inválido. Digite apenas números.");
                value.clear();
                io::stdin()
                    .read_line(&mut value)
                    .expect("Falha ao ler o valor");
            }
        }
    };

    println!("\nValor depositado: R$ {}", value);

    *user_bank_balance += value;
    println!("\nSaldo atual: R$ {}", user_bank_balance);
}

fn withdraw_value(user_bank_balance: &mut f64) {
    println!("\nInforme o valor do saque (somente números):");
    let mut value: String = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Falha ao ler o valor");

    let value: f64 = loop {
        match value.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("\nValor inválido. Digite apenas números.");
                value.clear();
                io::stdin()
                    .read_line(&mut value)
                    .expect("Falha ao ler o valor");
            }
        }
    };

    println!("\nValor do saque: R$ {}", value);
    if value > *user_bank_balance {
        println!("\nSaldo insuficiente para realizar o saque.");
        println!("\nSaldo atual: R$ {}", user_bank_balance);
        return;
    } else {
        *user_bank_balance -= value;
    }

    println!("\nSaldo atual: R$ {}", user_bank_balance);
}

fn show_user_and_balance(user_name: &mut String, user_bank_balance: &f64) {
    println!("Usuário: {}", user_name);
    println!("Saldo: {}", user_bank_balance);
}

fn rename_user(user_name: &mut String) {
    println!("Renomear usuário atual: {}", user_name);
    println!("Digite o novo nome para atualizar:");
    loop {
        let mut new_name = String::new();
        io::stdin()
            .read_line(&mut new_name)
            .expect("Falha ao ler o novo nome");
        new_name = new_name.trim().to_string();
        if new_name.len() > 0 {
            *user_name = new_name;
            println!("Usuário atualizado com sucesso!");
            break;
        } else {
            println!("Nome inválido. Digite um nome válido.");
        }
    }
}

fn exit_program(user_name: &mut String, user_bank_balance: &mut f64) {
    println!("Encerrando o programa...");
    println!(
        "\nObrigado por acessar nosso sistema!\nUsuário(a) {}. \nSeu saldo atual é de R$ {}",
        user_name, user_bank_balance
    );
    std::process::exit(0);
}

fn main() {
    let mut user_name = String::new();
    let mut user_bank_balance = 0.0;

    println!("\nIniciando sistema...");
    loop {
        if user_name.len() == 0 {
            println!("\nDigite o seu nome");
            io::stdin()
                .read_line(&mut user_name)
                .expect("Falha ao ler o nome");
            user_name = user_name.trim().to_string();
        } else {
            println!("\nEscolha uma opção:");
            println!("1 - Depositar");
            println!("2 - Sacar");
            println!("3 - Monstrar saldo e usuário");
            println!("4 - Renomear usuário");
            println!("5 - Sair");
            let mut option = String::new();

            io::stdin()
                .read_line(&mut option)
                .expect("Falha ao ler a opção");
            option = option.trim().to_string();
            match option.as_str() {
                "0" => secret_option(),
                "1" => insert_value(&mut user_bank_balance),
                "2" => withdraw_value(&mut user_bank_balance),
                "3" => show_user_and_balance(&mut user_name, &user_bank_balance),
                "4" => rename_user(&mut user_name),
                "5" => exit_program(&mut user_name, &mut user_bank_balance),
                _ => println!("\nOpção inválida!\nPor favor, escolha uma das opções válidas."),
            }
        }
    }
}
