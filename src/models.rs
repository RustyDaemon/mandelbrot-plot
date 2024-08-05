use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum PlotColorSchema {
    LinearGradient,
    CubicPolynomial,
    HueRotation,
    LogarithmicMapping,
    Custom,
    Palette,
}

impl FromStr for PlotColorSchema {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "palette" => Ok(PlotColorSchema::Palette),
            "custom" => Ok(PlotColorSchema::Custom),
            "hue" => Ok(PlotColorSchema::HueRotation),
            "log" => Ok(PlotColorSchema::LogarithmicMapping),
            "cubic" => Ok(PlotColorSchema::CubicPolynomial),
            "linear" => Ok(PlotColorSchema::LinearGradient),
            _ => Ok(PlotColorSchema::Palette),
        }
    }
}

impl ToString for PlotColorSchema {
    fn to_string(&self) -> String {
        match self {
            PlotColorSchema::Palette => "palette".to_string(),
            PlotColorSchema::Custom => "custom".to_string(),
            PlotColorSchema::HueRotation => "hue".to_string(),
            PlotColorSchema::LogarithmicMapping => "log".to_string(),
            PlotColorSchema::CubicPolynomial => "cubic".to_string(),
            PlotColorSchema::LinearGradient => "linear".to_string(),
        }
    }
}
