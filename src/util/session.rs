//! Systemd session management
use std::{path::Path, sync::OnceLock};

use gio::prelude::{AppInfoExt, AppLaunchContextExt};
use glib::VariantDict;
use stable_eyre::Result;
use zvariant::{OwnedObjectPath, Value};

use crate::{app::DBUS_SESSION, runtime, util::appid_from_desktop};

// maybe manage app list idk, maybe not
pub struct SessionManager {
    pub dbus: zbus::Connection,
    // pub systemd: Box<zbus_systemd::systemd1::ManagerProxy<'static>>,
}

pub static SESSION_MANAGER: OnceLock<SessionManager> = OnceLock::new();

pub(crate) fn init_session_manager() {
    SESSION_MANAGER.get_or_init(|| {
        let conn = DBUS_SESSION.wait().clone();

        SessionManager::new(conn)
    });
}

// pub fn appid_from_desktop(path: &str) -> Option<String> {
//     let path = Path::new(path);
//     path.file_stem().map(|s| s.to_string_lossy().to_string())
// }

pub fn systemd_unit_name(appid: &str) -> String {
    let id = ulid::Ulid::new();
    format!("app-{appid}-{id}")
}

// async fn adopt_launched_pid_to_systemd_scope(pid: i32, app_identifier: String) -> Result<()> {
//     let connection = zbus::Connection::session().await?;
//     let manager = zbus_systemd::systemd1::ManagerProxy::new(&connection).await?;
//     let unit = format!("{}.scope", systemd_unit_name(&app_identifier));

//     let _objp = adopt_scope(&manager, pid.try_into()?, &unit).await?;
//     println!("Successfully adopted PID {} to unit {}", pid, unit);

//     Ok(())
// }

#[derive(Debug)]
pub struct AdoptionRequest {
    pub pid: i32,
    pub app_identifier: String,
}

impl SessionManager {
    pub fn new(dbus: zbus::Connection) -> Self {
        Self { dbus }
    }

    pub fn async_wrap<F, Fut, T>(&self, func: F) -> tokio::task::JoinHandle<T>
    where
        F: FnOnce(&Self) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        // Clone the minimal state we need so we can move it into the spawned task.
        let me = Self {
            dbus: self.dbus.clone(),
        };
        runtime().spawn(async move { func(&me).await })
    }

    pub async fn adopt_app(&self, pid: u32, app_identifier: &str) -> Result<OwnedObjectPath> {
        let unit = format!("{}.scope", systemd_unit_name(&app_identifier));
        self.adopt_scope(pid, &unit).await
    }

    pub async fn adopt_scope(&self, pid: u32, unit_id: &str) -> Result<OwnedObjectPath> {
        let manager = zbus_systemd::systemd1::ManagerProxy::new(&self.dbus).await?;
        let mut pid_array = zbus::zvariant::Array::new(&zbus::zvariant::Signature::U32);
        pid_array.append(Value::U32(pid))?;
        // manager.exit();
        let res = manager
            .start_transient_unit(
                unit_id.into(),
                "replace".into(),
                vec![
                    ("PIDs".into(), pid_array.try_into()?),
                    (
                        "CollectMode".into(),
                        Value::Str("inactive-or-failed".into()).try_into()?,
                    ),
                    ("Slice".into(), Value::Str("app.slice".into()).try_into()?),
                ],
                Vec::new(),
            )
            .await?;
        Ok(res)
    }

    pub fn launch_desktop(&'static self, appinfo: &gio::DesktopAppInfo) -> Result<()> {
        let launch_ctx = gio::AppLaunchContext::new();
        let (sender, receiver) = async_channel::unbounded::<AdoptionRequest>();

        // wonder if we should move this into the struct itself...
        // message handling thingy so we can just pass msgs
        runtime().spawn(async move {
            tracing::info!("Spawning ephemeral adoption thread");
            while let Ok(request) = receiver.recv().await {
                tracing::info!(
                    "Processing adoption request for PID {} with app_id {}",
                    request.pid,
                    request.app_identifier
                );

                if let Err(e) = self
                    .adopt_app(request.pid as u32, &request.app_identifier)
                    .await
                {
                    tracing::error!("Failed to adopt PID to systemd scope: {}", e);
                }
            }
        });

        // Spawn a background task to process adoption requests. The closure passed to
        // async_wrap must accept a &SessionManager parameter, so provide one even if unused.
        //
        // let sm = self.clone();
        launch_ctx.connect_launched(glib::clone!(
            #[strong]
            sender,
            move |ctx, appinfo, v| {
                // println!("App launched: {:?}", appinfo.name());
                tracing::debug!(?ctx, full_context = ?v);
                let pid: Option<i32> = {
                    let vdict: VariantDict = v.get().unwrap();
                    vdict.lookup("pid").ok().flatten()
                };

                // we want to wrap it in systemd scope too
                if let Some(pid) = pid {
                    let appid = appinfo.id().unwrap_or_default();
                    if let Some(appid_clean) = appid_from_desktop(&appid) {
                        let request = AdoptionRequest {
                            pid,
                            app_identifier: appid_clean,
                        };

                        // // Send adoption request through channel
                        if let Err(e) = sender.try_send(request) {
                            tracing::error!("Failed to send adoption request: {}", e);
                        }

                        // refer to util::mod for old impl
                        // ok fair lets do that
                        tracing::error!("Could not get appid from '{appid}'");
                    }
                }
            }
        ));

        // actually launch here
        appinfo.launch_uris(&[], Some(&launch_ctx))?;

        Ok(())
    }
}
