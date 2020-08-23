use std::fmt::Display;

// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes to support alphabetical report cards, thereby making
// the second test pass.

pub enum Grade {
    Numeric(f32),
    Alphabetical(String),
}

impl Display for Grade {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let s: String = match self {
            Grade::Numeric(n) => n.to_string(),
            Grade::Alphabetical(g) => g.to_string(),
        };
        formatter.write_str(&s)
    }
}

pub struct ReportCard {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numeric(2.1), 
            student_name: "Tom Wriggle".to_string(), 
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: Grade::Alphabetical("A+".to_string()), 
            student_name: "Gary Plotter".to_string(), 
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
