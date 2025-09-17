# Rust Workshop 2025

Repository of example programs for ProgSoc's Rust Workshop for 2025.

## Contents of this Repository

This repository contains templates and solutions to each part of the Rust Workshop.
These are contained in the folders `src/parts` and `src/solutions` respectively.

**SPOILER WARNING:**
Look at the `src/solutions` folder *only* if you feel stuck and the workshop is moving too fast.
It it designed for reference purposes to see how your solution during the workshop compares.
(You could very well have found a better one!)

## Instructions

There are two ways you can use this repository after cloning it.
You can either use the workspace as-is and fill in the templates in `src/parts` as you go,
or you can create a new Rust binary application (using `cargo new`) and copy the templates in step by step.

### Option 1: Filling in the `src/parts` templates

When we get to a slide that says "**Example Program**",
it will come with a bracket (e.g., **"Example Program (Part 1)"**).
Please navigate to the `src/parts` folder for the corresponding template code
(in this example, `src/parts/part_1.rs`), and fill in the code as prompted.

Then, navigate to `src/main.rs`, modify the value of the variable `part` on Line 9 to the correct number
(`1` in this example), and ensure the variable `use_solution` on Line 8 is set to `false` to use your code.

Finally, to execute the code, run `cargo run --release` in the root folder
(on the same level as this `README.md` file).

### Option 2: Copying in the templates as you go

Here, you would have already made a new Rust binary folder of your own through `cargo new first-rust-app`
(where you can replace `first-rust-app` with whichever name you prefer).

When we get to a slide that says "**Example Program**",
it will come with a bracket (e.g., **"Example Program (Part 1)"**).
Please navigate to the `src/parts` folder for the corresponding template code
(in this example, `src/parts/part_1.rs`), and copy the entire file's contents into your `main.rs` file.

To run your code, you would then run `cargo run --release` in the root of the application you created.

## After the Workshop

As per **Option 1**, the Rust package that this repository contains is capable of running as-is.

To test out different parts of the workshop code, change the `part` variable to see different snapshots
of the content in action. Keep it an integer from 1 to 5 though.

You can even check your own implemented output against the workshop solution code by
toggling the `use_solution` boolean variable. If `true`, the workshop solution code will be used.
If `false`, your own implementation will be used.
