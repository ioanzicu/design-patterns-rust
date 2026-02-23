//  rustc 9_2_fibonacci_iterator.rs && ./9_2_fibonacci_iterator
//  rustc --test 9_2_fibonacci_iterator.rs -o 9_2_fibonacci_iterator_test

struct Fibonacci {
    curr: u32,
    next: u32,
    max: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            curr: 0,
            next: 1,
            max: u32::MAX,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        // Option 1. Use `checked_add` to prevent overflow
        // match self.curr.checked_add(self.next) {
        //     Some(next) => {
        //         self.curr = self.next;
        //         self.next = next;
        //         Some(current)
        //     }
        //     None => None, // overflow
        // }

        // Option 2. Pre-check before addition (manual guard)
        if self.curr > self.max - self.next {
            None // overflow
        } else {
            let next = self.next;

            self.next = self.curr + self.next;
            self.curr = next;
            Some(current)
        }
    }
}

fn main() {
    let mut fib_iter = Fibonacci::new();

    for i in 1..=10 {
        println!("#{:?}: {:?}", i, fib_iter.next());
    }

    let out = fib_iter.take(5).collect::<Vec<_>>();
    println!("Next 5: {:?}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_10_fibonacci_numbers() {
        let result: Vec<u32> = Fibonacci::new().take(10).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_take_after_manual_next() {
        let mut fib = Fibonacci::new();

        // Consume first 10 manually
        for _ in 0..10 {
            fib.next();
        }

        let next_five: Vec<u32> = fib.take(5).collect();
        assert_eq!(next_five, vec![55, 89, 144, 233, 377]);
    }

    #[test]
    fn test_iterator_stops_on_overflow() {
        let mut fib = Fibonacci::new();

        let mut last = 0;
        while let Some(val) = fib.next() {
            last = val;
        }

        // Larget FIbinacci that fits into u32
        assert_eq!(last, 1134903170);
    }
}
