use std::sync::Arc;
use clap::Parser;
use rayon::prelude::*;
use regex::RegexBuilder;
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;
use crate::commands::{AppCommands, Target};
use crate::prelude::MyResult;

mod prelude;
mod commands;

fn main() -> MyResult {
    let mut config = AppCommands::parse();
    if config.max_thread == 0 {
        config.max_thread = std::thread::available_parallelism().unwrap().get();
    }

    let builder = RegexBuilder::new(&*config.pattern).build();
    let Ok(regex) = builder else
    {
        println!("regex pattern is invalid!");
        return Ok(());
    };
    let arc_regex = Arc::new(regex);
    let arc_target = Arc::new(config.target);
    rayon::ThreadPoolBuilder::new().num_threads(config.max_thread).build_global()?;

    let iter = WalkDir::new(config.path).max_depth(config.max_depth).into_iter();
    iter.par_bridge().for_each(move |file| {
        let Ok(file) = file else { return; };
        let target = Arc::clone(&arc_target);
        let regex = Arc::clone(&arc_regex);
        tokio::runtime::Runtime::new().unwrap().block_on(async move {
            match *target {
                Target::Names => {
                    let name = file.file_name().to_str().unwrap();
                    let p = file.path().to_str().unwrap();

                    if !regex.is_match(name) && config.reverse {
                        println!("{}", p);
                        return;
                    }
                    let matches = regex.find_iter(name);
                    for _match in matches {
                        println!("{}", highlight(name, _match.start(), _match.end(),"\x1B[1;32m"));
                    }
                }
                Target::Contents => {
                    let p = file.path().to_str().unwrap();
                    let open = tokio::fs::File::open(p).await;
                    let Ok(mut open) = open else {
                        return;
                    };
                    let mut content = Vec::new();
                    open.read_to_end(&mut content).await.unwrap();
                    let content = content.as_slice();
                    let str_content = String::from_utf8_lossy(content);
                    let lines = str_content.split("\n").collect::<Vec<_>>();

                    for (index, line) in lines.iter().enumerate() {
                        if !regex.is_match(line) && config.reverse {
                            println!("{}:\n{}. {}", p, index + 1, line);
                            continue;
                        }
                        let matches = regex.find_iter(line);
                        for _match in matches {
                            let h_p = highlight(p, 0, p.len(),"\x1B[1;31m");
                            let h_l = highlight(line, _match.start(), _match.end(),"\x1B[1;32m");
                            println!("{}:\n{}. {}",h_p , index, h_l);
                        }
                    }
                }
            };
        })
    });
    Ok(())
}

fn highlight(input: &str, start: usize, end: usize,color:&str) -> String {
    let mut result = String::new();

    result.push_str(&input[..start]);
    //green "\x1B[1;32m"
    //red "\x1B[1;31m"
    result.push_str(color);
    result.push_str(&input[start..end]);
    result.push_str("\x1B[0m"); // Reset color

    result.push_str(&input[end..]);

    result
}