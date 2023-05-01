# RUST PLOT

This is a console-based graphing calculator written in Rust programming language. It is designed to solve and visualize various mathematical functions.

## Installation

To install the calculator, you need to have Rust installed on your computer. Follow the instructions provided on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust on your computer.

Once Rust is installed, you can clone the repository using Git:

```bash
git clone git@github.com:awoolfson/rust-plot.git
```

Then navigate to the project directory and run the following command to build and run the program:

```bash
cargo run
```

## Usage

The calculator supports various mathematical functions such as addition, subtraction, multiplication, division, trig functions, and more. To use the calculator, enter a mathemeatical function of the variable "x" after the the program prompts you "f(x) = ..."
For example, to plot a straight horizontal line type...

```
2 + 3
```

And press enter. The calculator will display the result:

```
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
****************************************************************
                                |
                                |
                                |
                                |
--------------------------------|-------------------------------
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

The calculator supports built in functions for log(), ln(), abs(), sqrt(), sin(), cos(), and tan(). Note that each of these must be used with
parenthesis. Also note that all instances of x must be lowercase. You can use exponentials with the "^" operator, multiplication with the "*" operator, and division with the "/" operator. The arguments are limited to --x_size and --y_size. These define the mathematical size of the window. The default is 32 for each.

Here are some more complicated functions to try out!

```
-x + sin(-x)
```

outputs...

```
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

```
abs(x)+-x
```
outputs...

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

```
x2^sin(x)*1.2
```

outputs...

```
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