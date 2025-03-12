use std::error::Error;
use crate::models::History;

pub fn export_history(history: &Vec<History>) -> Result<(), Box<dyn Error>> {
    // Create CSV writer
    let mut writer = match csv::Writer::from_path("time-commander.csv") {
        Ok(writer) => writer,
        Err(error) => panic!("{}", error)
    };

    // Add Headers
    writer.write_record(&["Date", "Start", "End", "Total", "Pauses"])?;

    // Add Records to file
    for record in history {
        writer.write_record(&[
            &record.record_date,
            &record.start_time,
            &record.end_time,
            &record.total_duration.to_string(),
            &record.total_pauses.to_string()
        ])?;
    }

    writer.flush()?;
    Ok(())
}
