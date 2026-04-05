fn historico_navegador() {
    let mut back: Vec<String> = Vec::new();
    let mut forward: Vec<String> = Vec::new();

    back.push("google.com".to_string());
    back.push("youtube.com".to_string());

    // voltar
    if let Some(pagina) = back.pop() {
        forward.push(pagina);
    }

    println!("Back: {:?}", back);
    println!("Forward: {:?}", forward);
}
