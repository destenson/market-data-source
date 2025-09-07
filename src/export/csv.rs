//! CSV export functionality for market data
//!
//! This module provides CSV export capabilities for OHLC and tick data,
//! enabling users to save generated market data for analysis in Excel,
//! pandas, or other CSV-compatible tools.

use crate::export::{DataExporter, ExportResult};
use crate::types::{OHLC, Tick};
use csv::{Writer, WriterBuilder};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// CSV exporter for market data
#[derive(Debug, Clone)]
pub struct CsvExporter {
    options: CsvOptions,
}

/// Configuration options for CSV export
#[derive(Debug, Clone)]
pub struct CsvOptions {
    /// Whether to include headers in the CSV file
    pub include_headers: bool,
    /// The delimiter character (default: ',')
    pub delimiter: u8,
    /// Whether to quote all fields
    pub quote_style: QuoteStyle,
    /// Buffer capacity for writer (in bytes)
    pub buffer_capacity: usize,
}

/// Quote style for CSV fields
#[derive(Debug, Clone, Copy)]
pub enum QuoteStyle {
    /// Quote only when necessary
    Necessary,
    /// Always quote fields
    Always,
    /// Never quote fields
    Never,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            include_headers: true,
            delimiter: b',',
            quote_style: QuoteStyle::Necessary,
            buffer_capacity: 8192,
        }
    }
}

impl Default for CsvExporter {
    fn default() -> Self {
        Self {
            options: CsvOptions::default(),
        }
    }
}

impl CsvExporter {
    /// Creates a new CSV exporter with default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new CSV exporter with custom options
    pub fn with_options(options: CsvOptions) -> Self {
        Self { options }
    }

    /// Sets whether to include headers
    pub fn include_headers(mut self, include: bool) -> Self {
        self.options.include_headers = include;
        self
    }

    /// Sets the delimiter character
    pub fn delimiter(mut self, delimiter: u8) -> Self {
        self.options.delimiter = delimiter;
        self
    }

    /// Sets the quote style
    pub fn quote_style(mut self, style: QuoteStyle) -> Self {
        self.options.quote_style = style;
        self
    }

    /// Creates a CSV writer with the configured options
    fn create_writer<W: Write>(&self, writer: W) -> Writer<W> {
        let mut builder = WriterBuilder::new();
        builder
            .delimiter(self.options.delimiter)
            .has_headers(self.options.include_headers)
            .buffer_capacity(self.options.buffer_capacity);

        // Configure quote style
        match self.options.quote_style {
            QuoteStyle::Always => builder.quote_style(csv::QuoteStyle::Always),
            QuoteStyle::Never => builder.quote_style(csv::QuoteStyle::Never),
            QuoteStyle::Necessary => builder.quote_style(csv::QuoteStyle::Necessary),
        };

        builder.from_writer(writer)
    }

    /// Writes OHLC data to a writer
    pub fn write_ohlc<W: Write>(&self, data: &[OHLC], writer: W) -> ExportResult<()> {
        let mut csv_writer = self.create_writer(writer);

        // Write headers if configured
        if self.options.include_headers {
            csv_writer.write_record(&[
                "timestamp",
                "open",
                "high",
                "low",
                "close",
                "volume",
            ])?;
        }

        // Write data rows
        for ohlc in data {
            csv_writer.write_record(&[
                ohlc.timestamp.to_string(),
                ohlc.open.to_string(),
                ohlc.high.to_string(),
                ohlc.low.to_string(),
                ohlc.close.to_string(),
                ohlc.volume.value().to_string(),
            ])?;
        }

        csv_writer.flush()?;
        Ok(())
    }

    /// Writes tick data to a writer
    pub fn write_ticks<W: Write>(&self, data: &[Tick], writer: W) -> ExportResult<()> {
        let mut csv_writer = self.create_writer(writer);

        // Write headers if configured
        if self.options.include_headers {
            csv_writer.write_record(&[
                "timestamp",
                "price",
                "volume",
                "bid",
                "ask",
            ])?;
        }

        // Write data rows
        for tick in data {
            csv_writer.write_record(&[
                tick.timestamp.to_string(),
                tick.price.to_string(),
                tick.volume.value().to_string(),
                tick.bid.map_or(String::new(), |b| b.to_string()),
                tick.ask.map_or(String::new(), |a| a.to_string()),
            ])?;
        }

        csv_writer.flush()?;
        Ok(())
    }

    /// Stream-writes OHLC data for large datasets
    pub fn stream_ohlc<P: AsRef<Path>, I>(&self, data: I, path: P) -> ExportResult<usize>
    where
        I: IntoIterator<Item = OHLC>,
    {
        let file = File::create(path)?;
        let mut csv_writer = self.create_writer(file);
        let mut count = 0;

        // Write headers if configured
        if self.options.include_headers {
            csv_writer.write_record(&[
                "timestamp",
                "open",
                "high",
                "low",
                "close",
                "volume",
            ])?;
        }

        // Stream write data
        for ohlc in data {
            csv_writer.write_record(&[
                ohlc.timestamp.to_string(),
                ohlc.open.to_string(),
                ohlc.high.to_string(),
                ohlc.low.to_string(),
                ohlc.close.to_string(),
                ohlc.volume.value().to_string(),
            ])?;
            count += 1;

            // Periodic flush for large datasets
            if count % 1000 == 0 {
                csv_writer.flush()?;
            }
        }

        csv_writer.flush()?;
        Ok(count)
    }

