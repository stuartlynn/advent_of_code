use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum SpaceType{
    Tree,
    Space
}



struct Map{
    layout : Vec<Vec<SpaceType>>,
    no_cols: usize,
    no_rows: usize
}

impl Map{

    pub fn load_from_file(path: &str)->Self{

        let input = File::open(path).expect("failed to open the input file");
        let buffered = BufReader::new(input);

        let mut layout: Vec<Vec<SpaceType>> = vec![];

        for line in buffered.lines(){
            let l = line.unwrap();
            let row: Vec<SpaceType> = l.chars().map(|c| {match c{
                '.' => Some(SpaceType::Space),
                '#' => Some(SpaceType::Tree),
                _ =>None
            }.unwrap()}).collect();
            layout.push(row);
        }
        let no_cols = layout[0].len();
        let no_rows = layout.len();
        Map{
            layout, 
            no_cols,
            no_rows
        }
    }

    pub fn at(&self, x:usize, y: usize)->Option<&SpaceType>{
        if y >= self.no_rows{
            return None
        }
        let x_wraped = x%self.no_cols;
        Some(&self.layout[y][x_wraped])
    }

}

struct Strategy{
    x_inc: usize,
    y_inc: usize
}

impl Strategy{
    pub fn run_on_map(&self,  map: &Map)->u32{
        let mut x =0;
        let mut y =0;
        let mut tree_count = 0;

        loop{
            match map.at(x,y){
                Some(SpaceType::Tree)=> tree_count+=1,
                None => {
                    println!("run ended at {},{} with {} trees", x,y,tree_count);
                    return tree_count;
                }
                _ =>{}
            }
            x = x+self.x_inc;
            y = y+self.y_inc;
        };
    }
}

fn main() {
    let map = Map::load_from_file("data/day3/input");
    println!("loaded map with, {} rows and {} cols", map.no_rows, map.no_cols);
    assert_eq!(map.at(0,0).unwrap(), &SpaceType::Space, "First square in row 1 should be space but isnt");
    assert_eq!(map.at(0,2).unwrap(), &SpaceType::Tree, "First square in row 3 should be tree but isnt");
    assert_eq!(map.at(31,2).unwrap(), &SpaceType::Tree, "First square in row 3 when wrapped should be tree but isnt");
    assert_eq!(map.at(31,map.no_rows+2), None, "Out of bound y coord should be None");

    let s1 = Strategy{x_inc:1, y_inc:1};
    let s2 = Strategy{x_inc:3, y_inc:1};
    let s3 = Strategy{x_inc:5, y_inc:1};
    let s4 = Strategy{x_inc:7, y_inc:1};
    let s5 = Strategy{x_inc:1, y_inc:2};

    let s1_result  = s1.run_on_map(&map);
    let s2_result  = s2.run_on_map(&map);
    let s3_result  = s3.run_on_map(&map);
    let s4_result  = s4.run_on_map(&map);
    let s5_result  = s5.run_on_map(&map);

    let total = s1_result*s2_result*s3_result*s4_result*s5_result;
    println!("Result is {}", total);
}
