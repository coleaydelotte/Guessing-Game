/**
 * Program prompts the user with a input asking for a number,
 * then checks to see if it is valid. If the number enter is
 * valid it compares it to a number generated in range from
 * 0 to 100 using: `rng.gen_range(0..100)` from the library: 
 * `rand`. After comparing the values, tells the user if the
 * entered value is higher, lower or the same. then either
 * prompts the user to guess another number or asks if the
 * user wants to play again.
 */
use std::io::stdin;
use rand::Rng;

fn get_input() -> i32 {
    let number_to_return: i32;
    loop
    {
        let mut input = String::new();
        println!("Please enter a number below: ");
        stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(number) => {
                number_to_return = number;
                break;
            }

            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }
    number_to_return
}

fn main_loop()
{
    let mut rng = rand::thread_rng();
    let secret_number: i32 = rng.gen_range(0..100);
    loop
    {
        let num: i32 = get_input();
        if num == secret_number
        {
            println!("You guessed the secret number: {}", secret_number);
            break;
        }
        else if num < secret_number
        {
            println!("The secret number is greater than the number you entered.");
        }
        else
        {
            println!("The secret number is less than the number you entered.");
        }
    }
}

fn replay_validation() -> bool
{
    loop
    {
        println!("Do you want to play again? (y/n)");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "y"
        {
            return true;
        }
        else if input.trim() == "n"
        {
            return false;
        }
        else
        {
            println!("Please enter a valid input.");
        }
    }
}

fn main()
{
    main_loop();
    let replay: bool = replay_validation();
    if replay
    {
        main();
    }
    else
    {
        println!("Thanks for playing!");
    }
}
