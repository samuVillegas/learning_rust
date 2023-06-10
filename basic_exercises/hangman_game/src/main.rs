use std::io;
use std::fs;
use rand::Rng;


fn main() {
    let file_path: String = "data/words.txt".to_string();
    let mut attempts = 5;
    let mut guess_letters = 0;
    println!("Welcome to Blackout Game");

    let word = choose_word(read_file(file_path));
    let len = word.len();
    let mut user_word = vec![' '; len];
    while attempts > 0 && guess_letters != len {
        print_current_word(&user_word);
        println!("You have {attempts} attempts");
        println!("Please input your letter: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: char = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let result_fill_word = fill_word(&word, &mut user_word, guess);
        if result_fill_word > 0 {
            guess_letters += result_fill_word;
        }else{
            attempts-=1;
        }
    }
    if attempts > 0 {
        println!("You won!");
    } else {
        println!("You lost!");
    }
}

fn read_file(file_path: String)-> String{
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents;
}

fn choose_word(content: String) -> String{
    let split: Vec<&str> = content.split("\n").collect();
    let len = split.len();
    let num_word = rand::thread_rng().gen_range(0..(len-1));
    let word = split[num_word].to_string();
    return word;
}

fn print_current_word(user_word: &Vec<char>){
    print!("Current word: ");
    for &letter in user_word{
        if letter == ' ' {
            print!("{}",'_');
        }else {
            print!("{}",letter);
        }
    }
    println!("");
}

fn fill_word(word: &String, user_word: &mut Vec<char>, user_letter: char) -> usize {
    let mut index = 0;
    let mut count = 0;
    for letter in word.chars(){
        if letter == user_letter {
            user_word[index] = user_letter;
            count += 1;
        } 
        index+=1;
    }
    return count;
}
