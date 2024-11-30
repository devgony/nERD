use ratatui::layout::Rect;

const GAP: u16 = 4;
const WIDTH: u16 = 20;
pub struct RectBuilder {
    x: u16,
    y: u16,
    screen_width: u16,
}

impl RectBuilder {
    pub fn new(screen_width: u16) -> Self {
        Self {
            x: GAP * 2,
            y: GAP,
            screen_width,
        }
    }

    pub fn get_rect(&mut self, len: usize) -> Rect {
        let rect = Rect::new(self.x, self.y, WIDTH, len as u16);

        self.x += WIDTH + GAP * 2;
        if (self.x + WIDTH) > self.screen_width {
            self.x = GAP * 2;
            self.y += len as u16 + GAP;
        }

        rect
    }
}
