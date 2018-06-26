// "adi_gpu" - Aldaron's Device Interface / GPU
//
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)
//
//! Interface with the GPU to render graphics or do fast calculations.

extern crate adi_gpu_base;

pub use adi_gpu_base::*;

/// Create a new Vulkan / OpenGL Display.
pub fn new_display<G: AsRef<Graphic>>(title: &str, icon: G)
	-> Result<Box<Display>, String>
{
	let mut err = "".to_string();

	// Try Vulkan first.
	#[cfg(any(
		target_os="macos", target_os="android", target_os="linux",
		target_os="windows", target_os="nintendo_switch"
	))]
	{
		extern crate adi_gpu_vulkan;

		match adi_gpu_vulkan::new(title, &icon) {
			Ok(vulkan) => return Ok(vulkan),
			Err(vulkan) => err.push_str(&vulkan),
		}
		err.push('\n');
	}

	// Fallback on OpenGL/OpenGLES
	#[cfg(any(
		target_os="android", target_os="linux", target_os="windows",
		target_os="web"
	))]
	{
		extern crate adi_gpu_opengl;

		match adi_gpu_opengl::new(title, &icon) {
			Ok(opengl) => return Ok(opengl),
			Err(opengl) => err.push_str(opengl),
		}
		err.push('\n');
	}

	// Give up
	err.push_str("No more backend options");
	Err(err)
}
