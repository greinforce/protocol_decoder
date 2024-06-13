use dioxus::prelude::*;
use crate::globals::*;
use crate::protocols::GDW_1376_2::GDW1376_2_ui::GDW1376_2_ui;
use crate::protocols::none_selected::none_selected_ui;
use crate::protocols::GDW_1376_3::GDW1376_3_ui::GDW1376_3_ui;

pub fn Desktop_init()->Element{
    let mut select_protocol =use_signal(||"选择协议".to_string());
    rsx! {
        style{{include_str!("res/style.css")}}
        div{class:"wrapper",
            div{
                class:"wrapper_one",
                div{
                class:"wrapper_one_left",
                button{"<"}
                button{"清除并粘贴"}
                button{">"}}
                div{
                class:"wrapper_one_right",
                label{"规约"}
                 select{
                        onchange:move |e| {
                                select_protocol.set(e.value().to_string());
                        },
                        class:"protocol_select",
                        option{

                            "--选择协议--"
                        }
                        option{

                            "国网1376.2集中器本地通信模块接口"
                        }
                        option{

                            "国网1376.3采集终端远程通信模块接口"
                        }
                }
                button{"打开规约"}
                button{"主站"}
                    }
                }
            div{
                class:"wrapper_two",
                {protocol_match_ui(select_protocol.read().clone().as_str())}
            }
        }
}
}
fn protocol_match_ui(select_protocol:&str)->Element{
    rsx!{
        div{
            match select_protocol{
                "--选择协议--" =>{none_selected_ui()}
                "国网1376.2集中器本地通信模块接口"=>{GDW1376_2_ui()}
                "国网1376.3采集终端远程通信模块接口"=>{GDW1376_3_ui()}
                _=>{rsx!{}}
            }
        }
    }
}



