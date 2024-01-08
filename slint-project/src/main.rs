slint::include_modules!();

use std::fs;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.on_get_samples(move || {
        let ui = ui_handle.unwrap();
        
        let paths = fs::read_dir("./samples").unwrap();

        return paths.map(|path| {
            let path = path.unwrap().path();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let content = fs::read_to_string(path).unwrap();
            Sample { name }
        }).collect();
    });

    ui.run()
}
