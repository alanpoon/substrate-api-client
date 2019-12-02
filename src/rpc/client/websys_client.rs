use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};
use super::{ResultE,OnMessageFn};
use std::sync::mpsc::Sender as ThreadOut;

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn start_rpc_client_thread(
  url: String,
  jsonreq: String,
  result_in: ThreadOut<String>,
  on_message_fn: OnMessageFn,
) {
  let ws = WebSocket::new(&url).unwrap();
  let ws_c = ws.clone();
  let on_message = {
    Closure::wrap(Box::new(move |evt: MessageEvent| {
        let msgg = evt.data()
                    .as_string()
        .expect("Can't convert received data to a string");
        console_log!("message event, received data: {:?}", msgg);
        let res_e = (on_message_fn)(&msgg);
        match res_e {
          ResultE::None=>{},
          ResultE::Close=>{
            ws_c.close_with_code(1000).unwrap();
          },
          ResultE::S(s)=>{
            result_in.send(s).unwrap();
          },
          ResultE::SClose(s)=>{
            result_in.send(s).unwrap();
            ws_c.close_with_code(1000).unwrap();
          }
        }
    }) as Box<dyn FnMut(MessageEvent)>)
  };
  
  ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
  ws.send_with_str(&jsonreq).unwrap();

}
