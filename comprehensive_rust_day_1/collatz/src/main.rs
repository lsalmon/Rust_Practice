/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1; 
    if n == 1 {
        length
    } else if n % 2 == 0 {
        length += collatz_length(n/2);
        length
    } else {
        length += collatz_length((3*n)+1);
        length
    }
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("{}", collatz_length(11));
}
