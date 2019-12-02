use ws::connect;
use std::sync::mpsc::Sender as ThreadOut;
use ws::{CloseCode,Handler, Result,Message,Sender,Handshake};
use std::thread;
use super::{ResultE,OnMessageFn};

use log::{debug, info};

pub struct RpcClient {
    pub out: Sender,
    pub request: String,
    pub result: ThreadOut<String>,
    pub on_message_fn: OnMessageFn,
}
impl Handler for RpcClient {
  fn on_open(&mut self,_: Handshake ) -> Result<()> {
      info!("sending request: {}", self.request);
      self.out.send(self.request.clone()).unwrap();
      Ok(())
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
      info!("got message");
      debug!("{}", msg);
      let msgg = msg.as_text().unwrap();
      let res_e = (self.on_message_fn)(&msgg);
      match res_e {
        ResultE::None=>{},
        ResultE::Close=>{
          self.out.close(CloseCode::Normal).unwrap();
        },
        ResultE::S(s)=>{
          self.result.send(s).unwrap();
        },
        ResultE::SClose(s)=>{
          self.result.send(s).unwrap();
          self.out.close(CloseCode::Normal).unwrap();
        }
      }
      Ok(())
  }
}


pub fn start_rpc_client_thread(
  url: String,
  jsonreq: String,
  result_in: ThreadOut<String>,
  on_message_fn: OnMessageFn,
) {
  let _client = thread::Builder::new()
      .name("client".to_owned())
      .spawn(move || {
          connect(url, |out| RpcClient {
              out,
              request: jsonreq.clone(),
              result: result_in.clone(),
              on_message_fn,
          })
          .unwrap()
      })
      .unwrap();
}
