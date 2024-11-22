use crate::google_sheet::fetch_google_sheet;
use crate::wa_gateway::send_wa_notification;
use cron::Schedule;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

pub async fn run_scheduler(
    sheet_id: &str,
    gid: &str,
    api_url: &str,
    phone_number: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let schedule = Schedule::from_str("0 * * * * *")?;

    for datetime in schedule.upcoming(chrono::Utc) {
        let now = chrono::Utc::now();
        let duration = datetime - now;

        if duration.to_std().is_ok() {
            sleep(Duration::from_secs(duration.to_std()?.as_secs())).await;

            let data = fetch_google_sheet(sheet_id, gid).await?;

            let payload = serde_json::json!({
                "message": format!("Data from sheet: {:?}", data),
                "to": phone_number
            });

            send_wa_notification(api_url, payload).await?;
        }
    }

    Ok(())
}
