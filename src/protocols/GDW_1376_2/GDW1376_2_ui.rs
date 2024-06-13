use dioxus::prelude::*;
pub fn GDW1376_2_ui()->Element{
    rsx!{
            textarea{
                    class:"msg_text",
                    placeholder:"请输入数据",
                    initial_value:"68 28 00 C3 05 35 00 00 00 00 00 00 CF 14 90 00 0F 85 01 06 00 10 02 01 01 06 00 00 01 11 00 00 01 00 04 81 BC F0 44 4F 16 16 "

            }
            div{
                class:"result_line",
                label{"解析结果："}
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