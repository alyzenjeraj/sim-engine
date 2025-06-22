# Multi-Agent 2D Coordination Simulator

## 🧠 Project Purpose

This project is a 2D multi-agent simulation platform built in Rust using the Bevy game engine. It is designed to simulate and study how multiple autonomous agents (robots, vehicles, etc.) can:

* Move within a shared environment
* Plan paths toward assigned goals
* Avoid collisions and deadlocks
* Coordinate with each other to resolve resource and path conflicts

This simulation serves as a **testbed** for learning, prototyping, and benchmarking multi-agent planning and coordination algorithms.

---

## 🎯 Project Goals

### ✅ Foundation

* Spawn multiple agents in a continuous 2D plane
* Assign goals to agents
* Basic straight-line motion toward goals
* Simple ECS (Entity Component System) architecture with Bevy

### ✅ Visual Simulation

* Real-time rendering of agents, movement, and world state
* Smooth updates with agent motion displayed visually

### 🧪 Algorithm Testbed

* Time-expanded path planning (e.g., A\* with time steps)
* Prioritized planning (greedy multi-agent planning)
* Conflict-Based Search (CBS)
* Potential Fields / ORCA for reactive collision avoidance
* Multi-Agent RRT
* Multi-Robot Task Allocation (MRTA)

---

## 🧱 Architecture Overview

### 🔄 ECS-Based Design

Built on Bevy's Entity-Component-System paradigm:

* **Entities**: Agents, goals, world boundaries, obstacles
* **Components**: Position, Velocity, Goal, ID, Task status
* **Systems**: Path planning, movement, conflict resolution, UI

### 📁 Modular Folder Structure

```
sim-engine/
├── src/
│   ├── main.rs                # Entry point
│   ├── app.rs                 # App setup and system registration
│   ├── constants.rs           # Global constants
│   ├── agents/                # Agent logic and components
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   └── systems.rs
│   ├── simulation/            # Global simulation parameters
│   │   ├── mod.rs
│   │   ├── resources.rs
│   │   └── systems.rs
│   ├── world/                 # Obstacles, map, bounds
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   └── systems.rs
│   └── ui/                    # (Future) GUI/visualization
│       ├── mod.rs
│       └── systems.rs
```

---

## 🌍 What It Simulates

| Element      | Description                                |
| ------------ | ------------------------------------------ |
| Agents       | Robots, vehicles, or drones with goals     |
| Goals        | Random or assigned destinations            |
| Obstacles    | Static objects (walls, shelves, etc.)      |
| Movement     | Velocity-based updates each frame          |
| Coordination | Avoiding collisions and handling conflicts |

---

## 📚 Learning Outcomes

This project teaches and reinforces:

* Rust fundamentals and idiomatic system design
* ECS architecture with Bevy
* Real-time simulation control loops
* Motion planning algorithms (A\*, RRT, CBS, etc.)
* Collision avoidance and safety rules
* Multi-agent communication and planning strategies
* Scalable software engineering patterns

---

## 🛠 Future Roadmap

### 🚧 Phase 1: Foundational Simulation

* [x] ECS setup with agent spawning
* [x] Basic goal-following with straight-line movement
* [ ] Simple boundary/world space
* [ ] Visual debugging (colors, IDs, etc.)

### 🤖 Phase 2: Planning and Avoidance

* [ ] Grid overlay or path planning graph
* [ ] Implement time-augmented A\*
* [ ] Add reservation table conflict checker
* [ ] Introduce prioritized planning / CBS

### 🌀 Phase 3: Reactive Control

* [ ] Add local avoidance via ORCA or potential fields
* [ ] Introduce dynamic obstacles or agent noise

### 📦 Phase 4: Task Allocation and Communication

* [ ] Assign and reassign tasks to agents
* [ ] Add messaging system for decentralized planning

### 🧪 Phase 5: Experimentation and Benchmarking

* [ ] Run test batches with N agents and collect metrics
* [ ] Compare planning approaches under stress
* [ ] Export agent trajectories and simulation logs

---

## 🚀 Getting Started

1. Install Rust and Bevy
2. Clone this repo
3. Run `cargo run`
4. Observe agents moving toward goals in a 2D world

---

## 🙌 Contributions Welcome

This project is in early development and aims to grow into a full-featured research-quality simulation. Contributions, feature ideas, or feedback are highly encouraged.
