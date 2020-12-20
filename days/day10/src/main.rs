use std::fs::read_to_string;

fn parse_data(string: &str)->Vec<usize>{
    string.lines()
          .map(|l| l.parse::<usize>().unwrap())
          .collect::<Vec<usize>>()
}

fn read_data()->Vec<usize>{
    let string = read_to_string("data/day_10/input")
                  .unwrap();
    parse_data(&string)
}

fn calc_diffs(adapters: &Vec<usize>) -> Vec<usize>{
    adapters.as_slice().windows(2).map(|pair| pair[1]-pair[0]).collect()
}
fn is_valid(adapters: &Vec<usize>)->bool{
    let diffs =calc_diffs(adapters);
    adapters[0]<3 && diffs.iter().filter(|x| **x>3).count() == 0
}
fn count_configurations(adapters: &Vec<usize>, level : usize)->usize{
    let mut total_valid =0;
    for i in 0..adapters.len()-1{
        if level ==0{
            println!("exploring removal of item {} ", i);
        }
        let mut new_config = adapters.clone();
        new_config.remove(i);
        if is_valid(&new_config){
            total_valid  += count_configurations(&new_config, level+1) + 1 
        }
    };
    total_valid
}

fn main() {
    let mut adapters = read_data();
    adapters.sort();
   
    let differences = calc_diffs(&adapters);
    let one_jolts = differences.iter().filter(|v| **v==1).count();
    let three_jolts = differences.iter().filter(|v| **v==3).count();
    println!("Adapter list {:?} ",adapters );
    println!("differences {:?} ",differences);
    println!("one {:?} three {:?}",one_jolts+1, three_jolts+1);
    println!("Answer {}", (one_jolts+1)*(three_jolts+1));

    let test_string = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    let mut test_adapters = parse_data(&test_string);
    test_adapters.sort(); 
    let differences = calc_diffs(&test_adapters);
    let one_jolts = differences.iter().filter(|v| **v==1).count();
    let three_jolts = differences.iter().filter(|v| **v==3).count();
    println!("Test Adapter list {:?} ",adapters );
    println!("Test differences {:?} ",differences);
    println!("Test one {:?} three {:?}",one_jolts+1, three_jolts+1);
    println!("Test Answer {}", (one_jolts+1)*(three_jolts+1));

    let configs = count_configurations(&test_adapters,0);
    println!("for test there are {} permutations",configs);
}
