/*Create a Rust function called greet_user that takes a user's name as a parameter and prints a personalized greeting message. */


fn greet_user(name :&str){
    println!("Hi {} , how are you doing!",name);
}

fn main() {
   println!("Enter the name ");
   let mut userName = String::new();
   std::io::stdin().read_line(&mut userName).expect("Something error happened");

   userName = userName.trim().to_string();
   greet_user(&userName);
}