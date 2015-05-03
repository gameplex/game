#![allow(dead_code)]

use std::rc::{Rc, Weak};
use entity::{Vec3f64, Entity};

struct Address {
    x: u64,
    y: u64,
    z: u64,
}

struct Element {
    name: String,
}

pub enum ChunkOption {
    Local(Rc<LocalChunk>),
    Remote(Rc<RemoteChunk>)
}

pub enum ChildOption {
    Local(Rc<LocalChunk>),
    Remote(Rc<RemoteChunk>),
    None
}

pub enum ParentOption {
    Local(Weak<LocalChunk>),
    Remote(Rc<RemoteChunk>),
    None
}

pub struct LocalChunk {
    address: Address,
    center: Vec3f64,
    //children: [ChildOption; 8],
    //FIXME: remove these and uncomment above
    child0: ChildOption,
    child1: ChildOption,
    child2: ChildOption,
    child3: ChildOption,
    child4: ChildOption,
    child5: ChildOption,
    child6: ChildOption,
    child7: ChildOption,
    //END FIXME
    child_number: Option<u8>,
    entities: Vec<Rc<Entity>>,
    mass: f64,
    parent: ParentOption,
    scale: u8,
    structure: u64,
    summary: Rc<Element>,
}

pub struct RemoteChunk {
    connection: i32,
}

impl Address {
    fn from_string(address: String) -> Option<Address> {
        let mut xyz: (u64, u64, u64) = (1, 1, 1);
        let mut xyz_add: (u64, u64, u64);

        for (i, c) in address.chars().enumerate() {
            if i > 63 {
                break;
            }
            xyz_add =  match c.to_digit(8) {
                Some(0) => (0, 0, 0),
                Some(1) => (0, 0, 1),
                Some(2) => (0, 1, 0),
                Some(3) => (0, 1, 1),
                Some(4) => (1, 0, 0),
                Some(5) => (1, 0, 1),
                Some(6) => (1, 1, 0),
                Some(7) => (1, 1, 1),
                _ => return None,
            };
            xyz = (xyz.0 << 1, xyz.1 << 1, xyz.2 << 1);
            xyz = (xyz.0 + xyz_add.0, xyz.1 + xyz_add.1, xyz.2 + xyz_add.2);
        }
        Some(Address {x: xyz.0, y: xyz.1, z: xyz.2})
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        String::from_str("")
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        (((self.x == other.x) && (self.y == other.y)) && (self.z == other.z))
    }
}


pub trait Chunk {
    fn get_child(&self, x: u8, y: u8, z: u8) -> ChildOption;
    fn tick(&self, time_delta: f64);
}

impl Chunk for LocalChunk {
    fn get_child(&self, x: u8, y: u8, z: u8) -> ChildOption {

        /* FIXME: remove
        let index = match (x, y, z) {
            (0, 0, 0) => Some(0),
            (0, 0, 1) => Some(1),
            (0, 1, 0) => Some(2),
            (0, 1, 1) => Some(3),
            (1, 0, 0) => Some(4),
            (1, 0, 1) => Some(5),
            (1, 1, 0) => Some(6),
            (1, 1, 1) => Some(7),
            _ => None,
        };

        if index == None {
            return ChildOption::None
        }

        match self.children[index] {
            ChildOption::Local(ref chunk) => ChildOption::Local(*chunk),
            ChildOption::Remote(ref chunk) => ChildOption::Remote(*chunk),
            ChildOption::None => ChildOption::None
        }*/
        ChildOption::None
    }

    fn tick(&self, time_delta: f64) {
    }
}

impl Chunk for RemoteChunk {
    fn get_child(&self, x: u8, y: u8, z: u8) -> ChildOption {
        ChildOption::None
    }
    fn tick (&self, time_delta: f64) {
    }
}

