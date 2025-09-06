//! Integration tests for PNG chart export functionality

#[cfg(feature = "png_export")]
mod png_export_tests {
    use market_data_source::{
        MarketDataGenerator, GeneratorConfig,
        export::{ChartBuilder, ChartExporter, to_png_ohlc, to_png_ticks},
    };
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_export_candlestick_chart() {
        // Generate test data
        let config = GeneratorConfig::volatile();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(50);

        // Create temporary directory for output
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("candlestick_chart.png");

        // Export to PNG
        let result = to_png_ohlc(&ohlc_data, &output_path);
        assert!(result.is_ok(), "Failed to export candlestick chart: {:?}", result);

        // Verify file exists and is valid PNG
        assert!(output_path.exists());
        let file_size = fs::metadata(&output_path).unwrap().len();
        assert!(file_size > 1000, "PNG file is too small: {} bytes", file_size);

        // Check PNG magic bytes
        let contents = fs::read(&output_path).unwrap();
        assert!(contents.len() >= 8);
        assert_eq!(
            &contents[0..8],
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            "Invalid PNG magic bytes"
        );
    }

    #[test]
    fn test_export_line_chart() {
        // Generate test data
        let config = GeneratorConfig::volatile();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let tick_data = generator.generate_ticks(100);

        // Create temporary directory for output
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("line_chart.png");

        // Export to PNG
        let result = to_png_ticks(&tick_data, &output_path);
        assert!(result.is_ok(), "Failed to export line chart: {:?}", result);

        // Verify file exists and is valid PNG
        assert!(output_path.exists());
        let file_size = fs::metadata(&output_path).unwrap().len();
        assert!(file_size > 1000, "PNG file is too small: {} bytes", file_size);

        // Check PNG header
        let contents = fs::read(&output_path).unwrap();
        assert!(contents.len() >= 8);
        assert_eq!(&contents[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    }

    #[test]
    fn test_custom_chart_configuration() {
        // Generate test data
        let config = GeneratorConfig::stable();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(30);

        // Create custom chart builder
        let chart_builder = ChartBuilder::new()
            .dimensions(1280, 720)
            .title("Custom Market Data Chart")
            .show_volume(true)
            .show_moving_average(true)
            .ma_period(10)
;

        // Create temporary directory for output
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("custom_chart.png");

        // Export with custom builder
        let exporter = ChartExporter::with_builder(chart_builder);
        let result = exporter.export_ohlc(&ohlc_data, &output_path);
        assert!(result.is_ok(), "Failed to export custom chart: {:?}", result);

        // Verify file exists
        assert!(output_path.exists());
        let metadata = fs::metadata(&output_path).unwrap();
        assert!(metadata.len() > 1000);
    }

    #[test]
    fn test_chart_without_volume() {
        // Generate test data
        let config = GeneratorConfig::bull_market();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(24);

        // Create chart without volume
        let chart_builder = ChartBuilder::new()
            .dimensions(800, 600)
            .title("Chart Without Volume")
            .show_volume(false)
            .show_moving_average(true)
            .ma_period(5);

        // Create temporary directory for output
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("no_volume_chart.png");

        // Export chart
        let exporter = ChartExporter::with_builder(chart_builder);
        let result = exporter.export_ohlc(&ohlc_data, &output_path);
        assert!(result.is_ok());

        // Verify file exists
        assert!(output_path.exists());
    }

    #[test]
    fn test_chart_without_moving_average() {
        // Generate test data
        let config = GeneratorConfig::bear_market();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(40);

        // Create chart without moving average
        let chart_builder = ChartBuilder::new()
            .dimensions(1024, 768)
            .title("Chart Without MA")
            .show_volume(true)
            .show_moving_average(false);

        // Create temporary directory for output
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("no_ma_chart.png");

        // Export chart
        let exporter = ChartExporter::with_builder(chart_builder);
        let result = exporter.export_ohlc(&ohlc_data, &output_path);
        assert!(result.is_ok());

        // Verify file exists
        assert!(output_path.exists());
    }

    #[test]
    fn test_large_dataset_performance() {
        // Generate large dataset
        let config = GeneratorConfig::volatile();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(1000);

        // Create temporary directory for output
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("large_dataset_chart.png");

        // Measure export time
        let start = std::time::Instant::now();
        let result = to_png_ohlc(&ohlc_data, &output_path);
        let duration = start.elapsed();

        assert!(result.is_ok());
        assert!(output_path.exists());
        
        // Ensure it completes in reasonable time (< 10 seconds)
        assert!(
            duration.as_secs() < 10,
            "Chart generation took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_empty_data_handling() {
        let empty_ohlc = vec![];
        let empty_ticks = vec![];

        let temp_dir = tempdir().unwrap();
        let ohlc_path = temp_dir.path().join("empty_ohlc.png");
        let tick_path = temp_dir.path().join("empty_ticks.png");

        // Both should return errors for empty data
        let ohlc_result = to_png_ohlc(&empty_ohlc, &ohlc_path);
        assert!(ohlc_result.is_err());
        assert_eq!(
            ohlc_result.unwrap_err().to_string(),
            "Cannot create chart from empty data"
        );

        let tick_result = to_png_ticks(&empty_ticks, &tick_path);
        assert!(tick_result.is_err());
        assert_eq!(
            tick_result.unwrap_err().to_string(),
            "Cannot create chart from empty data"
        );
    }

    #[test]
    fn test_minimal_data_chart() {
        // Test with minimal data (just 2 points)
        let config = GeneratorConfig::bear_market();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(2);
        let tick_data = generator.generate_ticks(2);

        let temp_dir = tempdir().unwrap();
        let ohlc_path = temp_dir.path().join("minimal_ohlc.png");
        let tick_path = temp_dir.path().join("minimal_ticks.png");

        // Both should succeed with minimal data
        assert!(to_png_ohlc(&ohlc_data, &ohlc_path).is_ok());
        assert!(to_png_ticks(&tick_data, &tick_path).is_ok());

        assert!(ohlc_path.exists());
        assert!(tick_path.exists());
    }
}