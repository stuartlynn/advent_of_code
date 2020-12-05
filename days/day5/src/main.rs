use std::fs::read_to_string;
use std::convert::TryFrom;

#[derive(Debug)]
struct Ticket{
    row: usize,
    col: usize,
    no: usize
}

impl TryFrom<&str> for Ticket{
    type Error = &'static str;
    
    fn try_from(value:&str) -> Result<Self,Self::Error>{
        if value.len()!= 10{
            return Err("Ticket id not 10 chars");
        }

        let b_string= value.replace("F","0")
                .replace("B","1")
                .replace("R", "1")
                .replace("L", "0");
        
        let (row,col)= b_string.split_at(7);
        let row = usize::from_str_radix(&row,2).unwrap();
        let col = usize::from_str_radix(&col,2).unwrap();
        Ok(Self{
            row,
            col,
            no: row*8 + col
        })
    }
}

fn read_data()->Vec<Ticket>{
    let file = read_to_string("data/day_5/input").unwrap();
    file.lines().map(|line|{ 
       Ticket::try_from(line).unwrap() 
    }).collect()
}

fn main() {
    let tickets = read_data();
    let mut ticket_nos:Vec<usize> = tickets.iter().map(|t| t.no).collect();
    let min = ticket_nos.iter().min().unwrap();
    let max = ticket_nos.iter().max().unwrap();

    for i in (*min..*max){
        if !ticket_nos.contains(&i) && (ticket_nos.contains(&(i+1)) && ticket_nos.contains(&(i-1))) {
            println!("missing {}",i);
        }
    }
    println!("max ticket no is {}", max );
    // println!("{:?}", ticket_nos)
}
