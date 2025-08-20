use bitcoin::Address;
use std::str::FromStr;

fn main() {
    let address_str = "tltc1pmdlud63r480wh4q5vnn453fmuzhsesyd0ct463h79g68c3mg7huqce94v8";
    println!("Trying to parse: {}", address_str);
    
    match Address::from_str(address_str) {
        Ok(addr) => {
            println!("✅ Successfully parsed: {}", addr);
            println!("Address type: {:?}", addr);
        }
        Err(e) => {
            println!("❌ Failed to parse: {:?}", e);
        }
    }
}
