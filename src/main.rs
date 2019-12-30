use druid::{AppLauncher, Data, Env, WindowDesc, Widget, PlatformError};
use druid::widget::{Label, Flex, Padding, Align};

// fn build_ui() -> impl Widget<()> {
//     Label::new("Hello world")
// }

// fn build_ui() -> impl Widget<()> {
//     Flex::row()
//         .with_child(
//             Flex::column()
//                 .with_child(Label::new("top left"), 1.0)
//                 .with_child(Label::new("bottom left"), 1.0), 
//             1.0)
//         .with_child(
//             Flex::column()
//                 .with_child(Label::new("top right"), 1.0)
//                 .with_child(Label::new("bottom right"), 1.0),
//             1.0)
// }

fn build_ui() -> impl Widget<AppState> {
    Padding::new(
        10.0,
        Flex::row()
            .with_child(
                Flex::column()
                    .with_child(Label::new(|data: &AppState, _env: &Env| data.top_left.clone()), 1.0)
                    .with_child(Align::centered(Label::new(|data: &AppState, _env: &Env| data.bottom_left.clone())), 1.0), 
                1.0)
            .with_child(
                Flex::column()
                .with_child(Label::new(|data: &AppState, _env: &Env| data.top_right.clone()), 1.0)
                .with_child(Align::centered(Label::new(|data: &AppState, _env: &Env| data.bottom_right.clone())), 1.0), 
                1.0))
}

// fn build_ui() -> impl Widget<AppState> {
//     Padding::new(
//         10.0,
//         Flex::row()
//             .with_child(
//                 Flex::column()
//                     .with_child(
//                         Button::new("Button", move |_, _, _| println!("Button pressed")), 1.0
//                     )
//                     .with_child(centered_label("bottom left"), 1.0), 
//                 1.0)
//             .with_child(
//                 Flex::column()
//                     .with_child(Label::new("top right"), 1.0)
//                     .with_child(centered_label("bottom right"), 1.0),
//                 1.0))
// }

fn main() -> Result<(), PlatformError> {
    let data = AppState::new();

    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(data)?;
    Ok(())
}

#[derive(Clone, Data)]
struct AppState {
    top_left: String,
    top_right: String,
    bottom_left: String,
    bottom_right: String,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            top_left: "top left".to_string(),
            top_right: "top right".to_string(),
            bottom_left: "bottom left".to_string(),
            bottom_right: "bottom right".to_string(),
        }
    }
}
