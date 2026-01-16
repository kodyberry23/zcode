use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, List, ListItem, ListState, StatefulWidget},
};

/// Simple scrollable list with scrollbar.
pub struct ScrollableList<'a> {
    pub items: Vec<ListItem<'a>>,
    pub block: Option<Block<'a>>,
}

impl<'a> ScrollableList<'a> {
    pub fn new(items: Vec<ListItem<'a>>) -> Self {
        Self { items, block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for ScrollableList<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut list = List::new(self.items);
        if let Some(block) = self.block {
            list = list.block(block);
        }
        list.render(area, buf, state);
    }
}
