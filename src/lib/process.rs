use postcard::to_stdvec;
use rusqlite::{params, Connection};
use serde_json::Value;

pub fn from_json_str(raw_json: &str, db: &Connection) {
    let values: Value = serde_json::from_str(raw_json).unwrap();

    values
        .get("labels")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .for_each(save(db))
}

fn save(db: &Connection) -> impl FnMut(&Value) + '_ {
    |label: &Value| {
        let raw_label: Option<crate::label::Label> = serde_json::from_value(label.clone()).ok();

        match raw_label {
            Some(label) => {
                let post_label = to_stdvec(&label).unwrap();
                db.execute(
                    "INSERT INTO labels (name, postcard) VALUES (?, ?)",
                    params![label.name, post_label],
                )
                .expect("Failed to insert label");
            }
            None => {}
        }
    }
}
