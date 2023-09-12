use crossterm::event::{Event, MouseEventKind, MouseButton};
use ratatui::{prelude::*, widgets::{StatefulWidget, Block, Borders, Widget, BorderType, Paragraph, Wrap}};
pub struct Button<'a>{
    text:Text<'a>,
    normal_block:Block<'a>,
    hovered_block:Block<'a>,
    pressed_block:Block<'a>,
    alignment:Alignment,
    wrap:Option<Wrap>,
    margin:Margin,
    scroll:(u16, u16),
    left_click:Option<Box<dyn FnMut()>>,
    right_click:Option<Box<dyn FnMut()>>,
    middle_click:Option<Box<dyn FnMut()>>,
}

impl Default for Button<'_>{
    fn default() -> Self{
        Self{
            text:Text::default(),
            normal_block:Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .fg(Color::White),
            hovered_block:Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .fg(Color::Green),
            pressed_block:Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .fg(Color::Red),
            alignment:Alignment::Left,
            wrap:Some(Wrap{trim:false}),
            margin:Margin{horizontal:1, vertical:1},
            scroll:(0,0),
            left_click:None,
            right_click:None,
            middle_click:None,
        }
    }
}

impl <'a> Button<'a>{
    pub fn text<T>(mut self, text:T) -> Button<'a> where T:Into<Text<'a>>{
        self.text = text.into();
        self
    }

    pub fn normal_block(mut self, block:Block<'a>) -> Button<'a>{
        self.normal_block = block;
        self
    }

    pub fn hovered_block(mut self, block:Block<'a>) -> Button<'a>{
        self.hovered_block = block;
        self
    }

    pub fn pressed_block(mut self, block:Block<'a>) -> Button<'a>{
        self.pressed_block = block;
        self
    }

    pub fn alignment(mut self, alignment:Alignment) -> Button<'a>{
        self.alignment = alignment;
        self
    }

    pub fn wrap(mut self, wrap:Wrap) -> Button<'a>{
        self.wrap = Some(wrap);
        self
    }

    pub fn margin(mut self, margin:Margin) -> Button<'a>{
        self.margin = margin;
        self
    }

    pub fn scroll(mut self, scroll:(u16, u16)) -> Button<'a>{
        self.scroll = scroll;
        self
    }

    pub fn left_click(mut self, f:Box<dyn FnMut()>) -> Button<'a>{
        self.left_click = Some(f);
        self
    }

    pub fn right_click(mut self, f:Box<dyn FnMut()>) -> Button<'a>{
        self.right_click = Some(f);
        self
    }

    pub fn middle_click(mut self, f:Box<dyn FnMut()>) -> Button<'a>{
        self.middle_click = Some(f);
        self
    }

}


impl StatefulWidget for Button<'_>{
    type State = Event ;
    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        let text = self.text;
        let block = self.normal_block.clone();
        let mut button = self.normal_block.clone();

        match state{
            Event::Mouse(mouse) => {
                match mouse.kind{
                    MouseEventKind::Moved => {
                        if mouse_in_rect(mouse.column, mouse.row, &area) {
                            //hover style
                            button = self.hovered_block;
                        }
                    },
                    MouseEventKind::Down(MouseButton::Left) => {
                        if mouse_in_rect(mouse.column, mouse.row, &area) {
                            //pressed style
                            button = self.pressed_block;
                            match self.left_click{
                                Some(ref mut f) => f(),
                                None => {}
                            };
                        }
                    },
                    MouseEventKind::Down(MouseButton::Right) => {
                        if mouse_in_rect(mouse.column, mouse.row, &area) {
                            //pressed style
                            button = self.pressed_block;
                            match self.right_click{
                                Some(ref mut f) => f(),
                                None => {}
                            };
                        }
                    },
                    MouseEventKind::Down(MouseButton::Middle) => {
                        if mouse_in_rect(mouse.column, mouse.row, &area) {
                            //pressed style
                            button = self.pressed_block;
                            match self.middle_click{
                                Some(ref mut f) => f(),
                                None => {}
                            };
                        }
                    },
                    _ =>{}
                }
            }
            _=>{}
        }
        
        let para = Paragraph::new(text)
            .alignment(self.alignment);
        let para = match self.wrap{
            Some(wrap) => para.wrap(wrap),
            None => para,
        };
        para.render(area.inner(&self.margin), buf);
        button.render(area, buf);
    }
}



pub fn mouse_in_rect(x:u16, y:u16, rect:&Rect) -> bool{
    if x >= rect.x && x <= rect.x + rect.width{
        if y >= rect.y && y <= rect.y + rect.height{
            return true;
        }
    }
    return false;
}
