use std::io;

pub fn input_string(output_message_for_user: String) -> String {
    let mut message = String::new();
     
    println!("\n{output_message_for_user}");
    
    io::stdin().read_line(&mut message).expect("Failed to read line");
    
    String::from(message.trim())
}
