mod sheet;
mod sheet_data;
mod summary;

use sheet::get_sheets_data;
use sheet_data::{SheetData};
use summary::generate_summary_tangki;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let sheet_id = env::var("SHEET_ID").expect("SHEET_ID not set in .env");
    let gid = env::var("GID").expect("GID not set in .env");
    let mut sheets_data = vec![
        SheetData {
            nama_sheet: "Sheet Rekap".to_string(),
            id_sheet: sheet_id.to_string(),
            gid: gid.to_string(),
            all_rows: None,
        },
    ];

    get_sheets_data(&mut sheets_data).await?;
    // let summary = generate_summary_tangki(&sheets_data);
    // println!("Summary Tangki: {:#?}", summary);

    Ok(())
}