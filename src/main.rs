use colored::Colorize;
use csv::Writer;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
struct Student {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Data {
    data: Vec<Student>,
}

// #[allow(dead_code)]
// fn get_filtered_students() -> Vec<Student> {
//     let mut file = File::open("data.json").expect("Failed to open file");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)
//         .expect("Failed to read file");

//     let parsed_data: Data = serde_json::from_str(&contents).expect("Failed to parse JSON");

//     let mut filtered_students: Vec<Student> = parsed_data
//         .data
//         .into_iter()
//         .filter(|s| s.id[..4].parse::<u32>().unwrap_or(0) >= 2021)
//         .filter(|s| {
//             (s.id.chars().nth(6) == Some('A') && s.id.chars().nth(7) == Some('7'))
//                 || (s.id.chars().nth(4) == Some('A') && s.id.chars().nth(5) == Some('7'))
//         })
//         .collect();
//     filtered_students.sort_by_key(|s| s.id[..4].to_string());
//     return filtered_students;
// }

// #[allow(dead_code)]
// fn filter_cs_students() {
//     let filtered_students = get_filtered_students();
//     // uncomment to write to json file
//     // let filtered_data = Data {
//     //     data: filtered_students.clone(),
//     // };

//     dbg!(filtered_students.len());
//     // let mut filtered_file = File::create("filtered.json").expect("Failed to create file");
//     // serde_json::to_writer_pretty(&mut filtered_file, &filtered_data)
//     //     .expect("Failed to write to file");

//     let mut csv_file = File::create("cs_students.csv").expect("Failed to create file");
//     writeln!(csv_file, "id,name").expect("Failed to write header");
//     for student in &filtered_students {
//         writeln!(csv_file, "{},{}", student.id, student.name).expect("Failed to write record");
//     }
// }

// #[allow(dead_code)]
// fn validate_csv() {
//     let filtered_students = get_filtered_students();
//     let filtered_ids: Vec<String> = filtered_students.iter().map(|s| s.id.clone()).collect();

//     let input_file = File::open("input.csv").expect("Failed to open input.csv");
//     let reader = BufReader::new(input_file);
//     let mut id_count: HashMap<String, u32> = HashMap::new();

//     for line in reader.lines().skip(1) {
//         if let Ok(id) = line {
//             *id_count.entry(id.to_uppercase()).or_insert(0) += 1;
//         }
//     }

//     for (id, count) in &id_count {
//         if !filtered_ids.contains(id) {
//             println!("ID {} is not part of the filtered students", id);
//         }
//         if *count > 1 {
//             println!(
//                 "ID {} appears more than once in input.csv (appears {} times)",
//                 id, count
//             );
//         }
//     }
// }

// #[allow(dead_code)]
// fn get_phd_students() -> Vec<Student> {
//     let mut file = File::open("data.json").expect("Failed to open file");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).expect("Failed to read file");
//     let parsed_data: Data = serde_json::from_str(&contents).expect("Failed to parse JSON");

//     let a_hostels = ["AH1", "AH2", "AH3", "AH4", "AH5", "AH6", "AH7", "AH8", "AH9"];
//     let c_hostels = ["CH1", "CH2", "CH3", "CH4", "CH5", "CH6", "CH7"];
//     let d_hostels = ["DH1", "DH2", "DH3", "DH4", "DH5", "DH6"];

//     parsed_data
//         .data
//         .into_iter()
//         .filter(|s| s.id.contains("PHX") && (a_hostels.contains(&s.hostel.as_str()) || c_hostels.contains(&s.hostel.as_str()) || d_hostels.contains(&s.hostel.as_str()) || s.hostel == "Full Time"))
//         .collect()
// }

// #[allow(dead_code)]
// fn write_phd_to_csv() {
//     let phd_students = get_phd_students();
//     dbg!(phd_students.len());
//     let mut csv_file = File::create("phd_students.csv").expect("Failed to create file");
//     writeln!(csv_file, "id,name").expect("Failed to write header");
//     for student in &phd_students {
//         writeln!(csv_file, "{},{}", student.id, student.name).expect("Failed to write record");
//     }
// }

// #[allow(dead_code)]
// fn remove_fields_from_csv(input_file: &str, output_file: &str) -> std::io::Result<()> {
//     let file = File::open(input_file)?;
//     let reader = BufReader::new(file);
//     let mut output = File::create(output_file)?;

//     for (index, line) in reader.lines().enumerate() {
//         let line = line?;
//         let mut fields: Vec<&str> = line.split(',').collect();

//         if index == 0 {
//             // Remove headers
//             fields.retain(|&field| field != "Hostel" && field != "Room" && field != "Degree" && field != "Email Address");
//         } else {
//             // Remove corresponding data fields
//             fields.drain(2..6);
//         }

//         writeln!(output, "{}", fields.join(","))?;
//     }
//     Ok(())
// }

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

fn save_to_csv(student: &Student) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("scanned.csv")?;

    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&[&student.id, &student.name])?;
    wtr.flush()?; // Ensure immediate save
    Ok(())
}

fn initialize_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(filename);

    if !path.exists() {
        let file = File::create(filename)?;
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file); 
        wtr.write_record(&["id", "name"])?; 
        wtr.flush()?;
        println!("Created {} with headers.", filename);
    } else {
        let file = File::open(filename)?;
        let reader = BufReader::new(&file);

        if reader.lines().next().is_none() {
            let mut file = OpenOptions::new().write(true).open(filename)?;
            writeln!(file, "id,name")?; 
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let complete_list = load_complete_list("dept_connect_complete.csv")?;
    let student_ids: HashSet<String> = complete_list.iter().map(|s| s.id.clone()).collect();

    initialize_csv("scanned.csv")?;
    loop {
        print!("Enter ID: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let id = input.trim().to_uppercase().to_string();

        if id.is_empty() {
            println!("{}", "ID cannot be empty!\n".red().bold()); 
            continue;
        }

        if student_ids.contains(&id) {
            if let Some(student) = complete_list.iter().find(|s| s.id == id) {
                // check if already exists
                if already_scanned(&id)? {
                    println!(
                        "{}",
                        format!("ID {} ({}) has already scanned.", id, student.name)
                            .red()
                            .bold()
                    );
                } else {
                    save_to_csv(student)?;
                    println!(
                        "{}",
                        format!(
                            "ID {} ({}) registered successfully.",
                            student.id, student.name
                        )
                        .green()
                        .bold()
                    );
                }
            }
        } else {
            println!(
                "{}",
                format!(
                    "ID {} is either invalid or not registered for Department Connect list.",
                    id
                )
                .red()
                .bold()
            )
        }

        println!("");
    }
    #[allow(unreachable_code)]
    Ok(())
}
