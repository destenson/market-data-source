// Python bindings for market-data-source using PyO3 with automated code generation

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::{
    MarketDataGenerator, GeneratorConfig, ConfigBuilder, OHLC, Tick, TimeInterval,
    export::{DataExporter, CSVExporter, JSONExporter, PNGExporter}
};

// Automated conversion trait for OHLC to Python dict
impl IntoPy<PyObject> for OHLC {
    fn into_py(self, py: Python) -> PyObject {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("timestamp", self.timestamp.timestamp()).unwrap();
        dict.set_item("open", self.open.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("high", self.high.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("low", self.low.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("close", self.close.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("volume", self.volume.to_f64().unwrap_or(0.0)).unwrap();
        dict.into()
    }
}

// Automated conversion trait for Tick to Python dict
impl IntoPy<PyObject> for Tick {
    fn into_py(self, py: Python) -> PyObject {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("timestamp", self.timestamp.timestamp()).unwrap();
        dict.set_item("bid", self.bid.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("ask", self.ask.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("spread", self.spread.to_f64().unwrap_or(0.0)).unwrap();
        dict.set_item("volume", self.volume.to_f64().unwrap_or(0.0)).unwrap();
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
        }.into_py(py)
    }
}

// Helper macro to generate Python wrapper methods
macro_rules! py_export_method {
    ($name:ident, $exporter_type:ty, $method:ident) => {
        fn $name(&mut self, path: &str, count: usize) -> PyResult<()> {
            let data = self.generator.generate_series(count);
            let exporter = <$exporter_type>::new();
            exporter.$method(&data, path)
                .map_err(|e| PyValueError::new_err(format!("Export failed: {}", e)))?;
            Ok(())
        }
    };
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
        self.inner.initial_price.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn volatility(&self) -> f64 {
        self.inner.volatility.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn trend(&self) -> f64 {
        self.inner.trend.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn min_price(&self) -> Option<f64> {
        self.inner.min_price.map(|p| p.to_f64().unwrap_or(0.0))
    }
    
    #[getter]
    fn max_price(&self) -> Option<f64> {
        self.inner.max_price.map(|p| p.to_f64().unwrap_or(0.0))
    }
    
    #[getter]
    fn volume_base(&self) -> f64 {
        self.inner.volume_base.to_f64().unwrap_or(0.0)
    }
    
    #[getter]
    fn volume_volatility(&self) -> f64 {
        self.inner.volume_volatility
    }
    
    #[getter]
    fn interval(&self) -> String {
        format!("{:?}", self.inner.interval)
    }
    
    #[getter]
    fn seed(&self) -> Option<u64> {
        self.inner.seed
    }
    
    fn __repr__(&self) -> String {
        format!(
            "GeneratorConfig(initial_price={}, volatility={}, trend={})",
            self.initial_price(),
            self.volatility(),
            self.trend()
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
    fn new(kwargs: Option<&pyo3::types::PyDict>) -> PyResult<Self> {
        let mut builder = ConfigBuilder::new();
        
        if let Some(dict) = kwargs {
            // Automatically extract and set all parameters from kwargs
            if let Ok(val) = dict.get_item("initial_price") {
                if let Some(v) = val {
                    builder = builder.initial_price_f64(v.extract::<f64>()?);
                }
            }
            if let Ok(val) = dict.get_item("volatility") {
                if let Some(v) = val {
                    builder = builder.volatility_f64(v.extract::<f64>()?);
                }
            }
            if let Ok(val) = dict.get_item("trend") {
                if let Some(v) = val {
                    builder = builder.trend_f64(v.extract::<f64>()?);
                }
            }
            if let Ok(val) = dict.get_item("min_price") {
                if let Some(v) = val {
                    builder = builder.min_price_f64(v.extract::<f64>()?);
                }
            }
            if let Ok(val) = dict.get_item("max_price") {
                if let Some(v) = val {
                    builder = builder.max_price_f64(v.extract::<f64>()?);
                }
            }
            if let Ok(val) = dict.get_item("volume_base") {
                if let Some(v) = val {
                    builder = builder.volume_base_f64(v.extract::<f64>()?);
                }
            }
            if let Ok(val) = dict.get_item("volume_volatility") {
                if let Some(v) = val {
                    builder = builder.volume_volatility(v.extract::<f64>()?);
                }
            }
            if let Ok(val) = dict.get_item("seed") {
                if let Some(v) = val {
                    builder = builder.seed(v.extract::<u64>()?);
                }
            }
            if let Ok(val) = dict.get_item("interval") {
                if let Some(v) = val {
                    let interval_str = v.extract::<&str>()?;
                    let interval = parse_interval(interval_str)?;
                    builder = builder.interval(interval);
                }
            }
        }
        
        let config = builder.build();
        let generator = MarketDataGenerator::new(config);
        Ok(PyMarketDataGenerator { generator })
    }
    
    /// Generate OHLC data series - returns list of dicts
    fn generate_series(&mut self, count: usize) -> Vec<OHLC> {
        self.generator.generate_series(count)
    }
    
    /// Generate tick data - returns list of dicts
    fn generate_ticks(&mut self, count: usize, spread: Option<f64>) -> Vec<Tick> {
        let spread_decimal = spread
            .map(|s| Decimal::from_f64_retain(s).unwrap_or(Decimal::new(1, 2)))
            .unwrap_or(Decimal::new(1, 2));
        self.generator.generate_ticks(count, spread_decimal)
    }
    
    /// Generate data between timestamps
    fn generate_series_between(&mut self, start: i64, end: i64) -> PyResult<Vec<OHLC>> {
        use chrono::{DateTime, Utc, TimeZone};
        
        let start_dt = Utc.timestamp_opt(start, 0)
            .single()
            .ok_or_else(|| PyValueError::new_err("Invalid start timestamp"))?;
        let end_dt = Utc.timestamp_opt(end, 0)
            .single()
            .ok_or_else(|| PyValueError::new_err("Invalid end timestamp"))?;
        
        Ok(self.generator.generate_series_between(start_dt, end_dt))
    }
    
    // Auto-generate export methods
    py_export_method!(to_csv, CSVExporter, export_ohlc);
    
    fn to_json(&mut self, path: &str, count: usize, lines: Option<bool>) -> PyResult<()> {
        let data = self.generator.generate_series(count);
        let exporter = JSONExporter::new(lines.unwrap_or(false));
        exporter.export_ohlc(&data, path)
            .map_err(|e| PyValueError::new_err(format!("Export failed: {}", e)))?;
        Ok(())
    }
    
    fn to_png(&mut self, path: &str, count: usize, kwargs: Option<&pyo3::types::PyDict>) -> PyResult<()> {
        let data = self.generator.generate_series(count);
        let mut exporter = PNGExporter::new();
        
        // Auto-extract PNG parameters from kwargs
        if let Some(dict) = kwargs {
            if let Ok(Some(v)) = dict.get_item("width") {
                if let Ok(Some(h)) = dict.get_item("height") {
                    exporter = exporter.with_dimensions(v.extract()?, h.extract()?);
                }
            }
            if let Ok(Some(v)) = dict.get_item("title") {
                exporter = exporter.with_title(v.extract()?);
            }
            if let Ok(Some(v)) = dict.get_item("volume") {
                exporter = exporter.with_volume(v.extract()?);
            }
        }
        
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
    
    /// Set new seed
    fn set_seed(&mut self, seed: u64) {
        self.generator.set_seed(seed);
    }
    
    /// Reset generator
    fn reset(&mut self) {
        let config = self.generator.config().clone();
        self.generator = MarketDataGenerator::new(config);
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
        fn $name() -> PyMarketDataGenerator {
            PyMarketDataGenerator {
                generator: MarketDataGenerator::new(GeneratorConfig::$config_method())
            }
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
    m.add("__author__", env!("CARGO_PKG_AUTHORS"))?;
    
    Ok(())
}