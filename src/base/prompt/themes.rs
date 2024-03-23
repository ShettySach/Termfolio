use leptos::Signal;
use leptos_use::{
    use_color_mode_with_options, use_cycle_list_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn, UseCycleListOptions, UseCycleListReturn,
};

// Last theme will be default
static THEMES: [&str; 4] = ["catppuccin", "nord", "classic", "tokyo-night"];

pub fn theme_changer() -> (Signal<ColorMode>, impl Fn() + Clone) {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .custom_modes(THEMES.iter().map(|&s| s.to_string()).collect())
            .initial_value(ColorMode::from(THEMES.last().unwrap().to_string())),
    );

    let UseCycleListReturn { state, next, .. } = use_cycle_list_with_options(
        THEMES
            .iter()
            .map(|s| ColorMode::Custom(s.to_string()))
            .collect::<Vec<ColorMode>>(),
        UseCycleListOptions::default().initial_value(Some((mode, set_mode).into())),
    );

    (state, next)
}
