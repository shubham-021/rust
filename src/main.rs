
// some basics
// fn main() {
    //String

    // let name = String::from("Shubham");
    // println!("{}",name);

    // number data type : u32 , u6 , u64 , usize , i32 , i6 , isize , like these 


    // let name = String::from("Shubham");
    // println!("{}",name);


    // fn get_length looks like this
    // fn get_length(name:String) -> usize {
    //     return name.len();
    // }

    // let name2 = name; // transferring ownership , now name2 is the owner of the address space where the actual string resides
    // println!("{}",name2);
    // println!("{}",name); // --> will give error , name is not allowed to use since it doesnt exist now


    // println!("{}",get_length(name));
    // passing name to get_length is also a transferring of ownership to the name variable that exists in the stack frame of get_length function.
    // println!("{}",get_length(name)); // --> will give error here


    // ways to get back the ownership
    // 1.

    // let (length , name) = get_length(name);
    // println!("{},{}",length,name);

    // fn get_length looks like this
    // fn get_length(name:String) -> (usize,String) {
    //     return (name.len(),name);
    // }

    // 2.
    // instead of passing the ownership directly , let it borrow it , pass the reference to string;
    // let length = get_length(&name);
    // println!("{},{}",length,name);

    // fn get_length looks like this
    // fn get_length(name:&str) -> usize {
    //     return name.len();
    // }


    // mutability and immutability
    // let name = String::from("Shubham");
    // name.push_str(" singh"); // --> will give error since name is not mut , it is immutable by default
    // can do this
    // let name2 = name.clone(); // --> clone the exact string to another address space , now name2 points to this address
    // println!("{}",name2);

    // let mut name = String::from("Shubham");
    // name.push_str(" Singh");
    // println!("{}",name);



// }

//Struct

struct Rect{
    width: f32,
    height:f32
}

impl Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }

    fn direct(){
        println!("Static fn");
    }
}
// fn main(){
//     let first = Rect {
//         width: 10.0,
//         height: 20.0
//     };

//     println!("{}",first.area());
//     Rect::direct();
// }


// Enums

enum Direction{
    North,
    South,
    East,
    West
}
// fn main(){
//     // let direction = Direction::North;
//     get_d(Direction::North);
//     get_d(Direction::South);
//     get_d(Direction::East);
//     get_d(Direction::West);
// }

fn get_d(dir:Direction){
    //patern matching
    // match dir {
    //     Direction::North => println!("North"),
    //     Direction::South => println!("South"),
    //     Direction::East => println!("East"),
    //     Direction::West => print!("West"),
    // }

    match dir {
        Direction::North => println!("North"),
        Direction::South => println!("South"),
        _ => println!("Horizontal dir") // catch any if above not matched
    }
}


// Enum with values

// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32,f32)
// }
 
// enum Which {
//     Area,
//     Perimeter
// }


// fn main(){
//     let sq_a = print_area(Shape::Square(10.0),Which::Area);
//     let ci_a = print_area(Shape::Circle(10.0),Which::Area);
//     let re_a = print_area(Shape::Rectangle(10.0,20.0),Which::Area);
//     println!("Area : ");
//     print!("Square : {} , Circle: {} , Rectangle: {}" , sq_a , ci_a , re_a);


//     let sq_p = print_area(Shape::Square(10.0),Which::Perimeter);
//     let ci_p = print_area(Shape::Circle(10.0),Which::Perimeter);
//     let re_p = print_area(Shape::Rectangle(10.0,20.0),Which::Perimeter);
//     println!("");
//     println!("Perimeters: ");
//     print!("Square : {} , Circle: {} , Rectangle: {}" , sq_p , ci_p , re_p);
// }

