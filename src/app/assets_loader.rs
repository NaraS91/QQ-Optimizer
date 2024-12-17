use egui::{
    load::{Bytes, BytesPoll},
    Image,
};
use enum_map::{Enum, EnumMap};

use crate::app::hsr::units::UnitKind;

use super::hsr::{
    light_cones::LightConeKind,
    relics::{RelicPart, RelicSet},
    units::SkillData,
};

#[derive(Enum)]
pub enum UnitImageFormat {
    Icon,
    IconScaled,
    SplashIcon,
}

#[derive(Enum)]
pub enum LightConeImageFormat {
    Full,
    Icon,
    Resized,
}

pub struct AssetsLoader<'a> {
    pub loader_prefix: &'a str,
}

const ORIGINAL_UNIT_SIZE: EnumMap<UnitImageFormat, (f32, f32)> =
    EnumMap::from_array([(160., 180.), (96., 112.), (376., 512.)]);

const ORIGINAL_LIGHT_CONE_SIZE: EnumMap<LightConeImageFormat, (f32, f32)> =
    EnumMap::from_array([(867., 1230.), (348., 408.), (86., 123.)]);
const ORIGINAL_RELIC_SET_SIZE: (f32, f32) = (128., 128.);
const RELIC_PIECE_SIZE: (f32, f32) = (256., 256.);
const ROPE_SIZE: (f32, f32) = (512., 512.);

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

    pub fn get_unit_base_data(&self, ctx: &egui::Context, unit: UnitKind) -> Option<Vec<u8>> {
        match ctx
            .try_load_bytes(&format!(
                "{}/assets/characters/data/{}",
                self.loader_prefix,
                unit.file_name()
            ))
            .unwrap()
        {
            BytesPoll::Ready {
                bytes: Bytes::Static(bytes),
                ..
            } => Some(bytes.to_vec()),
            BytesPoll::Ready {
                bytes: Bytes::Shared(bytes),
                ..
            } => Some(bytes.to_vec()),
            BytesPoll::Pending { .. } => None,
        }
    }

    pub fn get_unit_ability(&self, unit: UnitKind, skill: &SkillData) -> Image<'_> {
        Image::new(format!(
            "{}/assets/characters/abilities/{}/{}.webp",
            self.loader_prefix,
            unit.file_name(),
            skill.icon_path
        ))
    }
}
