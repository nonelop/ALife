use std::collections::HashMap;

use kiss3d::scene::SceneNode3d;

use crate::physics::body::PhysicsBody;
use crate::visual::shape_to_node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

pub struct World {
    body_id_counter: usize,
    node_id_counter: usize,
    body_id_map: HashMap<BodyId, Box<dyn PhysicsBody>>,
    node_id_map: HashMap<NodeId, SceneNode3d>,
    node_id_body_id_map: HashMap<NodeId, BodyId>,
    body_id_node_id_map: HashMap<BodyId, NodeId>
}

impl World {
    pub fn new() -> Self {
        World { 
            body_id_counter: 0, 
            node_id_counter: 0, 
            body_id_map: HashMap::new(), 
            node_id_map: HashMap::new(), 
            node_id_body_id_map: HashMap::new(), 
            body_id_node_id_map: HashMap::new()
        }
    }
    pub fn spawn(&mut self, body: Box<dyn PhysicsBody>) -> BodyId {
        let node = shape_to_node(&body.get_shape());
        let id = self.body_id_counter;

        self.body_id_map.insert(BodyId(self.body_id_counter), body);
        self.node_id_map.insert(NodeId(self.node_id_counter), node.unwrap());

        self.node_id_body_id_map.insert(
            NodeId(self.node_id_counter), BodyId(self.body_id_counter)
        );
        self.body_id_node_id_map.insert(
            BodyId(self.body_id_counter), NodeId(self.node_id_counter)
        );

        self.body_id_counter += 1;
        self.node_id_counter += 1;

        BodyId(id)
    }

    pub fn get_mut_body(&mut self, id: BodyId) -> Option<&mut dyn PhysicsBody> {
        match self.body_id_map.get_mut(&id) {
            Some(body) => Some(&mut **body),
            None => None
        }
    }

    pub fn get_many_mut_body(&mut self, a: BodyId, b: BodyId) 
    -> Option<(&mut dyn PhysicsBody, &mut dyn PhysicsBody)> {
        if a == b {
            return None;
        }

        // SAFETY:
        // выше мы проверили что a != b, то есть мы никогда не вернем две &mut на один обьект
        // также оператор ? после get_mut вернет None если обьекта с таким айди нету

        unsafe {
            let body_a: *mut Box<dyn PhysicsBody> = self.body_id_map.get_mut(&a)?;
            let body_b: *mut Box<dyn PhysicsBody> = self.body_id_map.get_mut(&b)?;

            Some((&mut **body_a, &mut **body_b))
        }
    }

    pub fn get_node(&self, id: BodyId) -> Option<&SceneNode3d> {
        match self.body_id_node_id_map.get(&id) {
            Some(node_id) => {
                match self.node_id_map.get(node_id) {
                    Some(node) => Some(node),
                    None => None
                }
            }
            None => None
        }
    }

    pub fn get_mut_node(&mut self, id: BodyId) -> Option<&mut SceneNode3d> {
        match self.body_id_node_id_map.get(&id) {
            Some(node_id) => {
                match self.node_id_map.get_mut(node_id) {
                    Some(node) => Some(node),
                    None => None
                }
            }
            None => None
        }
    }
}