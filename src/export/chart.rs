//! PNG chart generation module for market data visualization
//! 
//! This module provides functionality to generate candlestick and line charts
//! from market data using the plotters library.

use crate::types::{OHLC, Tick};
use plotters::prelude::*;
use std::error::Error;
use std::path::Path;

/// Trait for converting various color formats to RGBColor
pub trait ToRgbColor {
    fn to_rgb(self) -> RGBColor;
}

impl ToRgbColor for RGBColor {
    fn to_rgb(self) -> RGBColor {
        self
    }
}

impl ToRgbColor for (u8, u8, u8) {
    fn to_rgb(self) -> RGBColor {
        RGBColor(self.0, self.1, self.2)
    }
}

/// Chart builder for configuring and generating market data charts
#[derive(Debug, Clone)]
pub struct ChartBuilder {
    /// Width of the generated chart in pixels
    pub width: u32,
    /// Height of the generated chart in pixels  
    pub height: u32,
    /// Chart title
    pub title: String,
    /// Color for bullish (green) candles
    pub bullish_color: RGBColor,
    /// Color for bearish (red) candles
    pub bearish_color: RGBColor,
    /// Background color
    pub background_color: RGBColor,
    /// Grid color
    pub grid_color: RGBColor,
    /// Text color
    pub text_color: RGBColor,
    /// Volume bar color
    pub volume_color: RGBAColor,
    /// Moving average color
    pub ma_color: RGBColor,
    /// Show volume subplot
    pub show_volume: bool,
    /// Show moving average overlay
    pub show_moving_average: bool,
    /// Moving average period
    pub ma_period: usize,
    /// Show grid lines
    pub show_grid: bool,
    /// Line width for tick charts
    pub line_width: u32,
    /// Font family name
    pub font_family: String,
}

impl Default for ChartBuilder {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            title: "Market Data Chart".to_string(),
            bullish_color: GREEN,
            bearish_color: RED,
            background_color: WHITE,
            grid_color: RGBColor(230, 230, 230),
            text_color: BLACK,
            volume_color: RGBAColor(100, 100, 100, 0.5),
            ma_color: BLUE,
            show_volume: true,
            show_moving_average: true,
            ma_period: 20,
            show_grid: true,
            line_width: 1,
            font_family: if cfg!(target_os = "windows") {
                "Arial".to_string()
            } else if cfg!(target_os = "macos") {
                "Helvetica".to_string()  
            } else {
                "sans-serif".to_string()
            },
        }
    }
}

impl ChartBuilder {
    /// Create a new chart builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set chart dimensions
    pub fn dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set chart width
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set chart height
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Set chart title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set background color (accepts RGBColor or (u8, u8, u8) tuple)
    pub fn background_color<C>(mut self, color: C) -> Self 
    where 
        C: ToRgbColor
    {
        self.background_color = color.to_rgb();
        self
    }

    /// Set grid color
    pub fn grid_color<C: ToRgbColor>(mut self, color: C) -> Self {
        self.grid_color = color.to_rgb();
        self
    }

    /// Set SMA color
    pub fn sma_color<C: ToRgbColor>(mut self, color: C) -> Self {
        self.ma_color = color.to_rgb();
        self
    }

    /// Set text color
    pub fn text_color<C: ToRgbColor>(mut self, color: C) -> Self {
        self.text_color = color.to_rgb();
        self
    }

    /// Set line color (alias for ma_color for tick charts)
    pub fn line_color<C: ToRgbColor>(mut self, color: C) -> Self {
        self.ma_color = color.to_rgb();
        self
    }

    /// Set candlestick colors
    pub fn candlestick_colors<B: ToRgbColor, R: ToRgbColor>(mut self, bullish: B, bearish: R) -> Self {
        self.bullish_color = bullish.to_rgb();
        self.bearish_color = bearish.to_rgb();
        self
    }

    /// Set whether to show volume subplot
    pub fn show_volume(mut self, show: bool) -> Self {
        self.show_volume = show;
        self
    }

    /// Set whether to show moving average
    pub fn show_moving_average(mut self, show: bool) -> Self {
        self.show_moving_average = show;
        self
    }

    pub fn show_sma(mut self, period: usize) -> Self {
        self.show_moving_average = true;
        self.ma_period = period;
        self
    }

    /// Set moving average period
    pub fn ma_period(mut self, period: usize) -> Self {
        self.ma_period = period;
        self
    }

    /// Set line width for tick charts
    pub fn line_width(mut self, width: u32) -> Self {
        self.line_width = width;
        self
    }

