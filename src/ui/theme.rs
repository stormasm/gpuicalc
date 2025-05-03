use gpui::{Global, Rgba, rgba};

pub struct Colors {
    pub bg_window: Rgba,
    pub bg_button: Rgba,
    pub text: Rgba,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            bg_window: rgba(0xcccccc07),
            bg_button: rgba(0x45454535),
            text: rgba(0xffffffcc),
        }
    }
}

pub struct Fonts {
    pub family: String,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            family: "Noto Sans".into(),
        }
    }
}

#[derive(Default)]
pub struct Theme {
    pub colors: Colors,
    pub fonts: Fonts,
}

impl Global for Theme {}
