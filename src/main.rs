use std::collections::HashSet;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use colored::Colorize;
use csv::{Writer, WriterBuilder};

const COMPLETE_LIST_CSV: &str = "dept_connect_complete.csv";
const SCANNED_CSV: &str = "scanned.csv";
const SPECIAL_PREFIX: &str = "!";

struct Student {
    id: String,
    name: String,
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Load the complete list of students from a CSV file
/// This list is used to verify if a scanned ID is valid
/// and to retrieve the corresponding student name.
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

/// Save a scanned student's information to the CSV file
/// We want to save immediately to prevent data loss in case of crashes.
/// Although this is highly inefficient, it is acceptable for now
fn save_to_csv(student: &Student, access_type: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(SCANNED_CSV)?;

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
    let file = File::open(SCANNED_CSV)?;
    let reader = BufReader::new(file);
    for line in reader.lines().skip(1) {
        let line = line?;
        let fields = line.split(',').collect::<Vec<&str>>();
        if fields[0] == id {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Again we have to assume that the program might crash (the script exits) 
/// at any time or that the scanned.csv file might be modified externally
/// Hence we have to read the file every time we want to get the total count
/// instead of keeping a running count in memory
fn get_total_scanned() -> Result<i32, Box<dyn Error>> {
    let file = File::open(SCANNED_CSV)?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for _ in reader.lines().skip(1) {
        count += 1;
    }
    Ok(count)
}

fn attempt_save(student: &Student, access_type: &str) -> Result<(), Box<dyn Error>> {
    let count = get_total_scanned()?;
    let name = if student.name.is_empty() {
        "".to_string()
    } else {
        format!(" ({})", student.name)
    };
    // nasty way to print colored output but that's how the colored crate works
    if already_scanned(&student.id)? {
        println!(
            "{}",
            format!(
                "ID {}{} has already scanned. (total scanned: {})",
                student.id,
                name,
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
                name,
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
    let complete_list = load_complete_list(COMPLETE_LIST_CSV)?;
    let student_ids: HashSet<String> = complete_list.iter().map(|s| s.id.clone()).collect();

    initialize_csv(SCANNED_CSV)?;
    loop {
        print!("Enter ID: ");
        io::stdout().flush()?;

        let id = read_string().to_uppercase();
        if id.is_empty() {
            println!("{}", "ID cannot be empty!\n".red().bold());
            continue;
        }

        // This is a special case for allowing certain IDs that are not in the complete list to scan
        // However even for these IDs we still want to prevent duplicate scans
        if id.starts_with(SPECIAL_PREFIX) {
            // skip first 5 characters
            let id = &id[1 + SPECIAL_PREFIX.len()..];
            dbg!(&id);
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
