use std::collections::VecDeque;

fn round_robin(mut processos: Vec<(i32,i32)>, quantum: i32) {

    let mut fila: VecDeque<(i32,i32)> = VecDeque::new();

    for p in processos {
        fila.push_back(p);
    }

    let mut tempo = 0;

    while let Some((id, mut restante)) = fila.pop_front() {

        if restante > quantum {

            tempo += quantum;
            restante -= quantum;

            fila.push_back((id, restante));

        } else {

            tempo += restante;
            println!("Processo {} terminou no tempo {}", id, tempo);
        }
    }
}
