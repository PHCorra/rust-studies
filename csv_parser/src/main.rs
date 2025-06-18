use std::collections::HashMap;
use std::fs;

fn main() {
    let csv_content= match fs::read_to_string("data.csv") {
        Ok(content) => content,
        Err(error) => {
            println!("Error reading the file: {}", error);
            return;
        }
    };

    println!("CSV Content: ");
    println!("{}", csv_content);

    // Collect transform the csv content split into a Vec
    let lines: Vec<&str> = csv_content.split('\n').collect();

    println!("\n splited lines: ");
    for (index, line) in lines.iter().enumerate() {
        println!("Line {}: '{}'", index, line);
    }

    let mut csv_data: Vec<Vec<&str>> = Vec::new();

    for line in lines.iter() {
        if !line.trim().is_empty() {
            let fields: Vec<&str> = line.split(',').collect();
            csv_data.push(fields);
        }

    }

    println!("\nStructured Data: ");
    for (row_index, row) in csv_data.iter().enumerate() {
        println!("line {}: {:?}", row_index, row);
    }

    let headers = &csv_data[0];
    println!("\nHeaders: {:?}", headers);

    let mut records: Vec<HashMap<&str, &str>> = Vec::new();

    for data_row in csv_data.iter().skip(1) {
        let mut record = HashMap::new();
    
        for (i, value) in data_row.iter().enumerate() {
            if i < headers.len() {
                record.insert(headers[i], value.trim());
            }

        }
        records.push(record);
    }

    println!("\n data in hashmap: ");
    for (i, record) in records.iter().enumerate() {
        println!("Person {}: {:?}", i + 1, record);

        if let Some(nome) = record.get("nome") {
            if let Some(idade) = record.get("idade") {
                if let Some(cidade) = record.get("cidade") {
                    println!("  {} has {} years and live in {}", nome, idade, cidade);
                }
            }
        }
    }


    println!("\n CHALLENGE: ");
    let pessoas_salvador: Vec<_> = records.iter().filter(|record| record.get("cidade") == Some(&"Salvador")).collect();

    for record in pessoas_salvador {
        if let Some(nome) = record.get("nome") {
            println!("{} mora em Salvador", nome);
        }
    }
}