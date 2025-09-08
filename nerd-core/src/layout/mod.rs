use crate::models::{Entity, Position, Schema, Relationship};
use petgraph::{Graph, Undirected};
use std::collections::HashMap;
use std::f64::consts::PI;

pub struct LayoutEngine {
    width: f64,
    height: f64,
}

impl LayoutEngine {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn layout_entities(&self, schema: &mut Schema) {
        if schema.entities.is_empty() {
            return;
        }

        if schema.entities.len() == 1 {
            self.layout_single_entity(schema);
        } else {
            self.layout_force_directed(schema);
        }
    }

    fn layout_single_entity(&self, schema: &mut Schema) {
        if let Some((_, entity)) = schema.entities.iter_mut().next() {
            entity.position = Position {
                x: self.width / 2.0,
                y: self.height / 2.0,
            };
        }
    }

    fn layout_force_directed(&self, schema: &mut Schema) {
        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let mut node_indices = HashMap::new();
        
        for entity_name in schema.entities.keys() {
            let node_index = graph.add_node(entity_name.clone());
            node_indices.insert(entity_name.clone(), node_index);
        }
        
        for relationship in &schema.relationships {
            if let (Some(&from_idx), Some(&to_idx)) = (
                node_indices.get(&relationship.from_table),
                node_indices.get(&relationship.to_table),
            ) {
                graph.add_edge(from_idx, to_idx, ());
            }
        }
        
        let mut positions: HashMap<String, Position> = HashMap::new();
        
        self.initialize_positions(&mut positions, &schema.entities);
        
        for _ in 0..100 {
            self.apply_forces(&mut positions, &schema.entities, &schema.relationships);
        }
        
        for (entity_name, position) in positions {
            if let Some(entity) = schema.entities.get_mut(&entity_name) {
                entity.position = position;
            }
        }
    }

    fn initialize_positions(&self, positions: &mut HashMap<String, Position>, entities: &HashMap<String, Entity>) {
        let entity_count = entities.len() as f64;
        let radius = (self.width.min(self.height) / 4.0).min(200.0);
        let center_x = self.width / 2.0;
        let center_y = self.height / 2.0;

        for (i, entity_name) in entities.keys().enumerate() {
            let angle = 2.0 * PI * i as f64 / entity_count;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            
            positions.insert(entity_name.clone(), Position { x, y });
        }
    }

    fn apply_forces(
        &self,
        positions: &mut HashMap<String, Position>,
        entities: &HashMap<String, Entity>,
        relationships: &[Relationship],
    ) {
        let mut forces: HashMap<String, (f64, f64)> = HashMap::new();
        
        for entity_name in entities.keys() {
            forces.insert(entity_name.clone(), (0.0, 0.0));
        }
        
        self.apply_repulsion_forces(positions, &mut forces);
        
        self.apply_attraction_forces(positions, &mut forces, relationships);
        
        self.apply_forces_to_positions(positions, &forces);
        
        self.keep_within_bounds(positions);
    }

    fn apply_repulsion_forces(
        &self,
        positions: &HashMap<String, Position>,
        forces: &mut HashMap<String, (f64, f64)>,
    ) {
        let repulsion_strength = 5000.0;
        
        for (entity1, pos1) in positions {
            for (entity2, pos2) in positions {
                if entity1 == entity2 {
                    continue;
                }
                
                let dx = pos1.x - pos2.x;
                let dy = pos1.y - pos2.y;
                let distance = (dx * dx + dy * dy).sqrt().max(1.0);
                
                let force = repulsion_strength / (distance * distance);
                let fx = (dx / distance) * force;
                let fy = (dy / distance) * force;
                
                if let Some((curr_fx, curr_fy)) = forces.get_mut(entity1) {
                    *curr_fx += fx;
                    *curr_fy += fy;
                }
            }
        }
    }

