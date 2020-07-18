# Scheme Transpiler

This transpiles the input message to Scheme. Run with:

    go run main.go INPUT_FILE EXPRESSION

This outputs Scheme program, so you can pipe to, for example, `gosh` to
evaluate and print out the evaluation result of the EXPRESSION.

## Checkerboard example

https://message-from-space.readthedocs.io/en/latest/message33.html

See `checkerboard.input`. There's the `checkerboard` definition. According to
the interpretation on the page above, this takes two numbers as arguments:

    go run main.go ./checkerboard.input "ap ap checkerboard 7 0"

This command outputs Scheme program that evaluates "ap ap checkerboard 7 0" and
pass the result to `printout` in the prelude.scm. Currently, the printout
command is defined as `(print (serialize x))`. The `serialize` function takes a
value from the evaluator and creates a scheme world's value like this:

    ((0 . 0) (0 . 2) (0 . 4) (0 . 6) (1 . 1) (1 . 3) (1 . 5) (2 . 0) (2 . 2)
     (2 . 4) (2 . 6) (3 . 1) (3 . 3) (3 . 5) (4 . 0) (4 . 2) (4 . 4) (4 . 6)
     (5 . 1) (5 . 3) (5 . 5) (6 . 0) (6 . 2) (6 . 4) (6 . 6))

(This is a list of pixel positions to draw a checkerboard image.)

## Galaxy output

`$REPO_ROOT/messages/galaxy.txt` is a huge input that defines a `galaxy`
function. This function takes two values: a state and a coordinate. Per
https://message-from-space.readthedocs.io/en/latest/message39.html, the state
starts from `nil` and the coordinate starts from `(0 . 0)`:

    go run main.go ../../../messages/galaxy.txt "ap ap galaxy nil ap ap cons 0 0"

This outputs a tuple of three (or two) elements:

    (
      0
      (0 (0) 0 ())
      (((-1 . -3) ...) ((-7 . -3) (-8 . -2)) ()))

The first element is 0 or 1. If it's 0, it's a draw instruction. If it's 1, it's
a send instruction. So far, we haven't seen a send instruction, so we ignore
that.

The second element is the next state (e.g. the argument that we places `nil` for
this run).  The third element is a list of draw instruction like we saw in the
checkerboard example above. Note that this contains multiple images. In this
case, the first image has a dot at the coordinate (-1, -3) etc., the second
image has a coordinate (-7, -3) etc., and the last image doesn't have any dot.

As we saw, the next state `(0 (0) 0 ())` is given, so we can run this
iteratively:

    go run main.go ../../../messages/galaxy.txt \
        "ap ap galaxy
            ap ap cons
               0
               ap ap cons
                  ap ap cons 0 nil
                  ap ap cons
                     0
                     ap ap cons nil nil
            ap ap cons 0 0"

     (Formatted to make the argument understandable)

This process repeats to some extent. See the shared notes for details.

### draw.py

It's hard to render the images by hand. draw.py renders the image by taking an
output of galaxy.txt runs.
