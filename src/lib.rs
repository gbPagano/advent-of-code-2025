#[macro_export]
macro_rules! gen_main {
    ($func1:ident, $func2:ident) => {
        fn main() {
            use std::env;
            use std::fs;
            use std::path::Path;
            use std::time::Instant;

            let args: Vec<String> = env::args().collect();
            let is_test = args.iter().any(|arg| arg == "-t" || arg == "--test");

            let extension = if is_test { "test" } else { "input" };

            let source_path = file!().split(['/', '\\']).last().unwrap();
            let stem = source_path.strip_suffix(".rs").unwrap();

            // Ex: inputs/day01.test or inputs/day01.input
            let input_path = format!("inputs/{}.{}", stem, extension);

            println!("Input file: {}", input_path);

            let data = match fs::read_to_string(&input_path) {
                Ok(content) => content,
                Err(_) => {
                    eprintln!("\nERROR: File '{}' not found!", input_path);
                    std::process::exit(1);
                }
            };
            let data2 = data.clone();

            let start = Instant::now();
            let result_part1 = $func1(data);
            let duration = start.elapsed();

            println!("\nPart 1 executed in {:.2?}\nResult:", duration);
            println!("{:?}", result_part1);

            let start = Instant::now();
            let result_part2 = $func2(data2);
            let duration = start.elapsed();

            println!("\nPart 2 executed in {:.2?}\nResult:", duration);
            println!("{:?}", result_part2);
        }
    };
}
