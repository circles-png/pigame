use crate::error::Result;
use log::info;
use rppal::gpio::Gpio;
use std::ops::Index;
use strum::EnumCount;
use strum::VariantArray;

/// Return true if the input is active.
///
/// # Errors
///
/// If the GPIO pin cannot be accessed, an error is returned.
pub fn is_active(input: Input) -> Result<bool> {
    let is_high = Gpio::new()?
        .get(Input::GPIO_MAP[input as usize])?
        .into_input_pulldown()
        .is_high();
    info!("Input {:?} is {}", input, if is_high { "active" } else { "inactive" });
    Ok(is_high)
}

macro_rules! impl_input {
    ($($name:ident => $pin:expr,)*) => {
        /// Return the first active input.
        #[allow(missing_docs)]
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
            pub(crate) const GPIO_MAP: [u8; Input::COUNT] = [$($pin,)*];
            /// All inputs.
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
