extern crate winres;

fn main() {
    use std::io::Write;

    //setup resource
    let mut res = winres::WindowsResource::new();
    res.set_manifest_file("./manifest.xml");
    res.compile().unwrap();
}