//       - Define `mod cache`:
//           - Inside it, define a submodule `memory`:
//               - Function `pub fn get(key: &str) -> Option<String>`
//               - Function `pub fn set(key: &str, val: &str)`
//   - In `_ex10`, test retrieving and setting data via your module structure.



    pub mod memory{
        pub fn get(key:&str) -> Option<String>{
            println!("Getting key: {}", key);
            Some("value".to_string())
        }
        pub fn set(key:&str,val:&str){
            println!("Setting {} = {}", key, val);
        }
    }
