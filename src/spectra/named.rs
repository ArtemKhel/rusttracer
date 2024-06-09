use crate::spectra::{Spectrum, SpectrumEnum};

pub struct NamedSpectrum {}
impl NamedSpectrum {
    #[allow(clippy::new_ret_no_self)]
    fn new(name: &str) -> SpectrumEnum { todo!() }
}
impl Spectrum for NamedSpectrum {
    fn value(&self, wavelength: f32) -> f32 { todo!() }
}
