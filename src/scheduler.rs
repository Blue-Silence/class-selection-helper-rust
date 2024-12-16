use tokio::time::{sleep, Duration};
use crate::request::send_request; // 导入 send_request 函数
use crate::type_def::{Global, RoundData}; // 导入 RoundData 结构体
use crate::encrypt; // 导入 encrypt 模块
use chrono::Utc; // 导入 Utc

// 定义一个新的异步函数
pub async fn schedule_requests(global: Global, round: RoundData) {
    let interval = Duration::from_millis(global.time_by_ms); // 设置请求间隔

    loop {
        // 创建 RoundData 实例

        // 使用 encrypt 处理 RoundData
        let fst_c = round.elecClassList.first().unwrap();
        let (ciphertext, checkcode) = encrypt::encrypt(global.studentId.as_str(), fst_c.courseCode.as_str(), fst_c.teachClassId.as_str(), global.calendarId.as_str(), current_timestamp().as_str(), serde_json::to_string(&round).unwrap().as_str());

        // 发送请求
        if let Err(e) = send_request(ciphertext, checkcode, global.x_token.clone()).await {
            eprintln!("Error sending request: {:?}", e);
        }

        sleep(interval).await; // 等待指定的时间间隔
    }
} 


// 创建一个函数来获取当前时间戳
pub fn current_timestamp() -> String {
    let timestamp = Utc::now().timestamp_millis(); // 获取当前时间的时间戳（毫秒）
    timestamp.to_string() // 转换为字符串
}