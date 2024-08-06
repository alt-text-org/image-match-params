#image-match-rs parameter testing

This repo uses `image-match-rs` and runs various tests and examples that don't
need to be in the core library. Its original purpose was to vary the parameters
of the algorithm to find optimal values, and somewhat achieves that, but
currently moving ahead with the current "good enough" default parameters is more
of a priority.

This project also provides the `draw-debug` function, which takes input images
and draws debug information on them, showing how the algorithm is calculating
the signature.
