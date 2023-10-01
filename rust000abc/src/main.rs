#[path = "./modules/my_module.rs"]
mod my_module;

fn main() {
    println!("Hello, world!");
    my_module::hello();
}
