use serde::{Deserialize, Serialize};
use zbus::{dbus_interface, dbus_proxy, zvariant::Type, SignalContext};

use crate::{NotifStackEvent, NOTIF_DESTROY_CHANS};

pub const DBUS_OBJECT_PATH: &str = "/org/freedesktop/Notifications";
pub const DBUS_INTERFACE: &str = "org.freedesktop.Notifications";
pub type NotificationHintsMap<'a> = std::collections::HashMap<&'a str, zbus::zvariant::Value<'a>>;

/// D-Bus server information.
///
/// Specifically, the server name, vendor, version, and spec version.
const SERVER_INFO: [&str; 4] = [
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_VERSION"),
    "1.2",
];

/// D-Bus server capabilities.
///
/// - `actions`: The server will provide the specified actions to the user.
/// - `body`: Supports body text.
const SERVER_CAPABILITIES: [&str; 2] = ["actions", "body"];

// use bitflags to define the urgency level
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Type,
)]
/// Notification Level
///
// Most notifications should be at `Normal`, but we probably want to implement different levels of urgency
// Especially now that it's kind of planned to add CAP support to tauOS
// CAP alerts should be given a hint of `Critical` urgency, since someone's life might be at stake
// See https://github.com/tau-OS/meta/issues/28
// long-term todo: ask FEMA for access to their CAP server. Don't know what to do for other countries though.
// Japan has... J-Alert? Then there's NERV (haha evangelion reference) for even more rapid response
// NERV's API should be easy to access, so we probably want to use that instead of CAP for Japan
pub enum Urgency {
    Low = 0,
    #[default]
    Normal = 1,
    Critical = 2,
}
/// Notification Position
// Honestly I don't know if we would need this, since it would go against Helium HIG
// All notifications should be at a specific corner, and not move around
// Will probably remove this in the future
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    x: i32,
    y: i32,
}

/// Hints for the notification.
///
/// See https://specifications.freedesktop.org/notification-spec/latest/ar01s08.html
/// for more information.
/*
    D-Bus Signature: a{sv} (map of string->any/variant)
    example hint, serialized directly to JSON:

    {
      "urgency": {
         "zvariant::Value::Signature": "y",
         "zvariant::Value::Value": 1
        },
      "sender-pid": {
        "zvariant::Value::Signature": "x",
        "zvariant::Value::Value": 1342130
      }
    }

    todo: find way to serialize this properly with zbus, or just... delete this struct and just check values on the fly?
    most implementations of notifications I see just check for the presence of a hint from the hashmap, and then use it if it's there
    But I feel like we want to do this in a more type-safe way, support everything in the spec, and return as Option<T>
    for supported hints, If one of the hints is not supported, we can just not include it in the NotificationHints struct

    But then, none of the Rust notification daemons use the `dbus` crate instead of `zbus`, so they have an entirely different (procedural) way of doing this.
    Only reason I'm writing lots of boilerplate and re-defining specs here is that I like my code declarative and type-safe

    - Cappy, 2024-01-02

*/
#[derive(Debug, Serialize, Deserialize, Type)]
#[zvariant(signature = "a{sv}")]
pub struct NotificationHints {
    #[serde(rename = "sender-pid")]
    pub sender_pid: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "action-icons")]
    pub action_icons: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "desktop-entry")]
    pub desktop_entry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image-path")]
    #[serde(alias = "image_path")]
    pub image_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sound-file")]
    #[serde(alias = "sound_file")]
    pub sound_file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sound-name")]
    pub sound_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<Urgency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suppress-sound")]
    pub suppress_sound: Option<bool>,

    // image-data: (iiibiiay)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "image-data")]
    pub image_data: Option<(i32, i32, i32, bool, i32, i32, Vec<u8>)>,
    // // serialize this from x, y
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(alias = "x", alias = "y")]
    // #[serde(flatten)]
    // pub position: Option<Position>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub x: Option<i32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub y: Option<i32>,
}

/// D-Bus proxy for the `org.freedesktop.Notifications` interface.
/// This is a client-side proxy for the `org.freedesktop.Notifications` D-Bus interface.
///
/// This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
///
/// Source: `bus/dbus.xml`.
#[dbus_proxy(interface = "org.freedesktop.Notifications", assume_defaults = true)]
trait Notifications {
    /// CloseNotification method
    fn close_notification(&self, id: u32) -> zbus::Result<()>;

    /// GetCapabilities method
    fn get_capabilities(&self) -> zbus::Result<Vec<String>>;

