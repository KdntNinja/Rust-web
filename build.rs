extern crate diesel_migrations;

fn main() {
    // This build script is used to ensure that migrations are properly
    // processed at compile time
    println!("cargo:rerun-if-changed=migrations");
}
