# lets_rust
[![Ask DeepWiki](https://devin.ai/assets/askdeepwiki.png)](https://deepwiki.com/MasemeneMatlakanaBenny/lets_rust)

This repository is a collection of simple Rust programs and exercises, created as part of a journey to learn the Rust programming language. It includes basic examples covering fundamental concepts like variables, functions, arithmetic operations, conditional logic, and arrays.

## Programs

### Variable Swapping (`main.rs`)

A simple program that demonstrates how to swap the values of two integer variables. It initializes two numbers and uses a third temporary variable to switch their values, confirming the result with `assert_eq!`.

### Simple Calculator (`simple_calculator.rs`)

This file contains functions to perform basic arithmetic operations:
-   `sum(x, y)`: Returns the sum of two integers.
-   `prod(x, y)`: Returns the product of two integers.
-   `abs_diff(x, y)`: Returns the absolute difference between two integers.

## Beginner Exercises

The `rust_beginner_exercises` directory contains a series of small, focused exercises.

-   **`exercise_1.rs`**: Calculates and prints a person's age based on their birth year and a predefined current year.
-   **`exercise_2.rs`**: Calculates the area of a circle given its diameter. It demonstrates this using two different methods and asserts their equality.
-   **`exercise_3.rs`**: A program that uses a boolean variable (`is_sunny`) and an `if/else` statement to print a message based on the weather.
-   **`exercise_4.rs`**: Defines a string variable with a name and prints a simple greeting message.
-   **`exercise_5.rs`**: Creates an array of five integers and demonstrates how to iterate over it using a `for` loop to print each element.

## How to Run

Each `.rs` file is a standalone Rust program. To compile and run any of the files, you will need the Rust compiler (`rustc`).

1.  Navigate to the directory containing the file you want to run.
2.  Compile the file using `rustc`:
    ```sh
    rustc <filename>.rs
    ```
    For example:
    ```sh
    rustc simple_calculator.rs
    ```
3.  This will create an executable file in the same directory (e.g., `simple_calculator`).
4.  Run the executable:
    ```sh
    ./<filename>
    ```
    For example:
    ```sh
    ./simple_calculator