use std::collections::VecDeque;

fn buffer(cap: usize) {

    let mut fila: VecDeque<String> = VecDeque::with_capacity(cap);

    let mensagens = vec!["A","B","C","D","E"];

    for m in mensagens {

        if fila.len() == cap {
            fila.pop_front();
        }

        fila.push_back(m.to_string());
    }

    println!("{:?}", fila);
}
