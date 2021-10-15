use crate::mode::Mode;
use clap::ArgMatches;
use std::fs::File;
use std::io::prelude::*;
use std::process;
extern crate base64;
extern crate rpassword;

#[derive(Debug)]
pub struct Config {
    input_text: String,
    mode: Mode,
    output_text: String,
    password: String,
}

impl Config {
    pub fn new(input_text: String, mode: Mode, output_text: String, password: String) -> Self {
        Self {
            input_text,
            mode,
            output_text,
            password,
        }
    }

    pub fn input_text(&self) -> Vec<u8> {
        self.input_text.as_bytes().to_vec()
    }

    pub fn input_text_decode(&self) -> Vec<u8> {
        base64::decode(&self.input_text).unwrap()
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn output_text(&self) -> &String {
        &self.output_text
    }

    pub fn set_output_text(&mut self, output_text: String) {
        self.output_text = output_text;
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

pub fn parse(cli_parameters: &ArgMatches) -> Config {
    let mut mode: Mode = Mode::Encryption2D;
    let password: String = get_password(cli_parameters);
    let input_text: String = get_input_text(cli_parameters);
    let mut dimension: i8 = 2;

    if 0 == cli_parameters.occurrences_of("input_file")
        && 0 == cli_parameters.occurrences_of("text")
    {
        println!("You need to specify text or file to decrypt/encrypt");

        process::exit(0);
    }

    if 0 == cli_parameters.occurrences_of("output_file") {
        println!("You need to specify output file");

        process::exit(0);
    }

    if 1 == cli_parameters.occurrences_of("dimension") {
        dimension = cli_parameters
            .value_of("dimension")
            .unwrap()
            .parse::<i8>()
            .unwrap();
    }

    if 1 == cli_parameters.occurrences_of("decrypt") {
        if dimension == 2 {
            mode = Mode::Decryption2D
        } else if dimension == 3 {
            mode = Mode::Decryption3D
        } else if dimension == 4 {
            mode = Mode::Decryption4D
        }
    } else if 1 == cli_parameters.occurrences_of("encrypt") {
        if dimension == 2 {
            mode = Mode::Encryption2D
        } else if dimension == 3 {
            mode = Mode::Encryption3D
        } else if dimension == 4 {
            mode = Mode::Encryption4D
        }
    }

    Config::new(input_text, mode, String::from(""), password)
}

fn get_input_text(cli_parameters: &ArgMatches) -> String {
    let mut input_text: String = String::new();
    if 1 == cli_parameters.occurrences_of("text") {
        input_text = cli_parameters.value_of("text").unwrap().to_string();
    } else if 1 == cli_parameters.occurrences_of("input_file") {
        let file_name: String = cli_parameters.value_of("input_file").unwrap().to_string();
        let mut file: File = File::open(&file_name).expect("Failed to open a file");
        file.read_to_string(&mut input_text)
            .expect("Failed to read text");
    }

    input_text
}

fn get_password(cli_parameters: &ArgMatches) -> String {
    let password: String;
    if 1 == cli_parameters.occurrences_of("password") {
        password = cli_parameters.value_of("password").unwrap().to_string();
    } else {
        password = rpassword::read_password_from_tty(Some("Enter password: ")).unwrap();
    }

    password
}
