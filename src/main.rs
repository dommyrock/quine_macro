use closure_macro::capture_closure;

fn main() {
    #[capture_closure]
    fn example_function() {
        let r = || {
            println!("captures and prints the function's definition dynamically");
        };

        r(); // Invoke the closure
    }

    // Call the function
    example_function();
}
