use std::io;
use std::convert::TryFrom;

fn sort_list<const S : usize>(list_in : &mut Vec<u32>, mut buckets : [Vec<u32>; S]) -> Vec<u32> {
    let max_value = *list_in.iter().max().unwrap();
    let num_buckets : u32 = u32::try_from(buckets.len() - 1).unwrap();

    for i in list_in.iter_mut() {
        let bucket_index : u32 = (*i / max_value) * num_buckets;
        buckets[usize::try_from(bucket_index).unwrap()].push(*i);
    }

    let mut list_out : Vec<u32> = Vec::new(); 
    for b in buckets.iter_mut() {
        b.sort();
        list_out.append(b);
    }

    list_out
}

fn main() {
    let mut list : Vec<u32> = Vec::new();

    println!("Enter numbers (> 0) one after another, type stop to begin sorting :");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Err(_) => { 
                break;
            },
            Ok(num) => num
        };
        list.push(input);
    }
    println!("{:?}", list);

    // Sort list using 5 buckets
    const NUM_BUCKETS : usize = 5;

    // Default only works for up to 32 elements
    let mut buckets : [Vec<u32>; NUM_BUCKETS] = Default::default();
    let sorted_list : Vec<u32> = sort_list(&mut list, buckets);

    println!("{:?}", sorted_list);
}
