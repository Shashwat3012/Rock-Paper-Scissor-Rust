use std::io;
use rand::Rng;

fn main() {
    println!("$ Rock Paper Scissor $");

    println!("Enter the number for your choice");
    println!("1. Rock");
    println!("2. Paper");
    println!("3. Scissor");

    let mut score = 0;
    println!("Your Score: {}", score);

    let mut n = 1;
    while n < 6{
        let secret_number = rand::thread_rng().gen_range(1..=3);
        println!("Please input your guess.");
        let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Main logic for the game!
    if guess.trim().eq("1") {
        if secret_number == 1{
            println!("Draw: It was rock!");
        }
        else if secret_number == 2{
            println!("Loss: It was paper!");
        }
        else{
            println!("Win: It was scissor!");
            score += 1;
        }
        println!("Your Score: {}", score);
    }

    else if guess.trim().eq("2") {
        if secret_number == 1{
            println!("Win: It was rock!");
            score += 1;
        }
        else if secret_number == 2{
            println!("Draw: It was paper!");
        }
        else{
            println!("Loss: It was scissor!");
        }
        println!("Your Score: {}", score);
    }

    else if guess.trim().eq("3") {
        if secret_number == 1{
            println!("Loss: It was rock!");
        }
        else if secret_number == 2{
            println!("Win: It was paper!");
            score += 1;
        }
        else{
            println!("Draw: It was scissor!");
        }
        println!("Your Score: {}", score);
    }

    else{
        println!("Enter Valid Input!");
    }

    n += 1;
    }
}


/*
           guess              random generated
           rock(1)               paper(2)-------------------> loss
                                 scissor(3)-----------------> win
                                 rock(1)--------------------> draw


           paper(2)              rock(1)--------------------> win
                                 scissor(3)-----------------> loss
                                 paper(2)-------------------> draw


           scissor(3)            rock(1)--------------------> loss
                                 paper(2)-------------------> win
                                 scissor(3)-----------------> draw
 */