// fn print_area(pol:Shape , wch : Which) -> f32 {
//     // this 
//     match wch{
//         Which::Area => match pol{
//             Shape::Circle(r) => 3.14 * r * r,
//             Shape::Rectangle(w,h) => w*h,
//             Shape::Square(side) => side * side
//         },
//         Which::Perimeter => match pol{
//             Shape::Circle(r) => 2.0 * 3.14 * r,
//             Shape::Rectangle(w,h) => 2.0 * (w+h),
//             Shape::Square(side) => 4.0 * side
//         }
//     }

    // in rust , if you dont put semi-colon at the end it is assumed as a return value/statement

    // or
    // return match wch{
    //     Which::Area => match pol{
    //         Shape::Circle(r) => 3.14 * r * r,
    //         Shape::Rectangle(w,h) => w*h,
    //         Shape::Square(side) => side * side
    //     },
    //     Which::Perimeter => match pol{
    //         Shape::Circle(r) => 2.0 * 3.14 * r,
    //         Shape::Rectangle(w,h) => 2.0 * (w+h),
    //         Shape::Square(side) => 4.0 * side
    //     }
    // };

    // or
    // let ans = match wch{
    //     Which::Area => match pol{
    //         Shape::Circle(r) => 3.14 * r * r,
    //         Shape::Rectangle(w,h) => w*h,
    //         Shape::Square(side) => side * side
    //     },
    //     Which::Perimeter => match pol{
    //         Shape::Circle(r) => 2.0 * 3.14 * r,
    //         Shape::Rectangle(w,h) => 2.0 * (w+h),
    //         Shape::Square(side) => 4.0 * side
    //     }
    // };

    // return ans;
// }

// you can also have implementation on top of Enums

use std::{f32::consts::PI};

enum Shapes{
    Circle(f32),
    Square(f32),
    Rectangle(f32,f32)
}

impl Shapes {
    fn area(&self) -> f32 {
        return match self {
            Shapes::Circle(r) => PI * r * r,
            Shapes::Rectangle(w,h) => w*h,
            Shapes::Square(side) => side * side
        };
    }
}

// fn main(){
//     let shape_circle = Shape::Circle(10.0);
//     print!("Area of circle with radius 10.0 is {}",shape_circle.area());
// }


// error handling
// result enum

// use std::fs;

// fn main(){
//     let contents = fs::read_to_string("a.txt");
//     // returns a pre built enum named Result , which has generic value according to fn called , one is the result if no error and other is error
//     match contents {
//         Ok(contents) => println!("{}",contents),
//         Err(e) => println!("Error occurred while reading file")
//     }
// }

// option enum

// unlike many programming languages that uses a null or similar keyword to represent the absence of a value , Rust doesnt have null.
// if you ever have a fn that should return null , return an Option instead

// fn main(){
//     let ans = find_first_a(String::from("Shubham"));

//     match ans {
//         None => println!("No a exists in this given string"),
//         Some(val) => println!("first a exist at index {}" , val)
//     }
// }


// below both syntax are equivalent

// 1.
// if let Shape::Circle(x) = s {
//     println!("Radius: {}", x);
// }

// 2.
// match s {
//     Shape::Circle(x) => {
//         println!("Radius: {}", x);
//     }
//     _ => {}
// }



fn find_first_a(str:String) -> Option<u32> {
    let mut index = 0;
    for ch in str.chars(){
        if ch == 'a' {
            return Some(index);
        };
        index = index+1;
    }

    return None;
    //or simply just
    //None
}


//Generic and traits
//Generic over struct

// traits implementation


trait Shape {
    fn area(&self) -> f32;
}

struct Circle{
    radius: f32
}

impl Shape for Circle{
    fn area(&self) -> f32 {
        return PI*self.radius*self.radius;
    }
}

struct Rectangle{
    len: f32,
    wid: f32
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        return self.len*self.wid;
    }
}

fn print_area_of_shape<T:Shape>(s:T){
    println!("{}",s.area());
}

// fn main(){
//     let circle_area = Circle{
//         radius: 10.0
//     };

