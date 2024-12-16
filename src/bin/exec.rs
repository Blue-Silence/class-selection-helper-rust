use std::env;
use tokio; // 确保引入 tokio
use class_selection_helper::scheduler::schedule_requests; // 替换为你的 crate 名称
use class_selection_helper::type_def::{RoundData, ElecClass}; // 替换为你的 crate 名称
use serde_json; // 导入 serde_json

#[tokio::main]
async fn main() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: exec <global_json> <rounddata_json>");
        return;
    }

    let global_json = &args[1];
    let rounddata_json = &args[2];

    // 解析 global 和 rounddata JSON 字符串
    let global: String = parse_json_to_global(global_json).expect("Failed to parse global JSON");
    let round_data: RoundData = parse_json_to_round_data(rounddata_json).expect("Failed to parse rounddata JSON");

    // 调用 schedule_requests 函数
    schedule_requests(global, round_data).await;
}

// 示例解析函数（需要实现）
fn parse_json_to_global(json_str: &str) -> Result<String, serde_json::Error> {
    serde_json::from_str(json_str)
}

fn parse_json_to_round_data(json_str: &str) -> Result<RoundData, serde_json::Error> {
    serde_json::from_str(json_str)
} 