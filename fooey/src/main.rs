use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{thread, time};

const THE_JOKE_QUESTION: &str = "What do you call a bird with feathers and wings?";
const THE_JOKE_ANSWER: &str = "A bird.";
const NO_FUN_RESPONSE: &str = "Ok, well you're no fun...";
const WHAT_KIND_OF_JOKE_RESPONSE: &str = "A Funny one...";

const INITIAL_GREETING: &str = "Hello!";

const YES_ANSWER_CHOICE: &str = "Yes";
const NO_ANSWER_CHOICE: &str = "No";
const WHAT_KIND_ANSWER_CHOICE: &str = "What kind of joke?";

const ONE_SECOND: time::Duration = time::Duration::from_secs(1);
const TWO_SECONDS: time::Duration = time::Duration::from_secs(2);

fn main() -> std::io::Result<()> {
    let mut timesAskedToHearAJoke: i32 = 0;
    println!("{}", INITIAL_GREETING);

    ask_to_hear_a_joke()
}

fn ask_to_hear_a_joke() -> std::io::Result<()> {
    
    thread::sleep(ONE_SECOND);
    
    println!("Do you want to hear a joke?");
    
    thread::sleep(ONE_SECOND);
    
    let items = vec![YES_ANSWER_CHOICE, NO_ANSWER_CHOICE, WHAT_KIND_ANSWER_CHOICE];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            match items[index] {
                YES_ANSWER_CHOICE => tell_the_joke(),
                NO_ANSWER_CHOICE => respond_to_no_fun_user(),
                WHAT_KIND_ANSWER_CHOICE => respond_to_what_kind(),
                _ => panic!("oh no!"),
            }
        }

        None => panic!("User did not select anything"),
    }
}

fn tell_the_joke() -> std::io::Result<()> {
    println!("{}", THE_JOKE_QUESTION);
    thread::sleep(TWO_SECONDS);
    println!("...?");
    thread::sleep(TWO_SECONDS);
    println!("{}", THE_JOKE_ANSWER);
    Ok(())
}

fn respond_to_no_fun_user() -> std::io::Result<()> {
    println!("{}", NO_FUN_RESPONSE);
    thread::sleep(ONE_SECOND);
    Ok(())
}

fn respond_to_what_kind() -> std::io::Result<()> {
    println!("{}", WHAT_KIND_OF_JOKE_RESPONSE);
    thread::sleep(TWO_SECONDS);
    println!("So...");
    thread::sleep(TWO_SECONDS);
    ask_to_hear_a_joke()
}
