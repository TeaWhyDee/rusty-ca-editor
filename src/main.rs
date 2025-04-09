// use core::time;
use std::time::Duration;

use iced::widget::container::background;
use iced::widget::shader::wgpu::Color;
use iced::widget::{button, center, text, column, row, mouse_area};
use iced::{time, Element, Subscription, Theme};

mod automata;
mod consts;
use automata::{Rules3x3, DEFAULT_GRID, DEF_RULES_LIFE};
use consts::{INIT_GRID_X, INIT_GRID_Y, INIT_CELL_SIZE};


struct State {
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
            rules: DEF_RULES_LIFE()
        }
    }
}

fn run_ca(state: &mut State) {
    let rules = automata::DEF_RULES_LIFE();

    automata::ca(&mut state.grid, &rules);
}

pub fn main() -> iced::Result {
    // let mut grid = [[0; INIT_GRID_SIZE]; INIT_GRID_SIZE];

    // grid[6][6] = 1;
    // println!("{:#?}", grid);

    // iced::run("A cool counter", update, view)
    iced::application("A cool application", update, view)
        .subscription(sub_time)
        .theme(theme)
        .run()
}

fn update(state: &mut State, message: Message) {
    match message {
        // Message::Increment => ,
        Message::CellClicked(x, y) => {
            print!("Clicked {} {}", x, y);
            state.grid[x as usize][y as usize] = 1;
        }
        Message::CellRightClicked(x, y) => {
            state.grid[x as usize][y as usize] = 0;
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
    let b_size = INIT_CELL_SIZE;

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
                // On cell right click
                .on_right_press(Message::CellRightClicked(y.try_into().unwrap(), x.try_into().unwrap()));

            v_box = v_box.push(m_area);
        }
        h_box = h_box.push(v_box);
    }

    //
    // Add other UI
    //
    let ca_container = row![
        h_box,
        button( if state.running {"pause" } else {"run"} )
            .on_press(Message::PlayPause()),

        button( "step" )
            .on_press(Message::Step())
    ];

    ca_container.into()
}

#[derive(Debug, Clone)]
enum Message {
    CellClicked(u64, u64),
    CellRightClicked(u64, u64),
    PlayPause(),
    Step(),
    Tick(),
}

fn theme(state: &State) -> Theme {
    Theme::TokyoNight
}

fn sub_time(state: &State) -> Subscription<Message> {
    iced::time::every(std::time::Duration::new(0, 10000000)).map(|_id| { 
        Message::Tick()
    })
}
