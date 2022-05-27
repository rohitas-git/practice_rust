/* 
In some regard, logging is the same as using println, except that you can specify the importance of a message.
The levels you can usually use are -
error,
warn,
info,
debug, 
trace (error has the highest priority, trace the lowest).

*/

use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}