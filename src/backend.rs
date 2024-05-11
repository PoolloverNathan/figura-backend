use std::borrow::Cow;

trait Backend: Default {
    fn limits(&self, token: &str) -> ();
    fn version(&self) -> (&str, &str);
    fn motd(&self) -> Cow<str>;
}
