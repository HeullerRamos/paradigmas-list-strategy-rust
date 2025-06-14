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
use std::collections::HashSet;//para remover duplicatas
fn removeDuplicatas(lista: &mut Vec<i64>) {
    // 1. Cria um HashSet para rastrear os elementos que já vimos.
    let mut vistos = HashSet::new();

    // 2. Usa o método `retain` para manter apenas os elementos únicos.
    lista.retain(|elemento| vistos.insert(*elemento));
}
fn imprimirLista(lista: &Vec<i64>){
    println!("{:?}", lista);
}

fn executar_estrategia(lista: &mut Vec<i64>, estrategia: fn(&mut Vec<i64>)) {
    estrategia(lista);
}

fn main() {
    let mut lista: Vec<i64> = vec![2,3,1,5,4,1,3,5,3,1,34,4,12,3];
    executar_estrategia(&mut lista,ordemCrescente);
    executar_estrategia(&mut lista,removeDuplicatas);
    imprimirLista(&lista);
    
    
}
