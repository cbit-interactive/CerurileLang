pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number(){
        assert_eq!(Number::new("123"), Number(123))
    }

    #[derive(Debug, PartialEq)]
    pub struct Number(pub i32);

    impl Number {
        pub fn new(s: &str) -> Self {
            Self(s.parse().unwrap())
        }
    }
}
