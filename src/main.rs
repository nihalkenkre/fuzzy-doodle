#[derive(Default, Debug, Clone)]
enum MathOperation {
    #[default]
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
    Result,
}

#[derive(Default, Debug)]
struct State {
    value1: f32,
    value2: f32,
    operation: MathOperation,
    result: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Value(f32),
    Operation(MathOperation),
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Value(value) => match state.operation {
            MathOperation::None => state.value1 = value,
            _ => state.value2 = value,
        },
        Message::Operation(math_op) => match math_op {
            MathOperation::Add => state.operation = MathOperation::Add,
            MathOperation::Subtract => state.operation = MathOperation::Subtract,
            MathOperation::Multiply => state.operation = MathOperation::Multiply,
            MathOperation::Divide => state.operation = MathOperation::Divide,
            MathOperation::Result => {
                state.result = match state.operation {
                    MathOperation::Add => state.value1 + state.value2,
                    MathOperation::Subtract => state.value1 - state.value2,
                    MathOperation::Multiply => state.value1 * state.value2,
                    MathOperation::Divide => state.value1 / state.value2,
                    _ => 0f32,
                };

                state.value1 = 0f32;
                state.value2 = 0f32;
                state.operation = MathOperation::None;
            }
            MathOperation::None => state.operation = MathOperation::None,
        },
    }
}

fn view(state: &State) -> iced::widget::Column<'_, Message> {
    iced::widget::column![
        iced::widget::row![
            iced::widget::button("1").on_press(Message::Value(1.0)),
            iced::widget::button("2").on_press(Message::Value(2.0)),
            iced::widget::button("3").on_press(Message::Value(3.0))
        ],
        iced::widget::row![
            iced::widget::button("4").on_press(Message::Value(4.0)),
            iced::widget::button("5").on_press(Message::Value(5.0)),
            iced::widget::button("6").on_press(Message::Value(6.0))
        ],
        iced::widget::row![
            iced::widget::button("7").on_press(Message::Value(7.0)),
            iced::widget::button("8").on_press(Message::Value(8.0)),
            iced::widget::button("9").on_press(Message::Value(9.0))
        ],
        iced::widget::row![
            iced::widget::button(" "),
            iced::widget::button("0").on_press(Message::Value(0.0)),
            iced::widget::button(" ")
        ],
        iced::widget::row![
            iced::widget::button("+").on_press(Message::Operation(MathOperation::Add)),
            iced::widget::button("-").on_press(Message::Operation(MathOperation::Subtract)),
            iced::widget::button("x").on_press(Message::Operation(MathOperation::Multiply)),
            iced::widget::button("/").on_press(Message::Operation(MathOperation::Divide)),
        ],
        iced::widget::row![
            iced::widget::button("=").on_press(Message::Operation(MathOperation::Result)),
            iced::widget::text(state.result),
        ]
    ]
    .into()
}

pub fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("Fuzzy Doodle")
        .run()
}
