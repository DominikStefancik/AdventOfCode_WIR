use day_02::ReportsStats;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let reports = parse_input().expect("Error while parsing the input file");
    let reports_stats = ReportsStats::new(&reports);

    println!("Report stats:");
    println!("  Total: {}", reports_stats.total);
    println!("  Safe: {}", reports_stats.safe);
    println!("  Unsafe: {}", reports_stats.un_safe);

    let reports_stats = ReportsStats::new_with_problem_dampener(&reports);
    println!("---------------------------------------");
    println!("Report stats with the problem dampener:");
    println!("  Total: {}", reports_stats.total);
    println!("  Safe: {}", reports_stats.safe);
    println!("  Unsafe: {}", reports_stats.un_safe);

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<u8>>, Error> {
    let input_string =
        fs::read_to_string("./data/input.txt").expect("Error while reading the input file");
    let mut reports: Vec<Vec<u8>> = Vec::new();

    for line in input_string.lines() {
        let line_report: Vec<u8> = line
            .split_whitespace()
            .map(|item| item.trim().parse().expect("Error while parsing a number"))
            .collect();

        reports.push(line_report);
    }

    Ok(reports)
}
