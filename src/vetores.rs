fn main() {
    let mut notas: Vec<f32> = Vec::new();
    notas.push(10.0);
    notas.push(9.0);
    notas.push(8.3);

    println!("{:?}", notas);

    let notas_v2: Vec<f32> = vec![6.4, 7.0, 8.2, 9.0, 10.0]; //vec! is a macro for Vec::new()

    println!("{:?}", notas_v2);
}