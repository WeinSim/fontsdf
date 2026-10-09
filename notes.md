## TODO

- implement simple dragging view (like I had in logic gate simulator)
- draw SDFs of a 1 or 2 simple shapes
- then attempt quadratic bezier SDF
    - actually, since all quadratic beziers are identical up to affine-linear
      transformations, I could compute an exact SDF (or look at what iq has on
      his website) and just sample that from a texture in a smart way 
- then cubic beziers (make sure that ttf fonts actually use cubic beziers!)
- also need to figure out how to load ttf font files
- how to export rendered frame as png file?


### Quadratic Bezier SDF

Consider the graph $G$ of $f(x) = x^2$ for $x \in [0, 1]$.

Let $p = (x_0, y_0) \in [0, 1]^2$.

Let $d$ be the distance from $p$ to $G$.
let $d_x$ be the distance from $p$ to $(x, f(x)) \forall x \in [0, 1]$.
$d = \min_{x \in [0, 1]} d_x = \sqrt{\min_{x \in [0, 1]} d_x^2}$.

Expand: $d_x^2 = (x_0 - x)^2 + (y_0 - x^2)^2 
       = x_0^2 - 2 x_0 x + x^2 + y_0^2 - 2 y_0 x^2 + x^4
       = x^4 + (1 - 2 y_0) x^2 - 2 x_0 x + (x_0^2 + y_0^2)$

To find the minimum, take the derivative w.r.t. $x$ and set it to zero:
$(d_x^2)' = 4 x^3 + 2 (1 - 2 y_0) x - 2 x_0 \overset ! = 0$

Simplify:
$ x^3 + \frac{(1 - 2 y_0)}{2} x - \frac {x_0} 2 = 0 $

This is a reduced cubic with $p = (1 - 2 y_0) / 2$ and $q = x_0 / 2$.

Let $\Delta = (q / 2)^2 + (p / 3)^3
= (x_0 / 4)^2 + ((1 - 2 y_0) / 6)^3
= x_0^2 / 16 + (1 - 2 y_0)^3 / 216$.

Check the sign of $\Delta$:

$\Delta > 0 \iff x_0^2 / 16 > - (1 - 2 y_0)^3 / 16
= (2 y_0 - 1)^3 / 216
\iff x_0^2 > (2 y_0 - 1)^3 \cdot 16 / 216
= (2 y_0 - 1)^3 \cdot 2 / 27
$

...
