use iced::{
    advanced::{widget::operation::Focusable, Text},
    alignment::Vertical,
    widget::text::{Alignment, LineHeight, Shaping, Wrapping},
    Pixels, Rectangle, Size,
};

pub(crate) struct DimensionsState {
    pub(crate) char_height: f32,
    pub(crate) char_width: f32,
    pub(crate) byte_width: f32,
    pub(crate) group_spacing: f32,
    pub(crate) section_separator_spacing: f32,
    pub(crate) section_data_start: f32,
    pub(crate) section_ascii_start: f32,
    pub(crate) address_separator_x: f32,
    pub(crate) ascii_separator_x: f32,
    pub(crate) row_count: usize,
    pub(crate) address_char_len: usize,
    pub(crate) group_char_len: usize,
}

pub(crate) struct BoundsState {
    pub(crate) options: Rectangle,
    pub(crate) addr_input: Rectangle,
    pub(crate) show_ascii_checkbox: Rectangle,
    pub(crate) prev_format: Rectangle,
    pub(crate) next_format: Rectangle,
    pub(crate) text_format: Rectangle,
    pub(crate) prev_row_length: Rectangle,
    pub(crate) next_row_length: Rectangle,
    pub(crate) text_row_length: Rectangle,
}

pub(crate) struct InputState {
    pub(crate) value: String,
    pub(crate) focused: bool,
}

pub(crate) struct TextState {
    pub(crate) jumpto_text: String,
    pub(crate) jumpto_len: f32,
}

pub(crate) struct State {
    pub(crate) focused: bool,
    pub(crate) text_defaults: Text,
    pub(crate) options_open: bool,
    pub(crate) start_address: usize,
    pub(crate) selected_address: Option<usize>,
    pub(crate) dimensions: DimensionsState,
    pub(crate) addr_input: InputState,
    pub(crate) bounds: BoundsState,
    pub(crate) text: TextState,
}

impl Default for State {
    fn default() -> Self {
        State {
            dimensions: DimensionsState {
                char_height: 0.0,
                char_width: 0.0,
                byte_width: 0.0,
                group_spacing: 0.0,
                section_separator_spacing: 0.0,
                section_data_start: 0.0,
                section_ascii_start: 0.0,
                address_separator_x: 0.0,
                ascii_separator_x: 0.0,
                row_count: 0,
                address_char_len: 6,
                group_char_len: 8,
            },
            start_address: 0,
            selected_address: None,
            addr_input: InputState {
                value: String::new(),
                focused: false,
            },
            focused: false,
            text_defaults: Text {
                content: String::new(),
                bounds: Size::ZERO,
                size: Pixels::from(16.0),
                line_height: LineHeight::default(),
                font: iced::Font::MONOSPACE,
                align_x: Alignment::Left,
                align_y: Vertical::Top,
                shaping: Shaping::Advanced,
                wrapping: Wrapping::None,
            },
            options_open: false,
            bounds: BoundsState {
                options: Rectangle::default(),
                addr_input: Rectangle::default(),
                show_ascii_checkbox: Rectangle::default(),
                prev_format: Rectangle::default(),
                next_format: Rectangle::default(),
                text_format: Rectangle::default(),
                prev_row_length: Rectangle::default(),
                next_row_length: Rectangle::default(),
                text_row_length: Rectangle::default(),
            },
            text: TextState {
                jumpto_text: "Jump to".to_string(),
                jumpto_len: 0.0,
            },
        }
    }
}

impl State {
    pub(crate) fn update_dimensions(&mut self, row_length: f32) {
        self.dimensions.byte_width = self.dimensions.char_width * 2.5;
        self.dimensions.group_spacing = self.dimensions.char_width;
        self.dimensions.section_separator_spacing = self.dimensions.char_width * 2.0;

        self.dimensions.section_data_start = self.dimensions.char_width
            * self.dimensions.address_char_len as f32
            + self.dimensions.section_separator_spacing;
        self.dimensions.section_ascii_start = self.dimensions.section_data_start
            + self.dimensions.byte_width * row_length
            + (row_length / self.dimensions.group_char_len as f32 - 1.0)
                * self.dimensions.group_spacing
            + self.dimensions.section_separator_spacing;

        self.dimensions.address_separator_x =
            self.dimensions.section_data_start - self.dimensions.section_separator_spacing / 2.0;
        self.dimensions.ascii_separator_x =
            self.dimensions.section_ascii_start - self.dimensions.section_separator_spacing / 2.0;
        self.text.jumpto_len = self.text.jumpto_text.len() as f32 * self.dimensions.char_width;
    }

    pub(crate) fn update_bounds(&mut self, limits: &iced::advanced::layout::Limits) {
        let options_text = "Options";
        let options_width = options_text.len() as f32 * self.dimensions.char_width;
        self.bounds.options = Rectangle {
            x: limits.min().width,
            y: limits.max().height - self.dimensions.char_height * 1.5,
            width: options_width,
            height: self.dimensions.char_height * 1.5,
        };

        let total_width = limits.max().width;
        let jumpto_x = (total_width - self.text.jumpto_len - options_width) / 2.0;
        let input_x = jumpto_x + self.text.jumpto_len + self.dimensions.char_width;

        self.bounds.addr_input = Rectangle {
            x: input_x,
            y: limits.max().height - self.dimensions.char_height * 1.3,
            width: (self.dimensions.char_width + 1.0)
                * self.dimensions.address_char_len as f32
                * 1.1,
            height: self.dimensions.char_height * 1.1,
        };

        let panel_bounds = Rectangle {
            x: self.dimensions.char_width * 0.5,
            y: limits.max().height
                - self.dimensions.char_height * 1.5
                - self.dimensions.char_height * 4.0,
            width: limits.max().width - self.dimensions.char_width,
            height: self.dimensions.char_height * 4.0,
        };

        let label_width = 120.0;
        let offset_y = panel_bounds.y + self.dimensions.char_height * 0.5;
        let checkbox_size = self.dimensions.char_height * 0.8;
        let base_x = panel_bounds.x + self.dimensions.char_width + label_width;

        self.bounds.show_ascii_checkbox = Rectangle {
            x: base_x + 3.0 * self.dimensions.char_width,
            y: offset_y + self.dimensions.char_height * 2.0,
            width: checkbox_size,
            height: checkbox_size,
        };

        self.bounds.text_format = Rectangle {
            x: base_x + 2.0 * self.dimensions.char_width,
            y: offset_y + self.dimensions.char_height,
            width: self.dimensions.char_width * 3.0,
            height: self.dimensions.char_height,
        };

        self.bounds.prev_format = Rectangle {
            x: base_x,
            y: offset_y + self.dimensions.char_height,
            width: self.dimensions.char_width,
            height: self.dimensions.char_height,
        };

        self.bounds.next_format = Rectangle {
            x: base_x + 6.0 * self.dimensions.char_width,
            y: offset_y + self.dimensions.char_height,
            width: self.dimensions.char_width,
            height: self.dimensions.char_height,
        };

        self.bounds.prev_row_length = Rectangle {
            x: base_x,
            y: offset_y,
            width: self.dimensions.char_width,
            height: self.dimensions.char_height,
        };

        self.bounds.next_row_length = Rectangle {
            x: base_x + 6.0 * self.dimensions.char_width,
            y: offset_y,
            width: self.dimensions.char_width,
            height: self.dimensions.char_height,
        };

        self.bounds.text_row_length = Rectangle {
            x: base_x + 2.0 * self.dimensions.char_width,
            y: offset_y,
            width: self.dimensions.char_width * 3.0,
            height: self.dimensions.char_height,
        };
    }
}

impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}
