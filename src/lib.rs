pub mod image;
pub mod channel;
pub mod color_space;

pub fn casual_greeting(name: &str) -> String {
    format!("Hey {}! What's up?", name)
}
