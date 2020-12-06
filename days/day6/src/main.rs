use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fs::read_to_string;

#[derive(Debug)]
struct Person {
    answers: HashSet<char>,
}

impl TryFrom<&str> for Person {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let answers: HashSet<char> = value.chars().collect();
        Ok(Person { answers })
    }
}

#[derive(Debug)]
struct Group {
    people: Vec<Person>,
}

impl Group {
    fn combined_answers(&self) -> HashSet<char> {
        self.people.iter().fold(HashSet::new(), |mut acc, elem| {
            acc.extend(&elem.answers);
            acc
        })
    }

    fn combined_answers_2(&self) -> HashSet<char> {
        self.people
            .iter()
            .fold(None, |mut acc, elem| match acc {
                None => Some(elem.answers.clone()),
                Some(acc) => Some(acc.intersection(&elem.answers).copied().collect()),
            })
            .unwrap_or_default()
    }

    // fn combined_answers_2(&self) -> HashMap<char, usize> {
    //     let mut counts: HashMap<char, usize> = HashMap::new();
    //     for person in &self.people {
    //         for answer in &person.answers {
    //             if counts.contains_key(answer) {
    //                 *counts.get_mut(answer).unwrap() += 1;
    //             } else {
    //                 counts.insert(*answer, 1);
    //             }
    //         }
    //     }
    //     counts
    // }
}

impl TryFrom<&str> for Group {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let people: Vec<Person> = value
            .lines()
            .map(|l| Person::try_from(l.trim()).unwrap())
            .collect();
        Ok(Group { people })
    }
}

fn read_data() -> Vec<Group> {
    let content = read_to_string("data/day_6/input.txt").unwrap();
    content
        .split("\n\n")
        .map(|group_string| Group::try_from(group_string).unwrap())
        .collect()
}
fn main() {
    let data = read_data();
    let counts_1: Vec<usize> = data
        .iter()
        .map(|group| group.combined_answers().len())
        .collect();

    let counts_2: Vec<usize> = data
        .iter()
        .map(|group| group.combined_answers_2().len())
        .collect();

    let answer: usize = counts_1.iter().sum();
    let answer2: usize = counts_2.iter().sum();
    println!("answer is {}", answer);
    println!("answer2 is {}", answer2);
}
