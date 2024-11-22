use crate::sheet_data::{RowData, SheetData};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SummaryTangki {
    pub nama_tangki: String,
    pub jenis_tangki: String,
    pub frekuensi_oxonia: String,
    pub frekuensi_kaustik: String,
    pub frekuensi_deep_cleaning: String,
    pub sanitasi_oxonia: Option<String>,
    pub sanitasi_cleaning_kaustik: Option<String>,
    pub sanitasi_deep_cleaning: Option<String>,
}

pub fn generate_summary_tangki(sheets_data: &[SheetData]) -> Vec<SummaryTangki> {
    let jenis_tangki_order = vec![
        "Produk",
        "Cao",
        "Separator",
        "Filler",
        "Distilasi",
        "Buffer",
        "Mesin RO",
        "Tidak Diketahui",
    ];

    let mut unique_tangki: HashMap<String, SummaryTangki> = HashMap::new();

    for sheet in sheets_data {
        if let Some(rows) = &sheet.all_rows {
            for row in rows {
                let tangki = row.tangki.clone();
                let jenis_sanitasi = row.jenis_sanitasi.clone();
                let tanggal_sanitasi = row.tanggal_sanitasi.clone();

                let entry = unique_tangki.entry(tangki.clone()).or_insert(SummaryTangki {
                    nama_tangki: tangki.clone(),
                    jenis_tangki: row.jenis_tangki.clone(),
                    frekuensi_oxonia: "-".to_string(),
                    frekuensi_kaustik: "-".to_string(),
                    frekuensi_deep_cleaning: "-".to_string(),
                    sanitasi_oxonia: None,
                    sanitasi_cleaning_kaustik: None,
                    sanitasi_deep_cleaning: None,
                });

                if jenis_sanitasi == "Sanitasi Oxonia" {
                    if entry.sanitasi_oxonia.is_none() || tanggal_sanitasi > entry.sanitasi_oxonia.clone().unwrap() {
                        entry.sanitasi_oxonia = Some(tanggal_sanitasi.clone());
                    }
                } else if jenis_sanitasi == "Cleaning Kaustik" {
                    if entry.sanitasi_cleaning_kaustik.is_none() || tanggal_sanitasi > entry.sanitasi_cleaning_kaustik.clone().unwrap() {
                        entry.sanitasi_cleaning_kaustik = Some(tanggal_sanitasi.clone());
                    }
                } else if jenis_sanitasi == "Deep Cleaning" {
                    if entry.sanitasi_deep_cleaning.is_none() || tanggal_sanitasi > entry.sanitasi_deep_cleaning.clone().unwrap() {
                        entry.sanitasi_deep_cleaning = Some(tanggal_sanitasi.clone());
                    }
                }
            }
        }
    }

    let mut tangki_array: Vec<SummaryTangki> = unique_tangki.into_values().collect();
    tangki_array.sort_by(|a, b| {
        let index_a = jenis_tangki_order
            .iter()
            .position(|&x| x == a.jenis_tangki)
            .unwrap_or(usize::MAX);
        let index_b = jenis_tangki_order
            .iter()
            .position(|&x| x == b.jenis_tangki)
            .unwrap_or(usize::MAX);

        if index_a == index_b {
            a.nama_tangki.cmp(&b.nama_tangki)
        } else {
            index_a.cmp(&index_b)
        }
    });

    tangki_array
}
