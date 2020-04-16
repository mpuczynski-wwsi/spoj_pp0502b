/**
 P0502B - Tablice
Odwróć kolejność elementów w tablicy.

Wejście
Najpierw liczba testów t (t ≤ 100). Następnie dla każdego testu liczba n (n ≤ 100) i n liczb oddzielonych spacjami.

Wyjście
Dla każdego testu n liczb w porządku odwrotnym niż na wejściu.

Przykład
Wejście:
2
7 1 2 3 4 5 6 7
3 3 2 11

Wyjście:
7 6 5 4 3 2 1
11 2 3
 */


fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("err");

    let t:u8 = buffer.trim().parse().unwrap();
    for _ in 0..t {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("err");

        let list: Vec<&str> = buffer.split(' ').map(|x| x.trim()).collect();
        let reversed_list: Vec<&str> = list[1..].into_iter().rev().map(|&s| s).collect();

        println!("{}", reversed_list.join(" "));

    }
}
