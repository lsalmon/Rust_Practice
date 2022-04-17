use std::io;
use std::convert::TryFrom;

#[derive(Debug)]
struct Bucket {
    radix: u32,
    elements: Vec<u32>
}
 
impl Bucket {
    fn new(radix: u32) -> Self {
        Bucket {
            radix,
            elements: Vec::new()
        }
    }

    fn add(&mut self, element: u32) {
        self.elements.push(element);
    }
 
    fn remove(&mut self, element: u32) {
        if let Some(index) = self.elements.iter().position(|x| *x == element) {
            self.elements.remove(index);
        }
    }
}

fn return_radix(num : &mut u32, exponent : &u32) -> u32 {
    let mut digits : Vec<u32> = get_digits(num);
    digits.reverse();
    let value;
    let exp = *exponent;
    match digits.get(usize::try_from(exp).unwrap()) {
        None => value = 0,
        Some(index) => value = *index
    }

    value
}

fn get_digits(num : &mut u32) -> Vec<u32> {
    num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}

fn iterate_buckets(list_in : Vec<Bucket>, exponent : u32) -> Vec<Bucket> {
    let mut list_out : Vec<Bucket> = Vec::new();
    for i in 0..10 {
        list_out.push(Bucket::new(i));
    }

    for bucket in list_in.into_iter() {
        for mut num in bucket.elements.into_iter() {
            let radix = return_radix(&mut num, &exponent);

            let mut index = bucket.radix;
            // Construct new list of buckets
            if radix != bucket.radix {
                index = radix;
            }
                
            if let Some(bucket_out) = list_out.get_mut(index as usize) {
                bucket_out.add(num);
            }
        }
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
    //vec![170, 45, 75, 90, 2, 802, 2, 66];
    let mut max_value : u32 = *list.iter().max().unwrap();
    let max_exp = u32::try_from(get_digits(&mut max_value).len()).unwrap() + 1;
    let mut list_buckets : Vec<Bucket> = Vec::new();

    for i in 0..10 {
        list_buckets.push(Bucket::new(i));
    }

    // Generate first list of buckets from input list
    for mut num in list.iter_mut() {
        let radix = return_radix(&mut num, &0);
        for bucket in list_buckets.iter_mut() {
            if bucket.radix == radix {
                bucket.add(*num)
            } 
        }
    }

    // Iterate until it is solved
    for exponent in 1..max_exp {
        list_buckets = iterate_buckets(list_buckets, exponent);
    }

    // Flatten buckets into output vector
    let mut sorted_list: Vec<u32> = Vec::new();

    for mut bucket in list_buckets.into_iter() {
        sorted_list.append(&mut bucket.elements);
    }

    println!("{:?}", sorted_list);
}
