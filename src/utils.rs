use std::{error::Error, fs, io, path::Path};
use colored::Colorize;
#[cfg(unix)]
use std::fs::Metadata;

/////////////////////////////////////////////////////////////////////////////

// makes life easier to check if the command is help with a reusable function
pub fn is_help_flag(arg: &str) -> bool {
    arg == "-h" || arg == "--help"
}

/////////////////////////////////////////////////////////////////////////////

// reading a file contents
pub fn read_file(path: &str) -> Result<String, Box<dyn Error>>{
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

/////////////////////////////////////////////////////////////////////////////

// the lsw functions and enum

#[derive (Debug, Clone, Copy)]
pub enum FileType {
    Directory,
    Executable,
    File,
}

// i know it's a windows port of the tools but lets keep this if i end up using the windowsAPI i will remove this one
// tbh i dont really understand that unix thing
#[cfg(unix)]
fn is_executable(metadata: &fs::Metadata) -> bool {
    use std::{fs::metadata, os::unix::fs::PermissionsExt};
    let mode = metadata.permissions().mode();
    mode &0o11 != 0 // any executable bit set (owner/group/others)
}

#[cfg(windows)]

fn is_executable(path: &Path) -> bool {

    // the path.extension returns option so we need to unwrap it with Some and if else in case it didnt return extension aka not an executable
    if let Some(ext) = path.extension(){ 
        let ext_str = ext.to_string_lossy().to_ascii_lowercase(); // again string lossy to avoid errors with non UTF-8 
        matches!(ext_str.as_str(), "exe" | "bat" | "ps1" | "com") // checking against all windows executables i know
    }else {
        false
    }

}

pub fn list_dir(path: &str) -> io::Result<Vec<(String,FileType)>>{ // this is because the read file metadata and others reutrn io type errors i guess
    let metadata = fs::metadata(&path)?;
    if metadata.is_dir() {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().into_owned(); // using to string lossy to not fail if fail if the filename is not valid UTF-8
            let path = entry.path();
            let metadata = entry.metadata()?;
              let file_type = if metadata.is_dir() { // setting the file type according to the checks using the FileType enum
                FileType::Directory
            } else {
                #[cfg(unix)] // if unix which again is useless here but here we go
                 {
                    if is_executable(&metadata) {
                        FileType::Executable
                    } else {
                        FileType::File
                    }
                }
                #[cfg(windows)] // if windows platform check the file type
                {
                    if is_executable(&path){
                        FileType::Executable
                    }else {
                        FileType::File
                    }
                }
            };
            //let is_dir = entry.file_type()?.is_dir();
            entries.push((file_name, file_type));
        }
        Ok(entries)
    }else {
        let file_name = Path::new(path)
        .file_name()
        .map(|name| name.to_string_lossy().into_owned()) // using to string lossy to not fail if fail if the filename is not valid UTF-8
        .unwrap_or_else(|| String::from("unknown"));
        println!("{} is not a directory.\n",format!("{}", file_name).red().bold());
        Ok(Vec::new())
    }
}

/////////////////////////////////////////////////////////////////////////////

// the grep stuff

pub fn grep_run(config: GrepConfig) -> Result<(), Box<dyn Error>> /* idk what is this but apprently returns any type of error */ {
    // reading the folder contents
    
    let content = fs::read_to_string(config.filename)?; // the ? means if the read to string returns erorr it will be automatically returned from the whole function "run"
    
    grep_search(&config.query, content);
    //checks
    //println!("The content \n{content}");
    // println!("the text content is {:#}", content); // thisis another way to print all line
    Ok(())
}

pub fn grep_search(query: &str, contents: String){
    let lines = contents.lines();
    for line in lines{
        if line.to_lowercase().contains(&query.to_lowercase()){
            let new_line = highlight_line(line, query);
            println!("{new_line}");
        }
    }
}

fn highlight_line(line: &str, query: &str) -> String {
    let mut highlighted_line = String::new();
    for word in line.split_inclusive(char::is_whitespace){
        let trimmed = word.trim();

        if trimmed.eq_ignore_ascii_case(query){

            let whitespace = &word[trimmed.len()..];
            let colored_word = trimmed.red().bold();
            highlighted_line.push_str(&format!("{}{}", colored_word, whitespace));

        }else {
        highlighted_line.push_str(word);
        }
    }
    highlighted_line
}


// the struct for the config
pub struct GrepConfig {
    pub query: String,
    pub filename: String,
}

// making it have a new function instead of a separate parse_config function

impl GrepConfig {
    pub fn new(args: &[String]) -> Result<GrepConfig, &str> {
        // checking argunments length
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }else if args.len() > 3 {
            return Err("Too many arguments");
        }
        // things
        let query = args[1].clone();
        let filename = args[2].clone();
        
        //finally 
        
        Ok(GrepConfig { query, filename })
    } 
}