    /// Set font family
    pub fn font_family(mut self, font: impl Into<String>) -> Self {
        self.font_family = font.into();
        self
    }


    /// Generate a candlestick chart from OHLC data
    pub fn draw_candlestick_chart<P: AsRef<Path>>(
        &self,
        data: &[OHLC],
        output_path: P,
    ) -> Result<(), Box<dyn Error>> {
        if data.is_empty() {
            return Err("Cannot create chart from empty data".into());
        }

        // Create buffer for bitmap
        let mut buffer = vec![0u8; (self.width * self.height * 3) as usize];
        
        {
            let root = BitMapBackend::with_buffer(&mut buffer, (self.width, self.height))
                .into_drawing_area();
            root.fill(&self.background_color)?;

        // Calculate the layout - if showing volume, split the chart
        let (upper, lower) = if self.show_volume {
            let areas = root.split_evenly((2, 1));
            (areas[0].clone(), areas[1].clone())
        } else {
            let areas = root.split_evenly((1, 1));
            (areas[0].clone(), areas[0].clone())
        };

        // Find price range
        let min_price = data.iter()
            .map(|ohlc| ohlc.low)
            .fold(f64::INFINITY, f64::min);
        let max_price = data.iter()
            .map(|ohlc| ohlc.high)
            .fold(f64::NEG_INFINITY, f64::max);
        let price_margin = (max_price - min_price) * 0.1;

        // Build the price chart with TTF font support
        let mut price_chart = plotters::chart::ChartBuilder::on(&upper)
            .caption(&self.title, (self.font_family.as_str(), 40).into_font().color(&self.text_color))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(60)
            .build_cartesian_2d(
                0f64..(data.len() as f64),
                (min_price - price_margin)..(max_price + price_margin),
            )?;

        if self.show_grid {
            price_chart
                .configure_mesh()
                .x_desc("Time")
                .y_desc("Price")
                .label_style((self.font_family.as_str(), 15).into_font().color(&self.text_color))
                .axis_style(&self.grid_color)
                .draw()?;
        }

        // Draw candlesticks
        let candle_width = 0.7;
        for (idx, ohlc) in data.iter().enumerate() {
            let x = idx as f64;
            let color = if ohlc.close >= ohlc.open {
                self.bullish_color
            } else {
                self.bearish_color
            };

            // Draw the high-low line (wick)
            price_chart.draw_series(LineSeries::new(
                vec![(x, ohlc.low), (x, ohlc.high)],
                &color,
            ))?;

            // Draw the open-close rectangle (body)
            let (body_bottom, body_top) = if ohlc.close >= ohlc.open {
                (ohlc.open, ohlc.close)
            } else {
                (ohlc.close, ohlc.open)
            };

            price_chart.draw_series(std::iter::once(Rectangle::new([
                (x - candle_width / 2.0, body_bottom),
                (x + candle_width / 2.0, body_top),
            ], color.filled())))?;
        }

        // Draw moving average if enabled
        if self.show_moving_average && data.len() >= self.ma_period {
            let ma_values = self.calculate_sma(data);
            if !ma_values.is_empty() {
                price_chart.draw_series(LineSeries::new(
                    ma_values.iter().enumerate()
                        .map(|(idx, &val)| ((idx + self.ma_period - 1) as f64, val)),
                    &self.ma_color,
                ))?
                .label(format!("SMA({})", self.ma_period))
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &self.ma_color));
                
                price_chart.configure_series_labels()
                    .label_font((self.font_family.as_str(), 15).into_font().color(&self.text_color))
                    .background_style(&WHITE.mix(0.8))
                    .border_style(&BLACK)
                    .draw()?;
            }
        }

        // Draw volume chart if enabled
        if self.show_volume {
            let max_volume = data.iter()
                .map(|ohlc| ohlc.volume.as_f64())
                .fold(0f64, f64::max);

            let mut volume_chart = plotters::chart::ChartBuilder::on(&lower)
                .margin(10)
                .x_label_area_size(30)
                .y_label_area_size(60)
                .build_cartesian_2d(
                    0f64..(data.len() as f64),
                    0f64..(max_volume * 1.1),
                )?;

            if self.show_grid {
                volume_chart
                    .configure_mesh()
                    .x_desc("Time")
                    .y_desc("Volume")
                    .label_style((self.font_family.as_str(), 15).into_font().color(&self.text_color))
                    .axis_style(&self.grid_color)
                    .draw()?;
            }

            // Draw volume bars
            for (idx, ohlc) in data.iter().enumerate() {
                let x = idx as f64;
                let color = if ohlc.close >= ohlc.open {
                    self.bullish_color.mix(0.5)
                } else {
                    self.bearish_color.mix(0.5)
                };

                volume_chart.draw_series(std::iter::once(Rectangle::new([
                    (x - candle_width / 2.0, 0.0),
                    (x + candle_width / 2.0, ohlc.volume.as_f64()),
                ], color.filled())))?;
            }
        }

