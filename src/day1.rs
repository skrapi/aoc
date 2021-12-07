
pub fn parse_input_day1(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            l.parse::<usize>().unwrap()            
        }).collect()
}


pub fn solve_part1(input: Vec<usize>) -> usize {
    let mut number_of_larger_measurements = 0;
    let mut previous_measurement = 0;
    for (index, &measurement) in input.iter().enumerate() {
        if index == 0 {
            previous_measurement = measurement;
            continue;
        }

        if measurement > previous_measurement {
            number_of_larger_measurements += 1;
        }
        
        previous_measurement = measurement;
    }

    number_of_larger_measurements
}

