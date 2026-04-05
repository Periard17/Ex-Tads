fn remover_pares() {
    let mut v = vec![1,2,3,4,5,6,7,8];

    let mut i = 0;

    while i < v.len() {
        if v[i] % 2 == 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }

    println!("{:?}", v);
}
