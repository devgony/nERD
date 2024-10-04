mod util;

use cursive::view::scroll::layout;
use cursive::view::{Nameable, Resizable};
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextArea, TextView,
};
use cursive::{Cursive, CursiveExt};
use gluesql_core::data::Schema;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
// use util::{draw_entity, merge_horizontal};

fn generate_erd(schemas: Vec<Schema>) -> String {
    todo!()
    // schemas
    //     .into_iter()
    //     .map(|schema| draw_entity(schema))
    //     .fold("".to_string(), |acc, cur| merge_horizontal(&acc, &cur))
    //     .to_owned()
}

fn sync(s: &mut Cursive) {
    if let Some(Some(sqls)) = s.call_on_name("sql_view", |view: &mut TextArea| {
        Some(view.get_content().to_string())
    }) {
        let schemas = sqls
            .split(";")
            .filter_map(|sql| Schema::from_ddl(sql).ok())
            .collect::<Vec<_>>();

        /*
        ┌───────┐       ┌──────┐
        │ Table1│       │Table2│
        ├───────┤       ├──────┤
        │ Col1  ├─────┐ │Col1  │
        │ Col2  │     └─►Col2  │
        │       │       │      │
        └───────┘       └──────┘
        */

        let diagram = generate_erd(schemas);

        // draw diagram by ascii art
        // let diagram = schemas.iter().fold("", |acc, schema| {
        //     let table = schema.table_name;
        //     // let columns = schema
        //     //     .column_defs
        //     //     .iter()
        //     //     .fold(String::new(), |acc, column| {
        //     //         acc + &format!("{}: {}, ", column.name, column.data_type)
        //     //     });
        //     format!("{}: {:?}", table, columns)
        // });

        if let Some(mut erd_view) = s.find_name::<TextView>("erd_view") {
            erd_view.set_content(diagram);
        }

        // Add a view for each schema.
        // for schema in schemas {
        //     // let select = SelectView::<String>::new()
        //     //     .with_name(schema.table_name)
        //     //     .fixed_size((10, 5));
        //     // layout.add_child(Dialog::around(EditView::new().content(schema.table_name)));
        // }

        // Add the grid to the Cursive root.
        // s.add_layer(layout);
    }
}

fn main() {
    let mut siv = Cursive::default();
    let content = "CREATE TABLE t1 (a int, b int);\nCREATE TABLE t2 (c int, d int);";
    let sql_text_area = TextArea::new()
        .content(content)
        .with_name("sql_view")
        .fixed_size((40, 10));
    let sql_view = Dialog::around(
        LinearLayout::vertical()
            .child(sql_text_area)
            .child(Button::new("Sync", sync)),
    )
    .title("SQL View");

    let diagram = "";
    let erd_view = TextView::new(diagram)
        .with_name("erd_view")
        .fixed_size((100, 100));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(sql_view)
                .child(DummyView.fixed_width(1))
                .child(Dialog::around(erd_view).title("ERD View")),
        )
        .title("nERD"),
    );

    // let cb_sink = siv.cb_sink().clone();
    // std::thread::spawn(move || {
    //     cb_sink.send(Box::new(|s: &mut Cursive| sync(s))).unwrap();
    // });

    siv.run();
}

// use layout::core::base::Orientation;
// use layout::core::geometry::Point;
// use layout::core::style::*;
// use layout::core::utils::save_to_file;
// use layout::gv::{dump_ast, GraphBuilder};
// use layout::std_shapes::shapes::*;
// use layout::topo::layout::VisualGraph;
// use layout::{backends::svg::SVGWriter, gv};
// use std::fs;
// use svgbob::Settings;

// fn main() {
//     let contents = "digraph { a -> b [label=\"foo\"]; }";
//     let mut parser = gv::DotParser::new(&contents);
//     let tree = parser.process();
//     match tree {
//         Result::Err(err) => {
//             // parser.print_error();
//             // log::error!("Error: {}", err);
//         }
//         Result::Ok(g) => {
//             if false {
//                 gv::dump_ast(&g);
//             }
//             let mut gb = GraphBuilder::new();
//             gb.visit_graph(&g);
//             let mut vg = gb.get();

//             let mut cli = CLIOptions::new();
//             // cli.debug_mode = matches.get_flag("d");
//             // cli.disable_opt = matches.get_flag("no-optz");
//             // cli.disable_layout = matches.get_flag("no-layout");
//             cli.output_path = String::from("./out.svg");

//             let svg = generate_svg(&mut vg, cli);

//             convert_to_ascii(svg);
//         }
//     }
// }

// fn convert_to_ascii(svg: String) {}

// fn generate_svg(graph: &mut VisualGraph, options: CLIOptions) -> String {
//     let mut svg = SVGWriter::new();
//     graph.do_it(
//         options.debug_mode,
//         options.disable_opt,
//         options.disable_layout,
//         &mut svg,
//     );

//     svg.finalize()
//     // let res = save_to_file(&options.output_path, &content);
//     // if let Result::Err(err) = res {
//     //     // log::error!("Could not write the file {}", options.output_path);
//     //     // log::error!("Error {}", err);
//     //     return;
//     // }
//     // // log::info!("Wrote {}", options.output_path);
// }

// struct CLIOptions {
//     disable_opt: bool,
//     disable_layout: bool,
//     output_path: String,
//     debug_mode: bool,
// }

// impl CLIOptions {
//     pub fn new() -> Self {
//         Self {
//             disable_opt: false,
//             disable_layout: false,
//             output_path: String::new(),
//             debug_mode: false,
//         }
//     }
// }
