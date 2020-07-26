// Crate Dependencies ---------------------------------------------------------
// ----------------------------------------------------------------------------
extern crate cursive;
extern crate cursive_table_view;
//extern crate rand;

// STD Dependencies -----------------------------------------------------------
// ----------------------------------------------------------------------------
use std::cmp::Ordering;

// External Dependencies ------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, ResizedView};
use cursive::Cursive;
//use rand::Rng;

use cursive::menu::*;
// Modules --------------------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive_table_view::{TableView, TableViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn 
{
    Name,
    Count,
    Rate,
}

#[derive(Clone, Debug)]
struct Foo
 {
    up_dir: bool,
    name: String,
    count: String,
    rate: String,
}

impl TableViewItem<BasicColumn> for Foo {
    fn to_column(&self, column: BasicColumn) -> String {
        match column 
        {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Count => format!("{}", self.count),
            BasicColumn::Rate => format!("{}", self.rate),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        if self.up_dir || other.up_dir
        {
            return Ordering::Greater;
        }
        match column 
        {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Count => self.count.cmp(&other.count),
            BasicColumn::Rate => self.rate.cmp(&other.rate),
        }
    }
}
//

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
// one possible implementation of walking a directory only visiting files
fn visit_dirs_original(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("{:?}",path);
            /*
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
            */
        }
    }
    Ok(())
}



// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, items: &mut Vec<DirEntry>) -> io::Result<()> 
{
    if dir.is_dir() {
        
        for entry in fs::read_dir(dir)? {
            
            let entry = entry?;
            
          //  let path = entry.path();
          //  println!("{:?}",path);
            
            items.push(entry);
            /*
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
            */
        }
    }
    Ok(())
}

//
fn create_menu(siv:&mut Cursive)
{
    //
siv.menubar()
.add_subtree("File",
     MenuTree::new()
         .leaf("New", |s| s.add_layer(Dialog::info("New file!")))
         .subtree("Recent", MenuTree::new().with(|tree| {
             for i in 1..100 {
                 tree.add_leaf(format!("Item {}", i), |_| ())
             }
         }))
         .delimiter()
         .with(|tree| {
             for i in 1..10 {
                 tree.add_leaf(format!("Option {}", i), |_| ());
             }
         })
         .delimiter()
         .leaf("Quit", |s| s.quit()))
.add_subtree("Help",
     MenuTree::new()
         .subtree("Help",
                  MenuTree::new()
                      .leaf("General", |s| {
                          s.add_layer(Dialog::info("Help message!"))
                      })
                      .leaf("Online", |s| {
                          s.add_layer(Dialog::info("Online help?"))
                      }))
         .leaf("About",
               |s| s.add_layer(Dialog::info("Cursive v0.0.0"))));

siv.add_global_callback(cursive::event::Key::Esc, |s| s.select_menubar());
siv.set_autohide_menu(false);
//
}

mod function_keys;
use function_keys as fk;
fn main() {

    //let mut items: Vec<DirEntry> = Vec::new();
    //visit_dirs(Path::new("."), &mut items);
    
    let mut siv = cursive::default();
    create_menu(&mut siv);
    let mut horizontalLayout_MainPanels = LinearLayout::new(Orientation::Horizontal);
    horizontalLayout_MainPanels.add_child(ResizedView::with_full_screen(Dialog::around(create_table()).title("A title")));
    //horizontalLayout_MainPanels.add_child(ResizedView::with_fixed_size((0, 0), DummyView));
    horizontalLayout_MainPanels.add_child(ResizedView::with_full_screen(Dialog::around(create_table()).title("B title")));

    let mut horizontal_layout_functions = LinearLayout::new(Orientation::Horizontal);
    horizontal_layout_functions.add_child(ResizedView::with_full_width( cursive::views::TextView::new("[ F3 View ]").h_align(HAlign::Center)));
    siv.add_global_callback(cursive::event::Key::F3, fk::f3_view_file);
    horizontal_layout_functions.add_child(ResizedView::with_full_width( cursive::views::TextView::new("[ F4 Edit ]").h_align(HAlign::Center)));
    siv.add_global_callback(cursive::event::Key::F4, fk::f4_edit_file);
    horizontal_layout_functions.add_child(ResizedView::with_full_width( cursive::views::TextView::new("[ F5 Copy ]").h_align(HAlign::Center)));
    siv.add_global_callback(cursive::event::Key::F4, fk::f5_copy);
    horizontal_layout_functions.add_child(ResizedView::with_full_width( cursive::views::TextView::new("[ F6 Move ]").h_align(HAlign::Center)));
    siv.add_global_callback(cursive::event::Key::F4, fk::f6_move);
    horizontal_layout_functions.add_child(ResizedView::with_full_width( cursive::views::TextView::new("[ F7 Move ]").h_align(HAlign::Center)));
    siv.add_global_callback(cursive::event::Key::F4, fk::f7_mkdir);
    horizontal_layout_functions.add_child(ResizedView::with_full_width( cursive::views::TextView::new("[ F8 Move ]").h_align(HAlign::Center)));
    siv.add_global_callback(cursive::event::Key::F4, fk::f8_delete);

    let mut verticalLayout_MainPanels_Functions = LinearLayout::new(Orientation::Vertical);
    verticalLayout_MainPanels_Functions.add_child(horizontalLayout_MainPanels);
    verticalLayout_MainPanels_Functions.add_child(horizontal_layout_functions);
    //layout.required_size(cursive::XY{x:100,y:1});
    //siv.add_fullscreen_layer(ResizedView::with_full_screen(Dialog::around(layout).title("Table View Demo")));
    siv.add_fullscreen_layer(ResizedView::with_full_screen(verticalLayout_MainPanels_Functions));
    //siv.add_layer(ResizedView::with_full_screen;
    //siv.add_layer(layout);
   
    siv.run();
}
use std::time::{Duration, SystemTime};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
fn system_time_to_date_time(t: SystemTime) 
//-> DateTime<Utc> 
-> String
{
    let (sec, nsec) = match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };

    Utc.timestamp(sec, nsec).format("%a %b %e %Y %T ").to_string()
}

fn create_table() -> TableView<Foo, BasicColumn> {
    let mut items = Vec::new();
    //let mut rng = rand::thread_rng();
    let mut dir_entries: Vec<DirEntry> = Vec::new();
    match visit_dirs(Path::new("."), &mut dir_entries)
    {
        Ok(o) => println!(),
        Err(e) => println!()
    }
    items.push(Foo 
        {
        up_dir: true,
        name:  String::from(".."),
        count: String::from("UP DIR"),
        rate: String::from(""),
    });
    for dir_entry in dir_entries.iter()
    {
        let meta_data = fs::metadata(dir_entry.path());
        let res  = meta_data.unwrap();
        let file_size = res.len();
        let last_modified = res.modified();
        let date_time = system_time_to_date_time(last_modified.unwrap());
        items.push(Foo
             {
                up_dir: false,
                name: format!("{}", dir_entry.path().to_str().unwrap()),
                count: file_size.to_string(),
                rate: format!("{}", date_time.to_string()),
        });
    }
    /*
    for i in 0..50 {
        items.push(Foo {
            name: format!("Name {}", i),
            count: rng.gen_range(0, 255),
            rate: rng.gen_range(0, 255),
        });
    }
    */
    TableView::<Foo, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width_percent(40))
        .column(BasicColumn::Count, "Size", |c| c.align(HAlign::Center))
        .column(BasicColumn::Rate, "Last Modified", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(40)
        })
        .items(items)
        //.items(dir_entries)
}
