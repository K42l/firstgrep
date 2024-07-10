use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use clap::Parser;
use std::io::{self, Write};
use indicatif::ProgressBar;

#[derive(Parser)]
struct Cli {
    string_pattern: String,
    file_path: std::path::PathBuf,
    output_path: Option<String>
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let stdout = io::stdout();
    let handle = BufWriter::new(match &args.output_path{
        Some(ref path) => Box::new(File::create(path).unwrap()) as Box<dyn Write>,
        None => Box::new(stdout.lock()) as Box<dyn Write>
    });
    find_content(args, handle)
}

fn find_content(args:Cli, mut handle:BufWriter<Box<dyn Write>>) -> Result<(), io::Error>{
    match File::open(&args.file_path){
        Ok(mut file_content) =>{
            let lines_amount = BufReader::new(&file_content).lines().count();
            let pb = ProgressBar::new(lines_amount.try_into().unwrap());
            
            let _ = file_content.seek(std::io::SeekFrom::Start(0));
            let reader = BufReader::new(file_content);
            let mut lines_found  = 0;
            for (i, line) in reader.lines().enumerate(){
                match line{
                    Ok(content) =>{
                        if content.contains(&args.string_pattern){
                            writeln!(handle, "line {i} : {content}")?;
                            lines_found += 1;
                        };
                        if  !&args.output_path.is_none(){
                            pb.inc(1);
                        }
                    },
                    Err(error) =>{
                        return Err(error)        
                    }
                }
            }
            if  !&args.output_path.is_none(){
                pb.finish_with_message("Done!");
            }
            writeln!(handle, "Found {lines_found} occorrences of the string pattern {:?} in the provided file", &args.string_pattern)?;
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