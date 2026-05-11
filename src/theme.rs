use ratatui::style::{Color, Modifier, Style};

pub struct Theme;

impl Theme {
    pub const BG: Color = Color::Rgb(30, 30, 46);

    pub const PRIMARY: Color = Color::Rgb(139, 92, 246);
    pub const SECONDARY: Color = Color::Rgb(196, 181, 253);

    pub const SUCCESS: Color = Color::Rgb(34, 197, 94);

    pub const WARNING: Color = Color::Rgb(245, 158, 11);
    pub const ERROR: Color = Color::Rgb(239, 68, 68);

    pub const TEXT: Color = Color::Rgb(226, 232, 240);
    pub const MUTED: Color = Color::Rgb(148, 163, 184);

    pub const INFO: Color = Color::Rgb(56, 189, 248);

    pub fn primary() -> Style {
        Style::default().fg(Self::PRIMARY)
    }

    pub fn secondary() -> Style {
        Style::default().fg(Self::SECONDARY)
    }

    pub fn block() -> Style {
        Style::default().fg(Self::PRIMARY).bg(Self::BG)
    }

    pub fn text() -> Style {
        Style::default().fg(Self::TEXT)
    }

    pub fn muted() -> Style {
        Style::default().fg(Self::MUTED)
    }

    pub fn title() -> Style {
        Style::default()
            .fg(Self::SECONDARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected() -> Style {
        Style::default()
            .bg(Self::PRIMARY)
            .fg(Self::TEXT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success() -> Style {
        Style::default().fg(Self::SUCCESS)
    }

    pub fn warning() -> Style {
        Style::default().fg(Self::WARNING)
    }

    pub fn error() -> Style {
        Style::default().fg(Self::ERROR)
    }

    pub fn info() -> Style {
        Style::default().fg(Self::INFO)
    }
}
