const MIN_DIFF: i8 = 1;
const MAX_DIFF: i8 = 3;

pub struct ReportsStats {
    pub total: u16,
    pub safe: u16,
    pub un_safe: u16,
}

impl ReportsStats {
    pub fn new(reports: &Vec<Vec<u8>>) -> Self {
        let total = reports.len() as u16;
        let mut safe = 0;
        let mut un_safe = 0;

        for report in reports {
            if ReportsStats::is_report_safe(report) {
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

    pub fn new_with_problem_dampener(reports: &Vec<Vec<u8>>) -> Self {
        let total = reports.len() as u16;
        let mut safe = 0;
        let mut un_safe = 0;

        for report in reports {
            if ReportsStats::is_report_safe_with_problem_dampener(report) {
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

    fn is_report_safe(report: &Vec<u8>) -> bool {
        let first_number = report[0];
        let second_number = report[1];

        if first_number == second_number {
            return false;
        }

        let is_increasing = first_number < second_number;
        let iterator1 = report.iter();
        let iterator2 = report.iter().skip(1);

        for (first, second) in iterator1.zip(iterator2) {
            if !ReportsStats::are_numbers_safe(*first, *second, is_increasing) {
                return false;
            }
        }

        true
    }

    fn is_report_safe_with_problem_dampener(report: &Vec<u8>) -> bool {
        // the report is safe without having to remove any number
        if ReportsStats::is_report_safe(report) {
            return true;
        };

        for index in 0..report.len() {
            let mut report_clone = report.clone();
            report_clone.remove(index);

            if ReportsStats::is_report_safe(&report_clone) {
                return true;
            }
        }

        false
    }

    fn are_numbers_safe(first: u8, second: u8, is_increasing: bool) -> bool {
        let result: i8;
        if is_increasing {
            result = second as i8 - first as i8;
        } else {
            result = first as i8 - second as i8;
        }

        result >= MIN_DIFF && result <= MAX_DIFF
    }
}
