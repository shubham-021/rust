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

fn _serde() {
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

use borsh::{BorshSerialize , BorshDeserialize};

#[derive(BorshDeserialize,BorshSerialize,Debug,PartialEq)]
struct User2{
    id: u64,
    data: String,
    v : Vec<u32>
}


fn _borsh(){
    // BORSH (Binary Object Representation Serializer for Hashing) is a deterministic , binary serialization format often
    // used in RUST (and other languages) to encode and decode data in a consistent unambiguous way

    // it was originally developed by the NEAR Protocol team for use in smart contract , but you can use it any Rust project that needs a fast, predictable seralization layer.

    // cargo add borsh

    //convert this
    // [dependencies]
    // borsh = "1.5.7"

    // into this , to add derive too
    // [dependencies]
    // borsh = {version = "1.5.7" , features = ["derive"]}


    let main = User2{
        id:42,
        data: "Hello, Borsh".into(),
        v:vec![1,2,3]
    };

    let mut buffer: Vec<u8> = Vec::new();

    // main.serialize(&mut buffer).unwrap();

    //Deserialize
    // let deserialize = User2::try_from_slice(&mut buffer).unwrap();

    // assert_eq!(main,deserialize);
    // println!("Successfully serialized and deserialized: {:?}",buffer);
    // println!("Successfully serialized and deserialized: {:?}",deserialize);

    //or more safely

    let ans = main.serialize(&mut buffer);
    match ans{
        Ok(_) => {
            println!("Successfully serialized: {:?}",buffer);
            let deserialize = User2::try_from_slice(&mut buffer);
            match deserialize {
                Ok(u) => println!("Successfully deserialized: {:?}",u),
                Err(_) => println!("Error deserializing struct")
            }
        },
        Err(_) => println!("Error serializing struct")
    }
}

// mod shapes{
//     pub mod area;
//     pub mod perimeter;
// }

// use shapes::{area::square as square_a,perimeter::square as square_p};


// fn main(){
    // _serde();
    // let a = square_a(4);
    // let p = square_p(4);
    // print!("Area: {}, Perimeter: {}",a,p);
    // _borsh();
// }


fn lifetimes(){
    // lifetimes is a contruct that compiler(or more specifically , its borrow checker) uses to ensure all borrows are valid.
    // Specifically , a variable's lifetime begins when it is created and ends when it is destroyed.
    // While lifetimes and scope are often referred to together , they are not the same. 
}


// this gives error : missing lifetime specifier
fn longest_string(s1:&String , s2:&String) -> &String {
    if s1.len() > s2.len(){
        return &s1;
    }else{
        return &s2;
    }
}

fn main(){
    let str1 = String::from("Shubham");
    // let str2 = String::from("Singh");
    // let ans = longest_string(&str1, &str2);

    // lifetime specifier is necessary , what if we do something like
    let ans;
    {
        let str2 = String::from("Singh");
        ans = longest_string(&str1, &str2);
    }

    // here ans contains the reference to str2 , if str2 is the bigger string , but str2 doesnt exists outside of its block , so ans is pointing to nothing/dangling pointer
    // compiler needs to a reference to lifetime of both the string , compiler doesnt know which str is longer , it just implements that can be compiled correctly without if and buts.
    // if lifetime of str2 is same as ans , then its ok , no problem here , but if str2 lifetime's is valid in its block then its a problem 


    println!("{}",ans);
}