//     let rec_area = Rectangle{
//         len: 10.0,
//         wid:10.0
//     };

//     print_area_of_shape(circle_area);
//     print_area_of_shape(rec_area);
// }


// Macros

// cargo install cargo-expand
// cargo expand

// type of macros : 1) Declrative Macros , 2) Procedural Macros


// 1.Declarative Macros

macro_rules! say_hello {
    () => {
        println!("Hello, World");
    };
}

// fn main(){
//     say_hello!();
// }

// 2.Procedural Macros
// There are 3 types of procedural macros
// 1. Custom derive macros
// 2. Attribute-like macros
// 3. Function-like macros

// #[derive(Debug)] // --> debug macro
// struct User{
//     name:String,
//     password:String,
//     age:u32
// }


// fn main(){
//     let u = User{
//         name: String::from("Shubham"),
//         password: String::from("1234"),
//         age: 25
//     };

//     print!("{:?}",u); // debug --> o/p : User { name: "Shubham", password: "1234", age: 25 }
//     // print!("{}",u); // display --> this gives error , you cant print struct or any data directly if it doest implement display trait
// }


// print! wrote directly to stdout.
// format! created a String in memory.


use::std::fmt::{Display,Debug};

struct User{
    name:String,
    password:String,
    age:u32
}

impl Display for User{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"This is the user struct age: {}",self.age)
    }
}


// use debug macro instead, this is just for example
impl Debug for User{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"User {{ name: {} , password: {} , age: {} }}",self.name,self.password,self.age)
    }
}

// fn main(){
//     let u = User{
//         name: String::from("Shubham"),
//         password: String::from("1234"),
//         age: 25
//     };

//     //these work fine

//     // println!("{}",u.name);
//     // println!("{}",u.password);
//     // print!("{}",u.age);


//     // but this will not compile
//     // print!("{}",u); // error :`User` doesn't implement `std::fmt::Display`

//     // implemented Display trait for User , now printing the struct itself
//     println!("{}",u);

//     // implemented Debug trait
//     print!("{:?}",u);
// }

// #[derive(Debug,Clone, Copy)]
// struct Data{
//     is_male: bool,
//     age: u32
// }

#[derive(Debug,Clone)]
struct Data{
    is_male: bool,
    age: u32,
    name: String
}

fn main(){
    let x = 5;
    let y = x;
    // print!("{}",x);


    // When you assign or pass a variable in Rust:
    // If the type implements the Copy trait,
    // → the value is copied, ownership stays with the original.
    // If the type does not implement Copy,
    // → the value is moved, ownership transfers to the new variable.

//        Type                                                  Behavior on assignment          Implements `Copy`?      Example
// i32,u8,bool,f64,char                                             Copied                              Yes              `let y = x;` works           
// String,Vec<T>,Box<T>, custom structs with heap data              Moved                               No               `let y = s;` invalidates `s` 
// &T(reference)                                                    Copied(just copies pointer)         Yes               lightweight                  

// two types of traits here : 1) Copy 2) Clone


    // let u = Data{
    //     is_male: true,
    //     age: 25
    // };

    // let y = u;

    // print!("{:?} , {:?}",y,u); // using u gives error , borrow of moved value: `u` , struct doesnt have clone trait as default , if you want you can give it the copy , clone trait
    // after defining clone , copy , macro

    // print!("{:?},{:?}",u,y); // --> gives no error , now y is not moved ownership of u , instead y is a copy of struct u
    // prints this --> Data { is_male: true, age: 25 },Data { is_male: true, age: 25 }

    // you can not have something like strings which resides on heap and impl the copy trait , since heap allocated data requires ownership concept

    let f = Data{
        is_male: false,
        age:23,
        name: String::from("Fuggu")
    };

    let af = f.clone();
    print!("{:?}",af);
    // prints: Data { is_male: false, age: 23, name: "Fuggu" }

}