pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let count = num_string.chars().count() as u32;

    let mut sum = 0;
    for char in num_string.chars() {
        let digit = char.to_digit(10).unwrap();
        sum += digit.pow(count);
    }

    num == sum
}
