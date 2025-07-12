src/
  main.rs
  agent/
    mod.rs
    components.rs      // Agent, Velocity, EntityId, AgentConfig
    spawner.rs         // spawn_agents function and thread logic
    state.rs           // (optional) Agent state machines, status enums
  messaging/
    mod.rs
    types.rs           // VelocityMsg, VelocityMsgSender, AgentVelocityReceiver
    systems.rs         // receive_and_apply_velocity system
  reservation/
    mod.rs
    table.rs           // ReservationTable struct and atomic logic
    systems.rs         // (optional) Bevy systems for reservation updates
  systems/
    mod.rs
    movement.rs        // apply_velocity and other movement systems
    rendering.rs       // (optional) custom rendering helpers
  map/
    mod.rs
    graph.rs           // Map graph, nodes, edges, etc.
    pathfinding.rs     // Path planning algorithms
  utils/
    mod.rs
    timer.rs           // (optional) Custom timers, utilities

// High Level Components to Add:
// Robot Manager - is a thread applied for each robot to command its movements
// Do I need to implement some type of state machine here?
// Localized mission planner? Or a global one for this
// Definitely need a way to handle race conditions of having multiple simulatenous requests for reservations   

// Global manager? Oor a type of map manager
// States for bots:
// Idle/Stopped
// Free Roam - consider a hierarchical state machine
// Path Planning - this is a separate system that will be used to plan paths for the robots
// Path Following - this is a separate system that will be used to follow paths for the robots
// Charging

// The main draw of this project is the map manager - what allows the path edge reservations
// Howo do we make this atomic? How do we handle multiple recurring requests?
// Perhaps we also need some type of exposed API to call the robots and request paths - Or probably use BEVY ECS for this

// I want this to be a traffic simulator
-  [ ] some way to define a map in a JSON or yaml file? Maybe some other serialization tech
- graphically generate the track
- define lanes
- one way or 2 ways streets
- junctions
- homes
- charging/gas stations

Should I just give up on this multi threaded approahc and use the bevy and built in behaviour