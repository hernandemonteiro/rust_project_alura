mod novo;

use novo::{importado, types_in_rust};
mod constants;
use constants::{PI, VAR_GLOBAL};

fn teste(age: i8) {
    println!("Idade: {}", age);
}

fn sum(a: i8, b: i8) -> i8 {
    return a + b;
}

fn main() {
    // variavel imutável (não pode ser alterada)
    // tipo da variável é inferido pelo compilador
    let nome: &str = "Hernande Monteiro";
    // let mut nome: &str = "Hernande Monteiro";
    // para mudar o valor de uma variável, ela deve ser mutável

    println!("Hello, world!");
    println!("Olá {}!", nome);
    println!("Valor de PI: {}", PI);
    println!("Valor da variável global: {}", VAR_GLOBAL);
    importado();
    teste(26);
    println!("Soma: {}", sum(10, 20));

    let arr: &[i8] = &[1, 2, 3, 4, 5];

    for index in arr.iter().enumerate() {
        println!("Elemento => {} => Valor => {}", index.0, index.1);
    }

    let arr_length: usize = arr.len();
    for i in 1..arr_length {
        println!("i = {}", i);
    }

    for i in 1..50 {
        if i >= 31 {
            break;
        } else {
            println!("i2 = {}", i);
        }
    }

    let mut contador: i16 = 0;
    while contador <= 10 {
        println!("contador = {}", contador);
        contador += 1;
    }

    match arr.get(0) {
        Some(&1) => println!("Valor 1"),
        Some(&2) => println!("Valor 2"),
        Some(&3) => println!("Valor 3"),
        Some(&4) => println!("Valor 4"),
        Some(&5) => println!("Valor 5"),
        _ => println!("Valor não encontrado"),
    }

    let proposito = match arr.get(1) {
        Some(&1) => "Valor 1",
        Some(&2) => "Valor 2",
        Some(&3) => "Valor 3",
        Some(&4) => "Valor 4",
        Some(&5) => "Valor 5",
        _ => "Valor não encontrado",
    };

    println!("Propósito: {}", proposito);

    owner();

    pattern_matching();

    types_in_rust();

    erro();
}

fn owner() {
    let s1 = String::from("hello");
    rouba(s1);
    // para evitar o erro abaixo podemos usar um s1.clone() acima
    // println!("{}, world!", s1); // erro
}

fn rouba(s: String) {
    println!("{}", s);
}

fn pattern_matching() {
    for x in 0..=20 {
        println!(
            "{}: {}",
            x,
            match x {
                0 | 1 => "Pouco",
                2..=4 => "Algo",
                5..=10 => "Muito",
                _ if x % 2 == 0 => "Par",
                _ => "Muitíssimo",
            }
        )
    }
}

fn erro() {
    match result() {
        Ok(value) => println!("Sucesso: {}", value),
        Err(e) => println!("Erro: {}", e),
    }
}

fn result() -> Result<String, i32> {
    Ok(String::from("Tudo certo"))
}
