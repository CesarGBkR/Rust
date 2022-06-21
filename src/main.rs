use regex::Regex;

fn main(){
    println!("Hola");

//     // Regex
    let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    
    // Traer datos del usuario
    println!("Por favor introduce tu expresion: ");
    let mut expresion = String::new();
    std::io::stdin().read_line(&mut expresion).unwrap();

    // proceso(expresion, re_mult, "*")
    // proceso(expresion, re_add, "+")
    // proceso(expresion, re_add, "+")
    proceso(expresion, re_add, "+")
}

fn proceso(mut expresion: String, operation: Regex, operador: &str){
    loop{
        let caps = operation.captures(expresion.as_str());
        if caps.is_none(){
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let result = match operador {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };
        expresion = expresion.replace(cap_expression, &result.to_string())
    };
    println!("{}", expresion)
}