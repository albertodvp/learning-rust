use std::io;

fn main() {
    println!("Compute the n-th fib number (starting from 0 index). n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot parse the line");
    let n: u32 = input.trim().parse().expect("Cannot parse the number");
    println!("Result {}", fib(n));
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 1
    }
    if n == 1 {
        return 1
    }
    fib(n-1) + fib(n-2)
}

#[cfg(test)]
mod test {
    use crate::fib;

    #[test]
    fn fib0() {
        assert_eq!(fib(0), 1)
    }
    #[test]
    fn fib1() {
        assert_eq!(fib(1), 1)
    }
    
    #[test]
    fn fibn() {
        assert_eq!(fib(11), 144)
    }

}
