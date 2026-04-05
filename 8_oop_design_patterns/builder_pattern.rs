#[derive(Debug)]
pub struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
    is_resizable: bool,
    has_decorations: bool,
}

pub struct WindowConfigBuilder {
    title: String,
    width: Option<u32>,
    height: Option<u32>,
    is_resizable: Option<bool>,
    has_decorations: Option<bool>,
}

impl WindowConfigBuilder {
    pub fn new(title: String) -> Self {
        WindowConfigBuilder {
            title,
            width: None,
            height: None,
            is_resizable: None,
            has_decorations: None,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.is_resizable = Some(resizable);
        self
    }

    pub fn decorations(mut self, decorations: bool) -> Self {
        self.has_decorations = Some(decorations);
        self
    }

    pub fn build(self) -> WindowConfig {
        WindowConfig {
            title: self.title,
            width: self.width.unwrap_or(800),
            height: self.height.unwrap_or(600),
            is_resizable: self.is_resizable.unwrap_or(true),
            has_decorations: self.has_decorations.unwrap_or(true),
        }
    }
}

fn main() {
    let basic_window = WindowConfigBuilder::new("My App".to_string()).build();

    let custom_window = WindowConfigBuilder::new("Game Window".to_string())
        .width(1024)
        .height(768)
        .resizable(false)
        .build();

    let fullscreen_window = WindowConfigBuilder::new("Fullscreen".to_string())
        .decorations(false)
        .build();

    println!("Basic Window: {:?}", basic_window);
    println!("Custom Window: {:?}", custom_window);
    println!("Fullscreen Window: {:?}", fullscreen_window);
}
