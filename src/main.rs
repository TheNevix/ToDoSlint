slint::include_modules!();
fn main() -> Result<(), slint::PlatformError>{
    let main = MainApp::new()?;

    main.run()
}
