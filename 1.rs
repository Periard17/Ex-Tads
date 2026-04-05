fn inverter_vec() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut invertido = Vec::new();

    while let Some(valor) = v.pop() {
        invertido.push(valor);
    }

    println!("Invertido: {:?}", invertido);
}
