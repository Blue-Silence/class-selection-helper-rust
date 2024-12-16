
use class_selection_helper::encrypt;


fn main() {
    let a = "215xxxx";
    let o = "50002870003";
    let n = "1111111124926026";
    let c = "119";
    let l = "1734311096089";
    let s = r#"{"roundId":5433,"elecClassList":[{"teachClassId":1111111124926026,"teachClassCode":"5000287000301","courseCode":"50002870003","courseName":"大型数据库系统管理","teacherName":"唐剑锋"},{"teachClassId":1111111124928938,"teachClassCode":"42041303","courseCode":"420413","courseName":"软件工程管理与经济","teacherName":"杜博闻"},{"teachClassId":1111111124927462,"teachClassCode":"42040601","courseCode":"420406","courseName":"主机软件与管理","teacherName":"高珍,冯巾松"},{"teachClassId":1111111124927454,"teachClassCode":"42040401","courseCode":"420404","courseName":"数据分析与数据挖掘","teacherName":"赵钦佩,饶卫雄"},{"teachClassId":1111111124926589,"teachClassCode":"42037201","courseCode":"420372","courseName":"信息安全基础","teacherName":"尹长青"},{"teachClassId":1111111124926793,"teachClassCode":"42036101","courseCode":"420361","courseName":"软件测试","teacherName":"刘琴"},{"teachClassId":1111111124926805,"teachClassCode":"42035801","courseCode":"420358","courseName":"专业方向综合项目","teacherName":"唐剑锋"},{"teachClassId":1111111124932029,"teachClassCode":"54010230","courseCode":"540102","courseName":"形势与政策(4)","teacherName":"林秋琴"},{"teachClassId":1111111124929237,"teachClassCode":"42041203","courseCode":"420412","courseName":"人工智能导论","teacherName":"赵生捷,梁爽"},{"teachClassId":1111111124926751,"teachClassCode":"42034501","courseCode":"420345","courseName":"用户交互技术","teacherName":"沈莹"}],"withdrawClassList":[]}"#;

    println!("{:?}", encrypt::encrypt(a, o, n, c, l, s));



    test2();
}



use serde_json;
use class_selection_helper::type_def::RoundData;

fn test2() {
    let json_data = r#"
    {
        "roundId": 5433,
        "elecClassList": [
            {"teachClassId": 1111111124926026, "teachClassCode": "5000287000301", "courseCode": "50002870003", "courseName": "大型数据库系统管理", "teacherName": "唐剑锋"},
            {"teachClassId": 1111111124928938, "teachClassCode": "42041303", "courseCode": "420413", "courseName": "软件工程管理与经济", "teacherName": "杜博闻"},
            {"teachClassId": 1111111124927462, "teachClassCode": "42040601", "courseCode": "420406", "courseName": "主机软件与管理", "teacherName": "高珍,冯巾松"},
            {"teachClassId": 1111111124927454, "teachClassCode": "42040401", "courseCode": "420404", "courseName": "数据分析与数据挖掘", "teacherName": "赵钦佩,饶卫雄"},
            {"teachClassId": 1111111124926589, "teachClassCode": "42037201", "courseCode": "420372", "courseName": "信息安全基础", "teacherName": "尹长青"},
            {"teachClassId": 1111111124926793, "teachClassCode": "42036101", "courseCode": "420361", "courseName": "软件测试", "teacherName": "刘琴"},
            {"teachClassId": 1111111124926805, "teachClassCode": "42035801", "courseCode": "420358", "courseName": "专业方向综合项目", "teacherName": "唐剑锋"},
            {"teachClassId": 1111111124932029, "teachClassCode": "54010230", "courseCode": "540102", "courseName": "形势与政策(4)", "teacherName": "林秋琴"},
            {"teachClassId": 1111111124929237, "teachClassCode": "42041203", "courseCode": "420412", "courseName": "人工智能导论", "teacherName": "赵生捷,梁爽"},
            {"teachClassId": 1111111124926751, "teachClassCode": "42034501", "courseCode": "420345", "courseName": "用户交互技术", "teacherName": "沈莹"}
        ],
        "withdrawClassList": []
    }
    "#;

    // 反序列化 JSON 字符串为 RoundData 结构体
    let round_data: RoundData = serde_json::from_str(json_data).unwrap();
    println!("{:?}", round_data);

    // 序列化 RoundData 结构体为 JSON 字符串
    let serialized = serde_json::to_string(&round_data).unwrap();
    println!("{}", serialized);
}