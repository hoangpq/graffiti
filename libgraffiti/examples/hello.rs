use graffiti::{
    app::App,
    render::value_types::*,
    text::TextAlign,
    viewport::{Event, GlViewport, StyleProp},
};

fn main() {
    let mut app = unsafe { App::init() };
    let w = app.create_window("Hello", 800, 600);

    app.update_window_scene(w, &mut |v| {
        let el = v.create_element();
        let text = v.create_text_node();

        v.set_style(el, &StyleProp::BackgroundColor(Color::BLUE));
        v.set_style(el, &StyleProp::Color(Color::WHITE));
        v.set_style(el, &StyleProp::FontFamily("sans-serif".to_string()));
        v.set_style(el, &StyleProp::FontSize(20.));
        v.set_style(el, &StyleProp::TextAlign(TextAlign::Left));
        v.set_style(el, &StyleProp::LineHeight(30.));

        v.set_text(text, "Hello world!".to_string());

        v.insert_child(GlViewport::ROOT, 0, el);
        v.insert_child(el, 0, text);
    });

    // loop
    'outer: loop {
        for e in app.get_events(false) {
            if let Event::Close { .. } = e.event {
                break 'outer;
            }

            println!("{:?}", &e);
        }
    }
}