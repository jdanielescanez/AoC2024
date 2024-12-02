fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let result = std::fs::read_to_string(input_filename)
        .expect("Error reading input file")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|str| str.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            (report.is_sorted() || report.into_iter().rev().is_sorted())
                && report
                    .windows(2)
                    .all(|windows| (1..=3).contains(&windows[0].abs_diff(windows[1])))
        })
        .count();

    println!("\n{result:?}");
}
