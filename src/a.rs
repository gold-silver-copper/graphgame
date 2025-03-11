use std::collections::{HashMap, HashSet};

/// Unique identifier for a location
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocationID(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TravelDirection {
    North,
    South,
    West,
    East,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Up,
    Down,
}

/// Different types of connections between locations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionType {
    Path {
        mode: PathType,
        direction: TravelDirection,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathType {
    Road,
    Tunnel,
    River,
    Bridge,
    Portal,
}

/// Represents a connection between two locations
#[derive(Debug, Clone)]
pub struct Connection {
    pub destination: LocationID,
    pub connection_type: ConnectionType,
}

/// Represents a location in the game world
#[derive(Debug, Clone)]
pub struct Location {
    pub id: LocationID,
    pub name: String,
    pub connections: Vec<Connection>,
}

/// Manages all locations and their connections
pub struct LocationGraph {
    locations: HashMap<LocationID, String>, // Store names separately
    connections: HashMap<LocationID, Vec<Connection>>, // Store connections here
}

impl LocationGraph {
    /// Creates a new, empty graph
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
            connections: HashMap::new(),
        }
    }
    /// Adds a new location
    pub fn add_location(&mut self, id: LocationID, name: String) {
        self.locations.insert(id, name);
        self.connections.entry(id).or_insert_with(Vec::new);
    }

    /// Adds a connection between two locations
    pub fn add_connection(
        &mut self,
        from: LocationID,
        to: LocationID,
        connection_type: ConnectionType,
    ) {
        self.connections
            .entry(from)
            .or_insert_with(Vec::new)
            .push(Connection {
                destination: to,
                connection_type,
            });
    }

    /// Retrieves a location's connections
    pub fn get_connections(&self, id: LocationID) -> Option<&Vec<Connection>> {
        self.connections.get(&id)
    }
}
