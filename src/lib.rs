pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn multiply(left: usize, right: usize) -> usize {
    left * right // fix 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
