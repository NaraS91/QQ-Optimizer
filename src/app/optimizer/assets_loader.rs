use egui::Image;

use crate::app::hsr::units::UnitKind;
pub enum UnitImageFormat {
    Icon,
    IconScaled,
    SplashIcon,
}
pub enum LoaderType {
    Local,
    Web,
}
pub struct AssetsLoader {
    loader_prefix: String,
}

impl AssetsLoader {
    pub fn new(loader_type: LoaderType) -> Self {
        return AssetsLoader {
            loader_prefix: if matches!(loader_type, LoaderType::Local) {
                "file:/".to_owned()
            } else {
                "".to_owned()
            },
        };
    }
    pub fn get_unit_image(&self, unit_kind: UnitKind, format: UnitImageFormat) -> Image {
        let format_dir = match format {
            UnitImageFormat::Icon => "icon",
            UnitImageFormat::IconScaled => "icon_scaled",
            UnitImageFormat::SplashIcon => "splash_icon",
        };

        Image::new(format!(
            "{}/assets/characters/{}/{}.webp",
            self.loader_prefix,
            format_dir,
            unit_kind.file_name()
        ))
    }
}
