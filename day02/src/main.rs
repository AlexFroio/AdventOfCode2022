use common_utils;

fn score_from_str(s: &str) -> i32 {
    let v: Vec<char> = s.chars().collect();
    let p1:Throw = v[0].try_into().unwrap();
    let p2:Throw = v[2].try_into().unwrap();
    let mut scr: i32 = 0;

    if p1 == p2 {
        scr = i32::from(Outcome::Draw) + i32::from(p2);
    }
    else {
        if p1 == Throw::Rock {
            if p2 == Throw::Paper {
                scr = i32::from(Outcome::Win) +i32::from(p2);
            }
            else if p2 == Throw::Scissor {
                scr = i32::from(Outcome::Lose) + i32::from(p2);
            }
        }
        else if p1 == Throw::Paper {
            if p2 == Throw::Scissor {
                scr =i32::from(Outcome::Win) + i32::from(p2);
            }
            else if p2 == Throw::Rock {
                scr = i32::from(Outcome::Lose) + i32::from(p2);
            }
        }
        else if p1 == Throw::Scissor {
            if p2 == Throw::Rock {
                scr = i32::from(Outcome::Win) + i32::from(p2);
            }
            else if p2 == Throw::Paper {
                scr = i32::from(Outcome::Lose) + i32::from(p2);
            }
        }
    }
    
    
    println!( "score is {}", scr);
    return scr
}

fn score_from_str_flipped(s: &str) -> i32 {
    let v: Vec<char> = s.chars().collect();
    let p1:Throw = v[0].try_into().unwrap();
    let p2:Outcome = v[2].try_into().unwrap();
    let mut scr: i32 = 0;

    if p2 == Outcome::Draw {
        scr = i32::from(Outcome::Draw) + i32::from(p1);
    }
    else if p2 == Outcome::Win {
        if p1 == Throw::Paper {
            scr = i32::from(Outcome::Win) + i32::from(Throw::Scissor);
        }
        else if p1 == Throw::Rock {
            scr = i32::from(Outcome::Win) + i32::from(Throw::Paper);
        }
        else if p1 == Throw::Scissor {
            scr = i32::from(Outcome::Win) + i32::from(Throw::Rock);
        }
    }
    else if p2 == Outcome::Lose {
        if p1 == Throw::Paper {
            scr = i32::from(Outcome::Lose) + i32::from(Throw::Rock);
        }
        else if p1 == Throw::Rock {
            scr = i32::from(Outcome::Lose) + i32::from(Throw::Scissor);
        }
        else if p1 == Throw::Scissor {
            scr = i32::from(Outcome::Lose) + i32::from(Throw::Paper);
        }
    }

    return scr
}

#[derive(PartialEq)]
enum Throw {
    Rock,
    Paper,
    Scissor,
}

impl TryFrom<char> for Throw {
    type Error = ();
    fn try_from(var: char) -> Result<Self, Self::Error> {
        if (var == 'A') || (var == 'X') {
            Ok(Throw::Rock)
        }
        else if (var == 'B') || (var == 'Y') {
            Ok(Throw::Paper)
        }
        else if (var == 'C') || (var == 'Z') {
            Ok(Throw::Scissor)
        }
        else {
                Err(())
        }
    }
}

impl From<Throw> for i32 {
    fn from(var: Throw) -> Self {
        match var {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissor => 3
        }
    }
}

#[derive(PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<Outcome> for i32 {
    fn from(var: Outcome) -> Self {
        match  var{
            Outcome::Draw => 3,
            Outcome::Lose => 0,
            Outcome::Win => 6
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ();
    fn try_from(var: char) -> Result<Self, Self::Error> {
        match var {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _   => Err(())
        }
    }
}

fn main() {
    let contents = common_utils::open_file("day02\\src\\input2.txt");
    let mut match_sum = 0;
    if let Ok(playbook) = contents {
        
        for line in playbook.lines(){
            match_sum += score_from_str_flipped(line);
            println!("Sum of score is {}", match_sum);
        }
        
        println!("Expected score is {}", match_sum);
    }
    //println!("{:?}", contents);
}
