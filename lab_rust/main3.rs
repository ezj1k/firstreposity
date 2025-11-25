#[derive(Debug)]
struct Student {
    name: String,
    id: u32,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: String, id: u32) -> Student {
        Student {
            name,
            id,
            grades: Vec::new(),
        }
    }

    fn add_grade(&mut self, grade: f64) {
        self.grades.push(grade);
    }

    fn average(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            /*let mut total: f64 = 0.0;
            for x in &self.grades {
            total += *x;
            }
            return total; */
            self.grades.iter().sum::<f64>() / self.grades.iter().len() as f64
        }
    }

    fn highest_grade(&self) -> f64 {
        let mut highest: f64 = 0.0;
        for i in 0..self.grades.len() {
            if self.grades[i] > highest {
                highest = self.grades[i];
            }
        }
        return highest;
    }

    fn passed(&self) -> bool {
        if self.average() > 5.0 { true } else { false }
    }
}

struct Classroom {
    students: Vec<Student>,
}

impl Classroom {
    fn new() -> Classroom {
        Classroom {
            students: Vec::new(),
        }
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn class_average(&self) -> f64 {
        let mut total: f64 = 0.0;
        let mut count = 0.0;
        for i in &self.students {
            total += i.average();
            count += 1.0;
        }
        total / count
    }

    fn top_student(&self) -> &String {
        let mut name = &self.students[0].name;
        for i in 1..self.students.len() {
            if &self.students[i].average() > &self.students[i - 1].average() {
                name = &self.students[i].name;
            }
        }
        name
    }
}

pub fn main() {
    let mut clasa = Classroom::new();

    let mut student0 = Student::new("Iulii".to_string(), 0);
    let mut student1 = Student::new("Alex".to_string(), 1);
    let mut student2 = Student::new("Ion".to_string(), 2);
    let mut student3 = Student::new("Ana".to_string(), 3);
    let mut student4 = Student::new("Lia".to_string(), 4);
    //0
    for _ in 0..5 {
        student0.add_grade(10.0);
    }
    //1
    for _ in 0..3 {
        student1.add_grade(9.0);
    }
    for _ in 3..5 {
        student1.add_grade(8.0);
    }
    //2
    for _ in 0..2 {
        student2.add_grade(9.0);
    }
    for _ in 2..4 {
        student2.add_grade(8.0);
    }
    student2.add_grade(7.0);
    //3
    for _ in 0..2 {
        student3.add_grade(9.0);
    }
    student3.add_grade(8.0);
    student3.add_grade(5.0);
    student3.add_grade(3.0);
    //4
    student4.add_grade(4.0);
    student4.add_grade(5.0);
    student4.add_grade(3.0);
    student4.add_grade(2.0);
    student4.add_grade(4.0);

    clasa.add_student(student0);
    clasa.add_student(student1);
    clasa.add_student(student2);
    clasa.add_student(student3);
    clasa.add_student(student4);

    for i in &clasa.students {
        println!("Studentul: {:?}; ID: {}", i, i.id);
        println!("Media studentului: {}", i.average());
        println!("A trecut? {}", i.passed());
        println!("Cea mai mare nota: {}", i.highest_grade());
    }

    println!("Media clasei: {}", clasa.class_average());

    println!("cel mai bun student: {}", clasa.top_student())
}
