mod imp {
    use std::cell::RefCell;

    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use glib::subclass::InitializingObject;
    use gtk::glib::{self, clone};
    use gtk::subclass::prelude::*;
    use gtk::{self, CompositeTemplate, GridView, SearchEntry, TemplateChild};
    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(file = "src/window.blp")]
    pub struct Window {
        #[template_child]
        pub entry: TemplateChild<SearchEntry>,
        #[template_child]
        pub grid_view: TemplateChild<GridView>,
        pub original_icon_list: RefCell<Vec<String>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for Window {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
            // Get the outer window instance
            let obj = self.obj();
            self.entry.connect_search_changed(clone!(
                #[weak]
                obj,
                move |_entry| {
                    obj.filter_icons();
                }
            ));
        }
    }
    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}

use adw::Application;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use glib::Object;
use gtk::{
    gio, glib,
    prelude::{Cast, EditableExt, ListItemExt},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::icon::{self, IconButton};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }
    pub fn init_icons(&self) {
        let imp = self.imp();

        // Create model to hold icon names
        let gtk_theme = gtk::IconTheme::for_display(&gtk::gdk::Display::default().unwrap());
        let string_list = gtk::StringList::new(&[]);
        let mut icon_names = Vec::new();
        for icon_name in gtk_theme.icon_names() {
            string_list.append(&icon_name);
            icon_names.push(icon_name);
        }
        *imp.original_icon_list.borrow_mut() = icon_names
            .into_iter()
            .map(|gstring| gstring.to_string())
            .collect();

        // Create selection model
        let selection_model = gtk::NoSelection::new(Some(string_list));

        // Setup factory
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_, list_item| {
            // Cast Object to ListItem
            let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
            let icon_btn = IconButton::new("");
            list_item.set_child(Some(&icon_btn));
        });

        factory.connect_bind(move |_, list_item| {
            // First downcast the list_item to gtk::ListItem
            let list_item = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("The object should be a ListItem");

            let icon_btn = list_item
                .child()
                .expect("The list item should have a child")
                .downcast::<icon::IconButton>()
                .expect("The child should be an IconButton");

            let icon_name = list_item
                .item()
                .expect("The list item should have an item")
                .downcast::<gtk::StringObject>()
                .expect("The item should be a StringObject")
                .string();

            icon_btn.set_icon_name(&icon_name);
        });

        // Configure GridView
        imp.grid_view.set_model(Some(&selection_model));
        imp.grid_view.set_factory(Some(&factory));
    }

    pub fn filter_icons(&self) {
        let imp = self.imp();
        let search_text = imp.entry.get().text().to_string();
        let matcher = SkimMatcherV2::default();

        let filtered_icons: Vec<String> = imp
            .original_icon_list
            .borrow()
            .iter()
            .filter_map(|icon_name| {
                if matcher.fuzzy_match(icon_name, &search_text).is_some() {
                    Some(icon_name.clone())
                } else {
                    None
                }
            })
            .collect();

        let filtered_icon_refs: Vec<&str> = filtered_icons.iter().map(|s| s.as_str()).collect();
        let string_list = gtk::StringList::new(&filtered_icon_refs);
        let selection_model = gtk::NoSelection::new(Some(string_list));
        imp.grid_view.set_model(Some(&selection_model));
    }
}
