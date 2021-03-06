use failure;
use std::io;
use std::result;
pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Fail, Debug)]
#[fail(display = "Error loading shader {}: {}", shadername, error)]
pub struct LoadShaderError {
    shadername: String,
    error: io::Error,
}
impl LoadShaderError {
    pub fn new(shadername: &str, error: io::Error) -> LoadShaderError {
        LoadShaderError {
            shadername: shadername.to_string(),
            error,
        }
    }
}
#[derive(Fail, Debug)]
#[fail(display = "Failed to find example shader {}", example)]
pub struct FindExampleShaderError {
    example: String
}
impl FindExampleShaderError {
    pub fn new(example: &str) -> FindExampleShaderError {
        FindExampleShaderError {
            example: example.to_string(),
        }
    }
}
#[derive(Fail, Debug)]
#[fail(display = "Invalid shader ID: {}", id)]
pub struct InvalidShaderIdError {
    id: String
}
impl InvalidShaderIdError {
    pub fn new(id: &str) -> InvalidShaderIdError {
        InvalidShaderIdError {
            id: id.to_string()
        }
    }
}
#[derive(Fail, Debug)]
#[fail(display = "Error saving shader {}: {}", shadername, error)]
pub struct SaveShaderError {
    shadername: String,
    error: io::Error,
}
impl SaveShaderError {
    pub fn new(shadername: &str, error: io::Error) -> SaveShaderError {
        SaveShaderError {
            shadername: shadername.to_string(),
            error,
        }
    }
}
#[derive(Fail, Debug)]
#[fail(display = "The following uniforms are not supported: {:?}", unsupported_uniforms)]
pub struct UnsupportedUniformError {
    unsupported_uniforms: Vec<String>,
}
impl UnsupportedUniformError {
    pub fn new(unsupported_uniforms: Vec<String>) -> UnsupportedUniformError {
        UnsupportedUniformError {
            unsupported_uniforms,
        }
    }
}