            root.present()?;
        }
        
        // Save buffer as PNG using image crate
        let img = image::RgbImage::from_raw(self.width, self.height, buffer)
            .ok_or("Failed to create image from buffer")?;
        img.save(output_path.as_ref())?;
        
        Ok(())
    }

    /// Generate a line chart from tick data
    pub fn draw_line_chart<P: AsRef<Path>>(
        &self,
        data: &[Tick],
        output_path: P,
    ) -> Result<(), Box<dyn Error>> {
        if data.is_empty() {
            return Err("Cannot create chart from empty data".into());
        }

        // Create buffer for bitmap
        let mut buffer = vec![0u8; (self.width * self.height * 3) as usize];
        
        {
            let root = BitMapBackend::with_buffer(&mut buffer, (self.width, self.height))
                .into_drawing_area();
            root.fill(&self.background_color)?;

        // Find price range
        let min_price = data.iter()
            .map(|tick| tick.price)
            .fold(f64::INFINITY, f64::min);
        let max_price = data.iter()
            .map(|tick| tick.price)
            .fold(f64::NEG_INFINITY, f64::max);
        let price_margin = (max_price - min_price) * 0.1;

        // Build the chart with TTF font support
        let mut chart = plotters::chart::ChartBuilder::on(&root)
            .caption(&self.title, (self.font_family.as_str(), 40).into_font().color(&self.text_color))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(60)
            .build_cartesian_2d(
                0f64..(data.len() as f64),
                (min_price - price_margin)..(max_price + price_margin),
            )?;

        if self.show_grid {
            chart
                .configure_mesh()
                .x_desc("Time")
                .y_desc("Price")
                .label_style((self.font_family.as_str(), 15).into_font().color(&self.text_color))
                .axis_style(&self.grid_color)
                .draw()?;
        }

        // Draw the price line
        chart.draw_series(LineSeries::new(
            data.iter().enumerate()
                .map(|(idx, tick)| (idx as f64, tick.price)),
            &self.ma_color,
        ))?
        .label("Price")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &self.ma_color));

        // Draw bid/ask lines if available
        let has_bid_ask = data.iter().any(|tick| tick.bid.is_some() && tick.ask.is_some());
        if has_bid_ask {
            // Draw bid line
            chart.draw_series(LineSeries::new(
                data.iter().enumerate()
                    .filter_map(|(idx, tick)| tick.bid.map(|bid| (idx as f64, bid))),
                &self.bearish_color,
            ))?
            .label("Bid")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &self.bearish_color));

            // Draw ask line
            chart.draw_series(LineSeries::new(
                data.iter().enumerate()
                    .filter_map(|(idx, tick)| tick.ask.map(|ask| (idx as f64, ask))),
                &self.bullish_color,
            ))?
            .label("Ask")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &self.bullish_color));
        }

        //
        chart.configure_series_labels()
            .label_font((self.font_family.as_str(), 15).into_font().color(&self.text_color))
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

            root.present()?;
        }
        
        // Save buffer as PNG using image crate
        let img = image::RgbImage::from_raw(self.width, self.height, buffer)
            .ok_or("Failed to create image from buffer")?;
        img.save(output_path.as_ref())?;
        
        Ok(())
    }

    /// Calculate Simple Moving Average
    fn calculate_sma(&self, data: &[OHLC]) -> Vec<f64> {
        if data.len() < self.ma_period {
            return Vec::new();
        }

        let mut sma_values = Vec::new();
        for i in (self.ma_period - 1)..data.len() {
            let sum: f64 = data[(i + 1 - self.ma_period)..(i + 1)]
                .iter()
                .map(|ohlc| ohlc.close)
                .sum();
            sma_values.push(sum / self.ma_period as f64);
        }
        sma_values
    }
}

/// Chart exporter for PNG format
pub struct ChartExporter {
    builder: ChartBuilder,
}

impl Default for ChartExporter {
    fn default() -> Self {
        Self {
            builder: ChartBuilder::default(),
        }
    }
}

impl ChartExporter {
    /// Create a new chart exporter with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a chart exporter with a custom builder
    pub fn with_builder(builder: ChartBuilder) -> Self {
        Self { builder }
    }

