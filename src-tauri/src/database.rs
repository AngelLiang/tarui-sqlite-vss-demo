use tauri::AppHandle;
use rusqlite::Connection;
use rusqlite::ffi::sqlite3_auto_extension;
use sqlite_vss::{sqlite3_vector_init, sqlite3_vss_init};


pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    // 加载sqlite-vss
    unsafe {
        sqlite3_auto_extension(Some(sqlite3_vector_init));
        sqlite3_auto_extension(Some(sqlite3_vss_init));
    }

    // 打开数据库
    let db = Connection::open_in_memory()?;

    let version = get_version(&db);
    // println!("{}", version);

    // 创建向量数据表
    db.execute_batch(
        r"
    CREATE VIRTUAL TABLE vss_demo USING vss0(a(2));
    INSERT INTO vss_demo(rowid, a)
      VALUES
          (1, '[1.0, 2.0]'),
          (2, '[2.0, 2.0]'),
          (3, '[3.0, 2.0]')
    ",
    )?;


    Ok(db)
}

pub fn get_version(db: &Connection) -> Result<String, rusqlite::Error> {
    let (version, vector): (String, String) = db.query_row(
        "SELECT vss_version(), vector_to_json(?)",
        [[0x00, 0x00, 0x28, 0x42]],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    Ok(version)
}

pub fn add_vector(db: &Connection) -> Result<String, rusqlite::Error> {
    let result: Vec<(i64, f32)> = db
        .prepare(
            r"
          SELECT
            rowid,
            distance
          FROM vss_demo
          WHERE vss_search(a, '[1.0, 2.0]')
          LIMIT 3
        ",
        )?
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    let mut vec_str = String::new();
    vec_str.push_str("[");
    for (rowid, distance) in result {
        println!("rowid={rowid}, distance={distance}");
        vec_str.push_str(format!("rowid={rowid}, distance={distance} ").as_str());
    }
    vec_str.push_str("]");
    Ok(vec_str)
}
