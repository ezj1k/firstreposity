use lab1::Student;
use lab1::Classroom;
use lab1::introduce_grades;

fn main() {
    let mut clasa1 = Classroom::new();
    for id in 1..=5 {
        let student = Student::new("Name".to_string(), id as u32);
        clasa1.add_student(student);
    }

    let mut id = 1;
    for student in &mut clasa1.students {
        println!("For student {}", id);
        for _ in 1..=3 {
            student.add_grade(introduce_grades());
        }
        id += 1;
    }

    for student in &clasa1.students {
        println!("{:?}", student);
    }

    let the_class_average = clasa1.class_average();
    let the_best_student = clasa1.top_student();
    println!("the_class_average: {:?}", the_class_average);
    println!("the_best_student: {:?}", the_best_student);

    println!("-------------Passed--------------");
    for student in &mut clasa1.students {
        println!("Student {} - paseed: {}", student.id, student.passed());
    }

    println!("-------------Highest Grade--------------");
    for student in &mut clasa1.students {
        match student.highest_grade() {
            Some(max) => println!("For student {} - highest grade: {}", student.id, max),
            None => println!("For student {} - dont have marks!", student.id),
        }
    }
}