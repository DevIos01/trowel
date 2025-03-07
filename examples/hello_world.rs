#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use trowel::{App, AppResult, Buttons, Error};

struct DrawFerris {
    frame: i32, // Frame count
}

impl App for DrawFerris
{
    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, _buttons: Buttons) -> AppResult {
        self.frame += 1;
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
        where T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.frame == 1 {
            // Create a new character style
            let style = MonoTextStyle::new(&ascii::FONT_7X13, Rgb565::WHITE);

            Text::new("Hello, World!", Point::new(20, 30), style).draw(display)
                .map_err(|_| Error::DisplayErr)?;
        }
        Ok(())
    }
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn main() -> ! {
    trowel::run(DrawFerris { frame: 0 });
}
