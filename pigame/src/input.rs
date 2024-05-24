use std::ops::Index;
use strum::VariantArray;

use rust_gpiozero::Button;
use strum::EnumCount;

macro_rules! impl_input {
    ($($name:ident => $pin:expr,)*) => {
        #[must_use]
        pub fn is_active(input: Input) -> bool {
            Button::new_with_pulldown(Input::GPIO_MAP[input]).is_active()
        }

        #[derive(Debug, EnumCount, VariantArray, Copy, Clone)]
        pub enum Input {
            $($name,)*
        }

        impl Index<Input> for [u8; Input::COUNT] {
            type Output = u8;

            fn index(&self, index: Input) -> &Self::Output {
                &self[index as usize]
            }
        }

        impl Input {
            pub const GPIO_MAP: [u8; Input::COUNT] = [$($pin,)*];
            pub const ALL: &'static [Input] = Self::VARIANTS;
        }
    };
}

impl_input! {
    Left => 17,
    Right => 27,
    Up => 22,
    Down => 23,
    A => 5,
    B => 6,
    Hotkey => 13,
    Start => 19,
}
