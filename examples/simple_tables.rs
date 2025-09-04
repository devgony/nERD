// Simple ASCII table drawing examples
// This example shows static ASCII art diagrams

fn main() {
    // Example 1: Simple two-table diagram
    println!("Example 1: Simple Two Tables");
    println!("=============================\n");
    
    let simple_diagram = r#"
┌──────────┐   ┌──────────┐
│Users     │   │Posts     │
├──────────┤   ├──────────┤
│id        │   │id        │
│name      │   │user_id   │
│email     │   │title     │
└──────────┘   │content   │
               └──────────┘
"#;
    
    println!("{}", simple_diagram);
    
    // Example 2: Tables with relationships
    println!("\nExample 2: Tables with Foreign Key Relationship");
    println!("================================================\n");
    
    let relationship_diagram = r#"
┌──────────┐                  ┌──────────┐
│Orders    │                  │Customers │
├──────────┤                  ├──────────┤
│id        ├─────────────────→│id        │
│cust_id   │                  │name      │
│total     │                  │email     │
│date      │                  └──────────┘
└──────────┘
"#;
    
    println!("{}", relationship_diagram);
    
    // Example 3: Complex schema with multiple relationships
    println!("\nExample 3: Complex Database Schema");
    println!("===================================\n");
    
    let complex_diagram = r#"
┌──────────┐   ┌──────────┐   ┌──────────┐
│Products  │   │Orders    │   │Customers │
├──────────┤   ├──────────┤   ├──────────┤
│id        │←──┤product_id│   │id        │
│name      │   │customer  ├──→│name      │
│price     │   │quantity  │   │email     │
│stock     │   │date      │   │address   │
└──────────┘   └──────────┘   └──────────┘
"#;
    
    println!("{}", complex_diagram);
}