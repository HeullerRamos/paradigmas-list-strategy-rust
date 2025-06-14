fn filtrar_pares(vetor: &mut Vec<i64>) {
    let mut pares = Vec::new();

    for valor in vetor.iter() {
        if valor % 2 == 0 {
            pares.push(*valor);
        } 
    }

    *vetor = pares;
}

fn executar_estrategia(mut lista: Vec<i64>, estrategia: fn(&mut Vec<i64>)) {
    estrategia(&mut lista);
    println!("Resultado: {:?}", lista);
}

fn main() {
    let numeros = vec![5, 2, 8, 5, 2, 9, 1, 0, 4, 4];
    println!("Lista original: {:?}", numeros);

}
