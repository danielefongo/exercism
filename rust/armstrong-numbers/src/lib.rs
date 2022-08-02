pub fn is_armstrong_number(num: u32) -> bool {
    let mut vec: Vec<u32> = vec![];
    let mut actual_num = num;
    while actual_num > 0 {
        vec.push(actual_num % 10);
        actual_num /= 10;
    }
    vec.iter().map(|it| it.pow(vec.len() as u32)).sum::<u32>() == num
}
