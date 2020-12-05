use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryFrom;
use std::default::Default;
use std::fs::read_to_string;

extern crate regex;
use regex::Regex;

#[derive(Debug, Default)]
struct Passport{
    pub byr:Option<u32>,
    pub iyr:Option<u32>,
    pub eyr:Option<u32>,
    pub hgt:Option<String>,
    pub hcl:Option<String>,
    pub ecl:Option<String>,
    pub pid:Option<String>,
    pub cid:Option<u32>
}

impl Passport{
    pub fn is_valid(&self)->bool{
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }

    pub fn is_valid2(&self)->bool{
        self.validate_byr() &&
        self.validate_iyr() &&
        self.validate_eyr() &&
        self.validate_hgt() &&
        self.validate_hcl() &&
        self.validate_ecl() &&
        self.validate_pid()
    }
    fn validate_byr(&self)->bool{
        match self.byr{
            Some(val) => val>= 1920 && val <=2002,
            None => false 
        }
    }
    fn validate_iyr(&self)->bool{
        match self.iyr{
            Some(val) => val >= 2010 && val <= 2020,
            None => false 
        }
    }
    fn validate_eyr(&self)->bool{
        match self.eyr{
            Some(val) => val >= 2020 && val <= 2030,
            None => false 
        }
    }
    fn validate_hgt(&self)->bool{
        match &self.hgt{
            Some(val) =>{
                if val.contains("cm"){
                    let val: u32 = val.replace("cm","").parse().unwrap();
                    return val>=150 && val <= 193
                }
                else if val.contains("in"){
                    let val: u32 = val.replace("in","").parse().unwrap();
                    return val >= 59 && val <=76
                }
                else{
                    false 
                }
            }
            None=> false 
        }
    }
    fn validate_hcl(&self)->bool{
        let re = Regex::new(r"\#[0-9a-f]{6}\b").unwrap();
        match &self.hcl{
            Some(val)=> re.is_match(&val), 
            None=>false
        }
    }
    fn validate_ecl(&self)->bool{
        let allowed : Vec<String> = ["amb","blu","brn","gry","grn","hzl","oth"].iter().map(|s| String::from(*s)).collect();
        match &self.ecl{ 
            Some(val)=> allowed.contains(&val),
            None => false
        }
    }
    fn validate_pid(&self)->bool{
        let re = Regex::new(r"\b\d{9}\b").unwrap();
        match &self.pid{
            Some(val)=>  re.is_match(&val),
            None => false
        }
    }

    fn print_pass_fail(&self){
        println!("overall {}", self.is_valid2());
        println!("byr {} {:?}",self.validate_byr(),self.byr);
        println!("iyr {} {:?}",self.validate_iyr(),self.iyr);
        println!("eyr {} {:?}",self.validate_eyr(),self.eyr);
        println!("hgt {} {:?}",self.validate_hgt(),self.hgt);
        println!("hcl {} {:?}",self.validate_hcl(),self.hcl);
        println!("ecl {} {:?}",self.validate_ecl(),self.ecl);
        println!("pid {} {:?}",self.validate_pid(),self.pid);
    }
}

impl TryFrom<&str> for Passport{
    type Error = &'static str;

    fn try_from(value:&str)-> Result<Self,Self::Error>{
        let mut passport: Passport = Default::default(); 
        // println!("value {}",value);
        for v in value.replace("\n", " ").split(" "){
            if v.trim().is_empty(){
                continue 
            }

            let key_val: Vec<&str> = v.split(":").collect();
            let key= key_val[0].trim();
            let val = key_val[1].trim();
            // println!("{} : {}", key,val);
            match key{
                "byr"=> passport.byr = Some(val.parse::<u32>().unwrap()) ,
                "iyr"=> passport.iyr = Some(val.parse::<u32>().unwrap()),
                "eyr"=> passport.eyr = Some(val.parse::<u32>().unwrap()),
                "hgt"=> passport.hgt = Some(String::from(val)),
                "hcl"=> passport.hcl = Some(String::from(val)),
                "ecl"=> passport.ecl = Some(String::from(val)),
                "pid"=> passport.pid = Some(String::from(val)),
                "cid"=> passport.cid = Some(val.parse::<u32>().unwrap()),
                _ => println!("bad value for key {}", key )

            }
        }
        Ok(passport)
    }
}

fn parse_string(input: &str)->Vec<Passport>{

   let passports : Vec<Passport> = input.split("\n\n")
                                    .map(|a| Passport::try_from(a))
                                    .map(|a|a.unwrap())
                                    .collect();
    passports
}

fn parse_file() -> Vec<Passport>{
   let input = read_to_string("data/day_4/input").unwrap();
    parse_string(&input)
}

fn main() {
    println!("Hello, world!");
    let results = parse_file();
    let valid_count = results.iter().filter(|passport| passport.is_valid()).count();
    let valid_count2 = results.iter().filter(|passport| passport.is_valid2()).count();
    println!("there are {} valid passports",valid_count);
    println!("there are {} valid passports in second",valid_count2);
}
