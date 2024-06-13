use dioxus::prelude::{Signal,};
use dioxus::signals::GlobalSignal;

pub static PROTOCOL_NAME:GlobalSignal<Vec<String>> = Signal::global(Vec::new);

pub(crate) fn init_data(){
    PROTOCOL_NAME.write().push("--选择协议--".to_string());
    PROTOCOL_NAME.write().push("国网1376.2集中器本地通信模块接口".to_string());
    PROTOCOL_NAME.write().push("国网1376.3采集终端远程通信模块接口".to_string());
}