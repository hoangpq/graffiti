#[macro_use]
extern crate log;

mod app;
mod ffi;
mod generated;
mod layout;
mod render;
mod storage;
mod text;
mod window;

// for easier maintenance the whole project is split to multiple sub-systems
// - to some degree they are independent but to make things a bit simpler
//   they are also stateful & fully aware of any changes to the "scene"
// - this way, each can store exactly what it needs and stay straight
//   to the point (less abstractions)
// - for example renderer is interested in text color, text layout needs
//   to know about the actual str, flexbox needs to know about size, parents
//   and so on
// - some services are shared and need to know about each other,
//   this is expected and ok (Rc)
// - there's no DIC, injection is done by hand
//
// TODO: rename (it's not just listener)
pub trait SceneListener {
    fn update_scene(&mut self, msgs: &[generated::UpdateSceneMsg]);
}
