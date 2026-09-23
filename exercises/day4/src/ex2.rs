pub fn recursive_factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * recursive_factorial(n - 1)
    }
}

pub fn avg_list(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

pub fn vowels_count(string: &str) -> usize {
    string.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

pub fn operation_calculator(num1: f64, num2: f64, operator: char) -> Option<f64> {
    match operator {
        '+' => Some(num1 + num2),
        '-' => Some(num1 - num2),
        '*' => Some(num1 * num2),
        '/' => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn sort_bubble(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn cal_circle(radius: f64) {
    fn area_circle(radius: f64) {
        println!(
            "Area of circle: {:.2}",
            std::f64::consts::PI * radius * radius
        );
    }
    fn circumference_circle(radius: f64) {
        println!(
            "Circumference of circle: {:.2}",
            2.0 * std::f64::consts::PI * radius
        );
    }
    area_circle(radius);
    circumference_circle(radius);
}
