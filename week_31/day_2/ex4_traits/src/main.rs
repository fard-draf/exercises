trait Drawable {
    fn draw(&self) -> String;
}

struct Button {
    label: String,
}

impl Drawable for Button {
    fn draw(&self) -> String {
        format!("[BUTTON]: {}", self.label)
    }
}

struct TextField {
    text: String,
    is_focused: bool,
}

impl Drawable for TextField {
    fn draw(&self) -> String {
        let a = if self.is_focused {
            "activated".to_string()
        } else {
            "disable".to_string()
        };
        format!("[TEXT FIELD]: {} - [FOCUS]: {} ", self.text, a)
    }
}

fn render_ui(element: &[&dyn Drawable]) {
    element.iter().for_each(|e| {
        println!("{:?}", e.draw());
    });
}

fn main() {
    let but1 = Button {
        label: "Connect".to_string(),
    };

    let but2 = Button {
        label: "Log".to_string(),
    };

    let but3 = Button {
        label: "Delete".to_string(),
    };

    let but4 = Button {
        label: "Send".to_string(),
    };

    let text1 = TextField {
        text: "Welcome".to_string(),
        is_focused: true,
    };
    let text2 = TextField {
        text: "Last log: this morning".to_string(),
        is_focused: true,
    };
    let text3 = TextField {
        text: "Target deleted".to_string(),
        is_focused: false,
    };
    let text4 = TextField {
        text: "Transaction sended".to_string(),
        is_focused: false,
    };

    let actions: Vec<&dyn Drawable> =
        vec![&but1, &but2, &but3, &but4, &text1, &text2, &text3, &text4];

    render_ui(&actions);
}
