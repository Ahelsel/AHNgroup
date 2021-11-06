# Computational Redistricting in Rust (CS 199 Final Project)

## Group Information
* **Group Number:** 9
* **Member Names and NetIDs:** [Anthony Helsel](https://github.com/Ahelsel) (ahelsel2), [Neil Angajala](https://github.com/nang24) (neila2), [Harsh Tiwary](https://github.com/notnotharsh) (htiwary2)

## Project Introduction
This project is a Rust-based analysis and implementation of computational redistricting techniques to optimize different criteria (e.g. compactness, partisan asymmetry, etc.).

## System Overview
In this project, we wish to:
* present an initial map of districts for a region using a Rust-provided graphics crate
* program an algorithm (that takes in demographic data for each district and a function to optimize) to redraw the district set, optimizing for a certain fairness metric
* implement this algorithm with various fairness metric functions as parameters to generate different images for district redrawing for a certain region
* (if possible) write a function to generate hypothetical district maps of arbitrary shapes and sizes to examine what the redistricting algorithm does to a variety of datasets
## Possible Challenges
Some of these challenges include:
* having enough background about the various fairness metrics (the functions to be passed into the optimization algorithm) to implement them as functions that even can be optimized to provide an arbitrary "fairness" value
* being able to program the necessary graphics implementations to represent each redraw as a readable image
* having the computational power to handle the large datasets necessary for this project
  
## References
Part of the inspiration for the project is from [this](http://redistricting.cs.illinois.edu/2021_Illinois.pdf) research paper, a proposal by the University of Illinois' [Institute for Computational Redistricting](https://redistricting.cs.illinois.edu/) to redistrict the state of Illinois.
