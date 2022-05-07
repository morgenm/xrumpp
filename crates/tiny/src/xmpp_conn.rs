use tokio::sync::mpsc;
use libtiny_common::{ChanNameRef, MsgTarget, TabStyle};
use crate::ui::UI;
use libxrumpp_client::{XMPPClient, XMPPEvent};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

/*pub(crate) trait XMPPClient {
    fn get_serv_name(&self) -> &str;

    fn get_nick(&self) -> String;

    fn is_nick_accepted(&self) -> bool;
}

impl XMPPClient for libxrumpp_client::XMPPClient {
    fn get_serv_name(&self) -> &str {
        self.get_serv_name()
    }
}*/

pub(crate) trait Client {
    fn get_serv_name(&self) -> &str;
}

impl Client for XMPPClient {
    fn get_serv_name(&self) -> &str {
        self.get_serv_name()
    }
}

pub(crate) async fn task(
    rcv_ev: mpsc::Receiver<libxrumpp_client::XMPPEvent>,
    ui: UI,
    client: Box<dyn Client>,
) {
    let mut rcv_ev = ReceiverStream::new(rcv_ev);
    while let Some(ev) = rcv_ev.next().await {
        handle_conn_ev(&ui, &*client, ev);
        ui.draw();
    }
}

fn handle_conn_ev(
    ui: &UI, 
    client: &dyn Client, 
    ev: libxrumpp_client::XMPPEvent
) {
    use libxrumpp_client::XMPPEvent::*;
    match ev {
        Version => {
            ui.add_client_msg(
                "Version Info",
                &MsgTarget::AllServTabs {
                    serv: client.get_serv_name(),
                },
            );
        }
        Msg(msg) => {
            ui.add_client_msg(
                "Message",
                &MsgTarget::AllServTabs {
                    serv: client.get_serv_name(),
                },
            );
        }
    }
}