mod imp {
    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use glib::subclass::InitializingObject;
    use gtk::subclass::prelude::*;
    use gtk::{self, CompositeTemplate, Entry, TemplateChild};
    use gtk::{glib, prelude::*};
    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/github/arkye03/whisker/window.ui")]
    pub struct Window {
        #[template_child]
        pub entry: TemplateChild<Entry>,
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

            // Connect to "clicked" signal of `button`
            self.entry.connect_changed(|_| {
                println!("Entry changed");
            });
        }
    }
    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}

use adw::Application;
use glib::Object;
use gtk::{gio, glib};

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
}
