

use base64::{engine::general_purpose, Engine as _};
use std::collections::BTreeMap;
use shamir_algorithm::ShamirSS;

fn main() {

    let secret= b"Hello Shamir Shared Secret!!!!!";
    let numparts = 5;
    let miniumparts = 3;
    
    println!("Origin Shared Secret: {} ", String::from_utf8_lossy(secret));
    println!("Origin Shared Secret Bytes: {:?} ", secret);
    
    let keys=ShamirSS::split(numparts, miniumparts, secret.to_vec());
    if keys.is_ok(){
        let keys = keys.unwrap();
        let keysiter = keys.clone();
        for key in keysiter{

            let data=key.1;
            let string = general_purpose::STANDARD.encode(&data) ;

            print!("Key {}: [{}] [", key.0,string);

            for byte in &data.clone() {
                print!("{:02X} ", byte);
            }
            print!("]");
            println!(); 
        }
        let mut parts:BTreeMap<i32,Vec<u8>>=BTreeMap::new();
        for (key, value) in &keys {
            // Copy only entries with keys less than or equal to 3
            if *key <= miniumparts {
                parts.insert(*key, value.clone());
            }
        }

      
        let nshared=ShamirSS::join(parts);
        if nshared.is_ok(){
            
            let shared = nshared.unwrap();
            println!("Restaured Shared Secret Bytes: {:?} ", shared);
            let shared_string_value = String::from_utf8_lossy(shared.as_slice());
            println!("Restaured Shared Secret: {}",shared_string_value);
        }
        else{
            let msg=nshared.unwrap_err();
            println!("{msg}");
        }
    }else {
        let msg=keys.unwrap_err();
        println!("{msg}");
    }

}
