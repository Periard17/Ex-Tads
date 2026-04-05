fn rpn(expr: &str) -> f64 {
    let mut pilha: Vec<f64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a + b);
            }
            "-" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a - b);
            }
            "*" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a * b);
            }
            "/" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a / b);
            }
            num => {
                pilha.push(num.parse::<f64>().unwrap());
            }
        }
    }

    pilha.pop().unwrap()
}
