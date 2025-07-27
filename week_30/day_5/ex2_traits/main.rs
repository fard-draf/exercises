trait Widget {
    fn draw(&mut self, coordonates: impl Coordonates);
    fn refresh(&mut self);
}

trait Coordonates {
    fn define(&mut self);
    fn print(&self);
}

//==================================================================================
#[derive(Debug)]
struct Button {
    size: u8,
    shape: ButtonShape,
}

#[derive(Debug)]
struct ButtonShape {
    rayon: f32,
    is_cercle: bool,
}

impl Widget for Button {
    fn draw(&mut self, coordonates: (f32, f32)) {
        println!("DATA: {:#?}", self.shape);
        println!("BUTTON COORDONATES: {:?}", coordonates);
    }

    fn refresh(&mut self) {
        println!("REFRESH")
    }
}
//==================================================================================
struct DataPosition {
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

impl Coordonates for DataPosition {
    fn define(&mut self) {
        self.x = None;
        self.y = None;
        self.z = None
    }

    fn print(&self) {
        if let Some(x) = self.x {
            println!("x: {}", x)
        }
        if let Some(y) = self.y {
            println!("y: {}", y)
        }
        if let Some(z) = self.z {
            println!("z: {}", z)
        }
    }
}
//==================================================================================
#[derive(Debug)]
struct TextField {
    content: String,
    shape: TextFieldShape,
}

#[derive(Debug)]
struct TextFieldShape {
    size: u8,
    font: String,
}

impl Widget for TextField {
    fn draw(&mut self, coordonates: (f32, f32)) {
        println!("COORDONATES: {:?}", coordonates);
        println!("TEXT")
    }

    fn refresh(&mut self) {
        println!("REFRESH TEXT")
    }
}
//==================================================================================
struct Screen<'a, W: Widget> {
    widget: &'a mut W,
}

impl<'a, W: Widget> Screen<'a, W> {
    fn render(&self, coordonates: impl Coordonates) {
        self.widget.draw(coordonates);
    }
}

fn main() {}
