# Fast Rendering with adi_gpu
How texture sheets + adi_gpu = fast render.

## Order of GPU operations is important.
* Draw stuff on the Screen:
* Draw stuff using a specific GPU Program (slow switch).
* Draw stuff with a specific texture (slow switch).
* Draw stuff with a specific uniform & vertex attrib setting (fast switch). (ZO)

* Order of GPU Programs:
* 2D Overlay (solid,gradient,texture,fade\_tex,tint\_tex,gradient\_tex) - No order
* 3D Opaque (solid,gradient,texture,tint\_tex,gradient\_tex) - Closest first
* 3D Alpha (solid,gradient,texture,fade\_tex,tint\_tex,gradient\_tex) - Farthest first
* Deferred Shading - Light Overlay - No order

## Tips
* Make texture sheets!  This is necessary because you can change texture
coordinates which is a faster switch than changing the texture.  That's about
all you have to worry about.

## Might be faster?
* Put Model (vertices) over the rest of texture coordinates and uniforms.
