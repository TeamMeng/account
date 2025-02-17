use std::collections::HashMap;
use tokio::sync::OnceCell;

pub static STATUS_CODE: OnceCell<HashMap<usize, &str>> = OnceCell::const_new();

pub async fn code_init() {
    STATUS_CODE
        .get_or_init(|| async {
            HashMap::from([
                (0, "操作失败"),
                (1, "操作成功"),
                (400, "请求错误"),
                (401, "未经授权"),
                (500, "服务器错误"),
                (10001, "手机号格式不正确"),
                (10002, "密码格式不正确"),
                (15002, "密码格式不正确"),
            ])
        })
        .await;
}
