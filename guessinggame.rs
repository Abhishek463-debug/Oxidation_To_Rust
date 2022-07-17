use std::io;


pub fn get_input(number:&mut String) -> u8 {
    number.clear();

    io::stdin()
        .read_line(number)
        .unwrap();

    //println!("{}", number);
    let compare = number
                        .trim()
                        .parse::<u8>()
                        .expect("Can't unwrap!");

    compare

}
pub fn compare<'a>(actual_number: u8, compare_me: u8) -> &'a str {
    if compare_me < actual_number {
        "This number is too small!"
    }
    else if compare_me > actual_number {
        "This number is too large!"
    }
    else {
        "This number is exactly right! Nice job!"
    }
}
pub fn play<'b>() -> &'b str {
    let mut number = String::new();
    let guess_me: u8 = 100;
    println!("You have to guess a number between 0 and 255!");
    loop {
        let user_input = get_input(&mut number);

        let game_output = compare(guess_me, user_input);

        println!("{game_output}");

        if game_output == "This number is exactly right! Nice job!" {
            break;
        }
    }
    "You won this game!"
}
