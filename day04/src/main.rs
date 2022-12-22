use common_utils;

fn is_this_duty_overlap (s:&str) -> bool {

    let ranges = split_str_into_duties(s);
    let mut inside = false;
    for i in ranges[0]..=ranges[1] {
        print!("{} ", i);
        if (ranges[2]..(ranges[3] + 1)).contains(&i) { 
            inside = true 
        }
        else {
            //inside = false;
        }
    }
    println!("{}", inside);
    if !inside {
        return inside;
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
