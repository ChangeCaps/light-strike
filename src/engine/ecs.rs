use super::types::*;

macro_rules! impl_getter {
    ($func_name:ident, $func_name_mut:ident, $return_type:ident) => {
        fn $func_name(&self, id: ID) -> Option<&$return_type> {
            if self.valid_id(id) {
                if let Some(some) = &self.$func_name[id.index] { 
                    return Some(some);
                }
            }

            return None;
        }

        fn $func_name_mut(&mut self, id: ID) -> Option<&mut $return_type> {
            if self.valid_id(id) {
                if let Some(some) = &mut self.$func_name[id.index] { 
                    return Some(some);
                }
            }

            return None;        
        }
    }
}


#[derive(Clone, Copy)]
pub enum Component {
    Some(u64),
    None(Option<usize>),
} 

impl Component {
    pub fn unwrap_some(self) -> u64 {
        if let Component::Some(some) = self {
            return some;
        } else {
            panic!("The component contained None");
        }
    }

    pub fn unwrap_none(self) -> Option<usize> {
        if let Component::None(none) = self {
            return none;
        } else {
            panic!("The component contained Some");
        }
    }
}


pub struct ECS {
    pub position: Vec<Option<Vector2>>,
    pub rotation: Vec<Option<f32>>,
    gen: Vec<Component>,
    last: Option<usize>,
    next_gen: u64,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            position: Vec::new(),
            rotation: Vec::new(),
            gen: Vec::new(),
            last: None,
            next_gen: 0,
        }
    }  

    pub fn push(&mut self, position: Option<Vector2>, rotation: Option<f32>) -> ID {
        if let Some(last) = self.last {
            self.position[last] = position;
            self.rotation[last] = rotation;

            self.last = self.gen[last].unwrap_none();

            self.gen[last] = Component::Some(self.next_gen);

            return ID::new(last, self.next_gen);       
        }

        let index = self.gen.len();

        self.gen.push(Component::Some(self.next_gen));
        self.position.push(position);
        self.rotation.push(rotation);

        return ID::new(index, self.next_gen);
    }

    fn valid_id(&self, id: ID) -> bool {
        if let Component::Some(some) = self.gen[id.index] {
            return some == id.gen;
        }

        return false;
    }

    impl_getter!(position, position_mut, Vector2);
    impl_getter!(rotation, rotation_mut, f32);
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

