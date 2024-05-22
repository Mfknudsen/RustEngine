use std::io::{self, Write};

fn handle_string(name_input: String) -> String {
    let trimmed_name = name_input.trim();

    if !trimmed_name.is_empty() {
        let first_char = &trimmed_name[0..1].to_uppercase();
        let remaining_chars = &trimmed_name[1..];
        let modified_name = format!("{}{}", first_char, remaining_chars);
        modified_name
    } else {
        String::from("Mario")
    }
}

pub fn get_name_input() -> Result<String, io::Error> {
    let mut name_input = String::new();
    loop {
        print!("Write your name: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut name_input)?; 
        let player_name = handle_string(name_input);
        println!("Your name is: {}", player_name);
        return Ok(player_name);          
    }
}