    /// Export OHLC data as a candlestick chart
    pub fn export_ohlc<P: AsRef<Path>>(
        &self,
        data: &[OHLC],
        output_path: P,
    ) -> Result<(), Box<dyn Error>> {
        self.builder.draw_candlestick_chart(data, output_path)
    }

    /// Export tick data as a line chart
    pub fn export_ticks<P: AsRef<Path>>(
        &self,
        data: &[Tick],
        output_path: P,
    ) -> Result<(), Box<dyn Error>> {
        self.builder.draw_line_chart(data, output_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_chart_builder_default() {
        let builder = ChartBuilder::default();
        assert_eq!(builder.width, 1920);
        assert_eq!(builder.height, 1080);
        assert!(builder.show_volume);
        assert!(builder.show_moving_average);
        assert_eq!(builder.ma_period, 20);
    }

    #[test]
    fn test_chart_builder_configuration() {
        let builder = ChartBuilder::new()
            .dimensions(800, 600)
            .title("Test Chart")
            .show_volume(false)
            .show_moving_average(false)
            .ma_period(50);

        assert_eq!(builder.width, 800);
        assert_eq!(builder.height, 600);
        assert_eq!(builder.title, "Test Chart");
        assert!(!builder.show_volume);
        assert!(!builder.show_moving_average);
        assert_eq!(builder.ma_period, 50);
    }

    #[test]
    fn test_candlestick_chart_generation() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_candlestick.png");

        let data = vec![
            OHLC::new(100.0, 105.0, 99.0, 103.0, 1000, 1640995200),
            OHLC::new(103.0, 106.0, 102.0, 105.0, 1200, 1640995260),
            OHLC::new(105.0, 107.0, 104.0, 106.0, 1100, 1640995320),
        ];

        let exporter = ChartExporter::new();
        let result = exporter.export_ohlc(&data, &output_path);
        assert!(result.is_ok());

        // Verify the file was created
        assert!(output_path.exists());
        
        // Verify it's a valid PNG (check magic bytes)
        let contents = fs::read(&output_path).unwrap();
        assert!(contents.len() > 8);
        assert_eq!(&contents[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    }

    #[test]
    fn test_line_chart_generation() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_line.png");

        let data = vec![
            Tick::with_spread(100.0, 10, 1640995200, 99.5, 100.5),
            Tick::with_spread(100.5, 15, 1640995201, 100.0, 101.0),
            Tick::with_spread(101.0, 20, 1640995202, 100.5, 101.5),
        ];

        let exporter = ChartExporter::new();
        let result = exporter.export_ticks(&data, &output_path);
        assert!(result.is_ok());

        // Verify the file was created
        assert!(output_path.exists());
        
        // Verify it's a valid PNG
        let contents = fs::read(&output_path).unwrap();
        assert!(contents.len() > 8);
        assert_eq!(&contents[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    }

    #[test]
    fn test_empty_data_error() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_empty.png");

        let data: Vec<OHLC> = vec![];
        let exporter = ChartExporter::new();
        let result = exporter.export_ohlc(&data, &output_path);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot create chart from empty data");
    }

    #[test]
    fn test_sma_calculation() {
        let builder = ChartBuilder::new().ma_period(3);
        let data = vec![
            OHLC::new(100.0, 105.0, 99.0, 103.0, 1000, 1640995200),
            OHLC::new(103.0, 106.0, 102.0, 105.0, 1200, 1640995260),
            OHLC::new(105.0, 107.0, 104.0, 106.0, 1100, 1640995320),
            OHLC::new(106.0, 108.0, 105.0, 107.0, 1300, 1640995380),
        ];

        let sma_values = builder.calculate_sma(&data);
        assert_eq!(sma_values.len(), 2);
        assert!((sma_values[0] - 104.666).abs() < 0.01); // (103 + 105 + 106) / 3
        assert!((sma_values[1] - 106.0).abs() < 0.01);    // (105 + 106 + 107) / 3
    }

    #[test]
    fn test_chart_with_custom_colors() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_custom_colors.png");

        let builder = ChartBuilder::new()
            .dimensions(1024, 768)
            .title("Custom Colors Chart");

        let data = vec![
            OHLC::new(100.0, 105.0, 99.0, 103.0, 1000, 1640995200),
            OHLC::new(103.0, 106.0, 102.0, 101.0, 1200, 1640995260),
        ];

        let exporter = ChartExporter::with_builder(builder);
        let result = exporter.export_ohlc(&data, &output_path);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}
