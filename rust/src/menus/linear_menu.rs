use crate::states::contextual_input::{Direction1D, Direction2D};

pub struct LinearMenu<OptionType: Clone> {
    options: Vec<OptionType>,
    selected_index: usize,
}

impl<OptionType: Clone> LinearMenu<OptionType> {
    pub fn new(options: Vec<OptionType>) -> Self {
        assert!(!options.is_empty());
        Self {
            options,
            selected_index: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: Direction2D) {
        let option_offset = match direction.to_1d() {
            Direction1D::Right => 1,
            Direction1D::Left => self.options.len() - 1,
        };
        self.selected_index = (self.selected_index + option_offset) % self.options.len();
    }

    pub fn selected_option(&self) -> OptionType {
        return self.options[self.selected_index].clone();
    }

    pub fn iter<'a>(&'a self) -> LinearMenuIter<'a, OptionType> {
        LinearMenuIter {
            menu: self,
            next_index: 0,
        }
    }
}

pub struct LinearMenuItem<'a, OptionType: Clone> {
    item: &'a OptionType,
    selected: bool,
}

impl<'a, OptionType: Clone> LinearMenuItem<'a, OptionType> {
    pub fn item(&self) -> &OptionType {
        &self.item
    }

    pub fn selected(&self) -> bool {
        self.selected
    }
}

pub struct LinearMenuIter<'a, OptionType: Clone> {
    menu: &'a LinearMenu<OptionType>,
    next_index: usize,
}

impl<'a, OptionType: Clone> Iterator for LinearMenuIter<'a, OptionType> {
    type Item = LinearMenuItem<'a, OptionType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.menu.options.len() {
            return None;
        } else {
            let result = Some(LinearMenuItem {
                item: &self.menu.options[self.next_index],
                selected: self.menu.selected_index == self.next_index,
            });
            self.next_index += 1;
            return result;
        }
    }
}
