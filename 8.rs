fn balanceado(expr: &str) -> bool {
    let mut pilha = Vec::new();

    for c in expr.chars() {

        match c {

            '(' | '[' | '{' => pilha.push(c),

            ')' => {
                if pilha.pop() != Some('(') { return false }
            }

            ']' => {
                if pilha.pop() != Some('[') { return false }
            }

            '}' => {
                if pilha.pop() != Some('{') { return false }
            }

            _ => ()
        }
    }

    pilha.is_empty()
}
