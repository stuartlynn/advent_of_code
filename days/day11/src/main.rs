use std::fs::read_to_string;
use std::convert::TryFrom;

enum Position{
    Empty,
    Floor,
    Taken
}

struct SeatingArangement{
    seats: Vec<Position>,
    rows: usize,
    columns: usize
}
impl TryFrom<&str> for SeatingArangement{
    type Error = &'static str;
    fn try_from(string: &str)->Result<Self,Self::Error>{

        let parsed_vals: Result<Vec<Position>,_> = string.split_ascii_whitespace().map(|s| match s{
            "#"=> Ok(Position::Taken),
            "."=> Ok(Position::Floor),
            "L"=> Ok(Position::Empty),
            _ =>  Err("Unexpected character in file")
        }).collect();

        Ok(Self{
            seats:vec![],
            rows:0,
            columns:0
        })
    }
}

impl SeatingArangement{
    fn at(&self, x: usize, y:usize)->Option<&Position>{
        let idx = x+ y*self.rows;
        self.seats.get(idx)
    }
}


fn main() {
    println!("Hello, world!");
}
