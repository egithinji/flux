use egg_mode::KeyPair;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Config {
    pub con_token: KeyPair,
    pub acc_token: KeyPair,
}

impl Config {
   
    pub fn new() -> Config {
        //Create a new config object containing a connection and access token
        //Get the consumer secret
        let consumer_secret = get_key("consumer_secret.txt").trim().to_owned();
        let consumer_key = get_key("consumer_key.txt").trim().to_owned();
        let access_token_secret = get_key("access_token_secret.txt").trim().to_owned();
        let access_token = get_key("access_token.txt").trim().to_owned();

        let connection_token = KeyPair::new(consumer_key, consumer_secret);
        let access_token = KeyPair::new(access_token, access_token_secret);

        Config {
            con_token: connection_token,
            acc_token: access_token,
        }
    }

}

fn get_key(filepath: &str) -> String {


   let path = Path::new(filepath);
   //open the path in read-only mode.
   let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open the file: {}", e),
        Ok(file) => file,
   };

   //Read the file contents into a string
   let mut s = String::new();
   file.read_to_string(&mut s).expect("Problem reading key");
   s 
}

