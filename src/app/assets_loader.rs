use egui::{Image};

use crate::app::hsr::units::UnitKind;

use super::hsr::{
    light_cones::LightConeKind,
    relics::{RelicPart, RelicSet},
};
pub enum UnitImageFormat {
    Icon,
    IconScaled,
    SplashIcon,
}

pub enum LightConeImageFormat {
    Full,
    Icon,
    Resized,
}

pub struct AssetsLoader<'a> {
    pub loader_prefix: &'a str,
}

impl AssetsLoader<'_> {
    pub fn get_unit_image(&self, unit_kind: UnitKind, format: UnitImageFormat) -> Image<'_> {
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

    pub fn get_unit_frame(&self) -> Image<'_> {
        Image::new(format!(
            "{}/assets/characters/frame.webp",
            self.loader_prefix,
        ))
    }

    pub fn get_unit_placeholder(&self) -> Image<'_> {
        Image::new(format!(
            "{}/assets/characters/placeholder.webp",
            self.loader_prefix,
        ))
    }

    pub fn get_light_cone_image(
        &self,
        kind: LightConeKind,
        format: LightConeImageFormat,
    ) -> Image<'_> {
        let format_dir = match format {
            LightConeImageFormat::Icon => "icon",
            LightConeImageFormat::Full => "full",
            LightConeImageFormat::Resized => "resized",
        };

        Image::new(format!(
            "{}/assets/light_cones/{}/{}.webp",
            self.loader_prefix,
            format_dir,
            kind.file_name()
        ))
    }

    pub fn get_light_cone_placeholder(&self, format: LightConeImageFormat) -> Image<'_> {
        let format_dir = match format {
            LightConeImageFormat::Icon => "icon",
            LightConeImageFormat::Full => "full",
            LightConeImageFormat::Resized => "resized",
        };

        Image::new(format!(
            "{}/assets/light_cones/{}/placeholder.webp",
            self.loader_prefix, format_dir
        ))
    }

    pub fn get_light_cone_frame(&self, format: LightConeImageFormat) -> Image<'_> {
        let format_dir = match format {
            LightConeImageFormat::Icon => "icon",
            LightConeImageFormat::Full => "full",
            LightConeImageFormat::Resized => "resized",
        };

        Image::new(format!(
            "{}/assets/light_cones/{}/frame.webp",
            self.loader_prefix, format_dir
        ))
    }

    pub fn get_relic_image(&self, set: RelicSet, part: RelicPart) -> Image<'_> {
        Image::new(format!(
            "{}/assets/relics/{}_{}.webp",
            self.loader_prefix,
            set.file_name(),
            part.file_name()
        ))
    }

    pub fn get_relic_part_placeholder(&self, part: RelicPart) -> Image<'_> {
        Image::new(format!(
            "{}/assets/relics/{}.webp",
            self.loader_prefix,
            part.file_name()
        ))
    }

    pub fn get_relic_set_placeholder(&self, set: RelicSet) -> Image<'_> {
        Image::new(format!(
            "{}/assets/relics/{}.webp",
            self.loader_prefix,
            set.file_name()
        ))
    }
}
