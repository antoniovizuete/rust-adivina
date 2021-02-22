use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    const OK: &str = "Acertaste 🎉";
    println!("Adivina el numero!");

    let gen: u32 = rand::thread_rng().gen_range(1..100);
    println!("El numero generado es: {}", gen);

    loop {
        println!("Introduce el numero: ");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error al leer la entrada");
        println!("La entrada ha sido: {}", &buf);

        let buf: u32 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = match buf.cmp(&gen) {
            Ordering::Greater => "🔺Demasiado grande",
            Ordering::Less => "🔻Demasiado pequeño",
            Ordering::Equal => OK,
        };

        println!("{}", &result);
        if result == OK {
            break;
        }
    }
}
