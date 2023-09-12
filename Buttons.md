To create buttons in ratatui requires us to know the area rect the button takes up and and receive the mouse buttons input and position.

My first simple implementation of buttons in Chat-Tui looks like this...
```
pub struct Button<'a> {
    pub rect: Rect,
    pub app: App<'a>,
}

impl Button <'_> {
    pub fn new(rect:Rect, app:App<'_>) -> Button<'_> {
        Button {
            rect,
            app,
        }
    }


    pub fn mouse_in_rect(&self, x:u16, y:u16) -> bool {
        if x >= self.rect.x && x <= self.rect.x + self.rect.width {
            if y >= self.rect.y && y <= self.rect.y + self.rect.height {
                return true;
            }
        }
        false
    }

    pub fn clicked (&mut self) -> bool {
        let mut out = false;
        if self.app.input.mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if self.mouse_in_rect(self.app.input.mouse.column, self.app.input.mouse.row) {
                out = true;
            }
        }
        out
    }

}
```

The problem here is button needs to be fed this App struct to get to the mouse input. What we want to do instead is handle the input modularly rather than collecting the input in a single space and passing that around the program.  

