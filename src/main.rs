//use std::any;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use clap::Parser;
use std::io::{self, Write};
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;


#[derive(Parser)]
struct Cli {
    string_pattern: String,
    file_path: std::path::PathBuf
}

fn main() -> Result<(), std::io::Error> {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    let args = Cli::parse();
    let mut lines_found  = 0;
    //let mut count = 0;
    match File::open(&args.file_path){
        Ok(file_content) =>{
            let pb = ProgressBar::new(5159);
            let reader = BufReader::new(file_content);
            for line in reader.lines(){
                match line{
                    Ok(content) =>{
                        if content.contains(&args.string_pattern){
                            //println!("line : {}", content);
                            writeln!(handle, "line : {}", content)?;
                            lines_found += 1;
                        };
                        thread::sleep(Duration::from_millis(5));
                        pb.inc(1);
                    },
                    Err(error) =>{
                        return Err(error)        
                    }
                }
            }
            pb.finish_with_message("Done!");
            writeln!(handle, "Found {:?} occorrences of the string pattern in the provided file", lines_found)?;
        },
        Err(error) =>{
            match error.kind(){
                std::io::ErrorKind::NotFound => {
                   eprintln!("Error: File Not Found! Path: {:?}", &args.file_path);
                }
                _ => return Err(error)
            }
        }
    };
    Ok(())
}
