// icon.rs
use adw::prelude::*;
use glib::Object;
use gtk::gdk;
use gtk::subclass::prelude::*;
use gtk::{self, glib, CompositeTemplate};

mod imp {
    use super::*;
    use gtk::{glib, Image};

    #[derive(Default, CompositeTemplate)]
    #[template(file = "src/window/icon.blp")]
    pub struct IconButton {
        #[template_child]
        pub image: TemplateChild<Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IconButton {
        const NAME: &'static str = "IconButton";
        type Type = super::IconButton;
        type ParentType = gtk::Button;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for IconButton {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.connect_clicked(move |btn| {
                if let Some(icon_name) = btn.get_icon_name() {
                    let display = gdk::Display::default().unwrap();
                    let clipboard = display.clipboard();
                    clipboard.set_text(&icon_name);
                }
            });
        }
    }

    impl WidgetImpl for IconButton {}
    impl ButtonImpl for IconButton {}
}

glib::wrapper! {
    pub struct IconButton(ObjectSubclass<imp::IconButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl IconButton {
    pub fn new(icon_name: &str) -> Self {
        let obj: Self = Object::builder().build();
        obj.set_icon_name(icon_name);
        obj
    }

    pub fn set_icon_name(&self, icon_name: &str) {
        let imp = self.imp();
        imp.image.set_icon_name(Some(icon_name));
    }

    pub fn get_icon_name(&self) -> Option<String> {
        self.imp().image.icon_name().map(|gstr| gstr.to_string())
    }
}
