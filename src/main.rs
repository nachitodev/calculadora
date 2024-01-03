use text_io::read;
fn main() {
    loop {
// ToDo saber como poner los inputs dentro o seguido de los prints D:
    println!("
    [!] - Calculadora en Rust
    [!] - ¿Que quieres hacer?

    1 - Sumar
    2 - Restar
    3 - Dividir
    4 - Multiplicar

    ");

    let opt: i8 = read!();
    let opt_array: [&str; 5] = ["", "sumar", "restar", "dividir", "multiplicar"];
    let opt_acc: [&str; 5] = ["", "+", "-", "/", "*"];
    
    println!("  [!] - Vamos a {}, elige 2 numeros", opt_array[opt as usize]);
    println!("  Primer numero: ");
    let sum1: i128 = read!();
    println!("  Segundo numero: "); 
    let sum2: i128 = read!();

    let result = match opt {
        1 | 2 | 3 | 4 => {
            match opt {
                1 => sum1 + sum2,
                2 => sum1 - sum2,
                3 => sum1 / sum2,
                4 => sum1 * sum2,
                _ => unreachable!(),
            }
        }
        _ => {
            println!("Opción no válida");
            return;
        }
    };
    
    
    println!("{}  {:?} {:?} {:?} = {:?} \n", "\x1B[2J\x1B[1;1H", sum1, opt_acc[opt as usize], sum2, result);    
    println!("Esto se loopea infinitamente, CTRL + C para salir");
    }
}

