use std::io;

fn merge_tabs(vec_in : Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut vec_out : Vec<Vec<u32>> = Vec::new();

    let mut i = 0;
    while i <= vec_in.len()-1 {
        let tab1 : Vec<u32> = match vec_in.get(i) {
            None => {
                break;
            },
            Some(vec) => vec.to_vec()
        };
        let mut tab_merged : Vec<u32> = Vec::new();
        let tab2 : Vec<u32> = match vec_in.get(i+1) {
            None => {
                // Uneven number of vecs in vec
                vec_out.push(tab1);
                break;
            },
            Some(vec) => vec.to_vec()
        };

        // Move tab1 and tab2 to result buffer
        tab_merged.extend(tab1);
        tab_merged.extend(tab2);

        tab_merged.sort();

        vec_out.push(tab_merged);

        i = i+2;
    }

    vec_out
}

fn main() {
    let mut list : Vec<u32> = Vec::new();

    println!("Enter numbers (> 0) one after another, type stop to begin sorting :");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input : u32 = match input.trim().parse() {
            Err(_) => { 
                break;
            },
            Ok(num) => num
        };
        list.push(input);
    }

    println!("{:?}", list);

    let mut tabs : Vec<Vec<u32>> = Vec::new();

    // Generate first vector of vector from list (all individual numbers in a vector)
    for num in list.iter_mut() {
        let vec_num : Vec<u32> = vec![*num];
        tabs.push(vec_num);
    }

    // Iterate until it is solved
    while tabs.len() > 1 {
        tabs = merge_tabs(tabs);
    }

    // Flatten vector of vectors into output vector
    let mut sorted_list : Vec<u32> = Vec::new();

    for mut tab in tabs.into_iter() {
        sorted_list.append(&mut tab);
    }

    println!("{:?}", sorted_list);
}
