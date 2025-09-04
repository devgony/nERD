// Example demonstrating the grid layout algorithm used in nERD
// Shows how tables are positioned in a grid pattern

fn main() {
    println!("Grid Layout Algorithm Demonstration");
    println!("===================================\n");
    
    // Simulate the layout algorithm from engine.rs
    let layout_size = 100;
    let margin = 1;
    let entity_width = 12;
    let gap = 3;
    
    println!("Layout Parameters:");
    println!("  Layout Size: {}", layout_size);
    println!("  Margin: {}", margin);
    println!("  Entity Width: {}", entity_width);
    println!("  Gap: {}\n", gap);
    
    // Calculate how many entities fit per row
    let available_width = layout_size - 2 * margin;
    let entities_per_row = available_width / (entity_width + gap);
    
    println!("Calculated:");
    println!("  Available Width: {}", available_width);
    println!("  Entities per Row: {}\n", entities_per_row);
    
    // Simulate positioning multiple tables
    let table_names = vec![
        "Users", "Posts", "Comments", "Tags", "Categories",
        "Products", "Orders", "Customers", "Suppliers", "Inventory",
        "Employees", "Departments", "Projects", "Tasks", "Reports"
    ];
    
    let mut positions = Vec::new();
    
    for (index, name) in table_names.iter().enumerate() {
        let col = index % entities_per_row;
        let row = index / entities_per_row;
        
        let x = margin + col * (entity_width + gap);
        let y = margin + row * 8; // Assuming average height of 8
        
        positions.push((*name, x, y));
        
        println!("Table {:12} -> Position ({:3}, {:3}) [Row {}, Col {}]", 
                 name, x, y, row, col);
    }
    
    // Draw ASCII visualization of the grid
    println!("\n\nGrid Visualization:");
    println!("===================\n");
    
    draw_grid_visualization(&positions);
    
    // Show what happens when tables have different heights
    println!("\n\nVariable Height Layout:");
    println!("======================\n");
    
    demonstrate_variable_heights();
}

fn draw_grid_visualization(positions: &Vec<(&str, usize, usize)>) {
    let mut grid = vec![vec![' '; 100]; 40];
    
    for (name, x, y) in positions {
        // Draw a simple box for each table
        if *y < grid.len() && *x + 12 < grid[0].len() {
            // Top line
            for i in 0..12 {
                if *x + i < grid[0].len() {
                    grid[*y][*x + i] = '─';
                }
            }
            grid[*y][*x] = '┌';
            grid[*y][*x + 11] = '┐';
            
            // Table name
            if *y + 1 < grid.len() {
                grid[*y + 1][*x] = '│';
                grid[*y + 1][*x + 11] = '│';
                
                let name_chars: Vec<char> = name.chars().collect();
                for (i, ch) in name_chars.iter().take(10).enumerate() {
                    grid[*y + 1][*x + 1 + i] = *ch;
                }
            }
            
            // Bottom line
            if *y + 2 < grid.len() {
                for i in 0..12 {
                    if *x + i < grid[0].len() {
                        grid[*y + 2][*x + i] = '─';
                    }
                }
                grid[*y + 2][*x] = '└';
                grid[*y + 2][*x + 11] = '┘';
            }
        }
    }
    
    // Print the grid
    for (i, row) in grid.iter().enumerate() {
        let line: String = row.iter().collect();
        if line.trim().len() > 0 {
            print!("{:2} ", i);
            println!("{}", line);
        }
    }
}

fn demonstrate_variable_heights() {
    // Show how the algorithm handles tables with different numbers of columns
    let tables_with_heights = vec![
        ("Small", 2),   // 2 columns
        ("Medium", 5),  // 5 columns
        ("Large", 8),   // 8 columns
        ("Tiny", 1),    // 1 column
        ("Big", 10),    // 10 columns
    ];
    
    println!("When tables have different heights, the next row starts");
    println!("after the tallest table in the previous row:\n");
    
    let mut current_row_max_height = 0;
    let mut current_y = 1;
    
    for (i, (name, columns)) in tables_with_heights.iter().enumerate() {
        let height = columns + 3; // header + separator + bottom
        let x = 1 + (i % 3) * 15; // 3 tables per row
        
        if i > 0 && i % 3 == 0 {
            // New row
            current_y += current_row_max_height + 3; // Add gap
            current_row_max_height = height;
        } else {
            current_row_max_height = current_row_max_height.max(height);
        }
        
        println!("  {} table ({} cols) at ({}, {}) - height: {}", 
                 name, columns, x, current_y, height);
        
        // Draw simple representation
        println!("    ┌──────────┐");
        println!("    │{:10}│", name);
        println!("    ├──────────┤");
        for j in 0..*columns {
            println!("    │col{}      │", j + 1);
        }
        println!("    └──────────┘\n");
    }
}