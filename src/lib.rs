use core::sender;

mod core;

pub extern "C" fn test_func() -> i32 {
    42
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
