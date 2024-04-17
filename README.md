Learning Resources
-------------------------------

`Learning Processes and Resources`

# What does this repo accomplish
Learning Resources is an exercise based repo for beginner to intermediate learners.
It is designed to teach fundemental concepts of computer science and programming.
It also can be used as an exercise tool for individuals changing between languages.

The exercises are based on teaching computational logic and reasoning while also teaching syntax.
As such, each section will not be syntax driven only, it's designed to teach you the core concepts of programming as you progress through the course.
Recommendations are to fork the repo to your own github user, and then push your solutions to your own git repo as you finish them.

# System Setup
## Javascript Setup
- Confirm your system has the latest [node version](https://nodejs.org/en) installed
- After installation you should be able to run `node --version` and get the version data back

# Ryan's Note: v20.11.1

## Go Setup
- Confirm your system has the latest [Go Version](https://go.dev/doc/install) installed
- After installation you should be able to run `go version` and get the version data back

# Ryan's Note: Error

## Rust Setup
- Confirm your system has the latest [Rust Version](https://www.rust-lang.org/tools/install) installed
- After installation you should be able to run `cargo --version`

# Ryan's Note: Cargo 1.76.0 (c84b36747 2024-01-18)

# How to use this package
The learning resources are broken down into folders defined as `lessons`. 
Each lesson with have a README.md which explains the concept as a general whole.
There will be multiple files for each indivdual language. Feel free to either:
- Pick one language and run through the examples in a single language
- Execute the same exercise in each language, in order to understand the concept across multiple languages  
    - Java
    - Javascript
    - Python
    - Ruby
    - C++
    - C#
    - PHP
    - Golang
    - Rust
    - Typescript
    - Swift
    - Kotlin
    - Objective-C
    - Scala

# How to run

## Javascript run
- `node main.js` in the lesson of choice

## Go Run
- `go run main.go` in the lesson of choice

## Rust run
- go into the lesson of choice with a terminal
- run `rustc main.rs`
- run `./main`



##
## Git Commands and steps
##

Any new edits (be on your branch):
- git status
    - if uncommitted changes commit and push
    - git commit -a -m "Comment"
    - git push
- git checkout main
- git pull
- git checkout luckysandwich7-exersices
- git merge main
    - resolve any merge conflicts if any
- Then you are good to code away

- NOTE: If on the wrong branch use and have done edits:
    - git stash
    - git checkout luckysandwich7-excerises
    - git stash apply