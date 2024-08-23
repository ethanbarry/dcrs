/// A simple macro to grab input from the user.
#[macro_export]
macro_rules! prompt {
    ($prompt:expr) => {{
        print!("{}", $prompt);
        use std::io::Write;
        std::io::stdout()
            .flush()
            .expect("Writing to stdout failed!");
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => std::process::exit(0), // Handle EOF
            _ => {}
        };
        input.trim().to_string()
    }};
}
