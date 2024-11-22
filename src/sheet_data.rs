#[derive(Debug)]
pub struct SheetData {
    pub nama_sheet: String,
    pub id_sheet: String,
    pub gid: String,
    pub all_rows: Option<Vec<RowData>>,
}

#[derive(Debug)]
pub struct RowData {
    pub tanggal: String,
    pub nama_pelaksana: String,
    pub tangki: String,
    pub jenis_tangki: String,
    pub jenis_sanitasi: String,
    pub tanggal_sanitasi: String,
    pub jam_mulai: String,
    pub jam_selesai: String,
    pub hasil_luminometer: String,
    pub hasil_ph_meter: String,
}
