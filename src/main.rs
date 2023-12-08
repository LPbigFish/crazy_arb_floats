use std::ptr::slice_from_raw_parts;

fn main() {
    const N: usize = 1000;

    let time = std::time::Instant::now();
    let crazy = NotFloat::<N>::new("72509328750932857032985730598732098375093827039587320958730598327503298750329857302958732095873502987523");
    println!("Time: {:?}", time.elapsed());

    println!("{:?}", crazy);
}

#[derive(Debug)]
struct NotFloat<const N: usize> {    
    negative: bool,
    value: [u8; N],
    exponent: i128,
}

impl<const N: usize> NotFloat<N> {
    fn new(input: &str) -> Self {
        //check negativity
        let negative = input.starts_with("-");
        //create exponent value
        let exponent = (input.find('.').unwrap_or(input.len()) as isize - input.len() as isize) as i128 + 1;
        //create buffer
        let mut value = [0u8; N];
        //create only number String
        let mut slice = input.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).expect("How did you fail to parse a simple number?") as u8).collect::<Vec<u8>>();
        let mut binary: Vec<u8> = Vec::new();

        while (slice.len() > 1) || (slice.last().unwrap() != &0u8) {
            let (result, remainder) = Self::divide_str_by_two(&slice);
            binary.push(remainder as u8);
            slice = result;
        }
        println!("{:?}", binary.as_mut_slice().tap(|v| v.reverse()).into_iter().map(|x| x.to_string()).collect::<String>());

        binary.chunks(8).enumerate().for_each(|(i, chunk)| {
            let mut byte = 0u8;
            chunk.iter().enumerate().for_each(|(j, bit)| {
                byte += bit * 2u8.pow(j as u32);
            });
            value[i] = byte;
        });

        Self {
            negative,
            value: value.tap(|v| v.reverse()),
            exponent,
        }
    }

    fn divide_str_by_two(input: &Vec<u8>) -> (Vec<u8>, bool) {
        //convert input to a vector of digits
        let mut slice = input.clone();
        while slice.first().unwrap() == &0u8 {
            slice.pop();
        }
        //reverse it
        slice.reverse();
        //create final vector and carry variable
        let mut result = Vec::new();
        let mut carry = 0u8;

        //while there are still digits in the input vector
        while !slice.is_empty() {
            //pop the last digit and add the carry, times 10
            let sum = carry * 10 + slice.pop().expect("How did you pop this?");
            //push the result of dividing by 2 to the result vector (there is no decimal remainder)
            result.push((sum / 2) as u8);
            //set the carry to the remainder
            carry = sum % 2;
        }

        //return the result vector and the carry
        (result, carry == 1)
    }
}

trait Tap {
    fn tap(self, f: impl FnMut(&mut Self)) -> Self;
}

impl<T> Tap for T {
    fn tap(mut self, mut f: impl FnMut(&mut Self)) -> Self {
        f(&mut self);
        self
    }
}