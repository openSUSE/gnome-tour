use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};

use crate::{config, widgets::Window};

fn copy_desktop_file(file: gio::File) {
    // Copy .desktop file to user-config-dir and set Hidden=true property
    let mut home_path = glib::user_config_dir().clone();
    let file_name = String::from("autostart/") + config::APP_ID + ".desktop";
    home_path.push(&file_name);
    let dst = gio::File::for_path(&home_path);

    // Copy the source file to user config
    if let Err(e) = file.copy(
        &dst,
        gio::FileCopyFlags::OVERWRITE,
        gio::Cancellable::NONE,
        None,
    ) {
        log::error!(
            "Error copying {:?} to {:?}: {}",
            &file.path(),
            &home_path,
            e
        );
    }

    // Append "Hidden=true" to the end of the file to disable for this user
    // https://specifications.freedesktop.org/autostart-spec/latest/#id-1.3.4.3
    match dst.append_to(gio::FileCreateFlags::NONE, gio::Cancellable::NONE) {
        Ok(stream) => {
            if let Err(e) = stream.write("Hidden=true\n".as_bytes(), gio::Cancellable::NONE) {
                log::error!("Error removing from autostart: {}", e);
            }
        }
        Err(e) => log::error!("Error removing from autostart: {}", e),
    }
}

fn remove_from_autostart() {
    // Remove from autostart by default if the desktop file is in autostart
    for d in glib::system_config_dirs() {
        let mut path = d.clone();
        let file_name = String::from("autostart/") + config::APP_ID + ".desktop";
        path.push(&file_name);
        let file = gio::File::for_path(&path);
        if file.query_exists(gio::Cancellable::NONE) {
            log::info!("Remove from xdg autostart {:?})", &path);
            copy_desktop_file(file);
        }
    }
}

mod imp {
    use std::cell::OnceCell;

    use glib::WeakRef;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Application {
        pub(super) window: OnceCell<WeakRef<Window>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type ParentType = adw::Application;
        type Type = super::Application;
    }

    impl ObjectImpl for Application {}
    impl ApplicationImpl for Application {
        fn activate(&self) {
            self.parent_activate();
            let application = self.obj();

            let window = Window::new(&application);
            application.add_window(&window);
            window.present();
            self.window.set(window.downgrade()).unwrap();
        }

        fn shutdown(&self) {
            remove_from_autostart();
            self.parent_shutdown();
        }

        fn startup(&self) {
            self.parent_startup();
            let application = self.obj();
            // Quit
            let quit = gio::ActionEntry::builder("quit")
                .activate(move |app: &Self::Type, _, _| app.quit())
                .build();
            application.add_action_entries([quit]);

            application.set_accels_for_action("app.quit", &["<Control>q"]);
            application.set_accels_for_action("win.skip-tour", &["Escape"]);
        }
    }
    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    pub fn run() -> glib::ExitCode {
        log::info!("GNOME Tour ({})", config::APP_ID);
        log::info!("Version: {} ({})", config::VERSION, config::PROFILE);
        log::info!("Datadir: {}", config::PKGDATADIR);
        Self::default().run()
    }
}

impl Default for Application {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", config::APP_ID)
            .property("resource-base-path", "/org/gnome/Tour")
            .build()
    }
}
