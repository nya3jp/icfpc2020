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
