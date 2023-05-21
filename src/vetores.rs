fn main() {
    let mut notas: Vec<f32> = Vec::new();
    notas.push(10.0);
    notas.push(9.0);
    notas.push(8.3);

    println!("{:?}", notas);

    let notas_v2: Vec<f32> = vec![6.4, 7.0, 8.2, 9.0, 10.0]; //vec! is a macro for Vec::new()

    println!("{:?}", notas_v2);

    println!("Nota 1: {}", match notas_v2.get(0) {
        Some(n) => *n,
        None => 0.0
    });

    println!("Nota 3: {}", match notas_v2.get(2) {
        Some(n) => *n,
        None => 0.0
    });

    println!("Nota 10: {}", match notas_v2.get(10) {
        Some(n) => *n,
        None => 0.0
    });

    println!("=======================");
    for nota in notas {
        println!("Nota: {}", nota)
    }
    println!("=======================");
}