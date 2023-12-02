fn main() {
    const N: usize = 1000;

    let time = std::time::Instant::now();
    let crazy = NotFloat::<N>::new("3456789014021.8992148908174092818042974092817402918792014");
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
        let negative = input.starts_with('-');
        let exponent = (input.find('.').unwrap_or(input.len()) as isize - input.len() as isize) as i128 + 1;
        let mut value = [0u8; N];
        let mut slice = input.replace(".", "").replace("-", "");
        let mut binary: Vec<u8> = Vec::new();

        while (slice.len() > 1) || (slice.chars().last().unwrap() != '0') {
            slice = slice.trim_start_matches('0').to_string();
            let (result, remainder) = Self::divide_str_by_two(&slice);
            binary.push(remainder as u8);
            slice = result.iter().map(|n| n.to_string()).collect::<String>();
        }
        binary.reverse();
        println!("{:?}", binary.into_iter().map(|x| x.to_string()).collect::<String>());

        Self {
            negative,
            value,
            exponent,
        }
    }

    fn divide_str_by_two(input: &str) -> (Vec<u8>, bool) {
        let mut slice = input.chars().map(|c| c.to_digit(10).expect("Failed to parse!")).collect::<Vec<u32>>();
        slice.reverse();
        let mut result = Vec::new();
        let mut carry = 0u32;

        while !slice.is_empty() {
            let sum = carry * 10 + slice.pop().expect("How did you pop this?");
            result.push((sum / 2) as u8);
            carry = sum % 2;
        }

        (result, carry == 1)
    }
}