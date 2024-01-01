use std::fmt::{self, Display, Formatter};

use iced::{
    executor, mouse,
    widget::{
        button,
        canvas::{Frame, Geometry, Path, Program, Stroke},
        column, container, horizontal_space, row, text, vertical_space, Canvas,
    },
    window, Application, Command, Rectangle, Renderer, Settings, Theme,
};

fn main() -> iced::Result {
    TicTacToe::run(Settings {
        antialiasing: true,
        default_text_size: 24.0,
        window: window::Settings {
            position: window::Position::Centered,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SquareValue {
    X,
    O,
}

impl SquareValue {
    fn next(&self) -> Self {
        match self {
            SquareValue::X => SquareValue::O,
            SquareValue::O => SquareValue::X,
        }
    }
}

impl Display for SquareValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SquareValue::X => write!(f, "X"),
            SquareValue::O => write!(f, "O"),
        }
    }
}

type SquareArray = [Option<SquareValue>; 9];

#[derive(Debug, Clone, Copy)]
enum Message {
    SquareClicked(usize),
    PreviousTurn,
    NextTurn,
    StartNewGame,
}

struct TicTacToe {
    next_square_value: SquareValue,
    winner: Option<SquareValue>,
    turns: Vec<SquareArray>,
    turn_index: usize,
}

impl Application for TicTacToe {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            TicTacToe {
                next_square_value: SquareValue::X,
                winner: None,
                turns: vec![[None::<SquareValue>; 9]],
                turn_index: 0,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Tic Tac Toe - Iced")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SquareClicked(square_index) => {
                let mut next_squares = self.turns[self.turn_index];
                let square = next_squares[square_index];
                if self.winner.is_some()
                    || square.is_some()
                    || calculate_winner(&next_squares).is_some()
                {
                    return iced::Command::none();
                }

                next_squares[square_index] = Some(self.next_square_value);

                if self.turn_index + 1 < self.turns.len() {
                    self.turns[self.turn_index + 1] = next_squares;
                    self.turns.truncate(self.turn_index + 2);
                } else {
                    self.turns.push(next_squares);
                }

                self.turn_index += 1;
                self.next_square_value = self.next_square_value.next();
                self.winner = calculate_winner(&next_squares);
            }
            Message::PreviousTurn => {
                if self.turn_index == 0 {
                    return iced::Command::none();
                }

                self.turn_index -= 1;
                self.next_square_value = self.next_square_value.next();
            }
            Message::NextTurn => {
                if self.turn_index >= self.turns.len() - 1 {
                    return iced::Command::none();
                }

                self.turn_index += 1;
                self.next_square_value = self.next_square_value.next();
            }
            Message::StartNewGame => {
                self.turn_index = 0;
                self.winner = None;
                self.next_square_value = SquareValue::X;
                self.turns = vec![[None::<SquareValue>; 9]];
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let actions = row![
            button("←").on_press(Message::PreviousTurn),
            horizontal_space(2),
            button("→").on_press(Message::NextTurn),
            horizontal_space(2),
            button("Start new game").on_press(Message::StartNewGame),
        ];

        let mut board_buttons =
            self.turns[self.turn_index]
                .iter()
                .enumerate()
                .map(|(square_index, &square)| {
                    button(Canvas::new(Square { value: square }))
                        .width(100)
                        .height(100)
                        .on_press(Message::SquareClicked(square_index))
                });

        let status = text(if let Some(winner) = self.winner {
            format!("Player {} won!", winner)
        } else {
            format!("It's {}'s turn", self.next_square_value)
        });

        let board = container(row![column![
            row![
                board_buttons.next().unwrap(),
                horizontal_space(5),
                board_buttons.next().unwrap(),
                horizontal_space(5),
                board_buttons.next().unwrap(),
            ],
            vertical_space(5),
            row![
                board_buttons.next().unwrap(),
                horizontal_space(5),
                board_buttons.next().unwrap(),
                horizontal_space(5),
                board_buttons.next().unwrap(),
            ],
            vertical_space(5),
            row![
                board_buttons.next().unwrap(),
                horizontal_space(5),
                board_buttons.next().unwrap(),
                horizontal_space(5),
                board_buttons.next().unwrap(),
            ],
        ]]);

        let content = column![
            "Tic Tac Toe!",
            vertical_space(10),
            actions,
            vertical_space(10),
            status,
            vertical_space(10),
            board,
        ];
        container(content).padding(20).into()
    }
}

struct Square {
    value: Option<SquareValue>,
}

impl Program<Message> for Square {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        // We prepare a new `Frame`
        let mut frame = Frame::new(renderer, bounds.size());

        let stroke_width = 3.0;
        let padding = 3.0;

        match self.value {
            Some(SquareValue::X) => {
                frame.stroke(
                    &Path::line(
                        frame.center()
                            - iced::Vector::new(
                                bounds.width / 2.0 - padding,
                                bounds.height / 2.0 - padding,
                            ),
                        frame.center()
                            + iced::Vector::new(
                                bounds.width / 2.0 - padding,
                                bounds.height / 2.0 - padding,
                            ),
                    ),
                    Stroke::default().with_width(stroke_width),
                );
                frame.stroke(
                    &Path::line(
                        frame.center()
                            + iced::Vector::new(
                                -bounds.width / 2.0 + padding,
                                bounds.height / 2.0 - padding,
                            ),
                        frame.center()
                            + iced::Vector::new(
                                bounds.width / 2.0 - padding,
                                -bounds.height / 2.0 + padding,
                            ),
                    ),
                    Stroke::default().with_width(stroke_width),
                );
            }
            Some(SquareValue::O) => {
                frame.stroke(
                    &Path::circle(frame.center(), bounds.size().width / 2.0 - padding),
                    Stroke::default().with_width(stroke_width),
                );
            }
            None => {}
        }

        vec![frame.into_geometry()]
    }
}

fn calculate_winner(squares: &SquareArray) -> Option<SquareValue> {
    let lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for line in &lines {
        let [a, b, c] = line;
        if squares[*a].is_some() && squares[*a] == squares[*b] && squares[*a] == squares[*c] {
            return squares[*a];
        }
    }

    None
}
