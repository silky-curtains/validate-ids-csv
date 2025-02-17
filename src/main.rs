use std::collections::HashSet;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use colored::Colorize;
use csv::{Writer, WriterBuilder};

struct Student {
    id: String,
    name: String,
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn load_complete_list(filename: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut students = Vec::new();
    for line in reader.lines().skip(1) {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        let student = Student {
            id: fields[0].to_string(),
            name: fields[1].to_string(),
        };
        students.push(student);
    }
    // dbg!(&students);
    dbg!(&students.len());
    Ok(students)
}

fn save_to_csv(student: &Student, access_type: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("scanned.csv")?;

    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&[&student.id, &student.name, access_type])?;
    wtr.flush()?; // Ensure immediate save
    Ok(())
}

fn initialize_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(filename);

    if !path.exists() {
        let file = File::create(filename)?;
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
        wtr.write_record(&["id", "name", "accessType"])?;
        wtr.flush()?;
        println!("Created {} with headers.", filename);
    } else {
        let file = File::open(filename)?;
        let reader = BufReader::new(&file);

        if reader.lines().next().is_none() {
            let mut file = OpenOptions::new().write(true).open(filename)?;
            writeln!(file, "id,name,accessType")?;
            println!("{} was empty. Added headers.", filename);
        } else {
            println!("{} already exists and is not empty.", filename);
        }
    }
    Ok(())
}

fn already_scanned(id: &str) -> Result<bool, Box<dyn Error>> {
    let file = File::open("scanned.csv")?;
    let reader = BufReader::new(file);
    for line in reader.lines().skip(1) {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        if fields[0] == id {
            return Ok(true);
        }
    }
    Ok(false)
}

fn get_total_scanned() -> Result<i32, Box<dyn Error>> {
    let file = File::open("scanned.csv")?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for _ in reader.lines().skip(1) {
        count += 1;
    }
    Ok(count)
}

fn attempt_save(student: &Student, access_type: &str) -> Result<(), Box<dyn Error>> {
    let count = get_total_scanned()?;
    if already_scanned(&student.id)? {
        println!(
            "{}",
            format!(
                "ID {}{} has already scanned. (total scanned: {})",
                student.id,
                if student.name.is_empty() {
                    "".to_string()
                } else {
                    format!(" ({})", student.name)
                },
                count
            )
            .red()
            .bold()
        );
    } else {
        save_to_csv(student, access_type)?;
        println!(
            "{}",
            format!(
                "ID {}{} registered successfully. (total scanned: {})",
                student.id,
                if student.name.is_empty() {
                    "".to_string()
                } else {
                    format!(" ({})", student.name)
                },
                count + 1
            )
            .green()
            .bold()
        );
    }
    println!();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let complete_list = load_complete_list("dept_connect_complete.csv")?;
    let student_ids: HashSet<String> = complete_list.iter().map(|s| s.id.clone()).collect();

    initialize_csv("scanned.csv")?;
    loop {
        print!("Enter ID: ");
        io::stdout().flush()?;

        let id = read_string().to_uppercase();
        if id.is_empty() {
            println!("{}", "ID cannot be empty!\n".red().bold());
            continue;
        }
        if id.starts_with("ALLOW") {
            // skip first 5 characters
            let id = &id[5..];
            if already_scanned(id)? {
                println!(
                    "{}",
                    format!(
                        "ID {} has already scanned. (total scanned: {})\n",
                        id,
                        get_total_scanned()?
                    )
                    .red()
                    .bold()
                );
                continue;
            }

            let student = Student {
                id: id.to_string(),
                name: "".to_string(),
            };

            attempt_save(&student, "special")?;
            continue;
        }

        if student_ids.contains(&id) {
            if let Some(student) = complete_list.iter().find(|s| s.id == id) {
                attempt_save(student, "normal")?;
            }
        } else {
            println!(
                "{}",
                format!(
                    "ID {} is either invalid or not registered for Department Connect list. (total scanned: {})\n",
                    id, get_total_scanned()?
                )
                .red()
                .bold()
            )
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
