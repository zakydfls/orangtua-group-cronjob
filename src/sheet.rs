use reqwest::Error;
use serde_json::Value;
use crate::sheet_data::{SheetData, RowData};
use dotenv::dotenv;
use std::env;

pub async fn fetch_google_sheet() -> Result<Value, Box<dyn std::error::Error>> {
    dotenv().ok();
    let sheet_id = env::var("SHEET_ID").expect("SHEET_ID not set in .env");
    let gid = env::var("GID").expect("GID not set in .env");
    let url = format!(
        "https://docs.google.com/spreadsheets/d/{}/gviz/tq?tqx=out:json&tq&gid={}",
        sheet_id, gid
    );

    let response = reqwest::get(&url).await?.text().await?;

    let raw_data = &response[47..response.len() - 2];
    let parsed_data: Value = serde_json::from_str(raw_data)?;
    println!("Parsed Google Sheet Data: {:?}", parsed_data);

    Ok(parsed_data)
}

pub async fn get_sheets_data(sheets_data: &mut Vec<SheetData>) -> Result<(), Box<dyn std::error::Error>> {
    for sheet in sheets_data.iter_mut() {
        println!("Processing sheet: {}", sheet.nama_sheet);

        match fetch_google_sheet().await {
            Ok(sheet_data) => {
                println!("Loaded sheet data: {:?}", sheet_data);

                if let Some(rows) = sheet_data["table"]["rows"].as_array() {
                    let all_rows: Vec<RowData> = rows
                        .iter()
                        .map(|row| {
                            let empty_array = vec![];
                            let c = row["c"].as_array().unwrap_or(&empty_array);


                            RowData {
                                tanggal: c.get(0).and_then(|v| v["f"].as_str()).unwrap_or("Tidak Diketahui").to_string(),
                                nama_pelaksana: c.get(1).and_then(|v| v["v"].as_str()).unwrap_or("Tidak Diketahui").to_string(),
                                tangki: c.get(2).and_then(|v| v["v"].as_str()).unwrap_or("Tidak Diketahui").to_string(),
                                jenis_tangki: "Tidak Diketahui".to_string(), // Dapat diubah sesuai kebutuhan
                                jenis_sanitasi: c.get(3).and_then(|v| v["v"].as_str()).unwrap_or("Tidak Diketahui").to_string(),
                                tanggal_sanitasi: c.get(4).and_then(|v| v["f"].as_str()).unwrap_or("Tidak Diketahui").to_string(),
                                jam_mulai: c.get(5).and_then(|v| v["f"].as_str()).unwrap_or("Tidak Tersedia").to_string(),
                                jam_selesai: c.get(6).and_then(|v| v["f"].as_str()).unwrap_or("Tidak Tersedia").to_string(),
                                hasil_luminometer: c.get(7).and_then(|v| v["v"].as_str()).unwrap_or("-").to_string(),
                                hasil_ph_meter: c.get(8).and_then(|v| v["v"].as_str()).unwrap_or("-").to_string(),
                            }
                        })
                        .collect();

                    println!("All rows for sheet {}: {:?}", sheet.nama_sheet, all_rows);
                    sheet.all_rows = Some(all_rows);
                } else {
                    println!("No rows found for sheet {}", sheet.nama_sheet);
                }
            }
            Err(error) => {
                println!("Error processing sheet {}: {:?}", sheet.nama_sheet, error);
            }
        }
    }

    Ok(())
}
