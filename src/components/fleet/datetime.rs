use relm4::prelude::*;

kurage::generate_component!(Datetime {
    lbl: gtk::Label,
    fmt: String
}:
    init[lbl](root, sender, model, widgets) {
        // TODO: obtain `fmt` from settings?

        gtk::glib::timeout_add(std::time::Duration::from_secs(1), move || {
            sender.input(Self::Input::Update);
            gtk::glib::ControlFlow::Continue
        });
    }
    update(self, message, _sender) {
        Update => {
            let now = chrono::Local::now();
            self.lbl.set_label(&now.format(&self.fmt).to_string());
        },
    } => {}

    gtk::Box {
        // we need the box otherwise lbl will not be recognised by rustc
        #[local_ref] lbl ->
        gtk::Label {
            set_label: "…",
        },
    },
);
