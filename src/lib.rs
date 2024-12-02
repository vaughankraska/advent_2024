pub fn start_day(day: &str) {
    println!("Running Day {}", day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        start_day("00");
    }
}
