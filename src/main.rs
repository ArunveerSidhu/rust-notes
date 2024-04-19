#![allow(unused)]

use std::io::{self, Read};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
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


}
