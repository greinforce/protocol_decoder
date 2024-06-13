
use dioxus::prelude::*;
use dioxus_desktop::muda::Menu;
use dioxus_desktop::WindowBuilder;
mod Desktop_Init;
mod globals;
mod protocols;
use Desktop_Init::Desktop_init;



fn main() {

    let menu = Menu::new();
    let config =
        dioxus::desktop::Config::new()
            .with_window(WindowBuilder::default()
                .with_title("协议解析工具").with_maximized(true)).with_menu(menu);
    LaunchBuilder::desktop().with_cfg(config).launch(app);

}
fn app()->Element{
    Desktop_init()
}