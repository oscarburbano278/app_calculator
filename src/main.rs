use std::io;

fn main() {
    loop {
        show_menu();
        let number: i16 = get_input_from_keyboard(
            "Por favor, ingresa un número segun la operacion que desee realizar:".to_string(),
        );
        let mut result = 0;
        let mut number_a = 0;
        let mut number_b = 0;

        match number {
            1 => {
                println!("suma");
                number_a = get_input_from_keyboard("Ingrese el primer numero".to_string());
                number_b = get_input_from_keyboard("Ingrese el segundo valor".to_string());
                result = number_a + number_b;
            }
            2 => {
                println!("resta");
                number_a = get_input_from_keyboard("Ingrese el primer numero".to_string());
                number_b = get_input_from_keyboard("Ingrese el segundo valor".to_string());
                result = number_a - number_b;
            }
            3 => {
                println!("multiplicacion");
                number_a = get_input_from_keyboard("Ingrese el primer numero".to_string());
                number_b = get_input_from_keyboard("Ingrese el segundo valor".to_string());

                result = number_a * number_b;
            }
            4 => {
                println!("División");
                number_a = get_input_from_keyboard("Ingrese el primer numero".to_string());
                number_b = get_input_from_keyboard("Ingrese el segundo valor".to_string());
                result = number_a / number_b;
            }
            5 => {
                println!("...Saliendo");
                break;
            }
            _ => println!("Opps"),
        }

        show_result(number_a, number_b, result);
    }
}

fn show_result(number_a: i16, number_b: i16, result: i16) {
    println!(
        "el resultado es de {} y {} es igual a {}",
        number_a, number_b, result
    );
    println!("");
}
fn show_menu() {
    println!("1 = suma");
    println!("2 = resta");
    println!("3 = multiplicacion");
    println!("4 = Division");
    println!("5 = Salir");
}

fn get_input_from_keyboard(info: String) -> i16 {
    println!("{}", info);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    let input_number: i16 = input
        .trim()
        .parse()
        .expect("no se puede convertir a numero ");

    input_number
}
