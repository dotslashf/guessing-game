use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Tebak angkanya! 🔢");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Masukkan tebakanmu!: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Gagal membaca inputan");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tebakkanmu adalah: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Terlalu kecil 🤏"),
            Ordering::Greater => println!("Terlalu besar 💪"),
            Ordering::Equal => {
                println!("Kamu benar! 🏆");
                break;
            }
        }
    }
}
