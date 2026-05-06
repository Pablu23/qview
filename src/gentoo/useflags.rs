#[derive(Debug, Clone)]
pub struct UseFlag {
    pub name: String,
    pub enabled: bool,
    pub default: bool,
}

impl UseFlag {
    pub fn new(name: String, enabled: bool, default: bool) -> Self {
        UseFlag {
            name,
            enabled,
            default,
        }
    }
}
