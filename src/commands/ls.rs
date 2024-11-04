use crate::Terminal;

use std::{fs, path::PathBuf};


impl Terminal {
    pub fn ls(&mut self, final_output: &mut String) {

        final_output.push_str("\n");
        // let mut line_vec = Vec::new();
        let mut elements: Vec<String> = Vec::new();
            
        if let Ok(entries) = fs::read_dir(self.current_path.as_os_str()) {
                
            // println!("{}", prefix);
            for entry in entries {
                if let Ok(entry) = entry {



                    let path = entry.path();

                    let result = self.get_file_name_as_string(path);

                    elements.push(result);
                };
            }
        }

        for (i, element) in elements.iter().enumerate() {


            final_output.push_str(&element);

            if (i + 1) % 3 == 0 || i == elements.len() - 1 {
                final_output.push_str("\n");
            }
        }


        self.output.push(final_output.to_string());
    }

    fn get_file_name_as_string(&self, file_path_buf: PathBuf) -> String {
        let metadata = fs::metadata(&file_path_buf).expect("Unable to read metadata");
        
        let prefix = self.current_path.to_str().unwrap();
        let mut result: String = String::new();
        if metadata.is_dir() {
            if let Some(path_str) = file_path_buf.to_str() {
                if let Some(dir_name) = path_str.strip_prefix(prefix) {

                    let dir_name_string;
                    if cfg!(target_os = "windows") {
                        
                        dir_name_string = dir_name.to_owned() + "\\";
                        
                    } else {
                        
                        dir_name_string = dir_name.to_owned() + "/";
                        
                    }
                    
                    if dir_name_string.len() <= 15 {
                        result = format!("{:.15}", dir_name_string);
                        result = format!("{:<30}", result)
                    } else {
                        if cfg!(target_os = "windows") {
                            result = format!("{:.11}...\\", dir_name_string);
                            result = format!("{:<30}", result)
                        } else {
                            result = format!("{:.11}.../", dir_name_string);
                            result = format!("{:<30}", result)
                        }
                    }
                }
            }
        }else if metadata.is_file() {
            if let Some(path_str) = file_path_buf.to_str() {
                if let Some(file_name) = path_str.strip_prefix(prefix) {
                    let file_name_string = file_name.to_string();
                    if file_name_string.len() <= 15 {
                        result = format!("{:.15}", file_name_string);
                        result = format!("{:<30}", result)
                    } else {
                        result = format!("{:.12}...", file_name_string);
                        result = format!("{:<30}", result)
                    }
                }
            }
            
        }
        result
    }
}