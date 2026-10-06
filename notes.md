## TODO

### WGPU

- contiuously update uniform (based on mouse position?)
- where is the actual main loop? or do I not actually need one?

### FontSDF

- implement simple dragging view (like I had in logic gate simulator)
- draw SDFs of a 1 or 2 simple shapes
- then attempt quadratic bezier SDF
    - actually, since all quadratic beziers are identical up to affine-linear
      transformations, I could compute an exact SDF (or look at what iq has on
      his website) and just sample that from a texture in a smart way 
- then cubic beziers (make sure that ttf fonts actually use cubic beziers!)
- also need to figure out how to load ttf font files
- how to export rendered frame as png file?
