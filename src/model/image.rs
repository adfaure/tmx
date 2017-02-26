use std::io::Read;
use std::str::FromStr;
use error::Error;
use xml::attribute::OwnedAttribute;
use super::reader::{TmxReader, ElementReader, read_num};
use color::Color;
use model::data::Data;

#[derive(Debug, Default)]
pub struct Image {
    format: String,
    source: String,
    trans: Option<Color>,
    width: u32,
    height: u32,
    data: Option<Data>,
}

impl Image {
    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn trans(&self) -> Option<&Color> {
        self.trans.as_ref()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }

    fn set_format<S: Into<String>>(&mut self, format: S) {
        self.format = format.into();
    }

    fn set_source<S: Into<String>>(&mut self, source: S) {
        self.source = source.into();
    }

    fn set_trans(&mut self, color: Color) {
        self.trans = Some(color);
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    fn set_data(&mut self, data: Data) {
        self.data = Some(data);
    }
}

impl<R: Read> ElementReader<Image> for TmxReader<R> {
    fn read_attributes(&mut self, image: &mut Image, name: &str, value: &str) -> ::Result<()> {
        match name {
            "format" => {
                image.set_format(value);
            }
            "source" => {
                image.set_source(value);
            }
            "trans" => {
                let color = try!(Color::from_str(value));
                image.set_trans(color);
            }
            "width" => {
                let width = try!(read_num(value));
                image.set_width(width);
            }
            "height" => {
                let height = try!(read_num(value));
                image.set_height(height);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, image: &mut Image, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "data" => {
                let data = try!(self.on_data(attributes));
                image.set_data(data);
            }
            _ => {}
        };
        Ok(())
    }
}

