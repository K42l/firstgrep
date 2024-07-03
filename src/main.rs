use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    string_pattern: String,
    file_path: std::path::PathBuf
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match File::open(args.file_path){
        Ok(file_content) =>{
            let reader = BufReader::new(file_content);
            for line in reader.lines(){
                match line{
                    Ok(content) =>{
                        if content.contains(&args.string_pattern){
                            println!("line : {}", content)
                        }
                    },
                    Err(error) =>{
                        return Err(error.into())        
                    }
                }
            }
        },
        Err(error) =>{
            match error.kind(){
                std::io::ErrorKind::NotFound => {
                   println!("Error: File Not Found!")
                }
                _ => return Err(error)
            }
        }
    };
    Ok(())
}
