use crate::config::Config;
use crate::info::Info;
use crate::mode::Mode;
use crate::operation::{Operation2D, Operation3D, Operation4D};
use crate::vector::{Vector2D, Vector3D, Vector4D};
use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::io::Write;
extern crate base64;

mod config;
mod info;
mod mode;
mod operation;
mod password;
mod vector;

pub fn run() {
    let info: Info = info::Info::new();
    let cli_parameters: ArgMatches = get_cli_parameters(&info);
    let mut config: Config = config::parse(&cli_parameters);
    let password: Vec<u8> = password::encrypted_password(&config);

    match config.mode() {
        Mode::Decryption2D => decryption_2d(&mut config, &password),
        Mode::Decryption3D => decryption_3d(&mut config, &password),
        Mode::Decryption4D => decryption_4d(&mut config, &password),
        Mode::Encryption2D => encryption_2d(&mut config, &password),
        Mode::Encryption3D => encryption_3d(&mut config, &password),
        Mode::Encryption4D => encryption_4d(&mut config, &password),
    }

    save_results(&cli_parameters, &config);
}

fn decryption_2d(config: &mut Config, password: &[u8]) {
    println!("decryption_2d");
    let mut vector_2d: Vector2D = prepare_2d_vector(password);
    let mut operation_2d: Operation2D;
    let mut output_text: Vec<u8> = Vec::new();
    let mut number: u8;

    for operation_chunk in config.input_text_decode().chunks(3) {
        operation_2d = Operation2D::new(operation_chunk[0], operation_chunk[1], operation_chunk[2]);
        number = vector_2d.decode(&operation_2d);
        vector_2d.shift(&operation_2d);
        output_text.push(number);
    }

    config.set_output_text(String::from_utf8(output_text).unwrap());
}

fn decryption_3d(config: &mut Config, password: &[u8]) {
    println!("decryption_3d");
    let mut vector_3d: Vector3D = prepare_3d_vector(password);
    let mut operation_3d: Operation3D;
    let mut output_text: Vec<u8> = Vec::new();
    let mut number: u8;

    for operation_chunk in config.input_text_decode().chunks(4) {
        operation_3d = Operation3D::new(
            operation_chunk[0],
            operation_chunk[1],
            operation_chunk[2],
            operation_chunk[3],
        );
        number = vector_3d.decode(&operation_3d);
        vector_3d.shift(&operation_3d);
        output_text.push(number);
    }

    config.set_output_text(String::from_utf8(output_text).unwrap());
}

fn decryption_4d(config: &mut Config, password: &[u8]) {
    println!("decryption_4d");
    let mut vector_4d: Vector4D = prepare_4d_vector(password);
    let mut operation_4d: Operation4D;
    let mut output_text: Vec<u8> = Vec::new();
    let mut number: u8;

    for operation_chunk in config.input_text_decode().chunks(5) {
        operation_4d = Operation4D::new(
            operation_chunk[0],
            operation_chunk[1],
            operation_chunk[2],
            operation_chunk[3],
            operation_chunk[4],
        );
        number = vector_4d.decode(&operation_4d);
        vector_4d.shift(&operation_4d);
        output_text.push(number);
    }

    config.set_output_text(String::from_utf8(output_text).unwrap());
}

fn encryption_2d(config: &mut Config, password: &[u8]) {
    println!("encryption_2d");
    let mut vector_2d: Vector2D = prepare_2d_vector(password);
    let mut operation_2d: Operation2D;
    let mut output_text: Vec<u8> = Vec::new();

    for number in config.input_text().iter() {
        operation_2d = vector_2d.encode(number);
        output_text.push(*operation_2d.d1());
        output_text.push(*operation_2d.d2());
        output_text.push(*operation_2d.s());
        vector_2d.shift(&operation_2d);
    }

    config.set_output_text(base64::encode(output_text));
}

fn encryption_3d(config: &mut Config, password: &[u8]) {
    println!("encryption_3d");
    let mut vector_3d: Vector3D = prepare_3d_vector(password);
    let mut operation_3d: Operation3D;
    let mut output_text: Vec<u8> = Vec::new();

    for number in config.input_text().iter() {
        operation_3d = vector_3d.encode(number);
        output_text.push(*operation_3d.d1());
        output_text.push(*operation_3d.d2());
        output_text.push(*operation_3d.d3());
        output_text.push(*operation_3d.s());
        vector_3d.shift(&operation_3d);
    }

    config.set_output_text(base64::encode(output_text));
}

fn encryption_4d(config: &mut Config, password: &[u8]) {
    println!("encryption_4d");
    let mut vector_4d: Vector4D = prepare_4d_vector(password);
    let mut operation_4d: Operation4D;
    let mut output_text: Vec<u8> = Vec::new();

    for number in config.input_text().iter() {
        operation_4d = vector_4d.encode(number);
        output_text.push(*operation_4d.d1());
        output_text.push(*operation_4d.d2());
        output_text.push(*operation_4d.d3());
        output_text.push(*operation_4d.d4());
        output_text.push(*operation_4d.s());
        vector_4d.shift(&operation_4d);
    }

    config.set_output_text(base64::encode(output_text));
}

fn get_cli_parameters(info: &Info) -> ArgMatches {
    App::new(info.name())
        .version(info.version().as_str())
        .author("Michal Piotrowski <michal@eventhorizonlabs.eu>")
        .about(format!("{} - {}", info.bin_name(), info.name()).as_str())
        .arg(
            Arg::with_name("decrypt")
                .short("D")
                .long("decrypt")
                .help("Decrypt"),
        )
        .arg(
            Arg::with_name("dimension")
                .short("d")
                .long("dimension")
                .value_name("DIMENSION")
                .help("Sets a dimension")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("encrypt")
                .short("E")
                .long("encrypt")
                .help("Encrypt"),
        )
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input-file")
                .value_name("FILE")
                .help("Sets an input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Sets an output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("PASSWORD")
                .help("Sets a password")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("text")
                .short("t")
                .long("text")
                .value_name("TEXT")
                .help("Sets a text")
                .takes_value(true),
        )
        .get_matches()
}

fn prepare_2d_vector(password: &[u8]) -> Vector2D {
    let mut vector_2d: Vector2D = vector::Vector2D::new();
    vector_2d.init(password);

    vector_2d
}

fn prepare_3d_vector(password: &[u8]) -> Vector3D {
    let mut vector_3d: Vector3D = vector::Vector3D::new();
    vector_3d.init(password);

    vector_3d
}

fn prepare_4d_vector(password: &[u8]) -> Vector4D {
    let mut vector_4d: Vector4D = vector::Vector4D::new();
    vector_4d.init(password);

    vector_4d
}

fn save_results(cli_parameters: &ArgMatches, config: &Config) {
    let output_text: String = config.output_text().to_string();
    let file_name: String = cli_parameters.value_of("output_file").unwrap().to_string();
    let mut file: File = File::create(&file_name).expect("Failed to create a file");
    file.write_all(output_text.as_bytes())
        .expect("Failed to write to a file");
}
