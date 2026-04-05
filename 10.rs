use std::collections::VecDeque;

fn fila_banco() {
    let mut fila: VecDeque<i32> = VecDeque::new();

    for cliente in 1..6 {
        fila.push_back(cliente);
    }

    while let Some(cliente) = fila.pop_front() {
        println!("Atendendo cliente {}", cliente);
    }
}
