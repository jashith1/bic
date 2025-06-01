use std::{fs, path::Path};

use ignore::gitignore::Gitignore;
use walkdir::WalkDir;

use crate::util::{build_ignore_matcher::build_ignore_matcher, commit_data::CommitData};

pub fn reset(commit_hash: String) -> std::io::Result<()>{
    if !check_commit_hash(&commit_hash){
        println!("The given commit hash ({}) does not exist.", commit_hash);
        return Ok(())
    }

    delete_new_files(&build_ignore_matcher())?;
    revert_to_commit(commit_hash)?;

    Ok(())
}

fn delete_new_files(ignore: &Gitignore) -> std::io::Result<()>{
    let walker = WalkDir::new(".").into_iter().filter_entry(|entry| {
        let path = entry.path();
        // Don't descend into ignored folders
        !ignore.matched(path, path.is_dir()).is_ignore()
    });

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file(){
            continue;
        }

        fs::remove_file(&path)?;
    }

    Ok(())

}

fn check_commit_hash(commit_hash: &str) -> bool{
    Path::new(&format!(".bic/commits/{}.json", commit_hash)).exists()
}

fn revert_to_commit(commit_hash: String) -> std::io::Result<()>{
    let commit_string = fs::read_to_string(format!(".bic/commits/{}.json", commit_hash))?;
    let commit_data: CommitData = serde_json::from_str(&commit_string)?;

    for file in commit_data.files{
        let file_content = fs::read(format!(".bic/objects/{}", file.1))?;
        let file_path = file.0;

        fs::write(file_path, file_content)?;
    }

    Ok(())
}