// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/gradient.frag

#version 450
#extension GL_ARB_separate_shader_objects : enable // TODO: is needed?

layout (location = 0) in vec4 fragcolor;

layout (location = 0) out vec4 uFragColor;

void main() {
	uFragColor = fragcolor;
}
