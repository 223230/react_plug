use nih_plug::prelude::*;
use react_plug::prelude::*;

define_params! {
    TestParams {
        gain: FloatParam {
            name: "Gain",
            defaultValue: util::db_to_gain(0.0),
            range: FloatRange::Skewed {
                min: util::db_to_gain(-30.0),
                max: util::db_to_gain(30.0),
                factor: FloatRange::gain_skew_factor(-30.0, 30.0),
            },
            smoother: SmoothingStyle::Logarithmic(50.0),
            unit: " dB",
            value_to_string: formatters::v2s_f32_gain_to_db(2),
            string_to_value: formatters::s2v_f32_gain_to_db(),
        },
        bool_test: BoolParam {
            name: "Bool Test",
            defaultValue: false
        },
        int_test: IntParam {
            name: "Int Test",
            defaultValue: 0,
            range: IntRange::Linear { min: 0, max: 10 }
        }
    }
}

#[derive(Enum, PartialEq)]
pub enum EnumTest {
    A,
    B,
    C,
}

#[derive(Params)]
pub struct ExampleParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "reversed"]
    pub reversed: FloatParam,
    #[id = "bool_test"]
    pub bool_test: BoolParam,
    #[id = "int_test"]
    pub int_test: IntParam,
    #[id = "enum_test"]
    pub enum_test: EnumParam<EnumTest>,
}

impl Default for ExampleParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Linear {
                    min: util::db_to_gain(-60.0),
                    max: util::db_to_gain(6.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            reversed: FloatParam::new(
                "Reversed",
                0.0,
                FloatRange::Reversed(&FloatRange::Linear { min: 0.0, max: 1.0 }),
            ),
            bool_test: BoolParam::new("Bool Test", false),
            int_test: IntParam::new("Int Test", 0, IntRange::Linear { min: 0, max: 10 }),
            enum_test: EnumParam::new("Enum Test", EnumTest::A),
        }
    }
}

impl ExampleParams {
    pub fn new() -> Self {
        Self::default()
    }
}
