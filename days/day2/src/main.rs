use std::fs::File;
use std::io::{BufRead, BufReader,Error};
use std::path::Path;
use parse_display::{Display,FromStr};


#[derive(Debug, Display,FromStr)]
#[display("{min}-{max} {letter}")]
struct Constraint{
    min: usize,
    max: usize,
    letter: char
}

impl Constraint{
    
    pub fn is_valid(&self,password:&String)->bool{
        let letter_count = password.chars().filter(|c| *c==self.letter).count();
        letter_count >=self.min  && letter_count<= self.max
    }

    pub fn is_valid_new(&self, password : &String)->bool{
        let first_position = password.chars().nth(self.min-1).expect("String too short");
        let second_position = password.chars().nth(self.max-1).expect("String too short");
        (first_position == self.letter) ^ (second_position == self.letter)
    }
}

fn parse_constraint(line:&String)->Constraint{
    let segments: Vec<&str> = line.split(":").collect();
    let constraint_string = segments[0]; 
    let constraint: Constraint  = constraint_string.parse().unwrap();
    constraint
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

    
fn main() {
    println!("Hello, world!");
    let test_constraint = Constraint{letter:'a', min:1, max:2};
    assert!(! test_constraint.is_valid(&String::from("bcde")), "too few failed");
    assert!( test_constraint.is_valid(&String::from("baae")), "is valid failed");
    assert!(!test_constraint.is_valid(&String::from("aaae")), "to many failed");

    let (constraints,passwords)= read_file();
    assert_eq!(constraints.len(), passwords.len(),"constraint and password sizes don't match");
    let mut valid_count_1 = 0;
    let mut valid_count_2 = 0;
    for (constraint, password) in constraints.iter().zip(passwords.iter()){
        if constraint.is_valid(password){
            valid_count_1 +=1;
        }
        if constraint.is_valid_new( password){
            valid_count_2 +=1;
        }
    }
    println!("There are {} valid passwords in the first scheme and {} in the seccond", valid_count_1, valid_count_2);
}