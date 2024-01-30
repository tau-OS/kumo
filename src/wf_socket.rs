//! Functions to communicate with the Wayfire socket.
//! Reference:
//! https://github.com/WayfireWM/wayfire/blob/master/ipc-scripts/wayfire_socket.py
//!
//! Wayfire exposes a JSON IPC socket, which can be used to control the compositor.
//! The socket is located at the path specified by the WAYFIRE_SOCKET environment variable.
//!
//! Data typees for Wayfire commands are defined in the `wf_ipc` module.
use once_cell::sync::OnceCell;
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    sync::{Mutex, MutexGuard},
};
const WAYFIRE_SOCKET_ENV: &str = "WAYFIRE_SOCKET";
use color_eyre::Result;

use crate::wf_ipc;
use std::convert::TryInto;

pub fn get_wayfire_socket() -> Option<String> {
    std::env::var(WAYFIRE_SOCKET_ENV).ok()
}

static WAYFIRE_SOCKET: OnceCell<Mutex<WayfireSocket>> = OnceCell::new();

/// The Wayfire socket is a Unix socket that is used to communicate with the Wayfire compositor.
#[derive(Debug)]
pub struct WayfireSocket {
    pub socket: UnixStream,
}

impl WayfireSocket {
    /// The actual socket is created here, it is recommended to use the `new` function instead of this one.
    fn init_socket() -> Option<Self> {
        let socket_path = get_wayfire_socket()?;
        let socket = UnixStream::connect(socket_path).ok()?;
        Some(Self { socket })
    }

    /// Creates a new Wayfire socket, if one already exists, it will return that one instead.
    pub fn new() -> MutexGuard<'static, Self> {
        let socket = WAYFIRE_SOCKET.get_or_init(|| {
            let socket = WayfireSocket::init_socket().expect("Failed to create Wayfire socket");
            Mutex::new(socket)
        });
        socket.lock().expect("Failed to lock Wayfire socket")
    }

    pub fn send_json(&mut self, cmd: serde_json::Value) -> Result<serde_json::Value> {
        let json = serde_json::to_string(&cmd)?;
        let buf = json.as_bytes();
        //  header = len(data).to_bytes(4, byteorder="little")
        let header = (buf.len() as u32).to_le_bytes();
        self.socket.write_all(&header)?;
        self.socket.write_all(buf)?;
        self.read_message()
    }

    /*
        def read_exact(self, n):
            response = bytes()
            while n > 0:
                read_this_time = self.client.recv(n)
                if not read_this_time:
                    raise Exception("Failed to read anything from the socket!")
                n -= len(read_this_time)
                response += read_this_time

            return response
    */
    pub fn read_exact(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut response = Vec::new();
        while response.len() < n {
            let mut buf = vec![0; n - response.len()];
            let read_this_time = self.socket.read(&mut buf)?;
            if read_this_time == 0 {
                return Err(color_eyre::eyre::eyre!(
                    "Failed to read anything from the socket!"
                ));
            }
            response.extend(buf);
        }
        Ok(response)
    }

    /*
       def read_message(self):
           rlen = int.from_bytes(self.read_exact(4), byteorder="little")
           response_message = self.read_exact(rlen)
           response = js.loads(response_message)
           if "error" in response:
               raise Exception(response["error"])
           return response
    */

    // return json for now

    pub fn read_message(&mut self) -> Result<serde_json::Value, color_eyre::Report> {
        let rlen_bytes = self.read_exact(4)?;
        let rlen = u32::from_le_bytes(rlen_bytes.try_into().unwrap());
        let response_message = self.read_exact(rlen as usize)?;
        let response = serde_json::from_slice(&response_message)?;
        tracing::debug!(?response, "Received response");
        if let serde_json::Value::Object(map) = &response {
            if let Some(error) = map.get("error") {
                return Err(color_eyre::eyre::eyre!(error.to_string()));
            }
        }
        Ok(response)
    }

    pub fn close(&mut self) -> Result<()> {
        self.socket.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}
