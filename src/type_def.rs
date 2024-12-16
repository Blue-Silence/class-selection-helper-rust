use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Global {
    pub x_token: String,
    pub studentId: String,
    pub courseCode: String,
    pub calendarId: String,
    pub time_by_ms: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ElecClass {
    pub teachClassId: String,
    pub teachClassCode: String,
    pub courseCode: String,
    pub courseName: String,
    pub teacherName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundData {
    pub roundId: String,
    pub elecClassList: Vec<ElecClass>,
    pub withdrawClassList: Vec<ElecClass>,
}