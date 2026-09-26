use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    grades: Vec<u32>,
}

struct StudentManager {
    students: Vec<Student>,
    next_id: u32,
}

impl Student {
    fn new(id: u32, name: String, grades: Vec<u32>) -> Self {
        Self { id, name, grades }
    }
    fn describe(&self) -> String {
        format!(
            "id: {}\nName: {}\nGrades: {:?}",
            self.id, self.name, self.grades
        )
    }

    fn average(&self) -> Option<f32> {
        if self.grades.is_empty() {
            return None;
        }
        Some(self.grades.iter().sum::<u32>() as f32 / self.grades.len() as f32)
    }

    fn highest_score(&self) -> Option<&u32> {
        self.grades.iter().max()
    }

    fn lowest_score(&self) -> Option<&u32> {
        self.grades.iter().min()
    }
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: Vec::new(),
            next_id: 1,
        }
    }

    fn add_student(&mut self, name: String) {
        let student = Student::new(self.next_id, name, Vec::new());
        self.students.push(student);
        self.next_id += 1
    }

    fn list_students(&self) {
        if self.students.is_empty() {
            println!("(no students added)");
            return;
        }

        for student in &self.students {
            println!("{}", student.describe());
        }
    }

    fn find_by_id(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|student| student.id == id)
    }

    fn search(&self, query: &str) -> Vec<&Student> {
        let query_lower = query.to_lowercase();
        self.students
            .iter()
            .filter(|student| student.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    fn update_name(&mut self, id: u32, new_name: String) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.name = new_name;
            true
        } else {
            false
        }
    }

    fn add_grade(&mut self, id: u32, grade: u32) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.grades.push(grade);
            true
        } else {
            false
        }
    }

    fn remove_student(&mut self, id: u32) -> bool {
        let original_len = self.students.len();
        self.students.retain(|student| student.id != id);
        self.students.len() != original_len
    }
}

enum Command {
    AddStudent(String),
    Read,
    ReadById(u32),
    ReadByName(String),
    Update(u32, String),
    AddGrade(u32, u32),
    RemoveStudent(u32),
    HighestGrade(u32),
    LowestGrade(u32),
    AverageGrade(u32),
    Quit,
    Invalid,
}

fn parse_command(line: &str) -> Command {
    let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
    if parts.is_empty() {
        return Command::Invalid;
    }

    match parts[0] {
        "add" => {
            if parts.len() == 2 {
                Command::AddStudent(parts[1].to_string())
            } else {
                Command::Invalid
            }
        }
        "read" | "list" => {
            if parts.len() == 2 {
                match parts[1].parse::<u32>() {
                    Ok(id) => Command::ReadById(id),
                    Err(_) => Command::ReadByName(parts[1].to_string()),
                }
            } else {
                Command::Read
            }
        }
        "update" => {
            if parts.len() == 2 {
                let sub: Vec<&str> = parts[1].splitn(2, ' ').collect();
                if sub.len() == 2 {
                    match sub[0].parse::<u32>() {
                        Ok(id) => Command::Update(id, sub[1].to_string()),
                        Err(_) => Command::Invalid,
                    }
                } else {
                    Command::Invalid
                }
            } else {
                Command::Invalid
            }
        }
        "add_grade" => {
            if parts.len() == 2 {
                let sub: Vec<&str> = parts[1].splitn(2, ' ').collect();
                if sub.len() == 2 {
                    match sub[0].parse::<u32>() {
                        Ok(id) => match sub[1].parse::<u32>() {
                            Ok(grade) => Command::AddGrade(id, grade),
                            Err(_) => Command::Invalid,
                        },
                        Err(_) => Command::Invalid,
                    }
                } else {
                    Command::Invalid
                }
            } else {
                Command::Invalid
            }
        }
        "remove" => {
            if parts.len() == 2 {
                match parts[1].parse::<u32>() {
                    Ok(id) => Command::RemoveStudent(id),
                    Err(_) => Command::Invalid,
                }
            } else {
                Command::Invalid
            }
        }
        "average" => {
            if parts.len() == 2 {
                match parts[1].parse::<u32>() {
                    Ok(id) => Command::AverageGrade(id),
                    Err(_) => Command::Invalid,
                }
            } else {
                Command::Invalid
            }
        }
        "highest" => {
            if parts.len() == 2 {
                match parts[1].parse::<u32>() {
                    Ok(id) => Command::HighestGrade(id),
                    Err(_) => Command::Invalid,
                }
            } else {
                Command::Invalid
            }
        }
        "lowest" => {
            if parts.len() == 2 {
                match parts[1].parse::<u32>() {
                    Ok(id) => Command::LowestGrade(id),
                    Err(_) => Command::Invalid,
                }
            } else {
                Command::Invalid
            }
        }
        "quit" | "exit" => Command::Quit,
        _ => Command::Invalid,
    }
}

fn input() -> String {
    print!("Enter your command: ");
    io::stdout().flush().unwrap();

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");
    text.trim().to_string()
}

fn print_help() {
    println!(
        "Commands:\n- add <first name>\n- read\n- read <id>\n- read <name text>\n- average <id>\n- highest <id>\n- lowest <id>\n- update <id> <name>\n- add_grade <id> <grade>\n- remove <id>\n- quit"
    );
}

fn main() {
    let mut manager = StudentManager::new();

    println!("===== Student Management (CRUD demo) =====");
    print_help();
    loop {
        let command = parse_command(&input());

        match command {
            Command::AddStudent(name) => {
                manager.add_student(name);
                println!("Student Registered");
            }
            Command::Read => manager.list_students(),
            Command::ReadById(id) => match manager.find_by_id(id) {
                Some(student) => println!("{}", student.describe()),
                None => println!("No student with id {id}."),
            },
            Command::ReadByName(query) => {
                let matches = manager.search(&query);
                if matches.is_empty() {
                    println!("No students match \"{query}\".");
                } else {
                    for student in matches {
                        println!("{}", student.describe());
                    }
                }
            }
            Command::Update(id, new_name) => {
                if manager.update_name(id, new_name) {
                    println!("student {id}, updated");
                } else {
                    println!("No student with id {id}.")
                }
            }
            Command::AddGrade(id, grade) => {
                if grade > 100 {
                    println!("grade should be in between 0 to 100")
                } else if manager.add_grade(id, grade) {
                    println!("grade {grade} is added to student with id {id}.")
                } else {
                    println!("No student with id {id}.")
                }
            }
            Command::RemoveStudent(id) => {
                if manager.remove_student(id) {
                    println!("student {id} deleted");
                } else {
                    println!("No student with id {id}");
                }
            }
            Command::AverageGrade(id) => match manager.find_by_id(id) {
                Some(student) => match student.average() {
                    Some(avg) => println!("Average score of {} is {}.", student.name, avg),
                    None => println!("Student has no grade"),
                },
                None => println!("No student with id {id}."),
            },
            Command::HighestGrade(id) => match manager.find_by_id(id) {
                Some(student) => match student.highest_score() {
                    Some(avg) => println!("Highest score of {} is {}.", student.name, avg),
                    None => println!("Student has no grade"),
                },
                None => println!("No student with id {id}."),
            },
            Command::LowestGrade(id) => match manager.find_by_id(id) {
                Some(student) => match student.lowest_score() {
                    Some(avg) => println!("Lowest score of {} is {}.", student.name, avg),
                    None => println!("Student has no grade"),
                },
                None => println!("No student with id {id}."),
            },
            Command::Quit => {
                println!("Goodbye !!");
                break;
            }
            Command::Invalid => {
                println!("Unrecognized command!");
                print_help();
            }
        }
    }
}
