//use std::any;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use clap::Parser;
use std::io::{self, Write};
use indicatif::ProgressBar;
//use std::time::Duration;

#[derive(Parser)]
struct Cli {
    string_pattern: String,
    file_path: std::path::PathBuf,
    output_path: Option<String>
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let stdout = io::stdout();
    //let path = args.output_path;
    let mut handle = BufWriter::new(match &args.output_path{
        Some(ref path) => Box::new(File::create(&path).unwrap()) as Box<dyn Write>,
        None => Box::new(stdout) as Box<dyn Write>
    });
    //let mut handle = BufWriter::new(stdout.lock());s
    let mut lines_found  = 0;
    //let mut count = 0;
    match File::open(&args.file_path){
        Ok(mut file_content) =>{
            let lines_amount = BufReader::new(&file_content).lines().count();
            let pb = ProgressBar::new(lines_amount.try_into().unwrap());
            let _ = file_content.seek(std::io::SeekFrom::Start(0));
            let reader = BufReader::new(&file_content);
            for (i, line) in reader.lines().enumerate(){
                match line{
                    Ok(content) =>{
                        if content.contains(&args.string_pattern){
                            writeln!(handle, "line {i} : {:?}", content)?;
                            lines_found += 1;
                        };
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
