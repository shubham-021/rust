// Serde is a popular serialization and deserialization framework in rust . It provides a way to convert Rust data
// structure into different formats (serialization) and vice versa (deserialization).
// The most common use case invovle working with formats like JSON , YAML , TOML and others.

// do
// cargo add serde
// cargo add serde_json

//cargo add serde_json
    // cargo add serde     
    // Updating crates.io index
    //   Adding serde v1.0.228 to dependencies
    //          Features:
    //          + std
    //          - alloc
    //          - derive
    //          - rc
    //          - serde_derive
    //          - unstable

// doing this add , cargo doest add the whole lib at once , like here it only imported std , to use anything else you have to specify
// that in dependencies 

// currently in toml file we have :
//[dependencies]
// serde = "1.0.228"

// change that to
// [dependencies]
// serde = {version = "1.0.228" , features = ["derive"]}

// now you can import Serialize and Deserialize

use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug)]
struct User{
    username : String,
    password: String
}

fn main() {
    let u = User{
        username: String::from("Arka"),
        password: String::from("1234")
    };

    // serialize struct
    // let serialized_string = serde_json::to_string(&u);
    // match serialized_string {
    //     Ok(str) => print!("{}",str),
    //     Err(_e) => print!("error occurred while serializing")
    // };

    //deserialize string
    // let serialized_string = serde_json::to_string(&u);
    // match serialized_string {
    //     Ok(str) => {
    //         println!("{}",str);
    //         let deserialized_user:Result<User, _> = serde_json::from_str(&str);
    //         match deserialized_user {
    //             Ok(u) => print!("{:?}",u),
    //             Err(_e) => print!("error occurred while deserializing")
    //         };
    //     }
    //     Err(_e) => print!("error occurred while serializing")
    // };

    //or
    // let pretty = serde_json::to_string_pretty(&u).unwrap();
    // println!("{}", pretty);


    // serialize and deserialize with unwrap
    let serialized_str = serde_json::to_string_pretty(&u).unwrap();
    println!("{}", serialized_str);

    let deserialized_user:User = serde_json::from_str(&serialized_str).unwrap();
    println!("{:?}",deserialized_user);


    // why unwrap is risky
    // let value = some_result.unwrap();
    // it means:
    //     “If this Result is Ok, give me the value.
    //     If it's Err, immediately panic and crash the program.”
    //     So:
    //         Ok(value) → returns value
    //         Err(e) → panics (aborts with stack trace)


    // unwrap() is fine when:
    //     You know for sure something cant fail (like parse::<i32>("42"))
    //     Or you’re writing quick prototypes or scripts.

    //     But in production code, its dangerous because:
    //     It can cause your entire program to crash on a single recoverable error.
    //     You lose context about what went wrong.
    //     It bypasses Rust’s safety philosophy — explicit handling of all possibilities.
}
