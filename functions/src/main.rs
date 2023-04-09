fn main() {
    fn test_function() {
        println!("test")
    }

    test_function();

    outside_function()
}

fn outside_function() {
    println!("test_outiside")
}
