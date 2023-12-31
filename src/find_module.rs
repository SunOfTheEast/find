    use regex::Regex;
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::process;
    use colored::*;
    pub fn find<P: AsRef<Path>>(root: P, regex: &Regex, verbose: &i32) -> Result<Vec<String>, Box<dyn std::error::Error>>{
        let mut matches = Vec::new();
        walk_tree(root.as_ref(), regex, &mut matches, verbose)?;
        Ok(matches)
    }
    fn walk_tree(
        dir: &Path,
        regex: &Regex,
        matches: &mut Vec<String>,
        verbose: &i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("verbose: ");
        if dir.is_dir(){
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir(){
                    walk_tree(&path, regex, matches, verbose)?;
                } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()){
                    if *verbose != 0{
                        println!("{} ", path.to_string_lossy().to_string().red());//打印相应遍历的路径，都是用红色
                    }
                    if regex.is_match(filename){
                        matches.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        Ok(())
    }
