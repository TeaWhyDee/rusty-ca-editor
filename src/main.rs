// use core::time;
// use iced::widget::container::background;
// use iced::widget::shader::wgpu::Color;
use iced::widget::{button, center, column, mouse_area, row, scrollable, slider, text, Row, Space};
use iced::Length::{Fill, FillPortion, Shrink};
use iced::{Border, Element, Shadow, Subscription, Theme};

mod automata;
mod consts;
use automata::{Rules3x3, DEFAULT_GRID, DEF_RULES_LIFE, RAND_RULES};
use consts::{INIT_GRID_X, INIT_GRID_Y, INIT_CELL_SIZE, INIT_TICK_SPEED_MS, REAL_TICK_SPEED_MS};

const MAX_SPEED: u32 = 10;

struct CfgRuleEditor {
    symmetry: bool,
}

impl Default for CfgRuleEditor {
    fn default() -> CfgRuleEditor {
        CfgRuleEditor {
            symmetry: false,
        }
    }
}

struct State {
    cell_size: u16,
    grid: Vec<Vec<u8>>,
    running: bool,
    rules: Rules3x3,
    rule_editor: CfgRuleEditor,
    sim_speed: u32,
    sim_delta: u32,
}

impl Default for State {
    fn default() -> State {
        State {
            // grid: vec![vec![0;INIT_GRID_SIZE];INIT_GRID_SIZE],
            grid: DEFAULT_GRID(),
            running: false,
            // rules: DEF_RULES_LIFE(),
            rules: RAND_RULES(),
            cell_size: INIT_CELL_SIZE,
            rule_editor: Default::default(),
            sim_speed: 9,
            sim_delta: 0,
        }
    }
}

fn run_ca(state: &mut State) {
    automata::ca(&mut state.grid, &state.rules);
}

pub fn main() -> iced::Result {
    // let mut grid = [[0; INIT_GRID_SIZE]; INIT_GRID_SIZE];
    
    let r1 = 10 % 3;
    let r2 = 10 / 3;
    println!("{r1}");
    println!("{r2}");


    iced::application("A cool application", update, view)
        // .subscription(|_state| iced::event::listen().map(Message::Event))
        .subscription(sub_time)
        .theme(theme)
        .run()
}

fn update(state: &mut State, message: Message) {
    match message {
        // Global
        // ...
        Message::Step() => {
            run_ca(state);
        }
        Message::Tick() => {
            if state.running {
                state.sim_delta += 1;
                if state.sim_delta >= MAX_SPEED + 1 - state.sim_speed {
                    run_ca(state);
                    state.sim_delta = 0
                }
            }
            else {
                state.sim_delta = MAX_SPEED - state.sim_speed
            }
        }

        // Local
        // CA
        Message::CellClicked(x, y) => {
            print!("Clicked {} {}", x, y);
            let cell_state = state.grid[x as usize][y as usize];
            state.grid[x as usize][y as usize] = (cell_state + 1) % (state.rules.num_states);
        }
        Message::CellRightClicked(x, y) => {
            state.grid[x as usize][y as usize] = 0;
        }
        Message::CellScrolled(delta) => {
            println!("{delta}");
            state.cell_size = state.cell_size.saturating_add_signed(delta.into());
        }
        Message::PlayPause() => {
            state.running = !state.running;
        }
        Message::SpeedSliderChanged(val) => {
            state.sim_speed = val;
        }

        // Rule Editor
        Message::RuleEditorSymmetryToggle(is_checked) => {
            // state.rule_editor.symmetry = !state.rule_editor.symmetry;
            state.rule_editor.symmetry = is_checked;
        }
    }
}