    fn apply_attraction_forces(
        &self,
        positions: &HashMap<String, Position>,
        forces: &mut HashMap<String, (f64, f64)>,
        relationships: &[Relationship],
    ) {
        let attraction_strength = 100.0;
        let ideal_distance = 150.0;
        
        for relationship in relationships {
            if let (Some(pos1), Some(pos2)) = (
                positions.get(&relationship.from_table),
                positions.get(&relationship.to_table),
            ) {
                let dx = pos2.x - pos1.x;
                let dy = pos2.y - pos1.y;
                let distance = (dx * dx + dy * dy).sqrt().max(1.0);
                
                let force = attraction_strength * (distance - ideal_distance) / distance;
                let fx = (dx / distance) * force;
                let fy = (dy / distance) * force;
                
                if let Some((curr_fx, curr_fy)) = forces.get_mut(&relationship.from_table) {
                    *curr_fx += fx;
                    *curr_fy += fy;
                }
                
                if let Some((curr_fx, curr_fy)) = forces.get_mut(&relationship.to_table) {
                    *curr_fx -= fx;
                    *curr_fy -= fy;
                }
            }
        }
    }

    fn apply_forces_to_positions(
        &self,
        positions: &mut HashMap<String, Position>,
        forces: &HashMap<String, (f64, f64)>,
    ) {
        let damping = 0.1;
        let max_velocity = 10.0;
        
        for (entity_name, (fx, fy)) in forces {
            if let Some(position) = positions.get_mut(entity_name) {
                let vx = fx * damping;
                let vy = fy * damping;
                
                let velocity = (vx * vx + vy * vy).sqrt();
                if velocity > max_velocity {
                    let scale = max_velocity / velocity;
                    position.x += vx * scale;
                    position.y += vy * scale;
                } else {
                    position.x += vx;
                    position.y += vy;
                }
            }
        }
    }

    fn keep_within_bounds(&self, positions: &mut HashMap<String, Position>) {
        let margin = 50.0;
        
        for position in positions.values_mut() {
            position.x = position.x.clamp(margin, self.width - margin);
            position.y = position.y.clamp(margin, self.height - margin);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Column, RelationshipType, Dimensions};

    #[test]
    fn test_single_entity_layout() {
        let mut schema = Schema::new();
        let entity = Entity {
            name: "users".to_string(),
            columns: vec![
                Column {
                    name: "id".to_string(),
                    data_type: "INT".to_string(),
                    nullable: false,
                    is_primary_key: true,
                    is_foreign_key: false,
                    references: None,
                },
            ],
            position: Position::default(),
            dimensions: Dimensions { width: 20, height: 5 },
        };
        schema.entities.insert("users".to_string(), entity);

        let layout_engine = LayoutEngine::new(800.0, 600.0);
        layout_engine.layout_entities(&mut schema);

        let user_entity = schema.entities.get("users").unwrap();
        assert_eq!(user_entity.position.x, 400.0);
        assert_eq!(user_entity.position.y, 300.0);
    }

    #[test]
    fn test_multiple_entities_layout() {
        let mut schema = Schema::new();
        
        let users = Entity {
            name: "users".to_string(),
            columns: vec![],
            position: Position::default(),
            dimensions: Dimensions { width: 20, height: 5 },
        };
        
        let posts = Entity {
            name: "posts".to_string(),
            columns: vec![],
            position: Position::default(),
            dimensions: Dimensions { width: 20, height: 5 },
        };
        
        schema.entities.insert("users".to_string(), users);
        schema.entities.insert("posts".to_string(), posts);
        
        schema.relationships.push(Relationship {
            from_table: "posts".to_string(),
            from_column: "user_id".to_string(),
            to_table: "users".to_string(),
            to_column: "id".to_string(),
            relationship_type: RelationshipType::OneToMany,
        });

        let layout_engine = LayoutEngine::new(800.0, 600.0);
        layout_engine.layout_entities(&mut schema);

        let users_pos = &schema.entities.get("users").unwrap().position;
        let posts_pos = &schema.entities.get("posts").unwrap().position;
        
        assert!(users_pos.x != posts_pos.x || users_pos.y != posts_pos.y);
        assert!(users_pos.x >= 50.0 && users_pos.x <= 750.0);
        assert!(users_pos.y >= 50.0 && users_pos.y <= 550.0);
        assert!(posts_pos.x >= 50.0 && posts_pos.x <= 750.0);
        assert!(posts_pos.y >= 50.0 && posts_pos.y <= 550.0);
    }
}