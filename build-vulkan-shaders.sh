# Aldaron's Device Interface / GPU                                             #
# Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>       #
# Licensed under the MIT LICENSE                                               #
#                             _        _                                       #
#                            /@\——————/@\                                      #
# .———  .                   |   o    o   |     .————      .            .       #
# |   | |  .———   .———   .——.     []     .——.  |      .——    ———: ———: | .   . #
# |   | |  |   |  |   |  \   \   <##>   /   /  |      |   |    /    /  | |   | #
# |———  |  |   |  |   |   |   ¯        ¯   |   |   -- |   |   /    /   |  \ /  #
# |     |  |   |  |———    |                |   |    | |   |  /    /    |   |   #
# |     |   ———   |       |                |    ————  |   | :——— :———  |   |   #
#                 |        \              /                              __/   #
#                           ¯————————————¯                                     #
# gen-spirv.sh                                                                 #

SPIRV_OPT="spirv-opt --strip-debug --freeze-spec-const --eliminate-dead-const --fold-spec-const-op-composite --unify-const"
SRC=src/native_renderer/vulkan/glsl

OUT_UNOPTIMIZED=target/spv/unoptimized
OUT_OPTIMIZED=target/spv/optimized
OUT_RELEASE=target/spv/release

mkdir -p $OUT_UNOPTIMIZED/
mkdir -p $OUT_OPTIMIZED/
mkdir -p $OUT_RELEASE/

glslangValidator $SRC/solid.frag -V -o $OUT_UNOPTIMIZED/solid-frag.spv
glslangValidator $SRC/solid.vert -V -o $OUT_UNOPTIMIZED/solid-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/solid-frag.spv -o $OUT_OPTIMIZED/solid-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/solid-vert.spv -o $OUT_OPTIMIZED/solid-vert.spv

glslangValidator $SRC/gradient.frag -V -o $OUT_UNOPTIMIZED/gradient-frag.spv
glslangValidator $SRC/gradient.vert -V -o $OUT_UNOPTIMIZED/gradient-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/gradient-frag.spv -o $OUT_OPTIMIZED/gradient-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/gradient-vert.spv -o $OUT_OPTIMIZED/gradient-vert.spv

glslangValidator $SRC/texture.frag -V -o $OUT_UNOPTIMIZED/texture-frag.spv
glslangValidator $SRC/texture.vert -V -o $OUT_UNOPTIMIZED/texture-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/texture-frag.spv -o $OUT_OPTIMIZED/texture-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/texture-vert.spv -o $OUT_OPTIMIZED/texture-vert.spv

spirv-remap --map all --dce all --strip-all --input $OUT_OPTIMIZED/*.spv --output $OUT_RELEASE/

cp $OUT_RELEASE/* src/native_renderer/vulkan/res/
# cp $OUT_UNOPTIMIZED/* src/native_renderer/vulkan/res/
