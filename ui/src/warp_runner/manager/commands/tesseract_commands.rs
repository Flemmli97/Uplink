use derive_more::Display;
use futures::channel::oneshot;

#[derive(Display)]
pub enum TesseractCmd {
    #[display(fmt = "KeyExists {{ {key} }} ")]
    KeyExists {
        key: String,
        rsp: oneshot::Sender<bool>,
    },
}

impl std::fmt::Debug for TesseractCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
