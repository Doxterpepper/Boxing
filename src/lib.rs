use std::cmp::max;

const VERTICAL: &'static str = "│";
const HORIZONTAL: &'static str = "─";
const TOP_LEFT: &'static str = "┌";
const TOP_RIGHT: &'static str = "┐";
const BOTTOM_LEFT: &'static str = "└";
const BOTTOM_RIGHT: &'static str = "┘";

pub enum Alignment {
    Left,
    Right,
}

struct Formatting {
    padding: usize,
    alignment: Alignment,
    max_width: usize,
    padding_left: Option<usize>,
    padding_right: Option<usize>,
    padding_top: Option<usize>,
    padding_bottom: Option<usize>,
}

pub struct Box {
    message: String,
    format: Formatting,
}

impl Formatting {
    pub fn new() -> Formatting {
        Formatting {
            padding: 2,
            alignment: Alignment::Left,
            max_width: 80,
            padding_left: None,
            padding_right: None,
            padding_top: None,
            padding_bottom: None,
        }
    }
}

impl Box {
    /// Create a new boxed message
    pub fn new(message: String) -> Box {
        Box {
            message: message,
            format: Formatting::new(),
        }
    }

    /// Set the global padding on the box
    pub fn padding(mut self, pad: usize) -> Self {
        self.format.padding = pad;
        self
    }

    /// Set the alignment of the content
    pub fn alignment(mut self, align: Alignment) -> Self {
        self.format.alignment = align;
        self
    }

    /// Set the maximum width of the box before lines should wrap
    pub fn max_width(mut self, width: usize) -> Self {
        self.format.max_width = width;
        self
    }

    /// Set the padding on the bottom, overrides the global bottom padding
    pub fn padding_bottom(mut self, pad: usize) -> Self {
        self.format.padding_bottom = Some(pad);
        self
    }

    /// Set the padding on the top, overrides the global top padding
    pub fn padding_top(mut self, pad: usize) -> Self {
        self.format.padding_top = Some(pad);
        self
    }

    /// Set the padding on the left, overrides the global left padding
    pub fn padding_left(mut self, pad: usize) -> Self {
        self.format.padding_left = Some(pad);
        self
    }

    /// Set the padding on the right, overrides the global right padding
    pub fn padding_right(mut self, pad: usize) -> Self {
        self.format.padding_right = Some(pad);
        self
    }

    /// Boxed message to string
    pub fn to_string(self) -> String {
        let max_length = max_line_length(&self.message);
        let format = self.format;
        let top_padding = format.padding_top.unwrap_or(format.padding / 2);
        let bottom_padding = format.padding_bottom.unwrap_or(format.padding / 2);
        let right_padding = format.padding_right.unwrap_or(format.padding);
        let left_padding = format.padding_left.unwrap_or(format.padding);
        let total_horizontal_pad = right_padding + left_padding;

        let mut boxed_message = gen_top(max_length + right_padding + left_padding);
        boxed_message += &gen_vertical_padding(top_padding, max_length + total_horizontal_pad);
        boxed_message += &wrap_lines(self.message, &format, max_length);
        boxed_message += &gen_vertical_padding(bottom_padding, max_length + right_padding + left_padding);
        boxed_message += &gen_bottom(max_length + left_padding + right_padding);
        boxed_message
    }
}

/// Helper function to build the top of the box
fn gen_top(length: usize) -> String {
    let mut top = String::from(TOP_LEFT);
    top += &(0..length).map(|_| HORIZONTAL).collect::<String>();
    top += TOP_RIGHT;
    top += "\n";
    top
}

/// Helper function to build the bottom of the box
fn gen_bottom(length: usize) -> String {
    let mut bottom = String::from(BOTTOM_LEFT);
    bottom += &(0..length).map(|_| HORIZONTAL).collect::<String>();
    bottom += BOTTOM_RIGHT;
    bottom += "\n";
    bottom
}

/// Helper function to generate top and bottom padding of the box
fn gen_vertical_padding(pad: usize, length: usize) -> String {
    (0..pad).map(|_| format!("{}{}{}\n", VERTICAL, gen_whitespace(length), VERTICAL))
        .collect::<String>()
}

/// Helper function to generate padding left of the content
fn gen_left_padding(format: &Formatting, line_length: usize, max_length: &usize) -> String {
    let padding = match format.alignment {
        Alignment::Left => format.padding,
        Alignment::Right => format.padding + max_length - line_length,
    };
    gen_whitespace(padding)
}

/// Helper function to generate padding right of the content
fn gen_right_padding(format: &Formatting, line_length: usize, max_length: &usize) -> String {
    let padding = match format.alignment {
        Alignment::Right => format.padding,
        Alignment::Left => format.padding + max_length - line_length,
    };
    gen_whitespace(padding)
}

/// Wrap the message with the box on it's left and right
fn wrap_lines(message: String, format: &Formatting, max_length: usize) -> String {
    message.lines().map(|line| {
        let left_padding = gen_left_padding(format, line.len(), &max_length);
        let right_padding = gen_right_padding(format, line.len(), &max_length);
        format!("{}{}{}{}{}\n", VERTICAL, left_padding, line, right_padding, VERTICAL)
    }).collect::<String>()
}

/// Helper function to get the length of the longest line
fn max_line_length(message: &String) -> usize {
    let mut max_length = 0;
    for line in message.lines() {
        max_length = max(max_length, line.len())
    }
    max_length
}

/// Helper function to generate whitespace for padding
fn gen_whitespace(num: usize) -> String {
    (0..num).map(|_| " ").collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_padding() {
        let expected = "│            │\n│            │\n";
        let result = gen_vertical_padding(2, 12);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_basic_box() {
        let expected = "┌────────────┐
│            │
│  whatever  │
│  whatever  │
│            │
└────────────┘\n";
        let boxed_content = Box::new(String::from("whatever\nwhatever"));
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn test_left_align() {
        let expected = "┌──────────────────────────────────────────────────────────────────────┐
│                                                                      │
│  Lorem ipsum dolor sit amet,                                         │
│  consectetur adipiscing elit,                                        │
│  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘\n";
        let message = "Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = Box::new(String::from(message)).alignment(Alignment::Left);
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn test_right_align() {
        let expected = "┌──────────────────────────────────────────────────────────────────────┐
│                                                                      │
│                                         Lorem ipsum dolor sit amet,  │
│                                        consectetur adipiscing elit,  │
│  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘\n";
        let message = "Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = Box::new(String::from(message)).alignment(Alignment::Right);
        assert_eq!(expected, boxed_content.to_string());
    }
}
