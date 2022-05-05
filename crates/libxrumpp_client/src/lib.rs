//use xmpp::{ClientBuilder, ClientFeature, ClientType, Event, Agent};
use tokio::sync::mpsc;
use tokio::{pin, select};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use std::ptr::null;

extern crate libstrophe_sys;
use libstrophe_sys::*;

pub struct XMPPServerInfo {
    pub jid: String,
    pub pass: Option<String>,
    pub nick: String,
    pub host: String,
    pub port: u16
    //pub auto_join: Vec
}

enum XMPPCmd {
    Msg(String), // Send this message
    Quit(Option<String>) // 
}

pub struct XMPPEvent {

}

pub struct XMPPClient {
    channel_messages: mpsc::Sender<XMPPCmd>
}

impl XMPPClient {
    pub fn new(server_info: XMPPServerInfo) {
        
    }
    
    pub fn reconnect(&mut self) {

    }

    pub fn joinMUC( &mut self ) {

    }
}

// Connect and create main loop task for given server
fn xmpp_connect(server_info: XMPPServerInfo) -> (XMPPClient, mpsc::Receiver<XMPPEvent>) {
    // 
    // Create communication channels
    // 

    // Channel for returning XMPP events to user
    let (channel_send_events, channel_receive_events) = mpsc::channel::<XMPPEvent>(100);
    
    // Channel for commands from the user
    let (channel_send_cmds, channel_receive_cmds) = mpsc::channel::<XMPPCmd>(100);
    //
    // Main loop task
    //
    let task = xmpp_main_loop(server_info, channel_send_events, channel_receive_cmds);
    tokio::task::spawn_local(task);

    (
        XMPPClient {
            channel_messages: channel_send_cmds,
        },
        channel_receive_events
    )
}



// Main loop for a given server
async fn xmpp_main_loop(
    server_info: XMPPServerInfo, 
    mut channel_send_events : mpsc::Sender<XMPPEvent>,
    channel_receive_cmds: mpsc::Receiver<XMPPCmd>

) {
    let mut channel_receive_cmds = ReceiverStream::new(channel_receive_cmds).fuse();

    // Create XMPP client instance
    // Avatars feature is not enabled. Not sure how to accomplish this anyways
    /*let mut client: Agent = ClientBuilder::new(server_info.jid.as_str(), server_info.pass.unwrap().as_str())
        .set_client(ClientType::Pc, "xrumpp")
        .set_website("")
        .set_default_nick(server_info.nick.as_str())
        .enable_feature(ClientFeature::ContactList)
        .enable_feature(ClientFeature::JoinRooms)
        .build()
        .unwrap()[0];*/
    let mut client_conn: *mut xmpp_conn_t;
    let mut is_connected: bool = false;
    let mut ctx: *mut xmpp_ctx_t;
    let xmpp_run_task;
    unsafe {
        xmpp_initialize();
        ctx = xmpp_ctx_new(null(), null()); // Create context
        client_conn = xmpp_conn_new(ctx); // Create connection
        let jid = std::ffi::CString::new(server_info.jid).unwrap();
        xmpp_conn_set_jid(client_conn, jid.as_ptr()); // Set JID

        // Attempt connect
        let host = std::ffi::CString::new(server_info.host).unwrap();
        is_connected = match xmpp_connect_client(client_conn, host.as_ptr(), server_info.port, 
            Some(connection_handler), &mut ctx as *mut _ as *mut std::ffi::c_void) {
            0 => false,
            _ => true
        };

        // Run xmpp event loop in new thread
        let task = async move { xmpp_run(ctx); };
        xmpp_run_task = tokio::task::spawn_local(task);
    }
    // Receive command channel

    // Task for outgoing messages

    // Loop receiving and handling commands from use
    loop {
        select! {
            // Receive event from xmpp client
            /*Some(events) = client.wait_for_events().await => {
                for event in events {
                    channel_send_events.send(event).await.unwrap();
                }
            }*/

            // Receive command from user
            cmd = channel_receive_cmds.next() => {

            }
        }
    }

    tokio::join!(xmpp_run_task); // Wait for xmpp_run_task

    // Shutdown libstrophe
    unsafe {
        xmpp_conn_release(client_conn);
        xmpp_ctx_free(ctx);
        xmpp_shutdown();
    }
}

unsafe extern "C" fn connection_handler(
    conn: *mut xmpp_conn_t,
    event: xmpp_conn_event_t,
    error: ::std::os::raw::c_int,
    stream_error: *mut xmpp_stream_error_t,
    userdata: *mut ::std::os::raw::c_void
) {
    if status == XMPP_CONN_CONNECT {
        debug!("Connection xmpp");
    }
    else {
        debug!("Disconnected");
    }
}