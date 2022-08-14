use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;
use regex::Regex;

/*

    Asking the user to write the env file in order to get the environment variables from there for security reasons.
    With read_from_env function the program can read variables and use them in program. Be carefull with the visibility of
    .env file

*/

fn handling_env_file() {
    
    let path = Path::new(".env");

    match File::open(path) {
        Ok(mut file) => {
            let mut content = Vec::new();
            match file.read_to_end(&mut content) {
                Ok(_file) => {
                    if content.len() < 1 {
                        println!("You need to set some environment variables in order to start the server properly!");
                        return;
                    } else {
                        let env_content = String::from(str::from_utf8(&content).unwrap());
                        let string_vec: Vec<&str> = env_content.split("\n").collect();
                        for item in string_vec {

                            // Splitting the line into two parts with the equal sign
                            let split_line = item.split_once("=");
                            match split_line {
                                Some((first, second)) => {
                                    let key = first.to_string();
                                    let value = second.to_string();
                                    // Here with the regex we will check the illegal signs exists or not. Because we need to worry about
                                    // the structure
                                    let regex_check = Regex::new("^[a-zA-Z0-9_]+$").unwrap();
                                    if regex_check.is_match(key.as_str()) {
                                    // Here the program will map all the keys and values
                                    env::set_var(key, value);
                                    } else {
                                        println!("There is a problem in .env file content format!");
                                        return;
                                    }
                                },
                                None => {
                                    println!("There is a problem in .env file");
                                    return;
                                }
                            }                            
                        }
                    }
                },
                Err(_) => println!("Happened some problem")
            };        
        },
        Err(_) => {
            File::create(path).ok().expect("There is an error in creating the .env file!");
            println!("I have created .env file for you to fill it with environment variables!");
            return;
        }
    };

}


pub fn app_env() {
    handling_env_file();
}

