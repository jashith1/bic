//TO-DO allow messages, currently only looks through src folder (targetting everything also makes it target the bic subfolder), 
use std::{collections::HashMap, fs, path::Path};
use sha2::{Sha256, Digest};
use serde::Serialize;
use walkdir::WalkDir;

//commit data stored in ./bic/commits json files
#[derive(Serialize)]
struct CommitData {
    parent: String,
    message: String,
    timestamp: u64,
    files: HashMap<String, String>
}

pub fn commit() -> std::io::Result<()> {
    //make sure initialized
    if !Path::new(".bic").exists() {
        eprintln!("Error: not bic repository. Run `bic init` first.");
        std::process::exit(1);
    }


    let files_map = collect_files("./src")?;
   

    //create actual commit data
    let commit_data = CommitData  {
        parent: read_current_head()?,
        message: "Temporary commit message".to_string(),
        timestamp: get_current_timestamp(),
        files: files_map,
    };

    write_commit(&commit_data)?;
    println!("Commited created.");

    Ok(())
}

fn collect_files(directory: &str) -> std::io::Result<HashMap<String, String>> {
    let mut files_map: HashMap<String, String> = HashMap::new();

    //walkdir goes through subfolders too
    for entry in WalkDir::new(directory) {
        let entry = entry.unwrap();
        let path = entry.path();
        //walkdir treats a subfolder as a subentry too, ignore that.
        if !path.is_file() {
            continue;
        }

        let file_contents = fs::read_to_string(&path)?;
        let file_hash = calculate_hash(&file_contents);
        let file_name = path.strip_prefix(directory).unwrap().to_string_lossy().to_string();

        files_map.insert(file_name, file_hash.clone());
        store_file_object(&file_hash, &file_contents);
    }

    Ok(files_map)

}

fn calculate_hash(content: &str) -> String{
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn read_current_head() -> std::io::Result<String> {
    let content = fs::read_to_string(".bic/HEAD")?;
    Ok(content.trim().to_string())
}

fn get_current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time travel :scream:")
        .as_secs()
}

//writes the actual file to store into objects folder
fn store_file_object(hash: &str, content: &str) -> std::io::Result<()> {
    let object_path = format!(".bic/objects/{}", hash);
    
    // Only write if file doesn't already exist (deduplication)
    if !Path::new(&object_path).exists() {
        fs::write(&object_path, content)?;
    }
    
    Ok(())
}


//writes the json file in commits folder and change HEAD
fn write_commit(commit_data: &CommitData) -> std::io::Result<()> {
    //convert object into string to write into file
    let commit_string = serde_json::to_string_pretty(commit_data)
        .expect("Failed to serialize commit data");

    let file_hash = calculate_hash(&commit_string);
    
    let commit_path = format!(".bic/commits/{}.json", file_hash);
    fs::write(commit_path, commit_string)?;

    //change head to point to current commit
    fs::write(".bic/HEAD", file_hash)?;
    
    Ok(())
}

