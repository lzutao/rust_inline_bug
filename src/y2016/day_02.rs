
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2016/2");
    let mut code = String::new();
    let mut next_digit = 5;
    for line in INPUT.lines() {
        for char in line.chars() {
            next_digit = match char {
                'U' =>  match next_digit {
                            1..=3 => next_digit,
                            o => o - 3,
                        },
                'D' => match next_digit {
                            7..=9 => next_digit,
                            o => o + 3,
                        },
                'L' => match next_digit {
                            1 | 4 | 7 => next_digit,
                            o => o - 1,
                        },
                'R' => match next_digit {
                    3 | 6 | 9 => next_digit,
                    o => o + 1,
                },
                _ => unreachable!()
            };
        }
        code = format!("{}{}", code, next_digit);
    }

    println!("[PART 1] Bathroom code {}", code);
    let mut code = String::new();
    let mut next_digit = '5';
    for line in INPUT.lines() {
        for char in line.chars() {
            next_digit = match char {
                'U' =>  match next_digit {
                    '1' | '2' | '4' | '5' | '9' => next_digit,
                    '3' => '1',
                    '6' => '2',
                    '7' => '3',
                    '8' => '4',
                    'A' => '6',
                    'B' => '7',
                    'C' => '8',
                    'D' => 'B',
                    _ => unreachable!()
                },
                'D' => match next_digit {
                    'D' | 'A' | 'C' | '5' | '9' => next_digit,
                    '3' => '7',
                    '6' => 'A',
                    '7' => 'B',
                    '8' => 'C',
                    '2' => '6',
                    'B' => 'D',
                    '4' => '8',
                    '1' => '3',
                    _ => unreachable!()
                },
                'L' => match next_digit {
                    '1' | '2' | '5' | 'A' | 'D' => next_digit,
                    '3' => '2',
                    '6' => '5',
                    '7' => '6',
                    '8' => '7',
                    '4' => '3',
                    'B' => 'A',
                    'C' => 'B',
                    '9' => '8',
                    _ => unreachable!()
                },
                'R' => match next_digit {
                    '1' | '4' | '9' | 'C' | 'D' => next_digit,
                    '2' => '3',
                    '3' => '4',
                    '5' => '6',
                    '6' => '7',
                    '7' => '8',
                    '8' => '9',
                    'A' => 'B',
                    'B' => 'C',
                    _ => unreachable!()
                },
                _ => unreachable!()
            };
        }
        code = format!("{}{}", code, next_digit);
    }
    println!("[PART 2] Bathroom code {}", code);
}
