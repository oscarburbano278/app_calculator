use std::io;

fn main() {
    
    println!("Por favor, ingresa un número segun la operacion que desee realizar:");

    println!("1 = suma");
    println!("2 = resta");
    println!("3 = multiplicacion");
    println!("4 = Division");
    

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la línea");

    let numero:i16 = input.trim().parse().expect("no se puede convertir a numero ");

    match numero {
        1 => println!("suma"),

        2 => println!("resta"),
        3 => println!("multiplicacion"),
        4 => println!("divicion"),
        _ => println!("numero invalido"), 
    }

    println!("ingrese el primer numero a operar");

    let mut operar1 = String::new();
    io::stdin().read_line(&mut operar1).expect("Error al leer la línea");

    let numero1:i16 = operar1.trim().parse().expect("no se puede convertir a numero ");



    println!("ingrese el segundo numero a operar");

    let mut operar2 = String::new();
    io::stdin().read_line(&mut operar2).expect("Error al leer la línea");

    let numero2:i16= operar2.trim().parse().expect("no se puede convertir a un numero");

    let mut resultado = 0;

    if numero == 1 {
        resultado = numero1 + numero2;
    }
    else if numero == 2{
        resultado = numero1 - numero2;
    }
    else if numero == 3 {
        resultado = numero1 * numero2;
    }
    else if numero == 4 {
        resultado = numero1 / numero2;
    }     
    
    println!("el resultado es de {} y {} es igual a {}", numero1, numero2, resultado)


 }



