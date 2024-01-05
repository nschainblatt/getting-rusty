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
- Strings, vectors, hash maps, and many more complex types have a pointer, a length, and a capacity that resides on the stack in memory. The pointer on the stack points to the data that is stored on the heap. Data is stored on the heap typically when dealing with dynamic data, or data that changes.
- Basic types such as ints, floats, booleans, string literals and others all reside on the stack only. This is because these types are of fixed size known at compile time. It is also more efficient than being stored on the heap. A integer of type i32 will always take up the same amount of memory.

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
- Read some of chapter 7, which went over organizing code into different modules.
- Completed day 01 part 01 of advent of code in Rust.

## Week 3

### 12/31/2023
- Worked on day 01 part 02 of advent of code.

### 01/01/2024
- Finished reading chapter 7 which was about organizing your code into seperate modules.
- Modules: are a way to seperate your code into seperate scopes to control what code is public and private in your package. Modules are mostly used in seperate files but can be written all in one file by using the mod keyword. Modules are great for organizing related code and functionality into seperate modules/files so you know where to look for a specific feature or bug. 
- Packages: A cargo feature that lets you build, test, and share crates.
- Crates: A tree of modules that produces a library or an executable (lib.rs/main.rs) a crate can have both.
- Modules: Let you control the organization, scope, and privacy of paths, use the mod keyword to reference a module in a seperate foler or to create one in the current file within curly braces. (Typically organized into seperate modules but you have to declare with mod and then use the keyword use below).
- Use: The keyword than lets you bring in the functionality of a declared module into scope.
- Paths: A way of naming an item, such as a struct, function, or module.

### 01/02/2024
- Read section 8.1, which was about vectors.
- Vectors: are one of rusts collections that can hold a variable number of elements of all the same type. Vectors don't have to have the size known at compile time, they can grow larger and smaller throughout runtime. The vector you deal with in code is stored on the stack as a pointer to it's data which is stored on the heap. The same concepts with ownersip apply to vectors, if you create an immutable reference then you cannot have a mutable reference at the same time and you also can't push onto a vector with an existing immutable array.

### 01/03/2024
- Read the rest of chapter 8, learned more abouts the String type and the string slice &str, as well as hash maps.
- String: A type that has been coded into the standard library rather than being apart of the core language. String type is mutable, growable, and owned and encoded in UTF-8. String's data is stored on the head, with the pointer, length, and capacity being stored on the stack.
- str: A string slice that is part of the core language. str's are not mutable, they are static and are stored on the stack. str's are typically used as its reference version &str which is just called a reference to a string slice. These are also UTF-8 encoded. String literals are stored in the program's binary and are therefore string slices.
- HashMaps: HashMaps are a collection that stores a mapping of keys to values.

### 01/04/2024
- Completed the first exercise/project at the end of chapter 8.
- Project: Sort a list of integers, get the median, and then get the mode. Using vectors and hashmaps.
