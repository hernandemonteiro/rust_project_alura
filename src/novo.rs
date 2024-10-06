/// Importado - Função de exemplo
///
/// Está função é publica e pode ser chamada de qualquer lugar
///
/// Ela não retorna nada, por isso o tipo de retorno é ()
///
/// A unica função dela é mostrar algo na tela
///
/// # Exemplo
///
///
/// ```
/// importado();
///
/// ```
pub(crate) fn importado() -> () {
    println!("Arquivo importado!");
}

#[allow(dead_code)]
enum DiasDaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

pub(crate) fn types_in_rust() {
    let notas = [10f32, 9.5, 8f32, 6.0];
    notas.iter().for_each(|nota| {
        println!("Nota: {}", nota);
    });

    let notas2 = [6.5; 4];

    for nota in notas2.iter() {
        println!("Nota 2: {}", nota);
    }

    matriz();
    println!(
        "\nFim de semana: {}\n",
        eh_fim_de_semana(DiasDaSemana::Domingo)
    );

    cores();

    println!("{:?}", Some(String::from("Hernande")));

    vectors();
    using_structs();
}

fn matriz() {
    let matriz: [[i8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for linha in matriz.iter() {
        for coluna in linha.iter() {
            print!("{} ", coluna);
        }
        println!();
    }

    println!("Matriz[0][1] = {:?}", matriz[0]);
}

fn eh_fim_de_semana(dia: DiasDaSemana) -> bool {
    match dia {
        DiasDaSemana::Sabado | DiasDaSemana::Domingo => true,
        _ => false,
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RgbColor(u8, u8, u8),
    CymkColor {
        cyan: u8,
        yellow: u8,
        magenta: u8,
        black: u8,
    },
}

fn cores() {
    let c = Color::CymkColor {
        cyan: 1,
        yellow: 1,
        magenta: 1,
        black: 1,
    };

    match c {
        Color::Red => println!("Vermelho"),
        Color::Blue => println!("Azul"),
        Color::Green => println!("Verde"),
        Color::RgbColor(0, 0, 0) => println!("Preto"),
        Color::RgbColor(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
        Color::CymkColor {
            cyan,
            yellow,
            magenta,
            black,
        } => {
            println!("Cymk: {}, {}, {}, {}", cyan, yellow, magenta, black)
        }
    }
}

fn vectors() {
    let mut vector = vec![1, 2, 3, 4, 5];

    println!("Vector = {:?}", vector);

    vector.push(5);

    println!("Vector = {:?}", vector);
}

struct Pessoa {
    nome: String,
    idade: u8,
}

impl Pessoa {
    fn new(&mut self) -> Pessoa {
        Pessoa {
            nome: self.nome.clone(),
            idade: self.idade - 1,
        }
    }
}

fn using_structs() {
    let mut p: Pessoa = Pessoa {
        nome: String::from("Hernande"),
        idade: 25,
    };

    p = p.new();

    println!("Nome: {}\nIdade: {}", p.nome, p.idade);
}
