use chargrid_input::*;

macro_rules! convert_char_shift {
    ($lower:expr, $upper:expr, $shift:expr) => {
        KeyboardInput::Char(if $shift { $upper } else { $lower })
    };
}

#[allow(clippy::cognitive_complexity)]
fn keyboard_input_from_js_event_key_press(key_code: u8, shift: bool) -> Option<KeyboardInput> {
    let keyboard_input = match key_code {
        8 => keys::BACKSPACE,
        9 => keys::TAB,
        13 => keys::RETURN,
        27 => keys::ESCAPE,
        32 => KeyboardInput::Char(' '),
        33 => KeyboardInput::PageUp,
        34 => KeyboardInput::PageDown,
        35 => KeyboardInput::End,
        36 => KeyboardInput::Home,
        37 => KeyboardInput::Left,
        38 => KeyboardInput::Up,
        39 => KeyboardInput::Right,
        40 => KeyboardInput::Down,
        46 => KeyboardInput::Delete,
        48 => convert_char_shift!('0', ')', shift),
        49 => convert_char_shift!('1', '!', shift),
        50 => convert_char_shift!('2', '@', shift),
        51 => convert_char_shift!('3', '#', shift),
        52 => convert_char_shift!('4', '$', shift),
        53 => convert_char_shift!('5', '%', shift),
        54 => convert_char_shift!('6', '^', shift),
        55 => convert_char_shift!('7', '&', shift),
        56 => convert_char_shift!('8', '*', shift),
        57 => convert_char_shift!('9', '(', shift),
        65 => convert_char_shift!('a', 'A', shift),
        66 => convert_char_shift!('b', 'B', shift),
        67 => convert_char_shift!('c', 'C', shift),
        68 => convert_char_shift!('d', 'D', shift),
        69 => convert_char_shift!('e', 'E', shift),
        70 => convert_char_shift!('f', 'F', shift),
        71 => convert_char_shift!('g', 'G', shift),
        72 => convert_char_shift!('h', 'H', shift),
        73 => convert_char_shift!('i', 'I', shift),
        74 => convert_char_shift!('j', 'J', shift),
        75 => convert_char_shift!('k', 'K', shift),
        76 => convert_char_shift!('l', 'L', shift),
        77 => convert_char_shift!('m', 'M', shift),
        78 => convert_char_shift!('n', 'N', shift),
        79 => convert_char_shift!('o', 'O', shift),
        80 => convert_char_shift!('p', 'P', shift),
        81 => convert_char_shift!('q', 'Q', shift),
        82 => convert_char_shift!('r', 'R', shift),
        83 => convert_char_shift!('s', 'S', shift),
        84 => convert_char_shift!('t', 'T', shift),
        85 => convert_char_shift!('u', 'U', shift),
        86 => convert_char_shift!('v', 'V', shift),
        87 => convert_char_shift!('w', 'W', shift),
        88 => convert_char_shift!('x', 'X', shift),
        89 => convert_char_shift!('y', 'Y', shift),
        90 => convert_char_shift!('z', 'Z', shift),
        96 => KeyboardInput::Char('0'),
        97 => KeyboardInput::Char('1'),
        98 => KeyboardInput::Char('2'),
        99 => KeyboardInput::Char('3'),
        100 => KeyboardInput::Char('4'),
        101 => KeyboardInput::Char('5'),
        102 => KeyboardInput::Char('6'),
        103 => KeyboardInput::Char('7'),
        104 => KeyboardInput::Char('8'),
        105 => KeyboardInput::Char('9'),
        106 => KeyboardInput::Char('*'),
        107 => KeyboardInput::Char('+'),
        109 => KeyboardInput::Char('-'),
        110 => KeyboardInput::Char('.'),
        111 => KeyboardInput::Char('/'),
        112 => KeyboardInput::Function(1),
        113 => KeyboardInput::Function(2),
        114 => KeyboardInput::Function(3),
        115 => KeyboardInput::Function(4),
        116 => KeyboardInput::Function(5),
        117 => KeyboardInput::Function(6),
        118 => KeyboardInput::Function(7),
        119 => KeyboardInput::Function(8),
        120 => KeyboardInput::Function(9),
        121 => KeyboardInput::Function(10),
        122 => KeyboardInput::Function(11),
        123 => KeyboardInput::Function(12),
        186 => convert_char_shift!(';', ':', shift),
        187 => convert_char_shift!('=', '+', shift),
        188 => convert_char_shift!(',', '<', shift),
        189 => convert_char_shift!('-', '_', shift),
        190 => convert_char_shift!('.', '>', shift),
        191 => convert_char_shift!('/', '?', shift),
        192 => convert_char_shift!('`', '~', shift),
        219 => convert_char_shift!('[', '{', shift),
        220 => convert_char_shift!('\\', '|', shift),
        221 => convert_char_shift!(']', '}', shift),
        222 => convert_char_shift!('\'', '"', shift),
        _ => return None,
    };
    Some(keyboard_input)
}

pub fn from_js_event_key_press(key_code: u8, shift: bool) -> Option<Input> {
    keyboard_input_from_js_event_key_press(key_code, shift).map(Input::Keyboard)
}
