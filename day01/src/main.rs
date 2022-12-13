use std::{fs::File, io::{BufReader, Read}};

pub fn open_file(filename: &str)-> std::io::Result<String> {
    let file= File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    return Ok(contents);
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    num:u32,
    total_cal:u64,
}

impl Default for Elf {
    fn default() -> Self {
        Elf{num:1, total_cal:0}
    }
}


fn sum_cal(in_str:String) -> Vec<Elf>{
    let mut elves:Vec<Elf> = Vec::new();
    elves.push(Elf::default());
    for line in in_str.lines() {
        if line != "" {
            elves.last_mut().unwrap().total_cal += line.parse::<u64>().unwrap()
        }
        else {
            elves.push(Elf{num: elves.len() as u32 + 1, total_cal: 0});
        }
    }
    return elves;
}

fn main() {
    let contents = open_file("day01\\src\\input.txt");

    if let Ok(foo) = contents {
        let mut elves = sum_cal(foo);
        elves.sort_by(|a, b| b.total_cal.cmp(&a.total_cal));

        for elf in elves.clone() {
            println!("{}, {}", elf.num, elf.total_cal);
        }
        let top_3_sum = elves[0].total_cal + elves[1].total_cal + elves[2].total_cal;
        println!("{}", top_3_sum);
    }
    else{
        println!("You're an idiot");

    }

}
