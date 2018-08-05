// Copyright Jeron A. Lau 2017 - 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

extern crate adi_gpu;
extern crate aci_png;
extern crate awi;

use adi_gpu::DisplayTrait;

fn main() {
	let mut window = awi::Window::new("ADI_GPU Minimal Example",
		aci_png::decode(include_bytes!("../res/icon.png")).unwrap());
	let mut display = adi_gpu::Display::new(&window).unwrap();

	display.color((0.25, 0.25, 1.0));
	display.fog(Some((20.0, 10.0)));

	'app: loop {
		// Go through this frame's input.
		while let Some(input) = window.input() {
			use awi::Input::*;
			use awi::Msg::*;

			match input {
				Msg(Quit) | Msg(Back) => break 'app,
				_ => {},
			}
		}

		display.update();
	}
}
