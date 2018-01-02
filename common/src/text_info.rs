use ansi_colour::Colour;
use defaults::*;
use prototty::Cell;

/// Rich text settings
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TextInfo {
    pub foreground_colour: Colour,
    pub backrgound_colour: Colour,
    pub underline: bool,
    pub bold: bool,
}

impl Default for TextInfo {
    fn default() -> Self {
        Self {
            foreground_colour: DEFAULT_FG,
            backrgound_colour: DEFAULT_BG,
            underline: false,
            bold: false,
        }
    }
}

impl TextInfo {
    pub fn foreground_colour(self, colour: Colour) -> Self {
        Self { foreground_colour: colour, .. self }
    }
    pub fn backrgound_colour(self, colour: Colour) -> Self {
        Self { backrgound_colour: colour, .. self }
    }
    pub fn underline(self) -> Self {
        Self { underline: true, .. self }
    }
    pub fn bold(self) -> Self {
        Self { bold: true, .. self }
    }
    pub fn write_cell(&self, cell: &mut Cell) {
        cell.foreground_colour = self.foreground_colour;
        cell.background_colour = self.backrgound_colour;
        cell.bold = self.bold;
        cell.underline = self.underline;
    }
}