# My Journey With Getting Rusty

This file represents the steps I have taken so far in learning Rust.

## Week 1

### 12/23/2023
- Read the first three chapters of [The Rust Programming Language](https://doc.rust-lang.org/stable/book/).
- Completed all code-alongs.
- Completed 2/3 exercises at the end of the third chapter (fahrenheit_to_celsius, nth_fibonacci_number).

## Week 2

### 12/24/2023
- Completed the last exercise from the third chapter (twelve_days_of_christmas).
- Read all of chapter four (will read again to solidify the concept of ownership, references, and slices).

### 12/25/2023
- Read chapter four again to get a better grasp of Rust's memory management system.
- Started reading chapter five which involves structs.
  - A Struct is a type of data structure that groups together variables of different types under a single name.
  - Structs give the programmer the ability to create custom data types to represent a concept in a program.

### 12/26/2023
- Read all of chapter five.
- Learned about structs and their use cases.
- Learned about struct methods, otherwise known as associated functions.
- Uploaded a project named rectangles which demonstrates basic use cases of structs.

### 12/27/2023
- Worked on a to_do project to practice writing Rust without any guidance.
- Project is mostly done, just a few more things to add before diving into deeper concepts.
- This project lets me get used to the syntax and basic flow of Rust before moving on.
- I plan on doing mini projects throughout this book on top of the exercises to solidify my learning.
- Once I learn more about Rust, I plan on doing DSA (Data Structure and Algorithms) problems using Rust as I feel this is an excellent way to learn the language.
- I've got some more reading on Rust to do first.

### 12/28/2023
- Continued working on my to_do project.
- I've incorporated a multi-level menu system and am now allowing users to have multiple to_do lists.
- Users can add up to five todos per list (can easily use vectors but want to limit the todo capacity to increase descriptiveness and productivity).
- So far i've incorporated, structs, methods, functions, loops, ownership, and references (both mutable and immutable) with a various other few topics I found on my own that I have not come across in the book.
- Although simple, I really enjoy putting the new syntax and the new topics (ownership and references) to practice in a simple but satisfying CLI program.

### 12/29/2023
- Added a delete method for todo lists where it deletes the instance by a field name value.
- Read chapter 6 which was about enums, the match control flow construct and the if let concice control flow construct.
- Enums: are similar to structs in that they allow you to create custom types. They are different in the fact that an instance can be one of several variants defined in the enum definition, but each instance of an enum can only be one of its defined variants at a time. This allows you to maintain the same type but allow different variants within the type. Structs are for organizing related data with different types into a single type where each struct instance has every field. Enums are great for pattern matching with the match construct. Enums also can have their own methods just like structs.
- Match construct: Similar to a switch case in other programming languages, though it has more capabilities than just comparing values, such as pattern matching which can destructure tuples, enums, pointers, etc. It allows you to match a value to a pattern and then return a another value, you can also run a block and then return a value, but it must return the corresponding type thats expected from where the value is getting assigned to/returning to. A match must provide each possible outcome that there is. The match case has a default case to ensure this.

### 12/30/2023
- Read most of chapter 7, which went over organizing code into different modules.