    /// GetServerInformation method
    fn get_server_information(&self) -> zbus::Result<(String, String, String, String)>;

    /// Notify method
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        // hints: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        hints: NotificationHintsMap<'_>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;

    /// ActionInvoked signal
    #[dbus_proxy(signal)]
    fn action_invoked(&self, id: u32, action_key: &str) -> zbus::Result<()>;

    /// NotificationClosed signal
    #[dbus_proxy(signal)]
    fn notification_closed(&self, id: u32, reason: u32) -> zbus::Result<()>;
}

// Let's implement a server interface based on what we have for this client proxy

/// D-Bus server implementation for the `org.freedesktop.Notifications` interface.
///
/// This one was written manually, but based on the client proxy generated by `zbus-xmlgen`.
///
// Since zbus only generates client proxies, we have to write the server implementation manually
// because one would have to implement the methods manually for their server anyway
// todo: Let's implement the actual GTK4 Layer Shell interface to show these notifications,
// and then push these notifications to the shell
// Kind of hard to do this when you're mixing async and sync code, but welp
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsServer;

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl NotificationsServer {
    /// Close notification
    ///
    /// Selects a notification ID to close (and remove) from the feed.
    /// Once a notification is closed, it is removed from the feed entirely.
    async fn close_notification(&self, id: u32) -> Result<(), zbus::fdo::Error> {
        tracing::info!(?id, "CloseNotification");
        Ok(())
    }

    /// Server capabilities
    ///
    /// This implementation is capable of inputting data... :P
    fn get_capabilities(&self) -> Vec<String> {
        SERVER_CAPABILITIES.into_iter().map(String::from).collect()
    }

    /// Returns D-Bus server information.
    ///
    /// We will be returning the actual binary information here. In case someone wants to know which
    /// notification daemon is running, they can just call this method.
    async fn get_server_information(
        &self,
    ) -> Result<(String, String, String, String), zbus::fdo::Error> {
        Ok((
            SERVER_INFO[0].to_string(),
            SERVER_INFO[1].to_string(),
            SERVER_INFO[2].to_string(),
            SERVER_INFO[3].to_string(),
        ))
    }

    /// Notify
    ///
    /// The main entrypoint of this entire program.
    ///
    /// This method gets called with a notification is sent from an application.
    /// The code below should push the notification to the GTK4 Layer Shell interface
    /// and then display the notification on the screen for the user to see.
    #[tracing::instrument(skip(self))]
    async fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: NotificationHintsMap<'_>,
        // hints: NotificationHints,
        // ! GDBus.Error:org.freedesktop.zbus.Error: D-Bus format does not support optional values
        // need a better way to serialize a{sv} to our struct with optional values?
        expire_timeout: i32,
    ) -> Result<u32, zbus::fdo::Error> {
        // let hints: NotificationHints =
        // serde_json::from_value(serde_json::to_value(hints).unwrap()).unwrap();
        tracing::info!("Notify");
        // let hints = serde_json::to_string(&hints).unwrap();
        tracing::debug!(?hints, "hints");

        // let hints = serde_json::to_string(&hints).unwrap();

        // let hints: NotificationHints = serde_json::from_str(&hints).unwrap();


        // serialize hints to a struct

        let channel = NOTIF_DESTROY_CHANS.clone();

        // turn this arc into tuple
        let (tx, rx) = (channel.0.clone(), channel.1.clone());

        // send the notification to the notification stack
        let notification = crate::widget::Notification {
            title: summary.to_string(),
            body: body.to_string(),
            icon: Some(app_icon.to_string()),
            urgency: Urgency::Normal,
            id: 0, // hints.sender_pid as u32,
            sched: None,
        };

        // send the notification to the notification stack
        tx.send(NotifStackEvent::Added(notification)).await;

        Ok(0)
    }

    // Signals

    // These signals emit when something happens with the notification
    // In case one wants to capture these signals. just connect them
    // Right now we're just logging them for debugging purposes

    #[dbus_interface(signal)]
    #[tracing::instrument]
    async fn action_invoked(
        ctx: &SignalContext<'_>,
        id: u32,
        action_key: &str,
    ) -> zbus::Result<()> {
        tracing::trace!("ActionInvoked");
        Ok(())
    }

    #[dbus_interface(signal)]
    #[tracing::instrument]
    async fn notification_closed(
        ctx: &SignalContext<'_>,
        id: u32,
        reason: u32,
    ) -> zbus::Result<()> {
        tracing::trace!("NotificationClosed");
        Ok(())
    }
}
