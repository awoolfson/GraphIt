# GraphIt

GraphIt is a graphing calculator that can be operated from the terminal, or on the web [here](https://awoolfson.github.io/GraphIt/). The core engine of it is written in Rust, and compiled to WASM for the online version.

## Installation

To install the calculator, you need to have Rust installed on your computer. Follow the instructions provided on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust on your computer.

Once Rust is installed, you can clone the repository using Git:

```bash
git clone git@github.com:awoolfson/GraphIt.git
```

Then navigate to the project directory and run the following command to build and run the program for use in the terminal:

```bash
cargo run
```

If you would like to host an instance of the web page locally, you can use cargo to install wasm-pack for Rust, then run

```bash
wasm-pack build --target web
```

## Usage

### in the terminal

The calculator addition, subtraction, multiplication, division, trig functions, and more. To use the calculator, enter a function of the variable "x" after the the program prompts you "f(x) = ..."
For example, to plot a straight horizontal line type...

```text
x
```

And press enter. The calculator will display the result:

```text
                                |                              *
                                |                            **
                                |                          **
                                |                        **
                                |                      **
                                |                    **
                                |                  **
                                |                **
                                |              **
                                |            **
                                |          **
                                |        **
                                |      **
                                |    **
                                |  **
                                |**
-------------------------------**-------------------------------
                             ** |
                           **   |
                         **     |
                       **       |
                     **         |
                   **           |
                 **             |
               **               |
             **                 |
           **                   |
         **                     |
       **                       |
     **                         |
   **                           |
 **                             |
```

The calculator supports built in functions for log(), ln(), abs(), sqrt(), sin(), cos(), and tan(). Note that each of these must be used with
parenthesis. Also note that all instances of x must be lowercase. You can use exponentials with the "^" operator, multiplication with the "*" operator, and division with the "/" operator. Arguments include -xsize and -ysize, which define the mathematical size of the window. The default is 32 for each. -c allows you to choose from numerous different colors using the form "-c color", and -i generates an image inside of the images directory (see example 4).

Here are some more complicated functions to try out!

Example 1

```text
cargo run
-x + sin(-x)
```

outputs...

```text
****                            |
    **                          |
      *                         |
       *                        |
        *                       |
         **                     |
           *****                |
                **              |
                  *             |
                   *            |
                    *           |
                     **         |
                       *****    |
                            **  |
                              * |
                               *|
--------------------------------**------------------------------
                                | *
                                |  **
                                |    *****
                                |         **
                                |           *
                                |            *
                                |             *
                                |              **
                                |                *****
                                |                     **
                                |                       *
                                |                        *
                                |                         *
                                |                          **
                                |    
```

Example 2

```text
cargo run
abs(x)+-x
```

outputs...

```text
                                |
                 *              |
                  *             |
                   *            |
                    *           |
                     *          |
                      *         |
                       *        |
                        *       |
                         *      |
                          *     |
                           *    |
                            *   |
                             *  |
                              * |
                               *|
--------------------------------********************************
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
```

Example 3

```text
cargo run
x2^sin(x)*1.2
```

outputs...

```text
                                |
                                |                 *      *
                                |             *
                                |
                                |
                                |                  *
                                |            *          *
                                |                   *
                                |                      *
                                |                    **
                                |           *
                                |   **     *
                                |  *  **  *
                                | *     **
                                |
                                |*
-----------------------------****-------------------------------
                           **   |
                                |
                          *     |
                **              |
               *  *      *      |
                   *            |
                                |
    *         *     *   *       |
   * *                          |
                     * *        |
  *   *               *         |
             *                  |
                                |
                                |
       *                        |
```

Example 4

```text
cargo run -- -c cyan -i
sin(x)tan(x)
```

outputs...

```text
sin(x)tan(x)
                                |
                             *  |  *
                                |
                                |
                                |
                                |
                                |
                                |
    *                           |                           *
                                |
                                |
                                |
                      *         |         *
                 *              |              *
     *   *                    * | *                    *   *
      ***         ****         *|*         ****         ***
***---------***---------****----*----****---------***---------**
           *   *            *   |   *            *   *
   *                            |                            *
                                |
                       *        |        *
                                |
                *               |               *
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
                                |
```

![alt text](https://github.com/awoolfson/GraphIt/blob/master/images/examples/sintan.png?raw=true "image output")
