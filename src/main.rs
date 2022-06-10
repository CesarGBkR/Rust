fn main() {
    println!("Por favor introduce tu edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    let edad_int : u8 = edad.trim().parse().unwrap();

    if edad_int >= 18 {
        println!("Puedes entrar a la discoteca");
    } if
        edad_int >= 90 {
            println!("pero tal vez no deberías entrar.");
        } else {
        println!("No puedes entrar a la discoteca, eres menor de edad");
    }

    println!("Tienes {} años", edad_int);
}