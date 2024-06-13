use dioxus::prelude::*;
pub fn none_selected_ui()->Element{
    rsx!{

        textarea{
                    class:"msg_text",
                    placeholder:"请输入数据",
                    initial_value:"woshishabi"
            }
        div{
                class:"result_line",
                label{"结果：解析成功！"}
            }
            table{
                class:"packets_table",

                    th{
                    style{"font-size:10px"}
                        "帧域"
                    }
                    th{
                        "数据"
                    }
                    th{
                        "说明"
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