use rust_password_generator::{Params, get_user_input};


fn main() {
    println!("Enter the length of your password");
    let password_length = get_user_input();
    println!("Enter number of special characters");
    let special_chars = get_user_input();  
    println!("Enter number of 'numeric characters'");
    let numeric_chars = get_user_input();

    let params: Params = Params {
        password_length,
        special_chars,
        numeric_chars
    };

    params.generate_password()
}
