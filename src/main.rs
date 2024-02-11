mod crypto;

use base64::{engine::general_purpose, Engine as _};



use std::collections::HashMap;

use crypto::ShamirSS;
fn main() {

    let secret= b"Hello";
    let numparts = 5;
    let miniumparts = 3;

    let keys=ShamirSS::split(numparts, miniumparts, secret.to_vec());
    if keys.is_ok(){
        let keys = keys.unwrap();
        let keysiter = keys.clone();
        for key in keysiter{

            let data=key.1;
            let string = general_purpose::STANDARD.encode(&data) ;

            println!("Key {}: {}", key.0,string);

        }
        let mut parts:HashMap<i32,Vec<u8>>=HashMap::new();
        parts.insert(0, keys[&1].clone());
        parts.insert(1, keys[&2].clone());
        parts.insert(2, keys[&3].clone());
        let nshared=ShamirSS::join(parts);
        if nshared.is_ok(){
        
            let shared = nshared.unwrap();

            let string=String::from_utf8_lossy(shared.as_slice()).to_string();
            
            println!("Shared Key: {}", string);

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
