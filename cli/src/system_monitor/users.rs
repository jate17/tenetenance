use uzers::{all_users};
use crate::logs::{Logs};



///
/// Struct for user  
/// 
/// # Struct 
/// 
/// - UID: u32 
/// - Name: String 
/// - Groups: string inside the vector
/// 
/// 
#[derive(Debug, Clone)]
pub struct User {
    uid: u32,
    name: String,
    groups: Vec<String>
}   


/// Get all users info 
/// 
/// # Return 
/// 
/// - Struct user inside the vector 
pub fn get_users() -> Vec<User>{
    let users = unsafe { all_users()};
    
    let mut output: Vec<User> = Vec::new();
    
    for user in users {
        let uid = user.uid();  
        let groups: Vec<String> = user.groups()
            .unwrap()  // Estrae il valore, panic se None
            .iter()
            .map(|g| g.name().to_string_lossy().to_string())
            .collect();
        let name = user.name().to_string_lossy().to_string();

        output.push(User{
            uid,
            name,
            groups
        });
    }
    let _ = Logs::trace("Users check");
    output
}
