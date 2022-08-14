
mod core;
mod tooling;
mod postprocess;

fn main() {

    let sourcea: &str = "/home/bencaddyro/doit/3bf/openDIT/sourceA/";
    let sourceb: &str = "/home/bencaddyro/doit/3bf/openDIT/sourceB/";

    println!("Test !");
    println!("Source: {sourcea}");
    println!("Destination: {sourceb}");

    core::sync_a_b(sourcea, sourceb);
//     core::copy_a_b(sourcea, sourceb);
//     core::integrity_a_b(sourcea, sourceb);


}
