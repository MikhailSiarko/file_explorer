use iced::{widget::svg, Color, Theme};

pub enum SvgStyles {
    Themed,
    Light,
    Dark,
}

impl SvgStyles {
    fn from_theme(theme: &Theme) -> svg::Appearance {
        match theme {
            Theme::Light | Theme::GruvboxLight | Theme::SolarizedLight | Theme::TokyoNightLight => {
                svg::Appearance {
                    color: Some(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    }),
                }
            }
            _ => svg::Appearance {
                color: Some(Color {
                    r: 255.0,
                    g: 255.0,
                    b: 255.0,
                    a: 0.9,
                }),
            },
        }
    }

    fn light() -> svg::Appearance {
        svg::Appearance {
            color: Some(Color {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: 0.9,
            }),
        }
    }

    fn dark() -> svg::Appearance {
        svg::Appearance {
            color: Some(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            }),
        }
    }
}

impl svg::StyleSheet for SvgStyles {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> svg::Appearance {
        match self {
            SvgStyles::Themed => Self::from_theme(style),
            SvgStyles::Light => Self::light(),
            SvgStyles::Dark => Self::dark(),
        }
    }
}
