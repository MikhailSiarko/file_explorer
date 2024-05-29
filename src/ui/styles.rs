use iced::{theme, widget::svg, Color, Theme};

pub struct SvgStyles;

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

    pub fn themed() -> theme::Svg {
        theme::Svg::custom_fn(SvgStyles::from_theme)
    }

    pub fn light() -> theme::Svg {
        theme::Svg::custom_fn(|_| svg::Appearance {
            color: Some(Color {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: 0.9,
            }),
        })
    }
}
