extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    //Dev print pour test
    //println!("The secret number is {}", secret_number);

    loop
    {
        println!("Please input your guess.");

        //Instanciation d'une variable changeable, qui es liée à une chaîne vide.
        let mut guess = String::new();

        //Récupération du stdin + \n
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line");

        /*Cast propre de String à u32 => suppression des espaces et des \n, puis conversion.
        Si c'est bien un nombre, pas de réalise la conversion, sinon, skip le reste de l'itération*/
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     =>
            {
                println!("You win!");
                break;
            }
        }
    }

}
