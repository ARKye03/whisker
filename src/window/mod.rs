mod icon;

mod imp {
    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use glib::subclass::InitializingObject;
    use gtk::subclass::prelude::*;
    use gtk::{self, CompositeTemplate, Entry, GridView, TemplateChild};
    use gtk::{glib, prelude::*};
    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/github/arkye03/whisker/window.ui")]
    pub struct Window {
        #[template_child]
        pub entry: TemplateChild<Entry>,
        #[template_child]
        pub grid_view: TemplateChild<GridView>,
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
        }
    }
    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}

use adw::Application;
use glib::Object;
use gtk::{
    gio, glib,
    prelude::{Cast, ListItemExt},
    subclass::prelude::ObjectSubclassIsExt,
};

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
        for icon_name in gtk_theme.icon_names() {
            string_list.append(&icon_name);
        }

        // Create selection model
        let selection_model = gtk::NoSelection::new(Some(string_list));

        // Setup factory
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_, list_item| {
            // Cast Object to ListItem
            let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
            let icon_btn = icon::IconButton::new("");
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
    pub fn filter_icons() {}
}
