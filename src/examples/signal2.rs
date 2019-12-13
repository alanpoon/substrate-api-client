/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

///! Very simple example that shows how to get some simple storage values.
use futures_signals::signal::Mutable;
use futures::executor::block_on;
use std::thread;
use futures::future::ready;
use futures_signals::signal::SignalExt;
use std::sync::Arc;

fn run (m:Mutable<u8>){
    let mut last_update = std::time::Instant::now();
    thread::spawn(move|| {
        loop{
            let sixteen_ms = std::time::Duration::from_millis(3000);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            last_update =now;
            let k = m.get();
            println!("m {:?}",k+1);
            m.set(k+1);
        }
    });
}
async fn t(m:Mutable<u8>)->u8{
    let k =m.signal().wait_for(2).await.unwrap();
    println!("k {:?}",k);
    k
}
fn main() {
    let my_state = Mutable::new(5);
    run(my_state.clone());

    block_on(t(my_state));
}
