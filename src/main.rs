fn main() {
    println!("Introduce tu nombre: ");
    
    let mut nombre : String = String::new();

    std::io::stdin().read_line(&mut nombre).unwrap();

    println!("Hola, bienvenidx {}", nombre);
}