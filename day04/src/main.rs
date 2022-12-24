use common_utils;

fn is_this_duty_overlap (s:&str) -> bool {
    // This function's logic was flipped once the problem became needing to report
    // any overlap at all. 
    let ranges = split_str_into_duties(s);
    let mut inside = false;
    for i in ranges[0]..=ranges[1] {
        print!("{} ", i);
        if (ranges[2]..(ranges[3] + 1)).contains(&i) { 
            // Initially, for part 1, this was empty. We didn't want to flip a switch unless
            // we knew we had exited the set.
            inside = true 
        }
        else {
            //For part 1, inside started as True, would get set to false on exiting the set.
            //inside = false;
        }
    }
    println!("{}", inside);
    // For part 1, this was just `if inside` and then we would reset inside to true afterwards
    // I am annoyed at myself for having 2 return points but this was the easiest way to return a true
    // with how I worked out my pattern and design logic.
    if !inside {
        return inside;
        // If you're wondering why I'm annoyed, it's because in C Safety programming, multiple return paths from a 
        // function are considered unsafe design and no code should exist without explicit need to serve some sort of
        // effect, side effect or affect. AKA if you can prove the latter half of this is dead code, it should be 
        // reworked.
    }
    inside = false; 
    for i in ranges[2]..=ranges[3] {
        print!("{} ", i);
        if (ranges[0]..(ranges[1] + 1)).contains(&i) { 
            inside = true;
        }
        else {
            //inside = false;
        }

    }
    println!("{}", inside);
    return inside
}

fn split_str_into_duties (s: &str) -> Vec<u32>{
    let mut data: Vec<u32> = Vec::new();
    for sp in s.split(","){
        for sp2 in sp.split("-") {
            data.push(sp2.parse().unwrap())
        }
    }
    return data
}

fn main() {
    let contents = common_utils::open_file("day04\\src\\input2.txt");
    let mut sum: i32 = 0;
    if let Ok(baglist) = contents {
        for line in baglist.lines() {//.into_iter().chunks(3){
            sum += is_this_duty_overlap(line) as i32; 
            println!("{} and sum is {}", line, sum);

        }
        println!("Sum of found characters is {}", sum);
    }
    else {
        println!("Can't read the file idiot!")
    }
}
