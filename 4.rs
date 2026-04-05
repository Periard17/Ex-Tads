fn mesclar_ordenado() {
    let mut a = vec![1,3,5,7];
    let mut b = vec![2,4,6,8];

    a.extend(b);
    a.sort();

    println!("{:?}", a);
}
