use crate::Terminal;

use std::fs;

impl Terminal {
    pub fn ls(&mut self, final_output: &mut String) {


        let prefix = self.current_path.to_str().unwrap();
        println!("{}", prefix);
        
        if let Ok(entries) = fs::read_dir(self.current_path.as_os_str()) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(path_str) = path.to_str() {

                        if let Some(bruh) = path_str.strip_prefix(prefix) {

                            let bruh_bruh = format!("{}    ,", bruh);

                            final_output.push_str(bruh_bruh.as_str());
                            println!("{}", path_str);
                        } else {
                            final_output.push_str("Cannot ls for somehow");
                        }
                            

                    } else {
                        final_output.push_str("Failed to ls.");
                    }
                }
            }
        }
        self.output.push(final_output.to_string());
    }
}