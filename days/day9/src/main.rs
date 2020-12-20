use std::fs::read_to_string;
use std::convert::TryFrom;
use std::slice::Windows;
use std::collections::HashSet;

#[derive(Debug)]
struct Code{
    values:Vec<i64>
}


impl Code{
    fn find_pair(values: &[i64], target:i64)->bool{
        let hash_map: HashSet<i64> = values.into_iter().copied().collect();
        for val in &hash_map{
            let remainder = target- val;
            if hash_map.contains(&remainder){
                return true
            }
        }
        false
    }
    fn check_code(&self)-> Option<i64>{
        for window in self.values.as_slice().windows(26){
            let pair_found = Self::find_pair(&window[0..25], window[25]);
            if(!pair_found){
                return Some(window[25])
            }
        }   
        None
    }

    fn weakness(&self,target:i64)->Option<i64>{
        for region_start in 0..self.values.len(){
            let mut running_total = 0;
            for i in region_start..self.values.len(){
                running_total+= self.values[i];
                if(running_total==target){
                    let min = self.values[region_start..i].iter().min().unwrap(); 
                    let max= self.values[region_start..i].iter().max().unwrap(); 
                    return Some(min +max);
                }
                if(running_total>target){
                    break
                }
            } 
        }
        None
    }
}

impl TryFrom<&str> for Code{
    type Error = &'static str;

    fn try_from(value: &str)->Result<Self,Self::Error>{
        let values : Vec<i64>= value.lines().map(|l| l.parse::<i64>().unwrap()).collect();

        return Ok(Code{values})
    }
} 

fn code_from_string(s: &str)->Code{
    Code::try_from(s).unwrap()
}

fn read_code()->Code{
    let input = read_to_string("data/day_9/input").unwrap();
    code_from_string(&input)
}

fn main() {
    let code = read_code();
    match code.check_code(){
        Some(fail)=>{
            println!("code failed on number {}",fail);
            let weakness = code.weakness(fail).unwrap();
            println!("the weakness is {}", weakness);
        },
        None =>println!("Code checks out")
    };
}
