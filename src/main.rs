fn main(){
    println!("¿Cuál es tu nombre?: ");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    
    println!("Elige tu píldora, tienes dos opciones, la pildora azul(1) o la pildora roja(2): ");
    let mut pildora: String = String::new();
    std::io::stdin().read_line(&mut pildora).unwrap();
    let pildora_int : u8 = pildora.trim().parse().unwrap();

    if pildora_int == 1{
        println!("Muy bien, {}. Sígueme, elegiste la pildora roja", nombre);
    } else if pildora_int == 2{
        println!("Como prefieras {}. No nos veremos nunca más, elegiste la pildora azul", nombre);
    } else{
        println!("Has elegido mal")
    }
}