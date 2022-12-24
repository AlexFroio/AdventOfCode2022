use common_utils;
use std::str::{Lines, Chars};
use itertools::{Itertools, Chunk};

fn count_the_char_from_mult_bags (c: Chunk<Lines>) -> i32 {

    //let ch = split_and_decode_the_bag(s);
    let ch = compare_all_3_bags(c);
    let mut res:i32 = ch as i32;

    if res >= b'a' as i32 {
        // subtract the 'a' character's value from our character and start over from 1
        res -= b'a' as i32;
        res += 1;
    }
    else if res >= b'A' as i32 {
        // subtract the 'a' character's value from our character and start over from 27
        res -= b'A' as i32;
        res += 27;
    }

    //println!( "{} is value {}", ch, res);

    return res
}


fn count_the_char (s: &str) -> i32 {

    let ch = split_and_decode_the_bag(s);
    //let ch = compare_all_3_bags(s);
    let mut res:i32 = ch as i32;

    if res >= b'a' as i32 {
        res -= b'a' as i32;
        res += 1;
    }
    else if res >= b'A' as i32 {
        res -= b'A' as i32;
        res += 27;
    }

    //println!( "{} is value {}", ch, res);

    return res
}

fn compare_all_3_bags (mut c: Chunk<Lines>) -> char {
    // Takes 3 chunks of lines and tries to find a common item among the bags.
    let mut item = '[';
    let bag_1 = c.next().unwrap();
    let bag_2 = c.next().unwrap();
    let bag_3 = c.next().unwrap();

    // Compare bags 1 and 2 first. Come out of the comparison with a full list of matches.
    let mut matched_1_2: Vec<char> = Vec::new();
    for ch_1 in bag_1.chars() {
        for ch_2 in bag_2.chars() {
            if ch_1 == ch_2 {
                matched_1_2.push(ch_1);
            }
        }
    }
    // Iterate over the matched list from bags 1 and 2 for 3. Credit for limiting the list goes to DEX for breaking me out of a funk
    for ch_n in matched_1_2 {
        for ch_3 in bag_3.chars() {
            if ch_n == ch_3 {
                item = ch_n;
            }
        }
    }
    // tbh I am annoyed at how not scalable this is. Every bag increases code size by one double for loop.
    // Also it kinda all assumes we will always have a match between all n bags, so we have no case for no matches.

    return item
}

fn split_and_decode_the_bag (s: &str) -> char {
    let mut s1: Vec<char> = s.chars().collect();
    let s2:Vec<char> = s1.split_off(s1.len()/2);
    let mut item_1 = '[';
    //let mut item_2 = ']';

    for ch1 in s1 {
        for ch2 in &s2 {
            if ch1 == *ch2 {
                item_1 = ch1;
            }
        }
    }
    return item_1
}

fn main() {
    let contents = common_utils::open_file("day03\\src\\input2.txt");
    let mut sum: i32 = 0;
    if let Ok(baglist) = contents {
        for lines in &baglist.lines().into_iter().chunks(3){
            sum += count_the_char_from_mult_bags(lines);
            
        }
        println!("Sum of found characters is {}", sum);
    }

}