    /// Stream-writes tick data for large datasets
    pub fn stream_ticks<P: AsRef<Path>, I>(&self, data: I, path: P) -> ExportResult<usize>
    where
        I: IntoIterator<Item = Tick>,
    {
        let file = File::create(path)?;
        let mut csv_writer = self.create_writer(file);
        let mut count = 0;

        // Write headers if configured
        if self.options.include_headers {
            csv_writer.write_record(&[
                "timestamp",
                "price",
                "volume",
                "bid",
                "ask",
            ])?;
        }

        // Stream write data
        for tick in data {
            csv_writer.write_record(&[
                tick.timestamp.to_string(),
                tick.price.to_string(),
                tick.volume.value().to_string(),
                tick.bid.map_or(String::new(), |b| b.to_string()),
                tick.ask.map_or(String::new(), |a| a.to_string()),
            ])?;
            count += 1;

            // Periodic flush for large datasets
            if count % 1000 == 0 {
                csv_writer.flush()?;
            }
        }

        csv_writer.flush()?;
        Ok(count)
    }
}

impl DataExporter for CsvExporter {
    fn export_ohlc<P: AsRef<Path>>(&self, data: &[OHLC], path: P) -> ExportResult<()> {
        let file = File::create(path)?;
        self.write_ohlc(data, file)
    }

    fn export_ticks<P: AsRef<Path>>(&self, data: &[Tick], path: P) -> ExportResult<()> {
        let file = File::create(path)?;
        self.write_ticks(data, file)
    }
    
    fn export_ohlc_to_writer<W: Write>(&self, data: &[OHLC], writer: W) -> ExportResult<()> {
        self.write_ohlc(data, writer)
    }
    
    fn export_ticks_to_writer<W: Write>(&self, data: &[Tick], writer: W) -> ExportResult<()> {
        self.write_ticks(data, writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use rust_decimal::prelude::*;
    use rust_decimal::Decimal;
    use crate::types::{OHLC, Tick};
    use std::str::FromStr;

    #[test]
    fn test_csv_exporter_creation() {
        let exporter = CsvExporter::new();
        assert!(exporter.options.include_headers);
        assert_eq!(exporter.options.delimiter, b',');
    }

    #[test]
    fn test_csv_options_builder() {
        let exporter = CsvExporter::new()
            .include_headers(false)
            .delimiter(b';')
            .quote_style(QuoteStyle::Always);

        assert!(!exporter.options.include_headers);
        assert_eq!(exporter.options.delimiter, b';');
    }

    #[test]
    fn test_write_ohlc_to_buffer() {
        let exporter = CsvExporter::new();
        let data = vec![
            OHLC::new(Decimal::from(100), Decimal::from(105), Decimal::from(99), Decimal::from(103), 1000, 1234567890000),
            OHLC::new(Decimal::from(103), Decimal::from(104), Decimal::from(101), Decimal::from(102), 1500, 1234567891000),
        ];

        let mut buffer = Cursor::new(Vec::new());
        exporter.write_ohlc(&data, &mut buffer).unwrap();

        let csv_content = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(csv_content.contains("timestamp,open,high,low,close,volume"));
        assert!(csv_content.contains("1234567890000,100,105,99,103,1000"));
        assert!(csv_content.contains("1234567891000,103,104,101,102,1500"));
    }

    #[test]
    fn test_write_ticks_to_buffer() {
        let exporter = CsvExporter::new();
        let data = vec![
            Tick::new(Decimal::from_str("100.5").unwrap(), 500, 1234567890000),
            Tick::with_spread(Decimal::from(101), 750, 1234567891000, Decimal::from_str("100.9").unwrap(), Decimal::from_str("101.1").unwrap()),
        ];

        let mut buffer = Cursor::new(Vec::new());
        exporter.write_ticks(&data, &mut buffer).unwrap();

        let csv_content = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(csv_content.contains("timestamp,price,volume,bid,ask"));
        assert!(csv_content.contains("1234567890000,100.5,500,,"));
        assert!(csv_content.contains("1234567891000,101,750,100.9,101.1"));
    }

    #[test]
    fn test_no_headers() {
        let exporter = CsvExporter::new().include_headers(false);
        let data = vec![OHLC::new(Decimal::from(100), Decimal::from(105), Decimal::from(99), Decimal::from(103), 1000, 1234567890000)];

        let mut buffer = Cursor::new(Vec::new());
        exporter.write_ohlc(&data, &mut buffer).unwrap();

        let csv_content = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(!csv_content.contains("timestamp,open,high,low,close,volume"));
        assert!(csv_content.contains("1234567890000,100,105,99,103,1000"));
    }

    #[test]
    fn test_custom_delimiter() {
        let exporter = CsvExporter::new().delimiter(b';');
        let data = vec![OHLC::new(Decimal::from(100), Decimal::from(105), Decimal::from(99), Decimal::from(103), 1000, 1234567890000)];

        let mut buffer = Cursor::new(Vec::new());
        exporter.write_ohlc(&data, &mut buffer).unwrap();

        let csv_content = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(csv_content.contains("timestamp;open;high;low;close;volume"));
        assert!(csv_content.contains("1234567890000;100;105;99;103;1000"));
    }
}