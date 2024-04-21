#![allow(unused)]

use std::io::{self, Read};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    const PI: f32 = 3.14;

    //printing
    println!("what is your name?");
    let mut name: String = String::new();
    let greeting:&str = "nice to meet you";


    //taking input
    io::stdin().read_line(&mut name).expect("msg");

    println!("Hello {}, {}", name.trim_end(), greeting);

    //random number
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);

    //taking integer input
    println!("enter age");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("msg");

    let age:u32 = input.trim().parse().expect("");

    // comparing
    let voting_age = 18;

    match age.cmp(&voting_age) {
        Ordering::Greater => println!("Congratulations! {}, You can vote", name.trim_end()),
        Ordering::Less => println!("Sorry {}, You can't vote", name.trim_end()),
        Ordering::Equal => println!("Yay {}, you just got your voting rights", name.trim_end()),
    };

    //arrays

    let arr:[i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("1st: {}", arr[0]);
    println!("Length: {}", arr.len());

    //loop
    
    let mut loop_idx = 0;
    // loop {
    //     if arr[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }

    //     if arr[loop_idx] == 9{
    //         break;
    //     }
    //     println!("Val: {}", arr[loop_idx]);
    //     loop_idx += 1;
    // }

    while loop_idx < arr.len() {
        println!("Arr: {}", arr[loop_idx]);
        loop_idx += 1;
    }

    for val in arr.iter(){
        println!("val: {}", val);
    }

    //tuples

    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    let(v1, v2, v3) = my_tuple;
    println!("name: {}", v2 );
    println!("age: {}", v1 );
    println!("balance: {}", v3);


    //string
    
    // let mut st1:String = String::new();
    // st1.push('A');
    // st1.push_str(" word");
    // for word in st1.split_whitespace(){
    //     println!("{}", word);
    // }
    // let st2 = st1.replace('A', "Another");
    // println!("{}", st2);
    
    let st3 = String::from("x r t b h k k a m c ");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup(); 

    for char in v1{
        println!("{}", char);
    }

    let st4: &str = "Random String";
    let mut st5 = st4.to_string();
    println!("{}", st5);
    let byte_arr1: &[u8] = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String Length: {}", st6.len());
    st5.clear();

    let st6 = String::from("Just Some");
    let st7 = String::from(" Words");
    let st8 = st6 + &st7;
    for char in st8.bytes(){
        println!("{}", char);
    } 

    // enums

    enum Day {
        Monday,
        Teusday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool{
            match self {
                Day::Saturday | Day::Sunday => true, _ => false
            }
        }
    }
    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("naah"),
        Day::Teusday => println!("na"),
        Day::Wednesday => println!("yeah"),
        Day::Thursday => println!("hh"),
        Day::Friday => println!("xx"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend {}", today.is_weekend());

    // Vectors

    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1,2,3,4];
    vec2.push(5);

    println!("1st: {}", vec2[0]); 
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}",second),
        None => println!("no second value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());

    
    
    // functions

    println!("{} + {} = {}",3, 4,sum(3, 4));

    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));

    /*
    
    ownership
    
    Stack: Stores values in a last in first out format
    Data on the stack must have a defined fixed size
    
    Heap: When putting data on a heap you request a certain amount of space. The OS finds space available and returns an address for that space called a pointer.

    Rules:
        1. Each value has a variable that's called its owner
        2. There is only one owner at a time 
        3. When the owner goes out of scope the value disappears 
   
    */

    //Hashmaps
    
    let mut heroes = HashMap::new();
    heroes.insert("spiderman", "tom holland");
    heroes.insert("Thor", "Chris Hemsworth");
    
    for(k,v) in heroes.iter(){
        println!("{} = {}", k,v);
    }

    println!("Length: {}", heroes.len());

    if heroes.contains_key(&"Thor"){
        let the_thor = heroes.get(&"Thor");
        match the_thor {
            Some(x) => println!("Thor is a hero"),
            None => println!("Thor is not a hero"),
        }
    }

    //structs and traits
        

    trait Shape {
        fn new(length:f32, width:f32) -> Self;
        fn area(&self) -> f32;
    }


    struct Rectangle {length: f32, width: f32}
    struct Circle {length: f32, width: f32}

    // Implement the trait for rectangle
    impl Shape for Rectangle{
        // Constructor
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }

        // self allows us to refer to parameters for this struct
        fn area(&self) -> f32{
            return self.length * self.width;
        }
    }

    // Implement the trait for circle
    impl Shape for Circle{
        // Constructor
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }

        fn area(&self) -> f32{
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    // Create circle and rectangle with Shape
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area : {}", rec.area());
    println!("Circ Area : {}", circ.area());

    /*
        Crates: Modules that produce a library or executable
        Modules: Organize and handle privacy
        Packages: Build, test and share crates
        Paths: A way of naming an item such as a struct, function
     */

    order_food();

}

fn sum(a:i32, b:i32) -> i32{
    return a + b;
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum:i32 = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}