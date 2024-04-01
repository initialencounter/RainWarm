use fltk::{prelude::*, enums::Event, *};
mod get_sha256;
use get_sha256::get_sha256;

fn main() {
    let app = app::App::default();
    let buf = text::TextBuffer::default();
    let mut wind = window::Window::default().with_size(800, 400);
    let mut disp = text::TextDisplay::default_fill();
    wind.end();
    wind.show();

    disp.set_buffer(buf.clone());
    disp.handle({
        let mut dnd = false;
        let mut released = false;
        let mut buf = buf.clone();
        move |_, ev| match ev {
            Event::DndEnter => {
                dnd = true;
                true
            }
            Event::DndDrag => true,
            Event::DndRelease => {
                released = true;
                true
            }
            Event::Paste => {
                if dnd && released {
                    let mut path = app::event_text();
                    let sha256 = drag_file(&path);
                    let text = format!("{}------{}\n",path,sha256);
                    buf.append(&text);
                    dnd = false;
                    released = false;
                    true
                } else {
                    false
                }
            }
            Event::DndLeave => {
                dnd = false;
                released = false;
                true
            }
            _ => false,
        }
    });
    app.run().unwrap();
}

fn drag_file(data: &str) -> String{
    get_sha256(data)
}