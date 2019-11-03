use super::types::*;
use std::sync::{Arc, Mutex};


macro_rules! impl_getter {
    ($func_name:ident, $return_type:ty) => {
        pub fn $func_name(&mut self, id: ID) -> Option<&mut $return_type> {
            if self.valid_id(id) {
                return self.$func_name[id.index].as_mut();
            }

            panic!("Invalid ID");
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Gen {
    Some(u64),
    None(Option<usize>),
} 

impl Gen {
    pub fn unwrap_some(self) -> u64 {
        if let Gen::Some(some) = self {
            return some;
        } else {
            panic!("The component contained None");
        }
    }

    pub fn unwrap_none(self) -> Option<usize> {
        if let Gen::None(none) = self {
            return none;
        } else {
            panic!("The component contained Some");
        }
    }
}

pub enum ECSError {
    InvalidID,
}

pub struct ECS {
    pub polygon: Vec<Option<Vec<Vector2>>>,
    pub position: Vec<Option<Vector2>>,
    pub rotation: Vec<Option<f32>>,
    pub light: Vec<Option<(f32, f32, f32, f32)>>,
    pub velocity: Vec<Option<Vector2>>,
    pub gen: Vec<Gen>,
    last: Option<usize>,
    next_gen: u64,
    len: usize,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            polygon: Vec::new(),
            position: Vec::new(),
            rotation: Vec::new(),
            light: Vec::new(),
            velocity: Vec::new(),
            gen: Vec::new(),
            last: None,
            next_gen: 0,
            len: 0,
        }
    }  

    pub fn push(&mut self, position: Option<Vector2>, 
        rotation: Option<f32>, 
        light: Option<(f32, f32, f32, f32)>, 
        velocity: Option<Vector2>, 
        polygon: Option<Vec<Vector2>>) -> ID 
        {
        if let Some(last) = self.last {
            self.polygon[last] = polygon;
            self.position[last] = position;
            self.rotation[last] = rotation;
            self.light[last] = light;
            self.velocity[last] = velocity;

            self.last = self.gen[last].unwrap_none();

            self.gen[last] = Gen::Some(self.next_gen);

            return ID::new(last, self.next_gen);       
        }

        let index = self.gen.len();

        self.gen.push(Gen::Some(self.next_gen));
        self.polygon.push(polygon);
        self.position.push(position);
        self.rotation.push(rotation);
        self.light.push(light);
        self.velocity.push(velocity);

        self.len += 1;

        return ID::new(index, self.next_gen);
    }

    pub fn remove(&mut self, id: ID) -> Result<(), ECSError> {
        if self.valid_id(id) {
            self.polygon[id.index] = None;
            self.position[id.index] = None;
            self.rotation[id.index] = None;
            self.light[id.index] = None;
            self.velocity[id.index] = None;

            if let Gen::Some(last) = self.gen[id.index] {
                if last >= self.next_gen {
                    self.next_gen += 1;
                }
            }

            self.gen[id.index] = Gen::None(self.last);
            self.last = Some(id.index);

            return Ok(());
        }

        Err(ECSError::InvalidID)
    }

    pub fn valid_id(&self, id: ID) -> bool {
        if let Gen::Some(some) = self.gen[id.index] {
            return some == id.gen;
        }

        return false;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    impl_getter!(position, Vector2);
    impl_getter!(rotation, f32);
    impl_getter!(light, (f32, f32, f32, f32));
    impl_getter!(velocity, Vector2);
    impl_getter!(polygon, Vec<Vector2>);
}


// An identification token used for accessing   
#[derive(Clone, Copy)]
pub struct ID {
    pub(in super::ecs) index: usize,
    pub(in super::ecs) gen: u64,
}

impl ID {
    pub(in super::ecs) fn new(index: usize, gen: u64) -> ID {
        ID {
            index,
            gen
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn gen(&self) -> u64 {
        self.gen
    }
}
