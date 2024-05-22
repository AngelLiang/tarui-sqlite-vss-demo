use tauri::AppHandle;
use rusqlite::Connection;
use rusqlite::ffi::sqlite3_auto_extension;
use sqlite_vss::{sqlite3_vector_init, sqlite3_vss_init};
use std::fs;
use anyhow::Result;
use rusqlite::params;
use crate::ffi::sqlite3_simple_init;


pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, Box<dyn std::error::Error>> {
    // 加载sqlite-vss
    let res = unsafe {
        sqlite3_auto_extension(Some(sqlite3_vector_init));
        sqlite3_auto_extension(Some(sqlite3_vss_init));

        // simple
        sqlite3_auto_extension(Some(sqlite3_simple_init))
    };

    // 打开数据库
    // let db = Connection::open_in_memory()?;

    let mut app_dir = app_handle.path_resolver().app_data_dir().expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    // 数据库路径
    let sqlite_path = app_dir.join("tauri-sqlite-vss-demo.sqlite");
    fs::remove_file(sqlite_path.as_path().clone())?;
    // 打开数据库
    let mut db: Connection = Connection::open(sqlite_path).map_err(|e| {
        println!("{}", e);
        e
    })?;

    // 获取jieba的路径
    let jieba_dict_path = app_handle.path_resolver()
        .resolve_resource("jieba")
        .map(|path_buf| path_buf.to_str().unwrap_or("").to_owned()) // 修改这里
        .unwrap_or("".to_owned()); // 这里也做相应修改，确保返回String类型
    log::debug!("jieba_dict_path: {}", jieba_dict_path);
    db.query_row("SELECT jieba_dict(?)", params![jieba_dict_path], |_| Ok(()));

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

    // 创建全文索引
    db.execute_batch("
        CREATE VIRTUAL TABLE d USING fts5(id, text, tokenize = 'simple');
        INSERT INTO d (id, text) VALUES (1, '中华人民共和国国歌');
        INSERT INTO d (id, text) VALUES (2, '周杰伦');
        INSERT INTO d (id, text) VALUES (3, '周润发');
    ")?;
    // db.execute_batch("
    //     BEGIN;
    //     DELETE FROM d WHERE id = 3;
    //     INSERT INTO d (id, text) VALUES (3, '成龙');
    //     COMMIT;
    // ")?;
    db.execute_batch("
        UPDATE d
        SET text = '成龙'
        WHERE id = 3;
    ")?;

    let result = db.query_row("SELECT id FROM d WHERE text MATCH jieba_query('中华国歌')", [], |row| row.get::<_, i64>(0))?;
    println!("{}", result);
    let result = db.query_row("SELECT simple_highlight(d, 1, '[', ']') as text FROM d WHERE text MATCH jieba_query('中华国歌')", [], |row| {
        // println!("{}", row.get(1));
        row.get::<_, String>(0)
    })?;
    println!("{}", result);

    let result = db.query_row("SELECT id FROM d WHERE text MATCH simple_query('zhoujiel')", [], |row| row.get::<_, i64>(0))?;
    println!("{}", result);
    let result = db.query_row("SELECT simple_highlight(d, 1, '[', ']') as text FROM d WHERE text MATCH jieba_query('chen')", [], |row| row.get::<_, String>(0))?;
    println!("{}", result);

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
