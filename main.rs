fn ordemCrescente(lista: &mut Vec<i64>) {
    let n = lista.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && lista[j - 1] > lista[j] {
            lista.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn imprimirLista(lista: &Vec<i64>){
    println!("{:?}", lista);
}

fn executar_estrategia(lista: &mut Vec<i64>, estrategia: fn(&mut Vec<i64>)) {
    estrategia(lista);
}

fn main() {
    let mut lista: Vec<i64> = vec![2,3,1,5,4];
    executar_estrategia(&mut lista,ordemCrescente);
    imprimirLista(&lista);
    
    
}
