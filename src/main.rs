use druid::{
    widget::{Button, Flex, Label, Painter, Scroll, ViewSwitcher, WidgetExt},
    AppLauncher, Data, DelegateCtx, Env, FontDescriptor, FontFamily, Handled, Lens, LocalizedString, Rect, Selector, Target, Widget, WindowDesc, Command, RenderContext, Color, AppDelegate
};
use rfd::FileDialog;
use reverse_engineering_lib::{color_based_hex, view_file, extract_detail_exe};
use std::{sync::Arc, thread};

const UPDATE_CONTENT: Selector<Arc<String>> = Selector::new("update_content");
const TOGGLE_VIEW: Selector<()> = Selector::new("toggle_view");
const UPDATE_COLOR_DATA: Selector<Arc<Vec<u8>>> = Selector::new("update_color_data");
const UPDATE_EXE_DETAILS: Selector<Arc<String>> = Selector::new("update_exe_details");


#[derive(Clone, Data, Lens)]
struct AppState {
    hex_data: Arc<String>,
    color_data: Arc<Vec<u8>>,
    exe_details: Arc<String>,
    view_mode: ViewMode,
}

impl AppState {
    fn toggle_view_mode(&mut self) {
        match self.view_mode {
            ViewMode::Hex => self.view_mode = ViewMode::Color,
            ViewMode::Color => self.view_mode = ViewMode::Preferences,
            ViewMode::Preferences => self.view_mode = ViewMode::Hex,
        }
    }
}

#[derive(Clone, Data, PartialEq)]
enum ViewMode {
    Hex,
    Color,
    Preferences,
}

struct Delegate;

impl AppDelegate<AppState> for Delegate {
    
    fn command(&mut self, _ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env) -> Handled {
        match cmd {
            _ if cmd.is(UPDATE_CONTENT) => {
                if let Some(content) = cmd.get(UPDATE_CONTENT) {
                    data.hex_data = Arc::clone(content);
                    Handled::Yes
                } else {
                    Handled::No
                }
            },
            _ if cmd.is(UPDATE_COLOR_DATA) => {
                if let Some(color_data) = cmd.get(UPDATE_COLOR_DATA) {
                    data.color_data = Arc::clone(color_data);
                    Handled::Yes
                } else {
                    Handled::No
                }
            },
            _ if cmd.is(UPDATE_EXE_DETAILS) => {
                if let Some(details) = cmd.get(UPDATE_EXE_DETAILS) {
                    data.exe_details = Arc::clone(details);
                    Handled::Yes
                } else {
                    Handled::No
                }
            },
            _ if cmd.is(TOGGLE_VIEW) => {
                data.view_mode = match data.view_mode {
                    ViewMode::Hex => ViewMode::Color,
                    ViewMode::Color => ViewMode::Preferences,
                    ViewMode::Preferences => ViewMode::Hex,
                };
                Handled::Yes
            },
            _ => Handled::No,
        }
    }
    
}

impl AppState {
    fn new() -> Self {
        Self {
            hex_data: Arc::new("".into()),
            color_data: Arc::new(vec![]),
            exe_details: Arc::new("".into()),
            view_mode: ViewMode::Hex,
        }
    }
}

fn build_ui() -> impl Widget<AppState> {
    let toggle_button = Button::new("Toggle View")
        .on_click(|_ctx, data: &mut AppState, _env| {
            data.toggle_view_mode();
        });

    let content_view = ViewSwitcher::new(
        |data: &AppState, _env| data.view_mode.clone(),
        |view_mode, _data: &AppState, _env| match view_mode {
            ViewMode::Hex => Box::new(Scroll::new(Label::new(move |data: &AppState, _env: &Env| {
                data.hex_data.to_string()
            })).vertical()),
            ViewMode::Color => {
                let painter = Painter::new(move |ctx, data: &AppState, _env| {
                    if !data.color_data.is_empty() {
                        let size = ctx.size();
                        let rect = Rect::from_origin_size((0.0, 0.0), size);
                        ctx.fill(rect, &Color::WHITE);

                        for (i, &color_byte) in data.color_data.iter().enumerate() {
                            let square_size = 10.0;
                            let x = (i as f64 % (size.width / square_size)) * square_size;
                            let y = (i as f64 / (size.width / square_size)).floor() * square_size;
                            let square_rect = Rect::from_origin_size((x, y), (square_size, square_size));
                            let color = Color::rgba8(color_byte, color_byte, color_byte, 255);
                            ctx.fill(square_rect, &color);
                        }
                    }
                });
                Box::new(painter)
            },
            ViewMode::Preferences => {
                let details_label = Label::new(move |data: &AppState, _env: &Env| {
                    let details_lines = data.exe_details
                        .split(", ")
                        .map(|line| line.trim()
                            .replace("{", "")
                            .replace("}", "")
                            .replace("\":\"", ": ")
                            .replace("\"", ""))
                        .collect::<Vec<_>>()
                        .join("\n");
            
                    format!("Executable Details:\n{}", details_lines)
                })
                .with_line_break_mode(druid::widget::LineBreaking::WordWrap); // Ensure long lines wrap instead of extending horizontally
            
                // Wrapping the details label in a Scroll widget to allow vertical scrolling
                let scrollable_details = Scroll::new(details_label)
                    .vertical() // Enable vertical scrolling
                    .expand_width(); // Ensure the scroll area expands to the width of its container
            
                Box::new(scrollable_details)
            }
            ,
        },
    );

    Flex::column()
        .with_child(toggle_button)
        .with_flex_child(content_view, 1.0)
        .env_scope(|env, _data| {
            env.set(druid::theme::UI_FONT, FontDescriptor::new(FontFamily::MONOSPACE).with_size(12.0));
        })
}

fn main() {

    let initial_state = AppState::new();

    let launcher = AppLauncher::with_window(
        WindowDesc::new(build_ui())
            .title(LocalizedString::new("Hex Editor"))
            .window_size((720.0, 480.0))
    ).delegate(Delegate{}); 
    

    let external_handle = launcher.get_external_handle();

thread::spawn(move || {
    let file_path = FileDialog::new().pick_file().map(|p| p.to_string_lossy().into_owned());

    if let Some(path) = file_path {
        //let detail_exe = extract_detail_exe(&path);

        // Fetch and send hex content
        let hex_content = view_file(&path).unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        });
        external_handle.submit_command(UPDATE_CONTENT, Arc::new(hex_content), Target::Auto).expect("Failed to send hex content update command");

        // Fetch and send color data
        let color_data_result = color_based_hex(path.clone());
        match color_data_result {
            Ok(data) => {
                external_handle.submit_command(UPDATE_COLOR_DATA, Arc::new(data), Target::Auto).expect("Failed to send color data update command");
            },
            Err(e) => eprintln!("Error reading file for color data: {}", e),
        }
        match extract_detail_exe(&path) {
            Ok(details) => {
                let details_str = format!("{:?}", details); // Convert HashMap or details structure to String as needed
                external_handle.submit_command(UPDATE_EXE_DETAILS, Arc::new(details_str), Target::Auto).expect("Failed to send executable details update command");
            },
            Err(e) => eprintln!("Error extracting executable details: {}", e),
        }
    } else {
        println!("No file selected, exiting.");
        std::process::exit(0);
    }
});

launcher.launch(initial_state).expect("Failed to launch application");
}