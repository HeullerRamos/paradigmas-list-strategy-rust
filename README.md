# Estratégias para operar listas em Rust

## Funcionalidades

O programa contém as seguintes funções principais:

### Função `executar_estrategia`

A função **`executar_estrategia`** permite executar uma estratégia em um vetor de inteiros. Ela recebe uma lista mutável e uma assinatura de uma função como argumentos, e executa essa função sobre o vetor fornecido.

#### Assinatura
```rust
fn executar_estrategia(lista: &mut Vec<i64>, fn(&mut Vec<i64>));
```

### 1. `ordemCrescente` (Ordenação por Insertion Sort)
A função **`ordemCrescente`** é responsável por ordenar um vetor de números inteiros em ordem crescente usando o algoritmo **Insertion Sort**. Esse algoritmo percorre o vetor e insere cada elemento na posição correta dentro da parte já ordenada.

**Assinatura:**
```rust
fn ordemCrescente(lista: &mut Vec<i64>)
```
### 2. `filtrar_pares` (Filtragem de números pares únicos)

A função **`filtrar_pares`** recebe um vetor mutável de inteiros e filtra apenas os números pares, removendo quaisquer duplicatas. O vetor é modificado in-place para conter apenas os valores pares distintos.

**Assinatura:**

```rust
fn filtrar_pares(lista: &mut Vec<i64>)
