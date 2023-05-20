fn main() {
    match result() {
        Ok(msg) => println!("Msg: {}", msg),
        Err(err) => panic!("Erro: {}", err)
    };
}

fn result() -> Result<String, u16> {
    let condition: bool = false;

    if condition {
        Ok(String::from("Request successfully"))
    } else {
        Err(400)
    }
}