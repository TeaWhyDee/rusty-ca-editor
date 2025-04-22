// use core::time;
// use iced::widget::container::background;
// use iced::widget::shader::wgpu::Color;
use iced::widget::{button, center, column, mouse_area, row, text, scrollable, Space, Row};
use iced::Length::{Fill, Shrink};
use iced::{Element, Subscription, Theme};

mod automata;
mod consts;
use automata::{Rules3x3, DEFAULT_GRID, DEF_RULES_LIFE, RAND_RULES};
use consts::{INIT_GRID_X, INIT_GRID_Y, INIT_CELL_SIZE, INIT_TICK_SPEED_MS};


struct State {
    cell_size: u16,
    grid: Vec<Vec<u8>>,
    running: bool,
    rules: Rules3x3,
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
        // Message::Event(event) => match event {
        //     iced::event::Event::Mouse(mouse_event) => match mouse_event {
        //         iced::mouse::Event::WheelScrolled { delta } => {
        //             println!("scrolled");
        //             match delta {
        //                 iced::mouse::ScrollDelta::Lines { x, .. } => {
        //                     if x > 0.0 {
        //                         state.cell_size += 1;
        //                     }
        //                     else {
        //                         state.cell_size -= 1;
        //                     }
        //                 },
        //                 iced::mouse::ScrollDelta::Pixels { x, .. } => {
        //                     if x > 0.0 {
        //                         state.cell_size += 1;
        //                     }
        //                     else {
        //                         state.cell_size -= 1;
        //                     }
        //                 }
        //             }
        //         },
        //         _ => {
        //             println!("other mouse event");
        //         }
        //     },
        //     iced::event::Event::Keyboard(keyboard_event) => match keyboard_event {
        //         iced::keyboard::Event::KeyReleased { key, .. } => {
        //             println!("Key {:?} was pressed", key);
        //         }
        //         _ => {}
        //     },
        //     _ => {
        //         println!("other event");
        //     }
        // },
        // Local
        Message::CellClicked(x, y) => {
            print!("Clicked {} {}", x, y);
            state.grid[x as usize][y as usize] = 1;
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
        Message::Step() => {
            run_ca(state);
        }
        Message::Tick() => {
            if state.running {
                run_ca(state);
            }
        }
    }
}

fn view(state: &State) -> Element<Message> {
    let b_size = state.cell_size;

    let mut app_container = column!();

    //
    // Create the cell matrix
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
                            button::Style::default().with_background(iced::Color::from_rgb(0.5, 0.5, 0.5))
                        }
                        _ => if state.grid[y][x] == 0 {
                                button::Style::default().with_background(iced::Color::BLACK)
                        }
                        else {
                            button::Style::default().with_background(iced::Color::WHITE)
                        }
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

    //
    // Add buttons
    //
    let ca_container = row![
        h_box,
        button( if state.running {"pause" } else {"run"} )
            .on_press(Message::PlayPause()),

        button( "step" )
            .on_press(Message::Step())
    ];

    app_container = app_container.push(ca_container);
    app_container = app_container.push(Space::with_height(10));
    
    //
    // Rules display
    //
    for rules_type in &state.rules.rules_by_type {
        let mut rules_container = row!();

        for rule in rules_type {
            // Each cell state has own container of transitions
            let rule = rule.rule;

            let mut grid_3x3 = row!();
            for x in 0..3 {
                let mut v_box = column!();
                for y in 0..3 {
                    // Cell
                    // a button inside a mouse area
                    let b = button(text(" "))
                        .width(INIT_CELL_SIZE)
                        .height(INIT_CELL_SIZE)
                        .style( move |_, status| {
                            // Style the button
                            match status {
                                // button::Status::Hovered => {
                                //     button::Style::default().with_background(iced::Color::from_rgb(0.5, 0.5, 0.5))
                                // }
                                _ => if rule[y][x] == 0 {
                                    button::Style::default().with_background(iced::Color::BLACK)
                                }
                                else {
                                    button::Style::default().with_background(iced::Color::WHITE)
                                }
                            }
                        });
                    // On cell left click
                    // .on_press(
                    //     Message::CellClicked(y.try_into().unwrap(), x.try_into().unwrap())
                    // );

                    v_box = v_box.push(b);
                }
                grid_3x3 = grid_3x3.push(v_box);
            }

            // Push the 3x3 grid into the corresponding container
            rules_container = rules_container.push(grid_3x3).push(Space::with_width(4));
        }

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
                .height(INIT_CELL_SIZE * 5),
        );
    }

    // rules_containers[idx] = rules_containers[idx]

    app_container.into()
    // app_container.into()
}

#[derive(Debug, Clone)]
enum Message {
    // Global
    // Event(iced::event::Event),
    // Local
    CellClicked(u64, u64),
    CellRightClicked(u64, u64),
    CellScrolled(i8),
    PlayPause(),
    Step(),
    Tick(),
}

fn theme(_state: &State) -> Theme {
    Theme::TokyoNight
}

fn sub_time(_state: &State) -> Subscription<Message> {
    // 100000000 = 100ms
    iced::time::every(std::time::Duration::new(0, INIT_TICK_SPEED_MS * 1000 * 1000)).map(|_id| { 
        Message::Tick()
    })
}
