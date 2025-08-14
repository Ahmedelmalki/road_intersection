## Capital City Traffic Simulation

### Project Overview

This project is a traffic simulation designed to solve the traffic problem in a capital city. The primary objective is to create and visualize a traffic control strategy for a two-road intersection. The simulation will manage vehicles and traffic lights, dynamically adapting to traffic congestion to prevent gridlock and collisions.

### Features

- Two-Road Intersection: A simulation environment with two perpendicular roads, each with two lanes (one in each direction).

- Intelligent Traffic Lights: Traffic lights with red and green states, positioned at the intersection entries. The system uses a custom algorithm to prevent collisions and manage congestion.

- Dynamic Congestion Control: The traffic light logic adapts in real-time to prevent lanes from reaching their maximum capacity.

- Vehicle Behavior: Vehicles follow fixed routes (straight, left, right), maintain a safe distance from other vehicles, and obey traffic signals.

- Keyboard Controls:

    - ↑: Spawn a vehicle from the south.
    - ↓: Spawn a vehicle from the north.
    - →: Spawn a vehicle from the west.
    - ←: Spawn a vehicle from the east.
    - r: Spawn a vehicle from a random direction.
    - Esc: End the simulation.
    
### Getting Started

This project is built with Rust and uses the _macroquad_ crate for visualization.

1- Clone the repository:
```
git clone https://learn.zone01oujda.ma/git/aelmalki/road_intersection.git
```

Build and run the project:
```
cargo r
```


### Team

- Ahmed Elmalki

- Hasnae Lamrani

- Soufiane Hachimi
