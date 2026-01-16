use ratatui::style::{Color, Modifier, Style};

/// Theme trait providing semantic colors and styles.
pub trait Theme {
    // Base colors
    fn bg(&self) -> Color;
    fn fg(&self) -> Color;
    fn surface(&self) -> Color;
    fn border(&self) -> Color;

    // Semantic accents
    fn primary(&self) -> Color;
    fn success(&self) -> Color;
    fn warning(&self) -> Color;
    fn error(&self) -> Color;
    fn info(&self) -> Color;

    // Diff colors
    fn diff_add_bg(&self) -> Color;
    fn diff_add_fg(&self) -> Color;
    fn diff_del_bg(&self) -> Color;
    fn diff_del_fg(&self) -> Color;
    fn diff_context(&self) -> Color;

    // Component styles
    fn header_style(&self) -> Style {
        Style::default().fg(self.fg()).add_modifier(Modifier::DIM)
    }

    fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.fg())
            .bg(self.surface())
            .add_modifier(Modifier::BOLD)
    }

    fn focused_border(&self) -> Style {
        Style::default().fg(self.primary())
    }

    fn unfocused_border(&self) -> Style {
        Style::default().fg(self.border())
    }

    fn prompt_style(&self) -> Style {
        Style::default()
            .fg(self.fg())
            .bg(self.surface())
            .add_modifier(Modifier::BOLD)
    }
}

/// Catppuccin Mocha (dark) palette.
pub struct CatppuccinMocha;

impl Theme for CatppuccinMocha {
    fn bg(&self) -> Color {
        Color::Rgb(17, 17, 27)
    }
    fn fg(&self) -> Color {
        Color::Rgb(235, 235, 245)
    }
    fn surface(&self) -> Color {
        Color::Rgb(30, 30, 46)
    }
    fn border(&self) -> Color {
        Color::Rgb(69, 71, 90)
    }
    fn primary(&self) -> Color {
        Color::Rgb(137, 180, 250)
    }
    fn success(&self) -> Color {
        Color::Rgb(166, 227, 161)
    }
    fn warning(&self) -> Color {
        Color::Rgb(249, 226, 175)
    }
    fn error(&self) -> Color {
        Color::Rgb(243, 139, 168)
    }
    fn info(&self) -> Color {
        Color::Rgb(148, 226, 213)
    }
    fn diff_add_bg(&self) -> Color {
        Color::Rgb(22, 42, 29)
    }
    fn diff_add_fg(&self) -> Color {
        self.success()
    }
    fn diff_del_bg(&self) -> Color {
        Color::Rgb(47, 23, 28)
    }
    fn diff_del_fg(&self) -> Color {
        self.error()
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(108, 112, 134)
    }
}

/// Catppuccin Latte (light) palette.
pub struct CatppuccinLatte;

impl Theme for CatppuccinLatte {
    fn bg(&self) -> Color {
        Color::Rgb(239, 241, 245)
    }
    fn fg(&self) -> Color {
        Color::Rgb(76, 79, 105)
    }
    fn surface(&self) -> Color {
        Color::Rgb(248, 250, 252)
    }
    fn border(&self) -> Color {
        Color::Rgb(204, 208, 218)
    }
    fn primary(&self) -> Color {
        Color::Rgb(30, 102, 245)
    }
    fn success(&self) -> Color {
        Color::Rgb(60, 110, 113)
    }
    fn warning(&self) -> Color {
        Color::Rgb(223, 142, 29)
    }
    fn error(&self) -> Color {
        Color::Rgb(210, 15, 57)
    }
    fn info(&self) -> Color {
        Color::Rgb(4, 165, 229)
    }
    fn diff_add_bg(&self) -> Color {
        Color::Rgb(219, 236, 224)
    }
    fn diff_add_fg(&self) -> Color {
        self.success()
    }
    fn diff_del_bg(&self) -> Color {
        Color::Rgb(246, 226, 228)
    }
    fn diff_del_fg(&self) -> Color {
        self.error()
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(116, 118, 142)
    }
}
