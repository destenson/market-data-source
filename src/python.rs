// Python bindings for market-data-source using PyO3 with automated code generation

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use rust_decimal::prelude::*;

use crate::{
    MarketDataGenerator, GeneratorConfig, ConfigBuilder, OHLC, Tick, TimeInterval,
    export::{DataExporter, CsvExporter, JsonExporter, json::JsonOptions, chart::{ChartExporter, ChartBuilder}}
};

// Automated conversion trait for OHLC to Python dict
impl IntoPy<PyObject> for OHLC {
    fn into_py(self, py: Python) -> PyObject {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("timestamp", self.timestamp).unwrap();
        dict.set_item("open", self.open.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("high", self.high.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("low", self.low.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("close", self.close.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("volume", self.volume.value()).unwrap();
        dict.into()
    }
}

// Automated conversion trait for Tick to Python dict
impl IntoPy<PyObject> for Tick {
    fn into_py(self, py: Python) -> PyObject {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("timestamp", self.timestamp).unwrap();
        dict.set_item("price", self.price.to_f64().unwrap_or(0.0)).unwrap();
        
        // Handle optional bid/ask
        if let (Some(bid), Some(ask)) = (self.bid, self.ask) {
            dict.set_item("bid", bid.to_f64().unwrap_or(0.0)).unwrap();
            dict.set_item("ask", ask.to_f64().unwrap_or(0.0)).unwrap();
            dict.set_item("spread", (ask - bid).to_f64().unwrap_or(0.0)).unwrap();
        } else {
            dict.set_item("bid", self.price.to_f64().unwrap_or(0.0)).unwrap();
            dict.set_item("ask", self.price.to_f64().unwrap_or(0.0)).unwrap();
            dict.set_item("spread", 0.0).unwrap();
        }
        
        dict.set_item("volume", self.volume.value()).unwrap();
        dict.into()
    }
}

// Automated conversion for TimeInterval
impl IntoPy<PyObject> for TimeInterval {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            TimeInterval::OneMinute => "1m",
            TimeInterval::FiveMinutes => "5m",
            TimeInterval::FifteenMinutes => "15m",
            TimeInterval::ThirtyMinutes => "30m",
            TimeInterval::OneHour => "1h",
            TimeInterval::FourHours => "4h",
            TimeInterval::OneDay => "1d",
            TimeInterval::Custom(seconds) => return format!("{}s", seconds).into_py(py),
        }.into_py(py)
    }
}

/// Python wrapper for GeneratorConfig with automatic getters
#[pyclass(name = "GeneratorConfig")]
#[derive(Clone)]
pub struct PyGeneratorConfig {
    inner: GeneratorConfig,
}

#[pymethods]
impl PyGeneratorConfig {
    #[getter]
    fn initial_price(&self) -> f64 {
        self.inner.starting_price.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn volatility(&self) -> f64 {
        self.inner.volatility.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn trend_strength(&self) -> f64 {
        self.inner.trend_strength.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn trend_direction(&self) -> String {
        format!("{:?}", self.inner.trend_direction)
    }
    
    #[getter]
    fn min_price(&self) -> f64 {
        self.inner.min_price.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn max_price(&self) -> f64 {
        self.inner.max_price.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn base_volume(&self) -> u64 {
        self.inner.base_volume
    }
    
    #[getter]
    fn volume_volatility(&self) -> f64 {
        self.inner.volume_volatility
    }
    
    #[getter]
    fn time_interval(&self) -> String {
        format!("{:?}", self.inner.time_interval)
    }
    
    #[getter]
    fn seed(&self) -> Option<u64> {
        self.inner.seed
    }
    
    fn __repr__(&self) -> String {
        format!(
            "GeneratorConfig(initial_price={}, volatility={}, trend_strength={})",
            self.initial_price(),
            self.volatility(),
            self.trend_strength()
        )
    }
}

/// Python wrapper for MarketDataGenerator with automated methods
#[pyclass(name = "MarketDataGenerator")]
pub struct PyMarketDataGenerator {
    generator: MarketDataGenerator,
}

#[pymethods]
impl PyMarketDataGenerator {
    /// Create a new MarketDataGenerator with kwargs
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&Bound<'_, pyo3::types::PyDict>>) -> PyResult<Self> {
        let mut builder = ConfigBuilder::new();
        
        if let Some(dict) = kwargs {
            // Automatically extract and set all parameters from kwargs
            if let Ok(Some(val)) = dict.get_item("initial_price") {
                builder = builder.starting_price_f64(val.extract::<f64>()?);
            }
            if let Ok(Some(val)) = dict.get_item("volatility") {
                builder = builder.volatility_f64(val.extract::<f64>()?);
            }
            if let Ok(Some(val)) = dict.get_item("trend") {
                // For backwards compatibility, interpret single trend value as bullish/bearish direction
                let trend_val = val.extract::<f64>()?;
                use crate::config::TrendDirection;
                let direction = if trend_val > 0.0 {
                    TrendDirection::Bullish
                } else if trend_val < 0.0 {
                    TrendDirection::Bearish
                } else {
                    TrendDirection::Sideways
                };
                builder = builder.trend_f64(direction, trend_val.abs());
            }
            if let Ok(Some(val)) = dict.get_item("min_price") {
                if let Ok(Some(max_val)) = dict.get_item("max_price") {
                    builder = builder.price_range_f64(val.extract::<f64>()?, max_val.extract::<f64>()?);
                } else {
                    builder = builder.price_range_f64(val.extract::<f64>()?, 1e15);
                }
            } else if let Ok(Some(val)) = dict.get_item("max_price") {
                builder = builder.price_range_f64(1.0, val.extract::<f64>()?);
            }
            if let Ok(Some(val)) = dict.get_item("volume_base") {
                builder = builder.base_volume(val.extract::<u64>()?);
            }
            if let Ok(Some(val)) = dict.get_item("volume_volatility") {
                builder = builder.volume_volatility(val.extract::<f64>()?);
            }
            if let Ok(Some(val)) = dict.get_item("seed") {
                builder = builder.seed(val.extract::<u64>()?);
            }
            if let Ok(Some(val)) = dict.get_item("interval") {
                let interval_str = val.extract::<&str>()?;
                let interval = parse_interval(interval_str)?;
                builder = builder.time_interval(interval);
            }
        }
        
        let config = builder.build()
            .map_err(|e| PyValueError::new_err(format!("Configuration error: {}", e)))?;
        let generator = MarketDataGenerator::with_config(config)
            .map_err(|e| PyValueError::new_err(format!("Failed to create generator: {}", e)))?;
        Ok(PyMarketDataGenerator { generator })
    }
    
    /// Generate OHLC data series - returns list of dicts
    fn generate_series(&mut self, count: usize) -> Vec<OHLC> {
        self.generator.generate_series(count)
    }
    
    /// Generate tick data - returns list of dicts
    fn generate_ticks(&mut self, count: usize) -> Vec<Tick> {
        self.generator.generate_ticks(count)
    }
    
    /// Generate data between timestamps
    fn generate_series_between(&mut self, start: i64, end: i64) -> PyResult<Vec<OHLC>> {
        // Set starting timestamp
        self.generator.set_timestamp(start);
        
        // Calculate how many points to generate based on time interval
        let duration_ms = end - start;
        let interval_ms = match self.generator.config().time_interval {
            TimeInterval::OneMinute => 60_000,
            TimeInterval::FiveMinutes => 300_000,
            TimeInterval::FifteenMinutes => 900_000,
            TimeInterval::ThirtyMinutes => 1_800_000,
            TimeInterval::OneHour => 3_600_000,
            TimeInterval::FourHours => 14_400_000,
            TimeInterval::OneDay => 86_400_000,
            TimeInterval::Custom(secs) => secs as i64 * 1000,
        };
        
        let count = (duration_ms / interval_ms).max(1) as usize;
        Ok(self.generator.generate_series(count))
    }
    
    /// Export data to CSV file
    fn to_csv(&mut self, path: &str, count: usize) -> PyResult<()> {
        let data = self.generator.generate_series(count);
        let exporter = CsvExporter::new();
        exporter.export_ohlc(&data, path)
            .map_err(|e| PyValueError::new_err(format!("Export failed: {}", e)))?;
        Ok(())
    }
    
    /// Export data to JSON file
    #[pyo3(signature = (path, count, lines=None))]
    fn to_json(&mut self, path: &str, count: usize, lines: Option<bool>) -> PyResult<()> {
        let data = self.generator.generate_series(count);
        let exporter = if lines.unwrap_or(false) {
            JsonExporter::with_options(JsonOptions::json_lines())
        } else {
            JsonExporter::new()
        };
        exporter.export_ohlc(&data, path)
            .map_err(|e| PyValueError::new_err(format!("Export failed: {}", e)))?;
        Ok(())
    }
    
    /// Export data to PNG chart
    #[pyo3(signature = (path, count, **kwargs))]
    fn to_png(&mut self, path: &str, count: usize, kwargs: Option<&Bound<'_, pyo3::types::PyDict>>) -> PyResult<()> {
        let data = self.generator.generate_series(count);
        
        // Build the chart configuration from kwargs
        let mut builder = ChartBuilder::new();
        
        if let Some(dict) = kwargs {
            if let Ok(Some(width)) = dict.get_item("width") {
                if let Ok(Some(height)) = dict.get_item("height") {
                    let w: u32 = width.extract()?;
                    let h: u32 = height.extract()?;
                    builder = builder.dimensions(w, h);
                }
            }
            if let Ok(Some(title)) = dict.get_item("title") {
                let t: String = title.extract()?;
                builder = builder.title(&t);
            }
            if let Ok(Some(volume)) = dict.get_item("volume") {
                let v: bool = volume.extract()?;
                builder = builder.show_volume(v);
            }
        }
        
        let exporter = ChartExporter::with_builder(builder);
        
        exporter.export_ohlc(&data, path)
            .map_err(|e| PyValueError::new_err(format!("Export failed: {}", e)))?;
        Ok(())
    }
    
    /// Get current configuration
    #[getter]
    fn config(&self) -> PyGeneratorConfig {
        PyGeneratorConfig {
            inner: self.generator.config().clone(),
        }
    }
    
    /// Set a new seed for the random number generator
    fn set_seed(&mut self, seed: u64) -> PyResult<()> {
        // Recreate generator with same config but new seed
        let mut config = self.generator.config().clone();
        config.seed = Some(seed);
        self.generator = MarketDataGenerator::with_config(config)
            .map_err(|e| PyValueError::new_err(format!("Failed to set seed: {}", e)))?;
        Ok(())
    }
    
    /// Reset generator with new config
    fn reset(&mut self) -> PyResult<()> {
        let config = self.generator.config().clone();
        self.generator = MarketDataGenerator::with_config(config)
            .map_err(|e| PyValueError::new_err(format!("Failed to reset: {}", e)))?;
        Ok(())
    }
    
    fn __repr__(&self) -> String {
        format!("MarketDataGenerator(config={})", self.config().__repr__())
    }
}

// Helper function to parse interval strings
fn parse_interval(s: &str) -> PyResult<TimeInterval> {
    match s {
        "1m" => Ok(TimeInterval::OneMinute),
        "5m" => Ok(TimeInterval::FiveMinutes),
        "15m" => Ok(TimeInterval::FifteenMinutes),
        "30m" => Ok(TimeInterval::ThirtyMinutes),
        "1h" => Ok(TimeInterval::OneHour),
        "4h" => Ok(TimeInterval::FourHours),
        "1d" => Ok(TimeInterval::OneDay),
        _ => Err(PyValueError::new_err(format!("Invalid interval: {}", s))),
    }
}

// Auto-generate preset functions using a macro
macro_rules! preset_function {
    ($name:ident, $config_method:ident) => {
        #[pyfunction]
        fn $name() -> PyResult<PyMarketDataGenerator> {
            let config = GeneratorConfig::$config_method();
            let generator = MarketDataGenerator::with_config(config)
                .map_err(|e| PyValueError::new_err(format!("Failed to create generator: {}", e)))?;
            Ok(PyMarketDataGenerator { generator })
        }
    };
}

preset_function!(volatile_config, volatile);
preset_function!(stable_config, stable);
preset_function!(bull_market_config, bull_market);
preset_function!(bear_market_config, bear_market);

/// Python module definition with automatic registration
#[pymodule]
fn _market_data_source(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Auto-register classes
    m.add_class::<PyMarketDataGenerator>()?;
    m.add_class::<PyGeneratorConfig>()?;
    
    // Auto-register preset functions
    m.add_function(wrap_pyfunction!(volatile_config, m)?)?;
    m.add_function(wrap_pyfunction!(stable_config, m)?)?;
    m.add_function(wrap_pyfunction!(bull_market_config, m)?)?;
    m.add_function(wrap_pyfunction!(bear_market_config, m)?)?;
    
    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "")?;  // Empty string if not set
    
    Ok(())
}