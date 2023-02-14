use std::{
    fs::File,
    io::{Read, Write},
};

use nucypher_core::{umbral_pre::SecretKey, MessageKit, ProtocolObject};

fn main() {
    // make_mk();

    read_mks();
}

fn read_mks() {
    let files = [
        "message_kit_0_1.bin",
        "message_kit_0_2.bin",
        "message_kit_0_3.bin",
        "message_kit_0_4.bin",
        "message_kit_0_5.bin",
    ];

    for file_name in files.iter() {
        let mut file = File::open(file_name).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        if MessageKit::from_bytes(&bytes).is_err() {
            println!("Error: {}", file_name);
        };
    }
}

fn make_mk() {
    let sk = SecretKey::random();
    let pk = sk.public_key();

    let plaintext = b"Hello, world!";
    // let message_kit = MessageKit::new(&pk, plaintext);
    let message_kit = MessageKit::new(&pk, plaintext, None);

    // Write the message kit to a file
    let mut file = File::create("message_kit_0_1.bin").unwrap();
    file.write_all(&message_kit.to_bytes()).unwrap();
}
