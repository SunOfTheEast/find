pub mod find_module;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use colored::*;
use find_module::find;
fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        eprintln!("使用方式: {}<目标目录> <数量> <要搜索的正则表达式>..<-v>", args[0]); //可以输入多个要查找的关键字,最后一个参数代表是否verbose
        process::exit(1);
    }
    let number_of_exp = (&args[2]).trim().parse::<usize>().unwrap();

    if args.len() > 4 + number_of_exp{
        eprintln!("使用方式: {}<目标目录> <数量> <要搜索的正则表达式>..<-v>", args[0]);
        process::exit(1);
    }
    let mut verbose = 0;
    if args[args.len() - 1] == "-v"{//是否添加verbose选项用整型表示，不用bool的原因是方便拓展（例如之后要修改展示的粒度）
        verbose = 1;//
    }
    let mut number_searched = 0;
    loop {//利用一个loop查找多个关键字
        if number_searched == number_of_exp {
            break;
        }
        let pattern = &args[number_searched + 3];
        let regex = match Regex::new(pattern){
            Ok(re) => re,
            Err(err) => {
                eprintln!("err '{}' : {}", pattern, err);
                process::exit(1);
            }
        };
        match find(&args[1], &regex, &verbose){
            Ok(matches) => {
                if matches.is_empty(){
                    println!("{} times: {}", number_searched, "Not find!".bright_red()); //没有找到，输出亮红色提示
                }else {
                    println!("{} times: {}", number_searched, "find".green());
                    for file in matches{
                        println!("{}", file.bright_red());
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        }
        number_searched += 1;
    }
}
    /*let pattern = &args[2];
    let regex = match Regex::new(pattern){
        Ok(re) => re,
        Err(err) => {
            eprintln!("err '{}' : {}", pattern, err);
            process::exit(1);
        }
    };
    match find(&args[1], &regex){
        Ok(matches) => {
            if matches.is_empty(){
                println!("Not find!");
            }else{
                println!("find!");
                for file in matches{
                    println!("{}", file);
                }
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }*/




