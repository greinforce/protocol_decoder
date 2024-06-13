use dioxus::prelude::*;
use crate::globals::PROTOCOL_NAME;
pub fn GDW1376_3_ui()->Element{
    rsx!{
            textarea{
                    class:"msg_text",
                    placeholder:"请输入AT指令",
                    initial_value:"AT+GMR？"

            }
            div{
                class:"result_line",
                label{"AT指令解析结果："}
            }
            table{
                class:"packets_table",

                    th{
                    style{"font-size:10px"}
                        "AT指令"
                    }
                    th{
                        "所属指令集"
                    }
                    th{
                        "指令说明"
                    }
                        for i in 0..40{
                        tr{
                        td{""}
                        td{""}
                        td{""}
                        }
                    }
                }



    }

}