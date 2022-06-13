fn main() {
    println!("2 + 2 = {}", add(2, 2));
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;  // have to have this for add() to be within scope
                   // use cargo test to run the tests

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


