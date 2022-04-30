use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // Define metadata file names as a map
    let mut metadata_files: HashMap<String, bool> = HashMap::new();
    metadata_files.insert("assembly_data_report.jsonl".to_string(), false);
    metadata_files.insert("dataset_catalog.json".to_string(), false);
    metadata_files.insert("sequence_report.jsonl".to_string(), false);
    metadata_files.insert("unplaced.scaf.fna".to_string(), false);

    let file_check = check_for_data_files(metadata_files);

    if file_check {
        println!("Found data files!");
    } else {
        println!("No data files found!");
    }

    // Find the folder that starts with "GCA_" in the input directory
    let input_dir = "./input";
    let mut gca_dir = String::new();
    for entry in WalkDir::new(input_dir) {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap().starts_with("GCA_") {
            gca_dir = entry.path().to_str().unwrap().to_string();
            break;
        }
    }

    let genome_files = get_genome_files();

    let binary = read_genome_files(genome_files, gca_dir);

    println!("{:?}", binary);
}

// Convert a genome string to binary
// Input: genome string
// Output: binary vector
fn genome_to_binary(genome: String) -> Vec<(u8, u8)> {
    // The genome is a string that is millions of characters long
    // We need to convert it to a vector of tuples of (u8, u8)
    // The tuples are based on the character. A is 1,1, B is 1,0, C is 0,1, etc.
    // We need to convert the string to a vector of tuples
    let mut binary: Vec<(u8, u8)> = Vec::new();
    for c in genome.chars() {
        let mut b = (0, 0);
        match c {
            'A' => b = (1, 1),
            'C' => b = (1, 0),
            'G' => b = (0, 1),
            'T' => b = (0, 0),
            _ => (),
        }
        binary.push(b);
    }

    return binary;
}

// Check the input directory for data files
// Input: metadata file names
// Output: true if all files are found, false otherwise
fn check_for_data_files(mut metadata: HashMap<String, bool>) -> bool {
    let result = true;

    // Check if the input directory exists
    if !Path::new("input").exists() {
        return false;
    }

    // Walk the input directory
    for entry in WalkDir::new("input") {
        let entry = entry.unwrap();

        // Check if the entry is a file
        if entry.file_type().is_file() {
            // Check if the file name is in the metadata file map
            if metadata.contains_key(&entry.file_name().to_string_lossy().to_string()) {
                // Set the value to true
                metadata.insert(entry.file_name().to_string_lossy().to_string(), true);
            }
        }
    }

    // Check if all metadata files were found
    for (_key, value) in metadata {
        if !value {
            return false;
        }
    }

    return result;
}

// Get the genome files from the input directory
// Output: vector of genome file names
fn get_genome_files() -> Vec<String> {
    let mut genome_files: Vec<String> = Vec::new();

    // Walk the input directory
    for entry in WalkDir::new("input") {
        let entry = entry.unwrap();

        // Check if the entry is a file
        if entry.file_type().is_file() {
            // Check if the file name is in the metadata file map
            if entry
                .file_name()
                .to_string_lossy()
                .to_string()
                .ends_with(".fna")
            {
                // If the file does not start with "chr", skip it
                if !entry
                    .file_name()
                    .to_string_lossy()
                    .to_string()
                    .starts_with("chr")
                {
                    continue;
                }

                // Add the file name to the genome files vector
                genome_files.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }

    return genome_files;
}

// Read genome files and convert to binary.
// Input: vec of genome file names
// Output: vec of binary tuples
fn read_genome_files(genome_files: Vec<String>, gca_dir: String) -> Vec<(u8, u8)> {
    let mut genome_binary: Vec<Vec<(u8, u8)>> = Vec::new();

    // Read each genome file
    for file in genome_files {
        // Print the file name
        println!("Reading file: {}", file);

        // Open the file
        let mut file = File::open(format!("{}/{}", gca_dir, file)).unwrap();

        // Read the file into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Convert the string to binary
        let genome_binary_entry = genome_to_binary(contents);

        // Add the binary genome to the genome binary vector
        genome_binary.push(genome_binary_entry);
    }

    // Convert genome_binary from a vec of vec of tuples to a vec of tuples
    let mut genome_binary_final: Vec<(u8, u8)> = Vec::new();
    for genome_binary_entry in genome_binary {
        for genome_binary_entry_entry in genome_binary_entry {
            genome_binary_final.push(genome_binary_entry_entry);
        }
    }

    return genome_binary_final;
}
