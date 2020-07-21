extern crate cursive;

pub fn f3_view_file(siv:&mut cursive::Cursive)
{
    siv.add_layer(cursive::views::TextView::new("[ F3 View ]"));
}
