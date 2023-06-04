use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextArea};
use cursive::{Cursive, CursiveExt};

fn sync(s: &mut Cursive) {
    if let Some(Some(sql_text_area)) = s.call_on_name("sql_view", |view: &mut TextArea| {
        Some(view.get_content().to_string())
    }) {
        s.call_on_name("erd_view", |view: &mut SelectView<String>| {
            view.add_item_str(sql_text_area);
        });
    }
}

fn main() {
    let mut siv = Cursive::default();
    let sql_text_area = TextArea::new().with_name("sql_view").fixed_size((70, 30));
    let sql_view = Dialog::around(
        LinearLayout::vertical()
            .child(sql_text_area)
            .child(Button::new("Sync", sync)),
    )
    .title("SQL View");

    let erd_view = SelectView::<String>::new()
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

    siv.run();
}
