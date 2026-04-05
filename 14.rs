use std::collections::VecDeque;

fn palindromo(texto: &str) -> bool {

    let mut deque = VecDeque::new();

    for c in texto.to_lowercase().chars() {

        if c != ' ' {
            deque.push_back(c);
        }
    }

    while deque.len() > 1 {

        if deque.pop_front() != deque.pop_back() {
            return false;
        }
    }

    true
}
