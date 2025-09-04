// Example showing how to draw foreign key relationship lines between tables
// Similar to the test in finder.rs

fn main() {
    println!("Foreign Key Relationship Visualization");
    println!("======================================\n");
    
    // Example 1: Simple one-to-many relationship
    let one_to_many = r#"
┌──────────┐                  ┌──────────┐
│Department│                  │Employee  │
├──────────┤                  ├──────────┤
│id        │←─────────────────┤dept_id   │
│name      │                  │name      │
│location  │                  │salary    │
└──────────┘                  └──────────┘
"#;
    
    println!("One-to-Many Relationship:");
    println!("{}", one_to_many);
    
    // Example 2: Many-to-many relationship with junction table
    let many_to_many = r#"
┌──────────┐   ┌──────────┐   ┌──────────┐
│Students  │   │Enrollment│   │Courses   │
├──────────┤   ├──────────┤   ├──────────┤
│id        │←──┤student_id│   │id        │
│name      │   │course_id ├──→│name      │
│email     │   │grade     │   │credits   │
└──────────┘   └──────────┘   └──────────┘
"#;
    
    println!("\nMany-to-Many Relationship:");
    println!("{}", many_to_many);
    
    // Example 3: Complex relationships with curved lines
    let complex_relationships = r#"
┌──────────┐                     ┌──────────┐
│Author    │                     │Publisher │
├──────────┤   ┌──────────┐     ├──────────┤
│id        │←──┤author_id │  ┌─→│id        │
│name      │   │publisher ├──┘  │name      │
│bio       │   │title     │     │address   │
└──────────┘   │isbn      │     └──────────┘
               │pages     │
               └──────────┘
               Book Table
"#;
    
    println!("\nComplex Relationships:");
    println!("{}", complex_relationships);
    
    // Example 4: Self-referencing relationship
    let self_reference = r#"
┌────────────┐
│Employee    │
├────────────┤
│id          │←┐
│name        │ │
│manager_id  ├─┘ (self-reference)
│department  │
└────────────┘
"#;
    
    println!("\nSelf-Referencing Relationship:");
    println!("{}", self_reference);
    
    // Example 5: Multiple foreign keys from one table
    let multiple_fks = r#"
┌──────────┐                  ┌──────────┐
│Users     │                  │Messages  │
├──────────┤                  ├──────────┤
│id        │←─────────────────┤sender_id │
│username  │                  │receiver  ├─┐
│email     │←─────────────────┤content   │ │
└──────────┘                  │timestamp │ │
                              └──────────┘ │
                                          │
                                          └─┘
"#;
    
    println!("\nMultiple Foreign Keys:");
    println!("{}", multiple_fks);
    
    // Example with connection symbols like in finder.rs test
    demonstrate_connection_symbols();
}

fn demonstrate_connection_symbols() {
    println!("\n\nConnection Symbols Used in Diagrams:");
    println!("====================================\n");
    
    println!("Box Drawing:");
    println!("┌─────────┐  Top corners: ┌ ┐");
    println!("│ Content │  Sides: │");  
    println!("├─────────┤  Middle dividers: ├ ┤");
    println!("│ More    │");
    println!("└─────────┘  Bottom corners: └ ┘");
    
    println!("\nConnection Lines:");
    println!("Horizontal: ─────────");
    println!("Vertical:   │");
    println!("            │");
    
    println!("\nCorner Connections:");
    println!("┌─── Top-left to right");
    println!("└─── Bottom-left to right");
    println!("───┐ Left to top-right");
    println!("───┘ Left to bottom-right");
    
    println!("\nT-Junctions:");
    println!("├─── Left T-junction");
    println!("──┤  Right T-junction");
    println!("┬    Top T-junction");
    println!("┴    Bottom T-junction");
    
    println!("\nArrows (for foreign keys):");
    println!("→ Right arrow");
    println!("← Left arrow");
    println!("↑ Up arrow");
    println!("↓ Down arrow");
}