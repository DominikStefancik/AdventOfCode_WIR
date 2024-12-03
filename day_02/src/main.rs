use std::fs;
use std::io::Error;

struct ReportsStats {
    total: u16,
    safe: u16,
    un_safe: u16,
}

const MIN_DIFF: i8 = 1;
const MAX_DIFF: i8 = 3;

fn main() -> Result<(), Error> {
    let reports = parse_input().expect("Error while parsing the input file");
    let reports_stats = generate_reports_stats(reports);

    println!("Report stats:");
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

fn generate_reports_stats(reports: Vec<Vec<u8>>) -> ReportsStats {
    let total = reports.len() as u16;
    let mut safe = 0;
    let mut un_safe = 0;

    for report in reports {
        if is_report_safe(report.clone()) {
            safe += 1;
        } else {
            un_safe += 1;
        }
    }

    ReportsStats {
        total,
        safe,
        un_safe,
    }
}

fn is_report_safe(report: Vec<u8>) -> bool {
    let first_number = report[0];
    let second_number = report[1];

    if first_number == second_number {
        return false;
    }

    let is_increasing = first_number < second_number;
    let iterator1 = report.iter();
    let iterator2 = report.iter().skip(1);
    let are_numbers_safe = |first: u8, second: u8| {
        let result: i8;
        if is_increasing {
            result = second as i8 - first as i8;
        } else {
            result = first as i8 - second as i8;
        }

        result >= MIN_DIFF && result <= MAX_DIFF
    };

    for (first, second) in iterator1.zip(iterator2) {
        if !are_numbers_safe(*first, *second) {
            return false;
        }
    }

    true
}
