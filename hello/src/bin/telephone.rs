pub fn r#type(input: &str) -> i32 {
    let mut number = 0;
    for char in input.chars() {
        // Algorithmic solution:
        // let mut digit = ('A'..='Z')
        //     .collect::<Vec<char>>()
        //     .iter()
        //     .position(|&letter| letter == char)
        //     .unwrap() as i32
        //     / 3;
        // match char {
        //     'S' | 'V' | 'Y' | 'Z' => digit -= 1,
        //     _ => (),
        // }
        // number = number * 10 + digit + 2;

        // Simple solution:
        let digit = match char.to_ascii_uppercase() {
            'A' | 'B' | 'C' => 2,
            'D' | 'E' | 'F' => 3,
            'G' | 'H' | 'I' => 4,
            'J' | 'K' | 'L' => 5,
            'M' | 'N' | 'O' => 6,
            'P' | 'Q' | 'R' | 'S' => 7,
            'T' | 'U' | 'V' => 8,
            'W' | 'X' | 'Y' | 'Z' => 9,
            _ => 0,
        };
        number = number * 10 + digit;
    }
    number
}
