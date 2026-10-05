use std::ops::Add;

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
    value1: String,
    value2: String,
    last_value1: String,
    last_value2: String,
    operation: MathOperation,
    operation_str: String,
    last_operation_str: String,
    result: String,
}

#[derive(Debug, Clone)]
enum Message {
    Value(String),
    Operation(MathOperation),
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Value(value) => match state.operation {
            MathOperation::None | MathOperation::Result => {
                state.value1 = state.value1.clone().add(value.as_str());
            }
            _ => {
                state.value2 = state.value2.clone().add(value.as_str());
            }
        },
        Message::Operation(math_op) => match math_op {
            MathOperation::Add => {
                state.operation = MathOperation::Add;
                state.operation_str = "+".to_string();
            }
            MathOperation::Subtract => {
                state.operation = MathOperation::Subtract;
                state.operation_str = "-".to_string();
            }
            MathOperation::Multiply => {
                state.operation = MathOperation::Multiply;
                state.operation_str = "*".to_string();
            }
            MathOperation::Divide => {
                state.operation = MathOperation::Divide;
                state.operation_str = "/".to_string();
            }
            MathOperation::Result => {
                state.result = match state.operation {
                    MathOperation::Add => (state.value1.parse::<f32>().unwrap()
                        + state.value2.parse::<f32>().unwrap())
                    .to_string(),
                    MathOperation::Subtract => (state.value1.parse::<f32>().unwrap()
                        - state.value2.parse::<f32>().unwrap())
                    .to_string(),
                    MathOperation::Multiply => (state.value1.parse::<f32>().unwrap()
                        * state.value2.parse::<f32>().unwrap())
                    .to_string(),
                    MathOperation::Divide => (state.value1.parse::<f32>().unwrap()
                        / state.value2.parse::<f32>().unwrap())
                    .to_string(),
                    _ => "".to_string(),
                };

                state.last_value1 = state.value1.clone();
                state.last_value2 = state.value2.clone();
                state.last_operation_str = state.operation_str.clone();

                state.value1.clear();
                state.value2.clear();
                state.operation_str.clear();
                state.operation = MathOperation::Result;
            },
            MathOperation::None => {}
        },
    }
}

fn view(state: &State) -> iced::widget::Column<'_, Message> {
    let value1 = match state.operation {
        MathOperation::Result => &state.last_value1,
        _ => &state.value1,
    };

    let value2 = match state.operation {
        MathOperation::Result => &state.last_value2,
        _ => &state.value2,
    };

    let operation = match state.operation {
        MathOperation::Result => &state.last_operation_str,
        _ => &state.operation_str,
    };

    iced::widget::column![
        iced::widget::row![
            iced::widget::button("1").on_press(Message::Value("1".to_owned())),
            iced::widget::button("2").on_press(Message::Value("2".to_owned())),
            iced::widget::button("3").on_press(Message::Value("3".to_owned()))
        ],
        iced::widget::row![
            iced::widget::button("4").on_press(Message::Value("4".to_owned())),
            iced::widget::button("5").on_press(Message::Value("5".to_owned())),
            iced::widget::button("6").on_press(Message::Value("6".to_owned()))
        ],
        iced::widget::row![
            iced::widget::button("7").on_press(Message::Value("7".to_owned())),
            iced::widget::button("8").on_press(Message::Value("8".to_owned())),
            iced::widget::button("9").on_press(Message::Value("9".to_owned()))
        ],
        iced::widget::row![
            iced::widget::button(" "),
            iced::widget::button("0").on_press(Message::Value("0".to_owned())),
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
        ],
        iced::widget::row![
            iced::widget::text(value1).size(30),
            iced::widget::text(operation).size(30),
            iced::widget::text(value2).size(30),
        ],
        iced::widget::row![iced::widget::text(state.result.clone()).size(30),]
    ]
    .into()
}

pub fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("Fuzzy Doodle")
        .run()
}
