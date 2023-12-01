#![feature(iter_array_chunks)]

fn main() {
    const N: usize = 1000;
    let crazy = NotFloat::<N>::new("123.456");
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
        let negative = input.starts_with('-');
        let exponent = (input.find('.').unwrap_or(input.len()) as isize - input.len() as isize) as i128 + 1;
        let mut value = [0u8; N];
        let mut hold = 0u8;
        
        for (i, [x, y, z]) in input.replace(".", "").replace("-", "").chars().rev().array_chunks().enumerate() {
            let mut sum: u16 = format!("{}{}{}", z, y, x).parse().expect("Failed to parse");
            sum += hold as u16;
            hold = (sum / u8::MAX as u16) as u8;
            let final_value = (sum % u8::MAX as u16) as u8;
            
            value[i as usize] = final_value;
        }

        Self {
            negative,
            value,
            exponent,
        }
    }
}