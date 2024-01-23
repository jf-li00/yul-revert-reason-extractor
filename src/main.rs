use regex::Regex;
use std::{fs,env,collections::HashMap};
use walkdir::WalkDir;
use tiny_keccak::{Hasher,Keccak};
fn main() {
    let re = Regex::new(r#"revertReasonIfDebugFunction\(\"(.?*)\"\)"#).expect("unable to build new regex");
    let args : Vec<String> = env::args().collect();
    assert!(args.len()>=2);
    let mut raintable = HashMap::new();
    let solidity_repo_path = &args[1];
    for entry in WalkDir::new(solidity_repo_path).into_iter().filter_map(|e| e.ok()){
        let path = entry.into_path();
        match &path.extension(){
            Some(ext) =>{
                if ext != &"cpp" {
                    continue;
                }
            }
            None =>{
                continue;
            }
        }

        let content = fs::read_to_string(&path).expect("unable to read file");
        for cap in re.captures_iter(&content).map(|c| c.extract::<1>()){
            let revert_reason = &cap.1[0];
            let mut hasher =Keccak::v256();
            hasher.update(revert_reason.as_bytes());
            let mut out_put = [0u8;32];
            hasher.finalize(&mut out_put);
            let hash_hex = out_put.iter().map(|byte| format!("{:02x}",byte)).collect::<String>();
            println!("Revert function call found in {:?}", &path);
            println!("{:?} : {:}",revert_reason,hash_hex);
            raintable.insert(hash_hex, revert_reason.to_string());
        }
    }
    let json = serde_json::to_string_pretty(&raintable).expect("unable to dump raintable to json");
    fs::write("./out.json", &json).expect("Unable to write result into file");

}
