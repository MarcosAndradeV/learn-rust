mod build_full_name;
mod error_handler;
mod factory;
mod life_time;

#[allow(dead_code)]
fn tests() {
    factory::test();
    error_handler::test();
    build_full_name::test();
    life_time::test();
}

fn main() {
    println!("Hello World");
}
