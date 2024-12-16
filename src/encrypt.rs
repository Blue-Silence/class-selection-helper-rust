use aes::Aes128;
use block_modes::{Cbc, BlockMode};
use block_modes::block_padding::Pkcs7;
use md5;
use base64;
use std::str;
use urlencoding::encode;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn static_encrypt(input: &str) -> String {
    let key = b"fCm^7p1AwqKPTNjn";
    let iv = b"MaW5a2v%%I9em$UI";

    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(input.as_bytes());

    base64::encode(&ciphertext)
}


/*
    studentId: '2154284'; //this.studentInfo.studentId.toString(),
    courseCode: '50002870003'; //t.courseCode.toString(),
    teachClassId: '1111111124926026'; //t.teachClassId.toString(),
    calendarId: '119';// this.calendarId.toString(),
    time: '1734311096089'; //new Date().getTime().toString();
    body: `{"roundId":5433,"elecClassList":[{"teachClassId":1111111124926026,"teachClassCode":"5000287000301","courseCode":"50002870003","courseName":"大型数据库系统管理","teacherName":"唐剑锋"},{"teachClassId":1111111124928938,"teachClassCode":"42041303","courseCode":"420413","courseName":"软件工程管理与经济","teacherName":"杜博闻"},{"teachClassId":1111111124927462,"teachClassCode":"42040601","courseCode":"420406","courseName":"主机软件与管理","teacherName":"高珍,冯巾松"},{"teachClassId":1111111124927454,"teachClassCode":"42040401","courseCode":"420404","courseName":"数据分析与数据挖掘","teacherName":"赵钦佩,饶卫雄"},{"teachClassId":1111111124926589,"teachClassCode":"42037201","courseCode":"420372","courseName":"信息安全基础","teacherName":"尹长青"},{"teachClassId":1111111124926793,"teachClassCode":"42036101","courseCode":"420361","courseName":"软件测试","teacherName":"刘琴"},{"teachClassId":1111111124926805,"teachClassCode":"42035801","courseCode":"420358","courseName":"专业方向综合项目","teacherName":"唐剑锋"},{"teachClassId":1111111124932029,"teachClassCode":"54010230","courseCode":"540102","courseName":"形势与政策(4)","teacherName":"林秋琴"},{"teachClassId":1111111124929237,"teachClassCode":"42041203","courseCode":"420412","courseName":"人工智能导论","teacherName":"赵生捷,梁爽"},{"teachClassId":1111111124926751,"teachClassCode":"42034501","courseCode":"420345","courseName":"用户交互技术","teacherName":"沈莹"}],"withdrawClassList":[]}`;  // 原始字符串
*/
pub fn encrypt(studentId: &str, courseCode:&str, teachClassId:&str, calendarId: &str, time: &str, body: &str) -> (String, String) {
    let text = format!("{}&{}&{}&{}&{}&{}", studentId, courseCode, teachClassId, calendarId, time, body);
    let text2 = format!("{}+{}+{}+{}+{}+{}", studentId, courseCode, teachClassId, calendarId, time, body);

    let encoded_text = encode(&text).into_owned();

    let ciphertext = static_encrypt(&encoded_text);
    let checkcode = md5::compute(text2);

    return (ciphertext, format!("{:x}", checkcode));
}