use cursive::theme::{BaseColor, BorderStyle, Color, Palette, Theme};
use cursive::view::scroll::Scroller;
use cursive::view::{Nameable, Resizable, Selector};
use cursive::views::{
    DummyView, LinearLayout, NamedView, ResizedView, ScrollView, SelectView, TextView, ViewRef,
};
use cursive::{Cursive, CursiveRunnable, Vec2};

use crate::app::App;
use cursive::event::Event;
use cursive::event::Key::{Left, Right};
use std::rc::Rc;

pub fn refresh(siv: &mut Cursive, app: &App, select: &str) -> Result<(), String> {
    {
        let mut header_view: ViewRef<TextView> = match siv.find_name("header") {
            None => panic!("Header text view not found"),
            Some(v) => v,
        };
        header_view.set_content(app.get_workdir().to_str().unwrap_or(""));
    }

    let mut parent_view: ViewRef<SelectView> = match siv.find_name("parent") {
        None => panic!("Parent view not found"),
        Some(v) => v,
    };
    let mut selected_view: ViewRef<SelectView> = match siv.find_name("selected") {
        None => panic!("Selected view not found"),
        Some(v) => v,
    };
    let mut child_view: ViewRef<SelectView> = match siv.find_name("child") {
        None => panic!("Child view not found"),
        Some(v) => v,
    };

    parent_view.clear();
    selected_view.clear();
    child_view.clear();

    {
        let mut items = app.get_parent_dir_items()?;
        items.sort();
        let parent_name = app.get_parent_dir_name();
        let mut parent_selected_index: usize = 0;
        for (i, item) in items.iter().enumerate() {
            if item.eq(parent_name) {
                parent_selected_index = i;
            }
            parent_view.add_item_str(item);
        }
        if parent_selected_index > 0 {
            parent_view.set_selection(parent_selected_index);
            let mut scroll_view: ViewRef<ScrollView<ResizedView<NamedView<SelectView>>>> =
                match siv.find_name("parent_scroll") {
                    None => panic!("Parent scroll view not found"),
                    Some(v) => v,
                };
            scroll_view
                .get_scroller_mut()
                .scroll_to(Vec2::new(0, parent_selected_index));
        }
    }

    let mut items = app.get_workdir_items()?;
    if items.len() == 0 {
        return Ok(());
    }
    items.sort();
    siv.focus(&Selector::Name("selected"));

    {
        let mut selected_index: usize = 0;
        for (i, item) in items.iter().enumerate() {
            if item.eq(select) {
                selected_index = i;
            }
            selected_view.add_item_str(item);
        }
        if selected_index > 0 {
            selected_view.set_selection(selected_index);
            let mut scroll_view: ViewRef<ScrollView<ResizedView<NamedView<SelectView>>>> =
                match siv.find_name("selected_scroll") {
                    None => panic!("Selected scroll view not found"),
                    Some(v) => v,
                };
            scroll_view
                .get_scroller_mut()
                .scroll_to(Vec2::new(0, selected_index));
        }
    }

    let selected: String = match selected_view.selection() {
        None => "".into(),
        Some(item) => item.to_string(),
    };
    items = app.get_child_dir_items(&selected)?;
    items.sort();
    for item in items {
        child_view.add_item_str(item);
    }
    Ok(())
}

fn select_parent_dir(siv: &mut Cursive, app: &mut App) {
    let parent_dir_name = app.select_parent_dir();
    if !parent_dir_name.is_empty() {
        refresh(siv, app, &parent_dir_name);
    }
}

fn update_child_view(siv: &mut Cursive, app: &App, selected: &str) {
    let mut child_view: ViewRef<SelectView> = match siv.find_name("child") {
        None => panic!("Child view not found"),
        Some(v) => v,
    };
    child_view.clear();
    if selected.is_empty() {
        return;
    }
    let mut items = app
        .get_child_dir_items(selected)
        .unwrap_or_else(|_| Vec::new());
    items.sort();
    for item in items {
        child_view.add_item_str(item);
    }
}

fn select_child_dir(siv: &mut Cursive, app: &mut App) {
    let mut selected = String::new();
    {
        let selected_view: ViewRef<SelectView> = match siv.find_name("selected") {
            None => panic!("Selected view not found"),
            Some(v) => v,
        };
        selected = match selected_view.selection() {
            None => return,
            Some(selection) => selection.to_string(),
        };
    }
    if app.select_child_dir(&selected) {
        refresh(siv, app, "");
    }
}

pub fn initialise(siv: &mut CursiveRunnable) {
    let header = TextView::new("");
    let footer = TextView::new("");

    let parent = SelectView::<String>::new();
    let mut selected = SelectView::<String>::new();
    let child = SelectView::<String>::new();

    selected = selected.on_select(|s, item| match s.take_user_data::<App>() {
        Some(app) => {
            update_child_view(s, &app, item);
            s.set_user_data(app);
        }
        None => {}
    });

    let content = LinearLayout::horizontal()
        .child(
            ScrollView::new(parent.with_name("parent").max_width(16))
                .with_name("parent_scroll")
                .full_height(),
        )
        .child(DummyView)
        .child(
            ScrollView::new(selected.with_name("selected").full_width())
                .with_name("selected_scroll")
                .full_height(),
        )
        .child(DummyView)
        .child(
            ScrollView::new(child.with_name("child").full_width())
                .with_name("child_scroll")
                .full_height(),
        );

    let root = LinearLayout::vertical()
        .child(header.with_name("header"))
        .child(content.with_name("content").full_height())
        .child(footer.with_name("footer"));

    siv.add_fullscreen_layer(root.with_name("root").full_height());

    siv.set_on_pre_event(Event::Key(Left), |s| match s.take_user_data::<App>() {
        None => {}
        Some(mut app) => {
            select_parent_dir(s, &mut app);
            s.set_user_data(app);
        }
    });

    siv.set_on_pre_event(Event::Key(Right), |s| match s.take_user_data::<App>() {
        None => {}
        Some(mut app) => {
            select_child_dir(s, &mut app);
            s.set_user_data(app);
        }
    });
}

pub fn default_theme(siv: &mut CursiveRunnable) {
    let mut palette = Palette::default();
    palette.set_basic_color("Background", Color::TerminalDefault);
    palette.set_basic_color("Shadow", Color::TerminalDefault);
    palette.set_basic_color("View", Color::TerminalDefault);
    palette.set_basic_color("Primary", Color::Dark(BaseColor::Blue));
    palette.set_basic_color("Secondary", Color::Dark(BaseColor::Blue));
    palette.set_basic_color("Tertiary", Color::Dark(BaseColor::Blue));
    palette.set_basic_color("TitlePrimary", Color::Light(BaseColor::White));
    palette.set_basic_color("TitleSecondary", Color::Dark(BaseColor::White));
    palette.set_basic_color("Highlight", Color::Light(BaseColor::Blue));
    palette.set_basic_color("HighlightInactive", Color::Dark(BaseColor::Blue));
    palette.set_basic_color("HighlightText", Color::Dark(BaseColor::Black));

    let theme = Theme {
        shadow: false,
        borders: BorderStyle::None,
        palette,
    };

    siv.set_theme(theme);
}
