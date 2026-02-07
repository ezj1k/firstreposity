use std::io;

#[derive(Debug)]
pub struct Student {
    name: String,
    pub id: u32,
    grades: Vec<f64>,
}

impl Student {
    pub fn new(name: String, id: u32) -> Student {
        Student {
            name, id, grades: Vec::new(),
        }
    }

    pub fn add_grade(&mut self, grade: f64) {
        self.grades.push(grade);
    }

    pub fn average(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            self.grades.iter().sum::<f64>() / self.grades.len() as f64
        }
    }

    pub fn highest_grade(&self) -> Option<f64> {
        let mut max_grade: Option<f64> = None;
        for &grade in &self.grades {
            max_grade = match max_grade {
                Some(current_max) => Some(current_max.max(grade)),
                None => Some(grade),
            };
        }
        max_grade
    }

    pub fn passed(&self) -> bool {
        let avg_grade = self.average();
        if avg_grade >= 5.0 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Classroom {
    pub students: Vec<Student>,
}

impl Classroom {
    pub fn new() -> Classroom {
        Classroom {
            students: Vec::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn class_average(&self) -> f64 {
        let mut nr_grades_of_students = 0.0;
        let mut avg_grade_of_students = 0.0;
        for pupil in &self.students {
            let avg_grade = pupil.average();
            avg_grade_of_students += avg_grade;
            nr_grades_of_students += 1.0;
        }
        avg_grade_of_students / nr_grades_of_students
    }

    pub fn top_student(&self) -> Option<&Student> {
        let mut best_student: Option<&Student> = None;
        for pupil in &self.students {
            let avg_grade = pupil.average();
            best_student = match best_student {
                Some(current_max) => {
                    if avg_grade > current_max.average() {
                        Some(pupil)
                    } else {
                        Some(current_max)
                    }
                }
                None => Some(pupil),
            };
        }
        best_student
    }
}

pub fn introduce_grades() -> f64 {
    println!("Introduceți nota: ");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("eroare");
        match input.trim().parse::<f64>() {
            Ok(nr) if nr >= 1.0 && nr <= 10.0 => {return nr},
            Ok(_) => println!("Nota trebuie să fie între 1 și 10. Reîncercați:"),
            Err(_) => println!("Introduceți un număr valid:"),
        };
    }
}