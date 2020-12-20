use std::fs::read_to_string;
use std::convert::TryFrom;
#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct BagContent{
    bag_type : String,
    quantity: u32  
}

#[derive(Debug)]
struct Bag{
    bag_type: String,
    contents: Vec<BagContent>    
}

impl Bag{
    fn can_contain(&self,bag_type: &str, bags: &Vec<Bag>)->bool{
        let mut queue: Vec<&Bag>= vec![&self];
        while !queue.is_empty(){
            let bag = queue.pop().unwrap();

            if bag.directly_contains(bag_type){
                return true
            }
            
            else{
                for sub_bag in &bag.contents{
                    let b = bags.iter().find(|bag| bag.bag_type == sub_bag.bag_type).unwrap();
                    queue.push(b);
                }
            }
        }

        return false;
    }

    fn directly_contains(&self,bag_type: &str)->bool{
        self.contents
            .iter()
            .map(|bc| bc.bag_type.clone())
            .collect::<Vec<String>>()
            .contains(&String::from(bag_type))
    }

    fn sum_sub_bags(&self,bags: &Vec<Bag>)->u32{
        let mut total = 0;
        for bag_content in &self.contents{
            let sub_bag = bags.iter().find(|b| b.bag_type==bag_content.bag_type).unwrap(); 
            total += bag_content.quantity * (1 + sub_bag.sum_sub_bags(bags));
        }
        total
    }
}


impl TryFrom<&str> for Bag{
    type Error = &'static str;
    fn try_from(value:&str) -> Result<Self,Self::Error>{

        lazy_static! {
            static ref BagRE: Regex = Regex::new(r"(?P<bag_type>.*) bags contain (?P<content>.*).").unwrap();
        }

        let captures = BagRE.captures(value).unwrap();
        let bag_type  = String::from(captures.name("bag_type").unwrap().as_str());
        let content = captures.name("content").unwrap().as_str();
        match content=="no other bags"{
            false=>{
                let contents: Vec<BagContent> = content.split(",").map(|c| BagContent::try_from(c.trim()).unwrap()).collect();
                Ok(Self{
                    bag_type,
                    contents
                })
            },
            true=>{
                Ok(Self{
                    bag_type,
                    contents: vec![]
                })
            } 
        }
    }
} 

impl TryFrom<&str> for BagContent{
    type Error = &'static str;
    fn try_from(value:&str) -> Result<Self,Self::Error>{
        lazy_static! {
            static ref BagContentRE: Regex = Regex::new(r"(?P<number>\d*) (?P<bag_type>.*) bags?").unwrap();
        }

        let captures = BagContentRE.captures(value).unwrap();
        let bag_type  = String::from(captures.name("bag_type").unwrap().as_str());

        let quantity= captures.name("number").unwrap().as_str().parse::<u32>().unwrap();
        Ok(Self{
            bag_type,
            quantity
        })
    }
} 

fn parse_bag_list(text:&str)->Vec<Bag>{
    text.lines().map(|line| Bag::try_from(line).unwrap()).collect()
}
fn load_bag_list()->Vec<Bag>{
    let text = read_to_string("data/day_7/input").unwrap();
    parse_bag_list(&text)
}

fn main() {
    let bags = load_bag_list();
    let target_bag = "shiny gold";
    let mut count =0;

    for bag in &bags{
           if bag.can_contain(target_bag, &bags){
               count +=1;
           }
    }
    println!("answer {}",count);
    
    let result = bags.iter().find(|b| b.bag_type==target_bag).unwrap().sum_sub_bags(&bags);
    println!(" sum result {:?}",result);
}
