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
        let negative = input.starts_with('-');
        let exponent = (input.find('.').unwrap_or(input.len()) as isize - input.len() as isize) as i128 + 1;
        let mut value = [0u8; N];
        let mut slice = input.replace(".", "").replace("-", "").replace(" ", "");
        let mut binary: Vec<u8> = Vec::new();

        while (slice.len() > 1) || (slice.chars().last().unwrap() != '0') {
            slice = slice.trim_start_matches('0').to_string();
            let (result, remainder) = Self::divide_str_by_two(&slice);
            binary.push(remainder as u8);
            slice = result.iter().map(|n| n.to_string()).collect::<String>();
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

    fn divide_str_by_two(input: &str) -> (Vec<u8>, bool) {
        //convert input to a vector of digits
        let mut slice = input.chars().map(|c| c.to_digit(10).expect("Failed to parse!")).collect::<Vec<u32>>();
        //reverse it
        slice.reverse();
        //create final vector and carry variable
        let mut result = Vec::new();
        let mut carry = 0u32;

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