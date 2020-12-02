use std::fs::File;
use std::io::{BufRead, BufReader,Error};
use std::path::Path;

#[derive(Debug)]
struct Constraint{
    min: usize,
    max: usize,
    letter: char
}

fn parse_constraint(line:&String)->Constraint{
    let segments: Vec<&str> = line.split(":").collect();
    let constraint_string = segments[0]; 
    let letter = constraint_string.chars().last().unwrap();
    let range_string : Vec<&str> = constraint_string.split_ascii_whitespace().collect();
    let range_string = range_string[0];
    let range_string : Vec<&str>= range_string.split("-").collect();
    let min :usize= range_string[0].parse().unwrap();
    let max :usize= range_string[1].parse().unwrap();

    Constraint{
        min,
        max,
        letter
    }
}

fn parse_password(line:&String)->String{
    let password : Vec<&str> = line.split(":").collect();
    return String::from(password[1]).trim().to_string();
}

fn read_file()->(Vec<Constraint>,Vec<String>){
    let path = "data/day_2/input";
    let input = File::open(path).expect("failed to open the input file");
    let buffered = BufReader::new(input);
    let mut constraints : Vec<Constraint> = vec![];
    let mut passwords : Vec<String> = vec![];

    for line in buffered.lines(){
        let l = line.unwrap();
        let constraint = parse_constraint(&l);
        let password = parse_password(&l); 
        constraints.push(constraint);
        passwords.push(password);
    }
    return (constraints,passwords)
}

fn is_valid(constraint:&Constraint,password:&String)->bool{
   let letter_count = password.chars().filter(|c| *c==constraint.letter).count();
   letter_count >=constraint.min  && letter_count<= constraint.max
}

fn is_valid_new(constraint : &Constraint, password : &String)->bool{
    let first_position = password.chars().nth(constraint.min-1).expect("String too short");
    let second_position = password.chars().nth(constraint.max-1).expect("String too short");
     (first_position == constraint.letter) ^ (second_position == constraint.letter)
}
    
fn main() {
    println!("Hello, world!");
    let test:String = "1-3 b: cdefg".into();

    assert!(!is_valid(&Constraint{letter:'a', min:1, max:2}, &String::from("bcde")), "too few failed");
    assert!(is_valid(&Constraint{letter:'a', min:1, max:2}, &String::from("baae")), "is valid failed");
    assert!(!is_valid(&Constraint{letter:'a', min:1, max:2}, &String::from("aaae")), "to many failed");

    let (constraints,passwords)= read_file();
    assert_eq!(constraints.len(), passwords.len(),"constraint and password sizes don't match");
    let mut valid_count_1 = 0;
    let mut valid_count_2 = 0;
    for (constraint, password) in constraints.iter().zip(passwords.iter()){
        if is_valid(constraint, password){
            valid_count_1 +=1;
        }
        if is_valid_new(constraint, password){
            valid_count_2 +=1;
        }
    }
    println!("There are {} valid passwords in the first scheme and {} in the seccond", valid_count_1, valid_count_2);
}