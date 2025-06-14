fn executar_estrategia(mut lista: Vec<i64>, estrategia: fn(&mut Vec<i64>)) {
    estrategia(&mut lista);
    println!("Resultado: {:?}", lista);
}

fn main() {
    let numeros = vec![5, 2, 8, 5, 2, 9, 1, 0, 4, 4];
    println!("Lista original: {:?}", numeros);

}