fn view(state: &State) -> Element<Message> {
    let b_size = state.cell_size;

    let mut app_container = column!();

    //
    // CA
    // Cell matrix
    //
    let mut h_box = row!();
    for x in 0..INIT_GRID_X {
        let mut v_box = column!();
        for y in 0..INIT_GRID_Y {
            // Cell
            // a button inside a mouse area
            let b = button(text(" "))
                .width(b_size)
                .height(b_size)
                .style( move |_, status| {
                    // Style the button
                    match status {
                        button::Status::Hovered => {
                            button::Style{
                                background: None,
                                text_color: iced::Color::BLACK,
                                border: Border::default().width(2.0),
                                shadow: Shadow::default()
                            }
                            .with_background(iced::Color::from_rgb(0.5, 0.5, 0.5))
                        }
                        _ => button::Style::default()
                                .with_background(state.rules.colors[state.grid[y][x] as usize])
                    }
                })
                // On cell left click
                .on_press(
                    Message::CellClicked(y.try_into().unwrap(), x.try_into().unwrap())
                );

            let m_area = mouse_area(
                    center(b)
                        .width(b_size)
                        .height(b_size)
                )
                .on_scroll(|delta| Message::CellScrolled(
                    match delta {
                        iced::mouse::ScrollDelta::Lines { y, .. } => {
                            if y > 0.0 { 1 } else { -1 }
                        },
                        iced::mouse::ScrollDelta::Pixels { y, .. } => {
                            if y > 0.0 { 1 } else { -1 }
                        }
                    }
                ))

                // On cell right click
                .on_right_press(Message::CellRightClicked(y.try_into().unwrap(), x.try_into().unwrap()));

            v_box = v_box.push(m_area);
        }
        h_box = h_box.push(v_box);
    }

    // CA Buttons & Sliders
    let ca_buttons_container = column![
        button(if state.running {"pause"} else {"run"})
            .on_press(Message::PlayPause())
            .width(Fill),

        button("step")
            .on_press(Message::Step())
            .width(Fill),

        iced::widget::text("Speed"),
        slider(1..=10, state.sim_speed, Message::SpeedSliderChanged),
    ].width(FillPortion(2));

    let ca_container = row![
        h_box,
        ca_buttons_container,
    ];

    //
    // Rule Editor
    //
    let rule_editor_container = row![
        iced::widget::checkbox("Symmetry", state.rule_editor.symmetry)
            .on_toggle(Message::RuleEditorSymmetryToggle)
    ].width(FillPortion(4));

    //
    // App
    //
    let main_container = row![
        ca_container,
        iced::widget::vertical_rule(10),
        rule_editor_container,
    ];

    app_container = app_container.push(main_container); // column
    app_container = app_container.push(Space::with_height(10));
    
    //
    // Rules display
    //
    for rules_type in &state.rules.rules_by_type {
        let mut rules_container = row!();

        for rule in rules_type {
            // List of rules by 1,1 cell type
            let rule = rule.rule;

            let mut grid_3x3 = row!();
            for x in 0..3 {
                let mut v_box = column!();
                for y in 0..3 {
                    // Cell
                    let b = button(text(" "))
                        .width(INIT_CELL_SIZE)
                        .height(INIT_CELL_SIZE)
                        .style( move |_, status| {
                            // Style the button
                            match status {
                                _ => button::Style::default()
                                        .with_background(state.rules.colors[rule[y][x] as usize])
                            }
                        });

                    v_box = v_box.push(b);
                }
                grid_3x3 = grid_3x3.push(v_box);
            }
            // Push the 3x3 grid into the corresponding container
            rules_container = rules_container.push(grid_3x3).push(Space::with_width(4));
        }

        // add rule lists
        app_container = app_container.push(
            scrollable(rules_container)
                .direction(scrollable::Direction::Horizontal(
                    scrollable::Scrollbar::new()
                        .width(2)
                        .margin(2)
                        .scroller_width(6)
                        .anchor(scrollable::Anchor::Start),
                ))
                .width(Fill)
                .height(INIT_CELL_SIZE * 3 + 10),
        );
    }

    app_container.into()
}

#[derive(Debug, Clone)]
enum Message {
    // Global
    // Event(iced::event::Event),
    // Time
    Step(),
    Tick(),

    // Local
    // CA
    CellClicked(u64, u64),
    CellRightClicked(u64, u64),
    CellScrolled(i8),
    PlayPause(),
    SpeedSliderChanged(u32),

    // Rule Editor
    RuleEditorSymmetryToggle(bool),
}

fn theme(_state: &State) -> Theme {
    Theme::TokyoNight
}

fn sub_time(_state: &State) -> Subscription<Message> {
    // 100000000 = 100ms
    iced::time::every(std::time::Duration::new(0, REAL_TICK_SPEED_MS * 1000 * 1000)).map(|_id| { 
        Message::Tick()
    })
}
