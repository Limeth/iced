//! Configure a renderer.
use std::sync::Arc;

pub use crate::Antialiasing;

/// The settings of a [`Renderer`].
///
/// [`Renderer`]: ../struct.Renderer.html
#[derive(Debug, Clone)]
pub struct Settings {
    /// The output format of the [`Renderer`].
    ///
    /// [`Renderer`]: ../struct.Renderer.html
    pub format: wgpu::TextureFormat,

    /// The bytes of the font that will be used by default.
    ///
    /// If `None` is provided, a default system font will be chosen.
    pub default_font: Option<&'static [u8]>,

    /// The default size of text.
    ///
    /// By default, it will be set to 20.
    pub default_text_size: u16,

    /// The antialiasing strategy that will be used for triangle primitives.
    pub antialiasing: Option<Antialiasing>,

    /// If `Some`, this instance will be used, otherwise a new instance will be created.
    pub instance: Option<Arc<wgpu::Instance>>,

    /// If `Some`, this device/queue pair will be used, otherwise a new one will be requested.
    pub device_queue: Option<(Arc<wgpu::Device>, Arc<wgpu::Queue>)>,
}

impl PartialEq for Settings {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
            && self.default_font == other.default_font
            && self.default_text_size == other.default_text_size
            && self.antialiasing == other.antialiasing
            && {
                match (self.instance.as_ref(), other.instance.as_ref()) {
                    (Some(a), Some(b)) => Arc::ptr_eq(a, b),
                    (None, None) => true,
                    _ => false,
                }
            }
            && {
                match (self.device_queue.as_ref(), other.device_queue.as_ref())
                {
                    (Some((device_a, queue_a)), Some((device_b, queue_b))) => {
                        Arc::ptr_eq(device_a, device_b)
                            && Arc::ptr_eq(queue_a, queue_b)
                    }
                    (None, None) => true,
                    _ => false,
                }
            }
    }
}

impl Eq for Settings {}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            default_font: None,
            default_text_size: 20,
            antialiasing: None,
            instance: None,
            device_queue: None,
        }
    }
}
