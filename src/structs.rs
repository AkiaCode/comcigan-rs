use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Grade {
    pub classes: Vec<Class>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub days: Vec<Day>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    pub studies: Vec<Study>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Study {
    pub subject: String,
    pub teacher: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchoolData {
    pub grades: Vec<Grade>
}

impl SchoolData {
    pub fn grade(&self, grade: usize) -> &Grade {
        &self.grades[grade - 1]
    }
}

impl Grade {
    pub fn class(&self, class: usize) -> &Class {
        &self.classes[class - 1]
    }
}

impl Class {
    pub fn day(&self, day: usize) -> &Day {
        &self.days[day - 1]
    }
}

impl Day {
    pub fn study(&self, study: usize) -> &Study {
        &self.studies[study - 1]
    }
}

#[derive(Serialize, Deserialize)]
pub struct SchoolList {
    pub 학교검색: Vec<School>
}

#[derive(Serialize, Deserialize)]
pub struct School(u32, String, String, pub u32);

#[derive(Serialize, Deserialize)]
pub struct RawSchoolData {
    pub timetable: Vec<Vec<Vec<Vec<u32>>>>,
    pub subjects: Vec<String>,
    pub teachers: Vec<String>
}

pub struct RawSchoolDataKey {
    pub timetable: String,
    pub subjects: String,
    pub teachers: String,
    pub encode_header: String,
    pub url_piece: String
}

impl RawSchoolDataKey {
    pub fn clone(&self) -> RawSchoolDataKey {
        RawSchoolDataKey {
            timetable: self.timetable.clone(),
            subjects: self.subjects.clone(),
            teachers: self.teachers.clone(),
            encode_header: self.encode_header.clone(),
            url_piece: self.url_piece.clone()
        }
    }
}