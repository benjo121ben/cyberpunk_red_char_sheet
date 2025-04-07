use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LessonData {
    pub lessons_list: HashMap<String, Vec<Course>>,
    pub last_executed: String
    /* #[serde(rename = "main_course_data")]
    pub main_course_data: MainCourseData, */
}

/* #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonsList {
    pub courses: Vec<Course>,
    pub date: String,
} */

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    pub aula: String,
    pub end: String,
    pub name: String,
    pub note: Option<String>,
    pub start: String,
}

/* #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainCourseData {
    pub codice: String,
    pub codice_classe: Value,
    pub codice_classe_laurea: String,
    pub codice_facolta: String,
    pub codice_tipo: String,
    pub codice_tipo_ministeriale: String,
    pub descrizione: String,
    pub descrizione_aggiuntiva: Value,
    pub descrizione_facolta: String,
    pub descrizione_inglese: String,
    pub descrizione_tipo: String,
    pub sede_didattica: String,
    pub sede_didattica_estesa: String,
} */