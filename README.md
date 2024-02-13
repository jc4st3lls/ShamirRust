# ShamirRust
Shamir implementation in Rust

Resumint molt, aquest algoritme criptogràfic ens permet compartir un secret entre un número d’actors, i establir un mínim d’aquests per recuperar-lo o reconstruir-lo.

Per entendre-ho millor, imaginem que tenim un document confidencial xifrat amb una clau secreta que no sap ningú, però que hi ha una serie d’actors que tenen una clau cada un que ajuntant-les d’alguna manera, poden reconstruir la clau secreta del document xifrat. Però per acabar de arrodonir-ho, que només amb un cert número d’aquest actors, ja en tenim prou. 
Si ens parem a pensar una mica. això dins el món del blockchain, on la finalitat entre d’altres, és posar valor al contingut digital, aquest algoritme pot ser de molt ús, de fet ho és (compartir valor).
I també fora del món blockchain, en qualsevol sistema on la confidencialitat i la privacitat sigui necessària. Imaginem que, existeix un Sistema de Salut, amb dades de pacients, les quals només és poden visualitzar amb el consentiment de certes parts. Si aquestes dades estan xifrades, i per desxifrar-les és requereix que tots els actors (pacient + metge + sistema) o una part d’ells (pacient + sistema o metge + sistema) estiguin d’”acord”, una manera de controlar aquest accés podria ser amb l’ Schema Shamir. El mateix es pot aplicar a documents confidencials, contrasenyes amb privilegis alts, etc.

```Rust
let secret= b"Hello Shamir Shared Secret!!!!!";
let keys=ShamirSS::split(numparts, miniumparts, secret.to_vec());

...
let mut parts:BTreeMap<i32,Vec<u8>>=BTreeMap::new();
for (key, value) in &keys {
// Copy only entries with keys less than or equal to 3
   if *key <= miniumparts {
      parts.insert(*key, value.clone());
   }
}
let nshared=ShamirSS::join(parts);

...

```
Exemple d'us:
```Rust
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
            let shared_string_value = String::from_utf8_lossy(secret);
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
```

Referències:

[https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing](https://en.wikipedia.org/wiki/Shamirs_Secret_Sharing).

[https://github.com/blockchain/](https://github.com/blockchain/).

[https://github.com/blockchain/shamir](https://github.com/blockchain/shamir).


[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/jcastellsgH)
