extern crate gui_core_00;
extern crate glutin;

use gui_core_00::app::GuiApplication00;
use glutin::dpi::LogicalSize;

fn main() {

    GuiApplication00::new("Test Title".to_string(), LogicalSize {width: 1440.0, height: 900.0})
    .expect("Application startup failed, see logs");

}