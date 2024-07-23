use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Entry {
    pub date: String,
    pub matkamittarin_aloituslukema: f64,
    pub ammattiajo: f64,
    pub tuottamaton_ajo: f64,
    pub yksityinen_ajo: f64,
    pub matkamittarin_loppulukema: f64,
    pub käteisajotulot: f64,
    pub pankkikorttitulot: f64,
    pub luottokorttitulot: f64,
    pub kela_suorakorvaus: f64,
    pub taksikortti: f64,
    pub laskutettavat: f64,
}

pub fn initialize_db() -> Result<()> {
    let conn = Connection::open("data/data.db")?;
    create_table(&conn)?;
    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
                  id                      INTEGER PRIMARY KEY,
                  date                    TEXT NOT NULL,
                  matkamittarin_aloituslukema REAL NOT NULL,
                  ammattiajo              REAL NOT NULL,
                  tuottamaton_ajo         REAL NOT NULL,
                  yksityinen_ajo          REAL NOT NULL,
                  matkamittarin_loppulukema REAL NOT NULL,
                  käteisajotulot          REAL NOT NULL,
                  pankkikorttitulot       REAL NOT NULL,
                  luottokorttitulot       REAL NOT NULL,
                  kela_suorakorvaus       REAL NOT NULL,
                  taksikortti             REAL NOT NULL,
                  laskutettavat           REAL NOT NULL
                  )",
        [],
    )?;
    Ok(())
}

pub fn insert_entry(conn: &Connection, entry: &Entry) -> Result<()> {
    conn.execute(
        "INSERT INTO entries (date, matkamittarin_aloituslukema, ammattiajo, tuottamaton_ajo, yksityinen_ajo, matkamittarin_loppulukema, käteisajotulot, pankkikorttitulot, luottokorttitulot, kela_suorakorvaus, taksikortti, laskutettavat) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            entry.date,
            entry.matkamittarin_aloituslukema,
            entry.ammattiajo,
            entry.tuottamaton_ajo,
            entry.yksityinen_ajo,
            entry.matkamittarin_loppulukema,
            entry.käteisajotulot,
            entry.pankkikorttitulot,
            entry.luottokorttitulot,
            entry.kela_suorakorvaus,
            entry.taksikortti,
            entry.laskutettavat,
        ],
    )?;
    Ok(())
}

pub fn get_monthly_summary(conn: &Connection, month: &str) -> Result<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> {
    let mut stmt = conn.prepare(
        "SELECT 
            SUM(matkamittarin_aloituslukema), 
            SUM(ammattiajo), 
            SUM(tuottamaton_ajo), 
            SUM(yksityinen_ajo), 
            SUM(matkamittarin_loppulukema),
            SUM(käteisajotulot), 
            SUM(pankkikorttitulot), 
            SUM(luottokorttitulot), 
            SUM(kela_suorakorvaus), 
            SUM(taksikortti), 
            SUM(laskutettavat) 
         FROM entries 
         WHERE date LIKE ?1"
    )?;
    let rows = stmt.query_map(params![format!("{}%", month)], |row| {
        Ok((
            row.get::<_, Option<f64>>(0)?, 
            row.get::<_, Option<f64>>(1)?, 
            row.get::<_, Option<f64>>(2)?, 
            row.get::<_, Option<f64>>(3)?, 
            row.get::<_, Option<f64>>(4)?,
            row.get::<_, Option<f64>>(5)?, 
            row.get::<_, Option<f64>>(6)?, 
            row.get::<_, Option<f64>>(7)?, 
            row.get::<_, Option<f64>>(8)?, 
            row.get::<_, Option<f64>>(9)?,
            row.get::<_, Option<f64>>(10)?
        ))
    })?;

    let mut summary = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for row in rows {
        let (matkamittarin_aloituslukema, ammattiajo, tuottamaton_ajo, yksityinen_ajo, matkamittarin_loppulukema, käteisajotulot, pankkikorttitulot, luottokorttitulot, kela_suorakorvaus, taksikortti, laskutettavat): (Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>) = row?;
        summary.0 += matkamittarin_aloituslukema.unwrap_or(0.0);
        summary.1 += ammattiajo.unwrap_or(0.0);
        summary.2 += tuottamaton_ajo.unwrap_or(0.0);
        summary.3 += yksityinen_ajo.unwrap_or(0.0);
        summary.4 += matkamittarin_loppulukema.unwrap_or(0.0);
        summary.5 += käteisajotulot.unwrap_or(0.0);
        summary.6 += pankkikorttitulot.unwrap_or(0.0);
        summary.7 += luottokorttitulot.unwrap_or(0.0);
        summary.8 += kela_suorakorvaus.unwrap_or(0.0);
        summary.9 += taksikortti.unwrap_or(0.0);
        summary.10 += laskutettavat.unwrap_or(0.0);
    }
    Ok(summary)
}

pub fn get_entry_by_date(conn: &Connection, date: &str) -> Result<Option<Entry>> {
    let mut stmt = conn.prepare("SELECT date, matkamittarin_aloituslukema, ammattiajo, tuottamaton_ajo, yksityinen_ajo, matkamittarin_loppulukema, käteisajotulot, pankkikorttitulot, luottokorttitulot, kela_suorakorvaus, taksikortti, laskutettavat FROM entries WHERE date = ?1")?;
    let mut rows = stmt.query(params![date])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(Entry {
            date: row.get(0)?,
            matkamittarin_aloituslukema: row.get(1)?,
            ammattiajo: row.get(2)?,
            tuottamaton_ajo: row.get(3)?,
            yksityinen_ajo: row.get(4)?,
            matkamittarin_loppulukema: row.get(5)?,
            käteisajotulot: row.get(6)?,
            pankkikorttitulot: row.get(7)?,
            luottokorttitulot: row.get(8)?,
            kela_suorakorvaus: row.get(9)?,
            taksikortti: row.get(10)?,
            laskutettavat: row.get(11)?,
        }))
    } else {
        Ok(None)
    }
}