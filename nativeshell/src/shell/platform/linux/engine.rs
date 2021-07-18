use gtk::prelude::WidgetExt;

use super::{
    binary_messenger::PlatformBinaryMessenger,
    error::PlatformResult,
    flutter::{self, EngineExt, ViewExt},
    texture::PlatformTextureRegistry,
};

pub struct PlatformEngine {
    pub(super) view: flutter::View,
    texture_registry: PlatformTextureRegistry,
}

pub struct PlatformPlugin {
    pub name: String,
    pub register_func: Option<unsafe extern "C" fn(registrar: *mut std::os::raw::c_void)>,
}

impl PlatformEngine {
    pub fn new(plugins: &[PlatformPlugin]) -> Self {
        let project = flutter::DartProject::new();
        let view = flutter::View::new(&project);
        for plugin in plugins {
            let registrar = view.get_registrar_for_plugin(&plugin.name);
            if let Some(func) = plugin.register_func {
                unsafe {
                    func(registrar);
                }
            }
        }
        let texture_registrar = view.get_engine().get_texture_registrar();
        PlatformEngine {
            view,
            texture_registry: PlatformTextureRegistry::new(texture_registrar),
        }
    }

    pub fn new_binary_messenger(&self) -> PlatformBinaryMessenger {
        PlatformBinaryMessenger::new(self.view.get_engine().get_binary_messenger())
    }

    pub fn launch(&mut self) -> PlatformResult<()> {
        // This assumes the view has already been added to GtkWindow
        self.view.realize();
        Ok(())
    }

    pub fn shut_down(&mut self) -> PlatformResult<()> {
        Ok(())
    }

    pub(crate) fn texture_registry(&self) -> &PlatformTextureRegistry {
        &self.texture_registry
    }
}
