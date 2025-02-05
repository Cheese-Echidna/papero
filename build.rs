use winres;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().expect("Failed to compile resources");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("Not windows, its okay, but no icon this time")
}