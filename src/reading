fn main() {
    println!("Introduce tu nombre: ");
    
    // Obtener nombre por consola
    let mut nombre : String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // Obtener la edad por consola
    println!("Introduce tu edad: ");
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    
    // Comvertir esa edad en numero

    let edad_int : u8 = edad.trim().parse().unwrap();

    println!("Hola, bienvenidx {} de {} años", nombre, edad_int);
}