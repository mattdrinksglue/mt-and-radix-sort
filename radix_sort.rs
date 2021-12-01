use std::vec::Vec;
use std::num::Wrapping;
use std::convert::TryInto;
use std::fmt::Display;

const STATE_SIZE: usize = 624;
const VAL_1: Wrapping<u32> = Wrapping(0x80000000);
const VAL_2: Wrapping<u32> = Wrapping(0x7fffffff);
const VAL_3: Wrapping<u32> = Wrapping(0x9908b0df);

// TODO: find out why bitmask version runs slow
// TODO: make code look nicer and reduce number of copied arrays
// TODO: impl bucket sort version of radix sort

struct MT {
    state: [Wrapping<u32>; STATE_SIZE],
    next: u32,
}

impl MT {
    fn new(seed: u32) -> MT {
        let mut state = [Wrapping(seed); STATE_SIZE];
        for i in 1..STATE_SIZE {
            state[i] = Wrapping(1812433253) * (state[i-1] ^ (state[i-1] >> 30)) + Wrapping(i as u32);
        }
        MT{state, next: 0}
    }

    fn twist(&mut self) {
        const M: usize = 397;
        let first_half: usize = STATE_SIZE as usize - M;

        for i in 0..first_half {
            let bits: Wrapping<u32> = (self.state[i] & VAL_1) | (self.state[i+1] & VAL_2);
            self.state[i] = self.state[i+M] ^ (bits >> 1) ^ ((bits & Wrapping(1)) * VAL_3);
        }

        for i in first_half..STATE_SIZE-1 {
            let bits: Wrapping<u32> = (self.state[i] & VAL_1) | (self.state[i+1] & VAL_2);
            self.state[i] = self.state[i-first_half] ^ (bits >> 1) ^ ((bits & Wrapping(1)) * VAL_3);
        }
        let bits: Wrapping<u32> = (self.state[STATE_SIZE-1] & VAL_1) | (self.state[0] & VAL_2);
        self.state[STATE_SIZE-1] = self.state[M-1] ^ (bits >> 1) ^ ((bits & Wrapping(1)) * VAL_3);

        self.next = 0;
    }

    fn next_rand(&mut self) -> u32 {
        if self.next >= STATE_SIZE as u32 {
            self.twist();
        }

        let mut x: u32 = self.state[self.next as usize].0;
        self.next = self.next + 1;

        x ^= x >> 11;
        x ^= (x << 7) & 0x9d2c5680;
        x ^= (x << 15) & 0xefc60000;
        x ^= x >> 18;
        return x;
    }
}

fn print_vec<T: Display>(vec: &Vec<T>) {
    for i in vec.iter() {
        println!("{}", i);
    }
    println!("\n");
}

fn counting_sort(arr: &mut [u32], arr_len: usize, base: u32, nth_digit: u32) -> Vec<u32> {
    let mut output = vec![0; arr_len];

    let mut digit_count = vec![0; base.try_into().unwrap()];

    let exp: u32 = base.pow(nth_digit);

    for i in 0..arr_len {
        let index = arr[i] / exp;
        digit_count[(index % base) as usize] += 1;
    }

    for i in 1..base as usize {
        digit_count[i] += digit_count[i-1];
    }

    for i in (0..arr_len).rev() {
        let index = arr[i] / exp;
        output[digit_count[(index % base) as usize] - 1] = arr[i];
        digit_count[(index % base) as usize] -= 1;
    }

    output
}

fn radix_sort(arr: Vec<u32>, base: u32) -> Vec<u32> {
    let max: u32 = match arr.iter().max() {
        Some(x) => *x,
        _ => panic!("Empty array provided")
    };

    let mut output = arr;

    let mut digit = 0;
    while max as u64 > (base as u64).pow(digit) {
        let len = output.len();
        output = counting_sort(&mut output, len, base, digit);
        digit += 1;
    }

    output
}

fn counting_sort_16(arr: &mut [u32], arr_len: usize, nth_digit: u32) -> Vec<u32> {
    let mut output = vec![0; arr_len];

    let mut digit_count = vec![0; 16];

    let exp: u32 = 16_u32.pow(nth_digit);

    for i in 0..arr_len {
        let index = arr[i] / exp;
        digit_count[(index & 0xF) as usize] += 1;
    }

    for i in 1..16 as usize {
        digit_count[i] += digit_count[i-1];
    }

    for i in (0..arr_len).rev() {
        let index = arr[i] / exp;
        output[digit_count[(index & 0xF) as usize] - 1] = arr[i];
        digit_count[(index & 0xF) as usize] -= 1;
    }

    output
}

fn radix_sort_16(arr: Vec<u32>) -> Vec<u32> {
    let max: u32 = match arr.iter().max() {
        Some(x) => *x,
        _ => panic!("Empty array provided")
    };

    let mut output = arr;

    let mut digit = 0;
    while digit < 8 && u64::from(max) > 16_u64.pow(digit) {
        let len = output.len();
        output = counting_sort_16(&mut output, len, digit);
        digit += 1;
    }

    output
}

fn counting_sort_bitmask(arr: &mut [u32], arr_len: usize, exp: u32) -> Vec<u32> {
    let mut output = vec![0; arr_len];

    let mut digit_count = vec![0; exp as usize];

    for i in 0..arr_len {
        let index = arr[i] / exp;
        digit_count[(index & (exp-1)) as usize] += 1;
    }

    for i in 1..exp as usize {
        digit_count[i] += digit_count[i-1];
    }

    for i in (0..arr_len).rev() {
        let index = arr[i] / exp;
        output[digit_count[(index & (exp-1)) as usize] - 1] = arr[i];
        digit_count[(index & (exp-1)) as usize] -= 1;
    }

    output
}

// radix sort using a bitmask to speed it up a little
// takes in an exponent of 2, such that the mask can be (2^exp)-1
// and bitwise & can be performed
fn radix_sort_bitmask(arr: Vec<u32>, exp: u32) -> Vec<u32> {
    let max: u32 = match arr.iter().max() {
        Some(x) => *x,
        _ => panic!("Empty array provided")
    };

    let mut output = arr;

    let base_exp: u32 = 2_u32.pow(exp);

    let mut digit = 0;
    while digit < (32 / exp - 1) && max > base_exp.pow(digit) {
        let len = output.len();
        output = counting_sort_bitmask(&mut output, len, base_exp.pow(digit));
        digit += 1;
    }

    output
}

fn  main() {
    // generate a list of pseudo-random numbers
    let mut rand: MT = MT::new(6969420);

    let range = 20;
    let mut arr = vec![0; range];
    let mut arr2 = vec![0; range];
    for i in 0..range {
        arr[i] = rand.next_rand();
        arr2[i] = rand.next_rand();
    }

    // run radix sort on all of the random numbers

    let arr_o = radix_sort_bitmask(arr, 4);
    print_vec(&arr_o);

    let arr_o = radix_sort_16(arr2);
    print_vec(&arr_o);
}
