pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[no_mangle]
pub extern fn hello_rust(path :&str){
    println!("Hello rust dll!");
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
