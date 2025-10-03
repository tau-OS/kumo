use std::rc::Rc;

use gtk::prelude::*;
use libhelium::prelude::*;

#[derive(Debug)]
pub struct App {
    pub icon: gio::Icon,
    pub name: String,
    pub keywords: Vec<glib::GString>,
    pub description: String,
    pub deskappinfo: gio::DesktopAppInfo,
}

// note: we probably don't want to list ALL applications at once,
// so we can use search() to search what we want
//
//

// todo: function to create an App from path to desktop file OR implicitly lookup app ID
// with DesktopAppInfo::new()
// or return list using ::search()

#[relm4::factory(pub)]
impl relm4::factory::FactoryComponent for App {
    type Widgets = AppWidgets;
    type Init = gio::DesktopAppInfo;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = relm4::gtk::FlowBox;

    view! {
        #[root]
        gtk::FlowBoxChild {
            libhelium::Button {
                set_size: libhelium::ButtonSize::Xlarge,
                set_is_iconic: true,
                set_tooltip_text: Some(&self.name),
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 4,
                    set_margin_top: 8,
                    set_margin_bottom: 4,
                    set_margin_start: 2,
                    set_margin_end: 2,
                    gtk::Image {
                        set_icon_size: gtk::IconSize::Large,
                        set_from_gicon: &self.icon,
                    },
                    gtk::Label {
                        set_margin_top: 2,
                        set_label: &self.name,
                        set_justify: gtk::Justification::Center,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::End,
                        set_ellipsize: gtk::pango::EllipsizeMode::Middle,
                        set_single_line_mode: true,
                        set_wrap: true,
                        set_wrap_mode: gtk::pango::WrapMode::WordChar,
                        set_natural_wrap_mode: gtk::NaturalWrapMode::Word,
                        set_max_width_chars: 7,
                    },
                },
                // ? https://gtk-rs.org/gtk-rs-core/git/docs/gio/prelude/trait.AppInfoExt.html#method.launch
                connect_clicked[deskappinfo = self.deskappinfo.clone()] => move |_btn| {
                    if let Err(err) = crate::util::launch_desktop(&deskappinfo) {
                        tracing::error!(?err, "cannot launch app");
                    }
                }
            },
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &relm4::factory::DynamicIndex,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        let icon = init
            .icon()
            .unwrap_or_else(|| gio::Icon::for_string("application-x-executable").unwrap());
        let name = init.name().to_string();
        let keywords = init.keywords();
        let description = init.description().unwrap_or_default().to_string();
        App {
            icon,
            name,
            keywords,
            description,
            deskappinfo: init,
        }
    }
}

#[derive(Debug)]
pub struct AppFactory(pub std::rc::Rc<relm4::factory::FactoryVecDeque<App>>);

impl Default for AppFactory {
    #[allow(clippy::needless_for_each)]
    fn default() -> Self {
        let mut appfactory = relm4::factory::FactoryVecDeque::builder()
            .launch(gtk::FlowBox::default())
            .detach();

        let mut guard = appfactory.guard();
        // gio::DesktopAppInfo::search("meow");
        gio::AppInfo::all()
            .into_iter()
            .filter_map(|appinfo| gio::DesktopAppInfo::new(&*appinfo.id().unwrap_or_default()))
            // .take(6)
            .for_each(|x| _ = guard.push_back(x));
        tracing::trace!("AppFactory initialized");
        drop(guard);
        Self(Rc::new(appfactory))
    }
}

impl std::ops::Deref for AppFactory {
    type Target = gtk::FlowBox;

    fn deref(&self) -> &Self::Target {
        self.0.widget()
    }
}
impl AsRef<gtk::FlowBox> for AppFactory {
    fn as_ref(&self) -> &gtk::FlowBox {
        self
    }
}
impl AsRef<gtk::Widget> for AppFactory {
    fn as_ref(&self) -> &gtk::Widget {
        self.0.widget().upcast_ref()
    }
}
