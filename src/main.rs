mod build_full_name;
mod error_handler;
mod factory;

#[allow(dead_code)]
fn tests() {
    factory::test();
    error_handler::test();
    build_full_name::test();
}

fn main() {
    println!("Hello World");
}
