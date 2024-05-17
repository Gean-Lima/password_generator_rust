fn main() {
    let options = password_generator::PasswordOptions::get_user_inputs();
    let password = password_generator::generate_by_length_digits(options);

    println!("\nSenha: {password}");
}
