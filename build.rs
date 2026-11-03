// Embeds the program icon in the Windows .exe. Nothing to do on other systems.
fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().expect("couldn't embed the Windows icon");
    }
}
