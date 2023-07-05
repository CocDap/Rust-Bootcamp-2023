
//Exercise 1
// Complete this function to return the bigger number!
// Do not use:
// - another function call
// - additional variables
pub fn bigger(a: i32, b: i32) -> i32 {
    if a>b {
        return  a;
    } 
    else
    {
        return b;
    }
}

//Exercise 2
// Input: Provide an arbitrary value of number
// Check number is Positive or Negative or Zero
// Output: &str
fn check_number(number: i32) -> &'static str {
   if number < 0 {
       return "Negative";
   }
   else if number > 0 {
       return "Positive";
   }
   else {
       return "Zero";
   }
}

// Exercise 3
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    }
    else {
        "baz"
    }
}

// Exercise 4
// Determine if a given year is a leap year
// Implement logic
fn is_leap_year(year: i32) -> bool {
    // If a year is multiple of 400, 
    // then it is a leap year 
    if year % 400 == 0
    {
        return true; 
    }
    // Else If a year is multiple of 100, 
    // then it is not a leap year 
    if year % 100 == 0
    {
        return false; 
    } 
    // Else If a year is multiple of 4, 
    // then it is a leap year 
    if year % 4 == 0 
    {
        return true; 
    } 
    return false; 
}

// Exercise 5
// Calculate the factorial of a number
// Implement logic
fn factorial(n: u32) -> u32 {
    let mut f:u32 = 1;
    if n == 0 {
        return 1;
    }
    else {
        for k in 1..n+1 {
            f = f*k;
        }
    }
    return f;
}

// Exercise 6
// Check if a number is prime
// Implement logic

fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    else {
        for i in 2.. n+1 / 2 {
            // if n is divisible by i, then n is not prime
            // change flag to 1 for non-prime number
            if n % i == 0 {
              return false;
            }
          }
    }
    return true;
    
}


// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }
    // Test for exercise 1
    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
    // Test for exercise 2
    #[test]
    fn test_check_number_positive() {
        let result = check_number(10);
        assert_eq!(result, "Positive");
    }
    // Test for exercise 2
    #[test]
    fn test_check_number_negative() {
        let result = check_number(-5);
        assert_eq!(result, "Negative");
    }
    // Test for exercise 2
    #[test]
    fn test_check_number_zero() {
        let result = check_number(0);
        assert_eq!(result, "Zero");
    }

    // Test for exercise 3
    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    // Test for exercise 3
    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    // Test for exercise 3
    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }

    // Test for exercise 4
    #[test]
    fn test_leap_year() {
        assert_eq!(is_leap_year(2020), true);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(1600), true);
    }

    // Test for exercise 4
    #[test]
    fn test_non_leap_year() {
        assert_eq!(is_leap_year(2021), false);
        assert_eq!(is_leap_year(1900), false);
        assert_eq!(is_leap_year(1800), false);
    }

    // Test for exercise 5
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    // Test for exercise 6
    #[test]
    fn test_prime_number() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(19), true);
    }
    // Test for exercise 6
    #[test]
    fn test_non_prime_number() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(15), false);
    }


}