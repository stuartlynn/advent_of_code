extern crate csv;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn load_data()-> Vec<i32>{
    let mut reader = csv::Reader::from_path("data/day_1/input").expect("could not load input file");
    reader.deserialize().map(|result| {
        // let string_value: String = result.unwrap();
        let num : i32 = result.unwrap(); 
        num
    }).collect()
}

pub fn find_pair_sums_to(data: &HashSet<i32>, target: i32)->Option<[i32;2]>{
    for datum in data{
        let looking_for = target - datum;
        if data.contains(&looking_for){
            return Some([*datum, looking_for])
        }
    }
    None
}

pub fn find_match2(data: &HashSet<i32>, target: i32)->Option<[i32;2]>{
    find_pair_sums_to(data, target)
}

pub fn find_match3(data: &HashSet<i32>, target: i32)->Option<[i32;3]>{
    for num1 in data.into_iter(){
        let remainder = target - num1;
        match find_pair_sums_to(data,remainder){
            Some(pair) => return Some([pair[0], pair[1], *num1]),
            _ => {}
        }
    }
    return None
}

pub fn run_problem_1(){
    let data = load_data();
    let hashset : HashSet<i32> = HashSet::from_iter(data.iter().cloned());
    let matches2 = find_match2(&hashset, 2020);

    match matches2{
        Some(result) =>{
            println!("matches are {:?}",result);
            println!("Their product is {}", result[0] * result[1])
        },
        None => println!("No matches found") 
    }

    let matches3 = find_match3(&hashset, 2020);

    match matches3{
        Some(result) =>{
            println!("matches are {:?}",result);
            println!("Their product is {}", result[0] * result[1] * result[2])
        },
        None => println!("No matches found") 
    }
}
