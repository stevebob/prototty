use pager_prototty::App;
use prototty_graphical_::*;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    let context = Context::new(ContextDescription {
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGA.ttf").to_vec(),
        },
        title: "Pager".to_string(),
        window_dimensions: WindowDimensions::Windowed(Dimensions {
            width: 640.,
            height: 480.,
        }),
        cell_dimensions: Dimensions {
            width: 12.,
            height: 12.,
        },
        font_dimensions: Dimensions {
            width: 12.,
            height: 12.,
        },
        underline_width: 0.1,
        underline_top_offset: 0.8,
    })
    .unwrap();
    context.run_app(App::new(text))
}
