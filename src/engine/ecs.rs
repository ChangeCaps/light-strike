use super::types::*;
use std::sync::{Arc, Mutex};


macro_rules! impl_getter {
    ($func_name:ident) => {
        macro_rules! $func_name {
            ($ecs:expr, $id:expr) => {
                {
                    if !$ecs.valid_id($id) {
                        panic!();
                    } 

                    $ecs.$func_name[$id.index()].lock().unwrap().as_mut().unwrap()
                }
            };
        }
    }
}

impl_getter!(position);
impl_getter!(rotation);
impl_getter!(light);
impl_getter!(velocity);

#[derive(Clone, Copy)]
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


pub struct ECS {
    pub polygon: Vec<Arc<Mutex<Option<Vec<Vector2>>>>>,
    pub position: Vec<Arc<Mutex<Option<Vector2>>>>,
    pub rotation: Vec<Arc<Mutex<Option<f32>>>>,
    pub light: Vec<Arc<Mutex<Option<f32>>>>,
    pub velocity: Vec<Arc<Mutex<Option<Vector2>>>>,
    gen: Vec<Gen>,
    last: Option<usize>,
    next_gen: u64,
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
        }
    }  

    pub fn push(&mut self, position: Option<Vector2>, 
        rotation: Option<f32>, 
        light: Option<f32>, 
        velocity: Option<Vector2>, 
        polygon: Option<Vec<Vector2>>) -> ID 
        {
        if let Some(last) = self.last {
            self.polygon[last] = Arc::new(Mutex::new(polygon));
            self.position[last] = Arc::new(Mutex::new(position));
            self.rotation[last] = Arc::new(Mutex::new(rotation));
            self.light[last] = Arc::new(Mutex::new(light));
            self.velocity[last] = Arc::new(Mutex::new(velocity));

            self.last = self.gen[last].unwrap_none();

            self.gen[last] = Gen::Some(self.next_gen);

            return ID::new(last, self.next_gen);       
        }

        let index = self.gen.len();

        self.gen.push(Gen::Some(self.next_gen));
        self.polygon.push(Arc::new(Mutex::new(polygon)));
        self.position.push(Arc::new(Mutex::new(position)));
        self.rotation.push(Arc::new(Mutex::new(rotation)));
        self.light.push(Arc::new(Mutex::new(light)));
        self.velocity.push(Arc::new(Mutex::new(velocity)));

        return ID::new(index, self.next_gen);
    }

    pub fn valid_id(&self, id: ID) -> bool {
        if let Gen::Some(some) = self.gen[id.index] {
            return some == id.gen;
        }

        return false;
    }
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