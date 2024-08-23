# dcrs

This is a simple [dc](https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html)-inspired arbitrary precision calculator.
It uses [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) and a traditional stack-based implementation.

Right now, addition, subtraction, multiplication, division, integer exponentiation of rational bases, and cube and square roots are implemented.
The modulo operator is defined, but unimplemented. There are three control characters, 'p', 'c', and 't', which print and clear the stack, and display the top
element of the stack, respectively.

There are no registers or macros.
