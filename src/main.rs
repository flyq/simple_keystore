pub use ethkey::prelude::*;
mod ethkey;

fn main() {
   
   let key = EthAccount::load_or_generate("./test.json", "test")	
        .expect("should load or generate new eth key");

    println!("{:?}", key.address());

    let message = [7_u8; 32];

    // sign the message
    let signature = key.sign(&message).unwrap();

    // verify the signature
    let result = key.verify(&signature, &message).unwrap();
    println!("{}", if result {"verification ok"} else {"wrong signature"});
    //    key.change_password("test");
    println!("{:?}", key);
    println!("{:?}", key.kestore_path());
    println!("{:?}", key.public());
    println!("{:?}", key.secret());    
}