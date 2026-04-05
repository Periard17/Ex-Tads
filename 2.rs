use std::collections::HashMap;

fn contar_ocorrencias() {
    let frase: Vec<char> = "estrutura de dados".chars().collect();
    let mut contador = HashMap::new();

    for c in &frase {
        *contador.entry(c).or_insert(0) += 1;
    }

    println!("{:?}", contador);
}
