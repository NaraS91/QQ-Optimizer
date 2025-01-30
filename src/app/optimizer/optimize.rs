use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::{thread, time};

use egui::{Margin, Ui};
use enum_map::Enum;
use strum::IntoEnumIterator;

use self::{
    optimized_unit_card::OptimizedUnit,
    relics_filter_card::{RelicsLevelFilter, RelicsStatsFilter},
    requirements_card::Requirements,
};
use crate::app::{
    hsr::{
        relics::{
            buffs::{get_cavern_set_effects, get_planar_set_effects},
            CavernSet, PlanarSet, Relic, RelicSet,
        },
        units::{Modifier, Unit, UnitKind},
    },
    light_cones_store::LightConesStore,
    relics_store::RelicsStore,
    COLOR_PALLET,
};

mod optimized_unit_card;
mod relics_filter_card;
mod requirements_card;

pub struct ThreadMessage {
    optimal_set: [Option<Relic>; 6],
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Optimize {
    unit_kind: UnitKind,
    #[serde(skip_serializing, skip_deserializing)]
    pub main_unit_buffs: Vec<(Unit, Modifier)>,
    unit_card: OptimizedUnit,
    relics_level_filter: RelicsLevelFilter,
    relics_stat_filter: RelicsStatsFilter,
    requirements: Requirements,
    optimal_set: Option<[Option<Relic>; 6]>,

    //ui state
    is_optimizing: bool,
    #[serde(skip_serializing, skip_deserializing)]
    channels: Option<(Arc<Sender<ThreadMessage>>, Arc<Receiver<ThreadMessage>>)>,
}

impl Default for Optimize {
    fn default() -> Self {
        Optimize::new(UnitKind::Qingque)
    }
}

impl Optimize {
    pub fn new(unit_kind: UnitKind) -> Optimize {
        let (tx, rx) = channel();
        Optimize {
            unit_kind: unit_kind,
            main_unit_buffs: Vec::new(),
            unit_card: OptimizedUnit::new(unit_kind),
            relics_level_filter: RelicsLevelFilter::new(0),
            relics_stat_filter: RelicsStatsFilter::new(),
            requirements: Requirements::new(),
            optimal_set: None,
            is_optimizing: false,
            channels: Some((Arc::new(tx), Arc::new(rx))),
        }
    }

    pub fn set_main_unit_buffs(&mut self, buffs: Vec<(Unit, Modifier)>) {
        self.main_unit_buffs = buffs;
    }

    pub fn show_ui(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        app_ctx: &mut crate::app::AppContext,
    ) {
        let top_card = egui::containers::Frame::default().outer_margin(Margin {
            left: 0.,
            right: 0.,
            top: 10.,
            bottom: 0.,
        });

        egui::containers::TopBottomPanel::top("top_cards")
            .frame(top_card)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |ui| self.show_in_top_panel(ui, app_ctx));

        let bottom_card = egui::containers::Frame::default().outer_margin(Margin {
            left: 0.,
            right: 0.,
            top: 10.,
            bottom: 0.,
        });

        egui::containers::TopBottomPanel::bottom("relic_optimizing")
            .frame(bottom_card)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |ui| self.show_in_bottom_panel(ctx, ui, app_ctx));
    }

    fn show_in_bottom_panel(
        &mut self,
        ctx: &egui::Context,
        ui: &mut Ui,
        app_ctx: &mut crate::app::AppContext,
    ) {
        self.show_optimizing_button(ctx, ui, app_ctx);
    }

    fn show_optimizing_button(
        &mut self,
        ctx: &egui::Context,
        ui: &mut Ui,
        app_ctx: &mut crate::app::AppContext,
    ) {
        let main_unit_op = app_ctx.units_store.get_unit(ctx, self.unit_kind);
        if main_unit_op.is_none() {
            ui.add_enabled(false, egui::Button::new("Loading..."));
            return;
        }

        if self.is_optimizing {
            ui.add_enabled(!self.is_optimizing, egui::Button::new("Optimizing..."));
            if let Ok(msg) = self.channels.as_ref().unwrap().1.try_recv() {
                self.optimal_set = Some(msg.optimal_set);
                self.is_optimizing = false;
            }
            return;
        }

        if ui.button("Optimize!").clicked() {
            self.is_optimizing = true;
            let main_unit = app_ctx
                .units_store
                .get_unit(ctx, self.unit_kind)
                .expect("Checked earlier");
            let base_team = main_unit
                .unique_data
                .team
                .map(|op| op.and_then(|unit_kind| app_ctx.units_store.get_unit(ctx, unit_kind)));
            let relics_stat_filter = self.relics_stat_filter.clone();
            let sender = self.channels.clone().unwrap().0;
            let buffs = self.main_unit_buffs.clone();
            let cloned_relic_store = app_ctx.relics_store.clone();
            let cloned_lightcones_store = app_ctx.light_cones_store.clone();
            let cloned_ctx = ctx.clone();
            thread::spawn(move || {
                Self::optimize(
                    cloned_ctx,
                    main_unit.clone(),
                    base_team,
                    buffs,
                    relics_stat_filter,
                    &cloned_relic_store,
                    &cloned_lightcones_store,
                    sender,
                )
            });
        }
    }

    fn optimize(
        ctx: egui::Context,
        mut main_unit: Unit,
        base_team: [Option<Unit>; 3],
        buffs: Vec<(Unit, Modifier)>,
        relics_stats_filter: RelicsStatsFilter,
        relics_store: &RelicsStore,
        light_cones_store: &LightConesStore,
        tx: Arc<Sender<ThreadMessage>>,
    ) {
        let mut relics = relics_store.get_all_by_parts();
        let mut optimal_set: [Option<Relic>; 6] = [None; 6];
        let mut max_dmg = 0.;
        let optimization_target = &main_unit.kind.get_optimization_targets().unwrap()[0];

        main_unit.start_optimization(&light_cones_store);
        main_unit.reset_buffs();

        let team = &[
            base_team[0].as_ref(),
            base_team[1].as_ref(),
            base_team[2].as_ref(),
        ];
        for (buffer, modifier) in buffs.iter().as_ref() {
            main_unit.buff(
                team,
                modifier.clone(),
                buffer,
                &light_cones_store,
                &relics_store,
            );
        }

        relics[2] = Self::filter_parts(&relics[2], relics_stats_filter.body);
        relics[3] = Self::filter_parts(&relics[3], relics_stats_filter.feet);
        relics[4] = Self::filter_parts(&relics[4], relics_stats_filter.sphere);
        relics[5] = Self::filter_parts(&relics[5], relics_stats_filter.rope);

        let _cavern_relics_by_set = (0..4)
            .map(|relic_part| {
                let mut relics_by_set: Vec<Vec<Relic>> = Vec::new();
                for set in CavernSet::iter() {
                    relics_by_set.push(
                        relics[relic_part]
                            .iter()
                            .filter_map(|relic| {
                                if relic.set == RelicSet::Cavern(set) {
                                    Some(*relic)
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    );
                }
                relics_by_set
            })
            .collect::<Vec<Vec<Vec<Relic>>>>();

        let _planar_relics_by_set = (4..6)
            .map(|relic_part| {
                let mut relics_by_set: Vec<Vec<Relic>> = Vec::new();
                for set in PlanarSet::iter() {
                    relics_by_set.push(
                        relics[relic_part]
                            .iter()
                            .filter_map(|relic| {
                                if relic.set == RelicSet::Planar(set) {
                                    Some(*relic)
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    );
                }
                relics_by_set
            })
            .collect::<Vec<Vec<Vec<Relic>>>>();

        println!(
            "{}, {}, {}, {}, {}, {}",
            relics[0].len(),
            relics[1].len(),
            relics[2].len(),
            relics[3].len(),
            relics[4].len(),
            relics[5].len()
        );
        let mut i = 0;

        for relic_parts in &mut relics {
            for relic in relic_parts {
                relic.optimize();
            }
        }

        let _cavern_sets = CavernSet::iter().map(|cs| cs).collect::<Vec<CavernSet>>();
        let _planar_sets = PlanarSet::iter().map(|ps| ps).collect::<Vec<PlanarSet>>();

        let mut best_relic_sets: Vec<([Option<Relic>; 6], u32)> = Vec::new();
        let sets_num = 5;

        for _i in 0..sets_num {
            best_relic_sets.push(([None; 6], 0));
        }

        //looping ~= 1.7 ms
        //looping + relics swap: 100_000 runs per 40ms
        //opt: 100_000 runs per 70ms
        //new opt: 100_000 runs per
        let mut timer_start = time::Instant::now();
        relics[0].iter().for_each(|head| {
            main_unit.update_opt_relic(head);
            relics[1].iter().for_each(|hand| {
                main_unit.update_opt_relic(hand);
                relics[2].iter().for_each(|body| {
                    main_unit.update_opt_relic(body);
                    relics[3].iter().for_each(|feet| {
                        main_unit.update_opt_relic(feet);
                        relics[4].iter().for_each(|sphere| {
                            main_unit.update_opt_relic(sphere);
                            relics[5].iter().for_each(|rope| {
                                main_unit.update_opt_relic(rope);
                                let (avg_dmg, _, _) = optimization_target.dmg(&main_unit, team, &relics_store, &light_cones_store);

                                if avg_dmg > max_dmg {
                                    optimal_set = [Some(*head), Some(*hand), Some(*body), Some(*feet), Some(*sphere), Some(*rope)];
                                    max_dmg = avg_dmg;
                                }
                                i += 1;
                                if i % 1_000_000 == 0 {
                                    println!("iter: {}, performance in the last iter: 1000000 runs / {} ms",i, timer_start.elapsed().as_micros());
                                    timer_start = time::Instant::now();
                                }
                            });
                        });
                    });
                });
            });
        });

        let _ = tx.send(ThreadMessage {
            optimal_set: optimal_set,
        });
        ctx.request_repaint();
        main_unit.reset_buffs();
        main_unit.end_optimization();
    }

    fn optimize_configs(
        _best_relic_sets: &mut Vec<([Option<Relic>; 6], u32)>,
        _sets_num: usize,
        main_unit: &mut Unit,
        _team: &[Option<Unit>; 3],
        cavern_relics_by_set: &Vec<Vec<Vec<Relic>>>,
        planar_relics_by_set: &Vec<Vec<Vec<Relic>>>,
        configs: Vec<[usize; 6]>,
        _relics_store: &RelicsStore,
        _lc_store: &LightConesStore,
    ) {
        configs.iter().for_each(|config| {
            Self::update_set_effects(main_unit, config);
            cavern_relics_by_set[config[0]][0].iter().for_each(|head| {
                main_unit.update_opt_relic(head);
                cavern_relics_by_set[config[1]][1].iter().for_each(|hands| {
                    main_unit.update_opt_relic(hands);
                    cavern_relics_by_set[config[2]][2].iter().for_each(|body| {
                        main_unit.update_opt_relic(body);
                        cavern_relics_by_set[config[3]][3].iter().for_each(|feet| {
                            main_unit.update_opt_relic(feet);
                            planar_relics_by_set[config[4]][4]
                                .iter()
                                .for_each(|sphere| {
                                    main_unit.update_opt_relic(sphere);
                                    planar_relics_by_set[config[5]][5].iter().for_each(|rope| {
                                        main_unit.update_opt_relic(rope);
                                    })
                                })
                        })
                    })
                })
            })
        })
    }

    fn update_set_effects(unit: &mut Unit, config: &[usize; 6]) {
        let mut caver_sets = [0; CavernSet::LENGTH];
        for i in 0..4 {
            caver_sets[config[i]] += 1;
        }
        caver_sets
            .iter()
            .enumerate()
            .for_each(|(relic_set, &count)| {
                if count >= 2 {
                    if count == 4 {
                        unit.add_set_effects(get_cavern_set_effects(
                            CavernSet::from_usize(relic_set),
                            true,
                        ))
                    } else {
                        unit.add_set_effects(get_cavern_set_effects(
                            CavernSet::from_usize(relic_set),
                            false,
                        ))
                    }
                }
            });

        if config[4] == config[5] {
            unit.add_set_effects(get_planar_set_effects(PlanarSet::from_usize(config[4])));
        }
    }

    fn filter_parts(relics: &Vec<Relic>, bitmap: u32) -> Vec<Relic> {
        relics
            .iter()
            .filter_map(|relic| {
                if 1 << relic.main_stat.0.unique_id() & bitmap != 0 {
                    Some(*relic)
                } else {
                    None
                }
            })
            .collect()
    }

    fn show_in_top_panel(&mut self, ui: &mut Ui, app_ctx: &mut crate::app::AppContext) {
        let width = ui.available_width();
        let card_width = width * 0.32;
        let card_spacing = width * 0.02;
        //print!("{}",width);
        let unit_card = egui::containers::Frame::default()
            .fill(COLOR_PALLET.card())
            .rounding(egui::Rounding::same(5.0))
            .inner_margin(Margin::symmetric(10., 10.))
            .outer_margin(Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 10.,
            });

        egui::containers::SidePanel::left("optimized_unit")
            .frame(unit_card)
            .resizable(false)
            .exact_width(card_width)
            .show_separator_line(false)
            .show_inside(ui, |ui| self.unit_card.show_ui(ui, app_ctx));

        let relics_reqs = egui::containers::Frame::default()
            .fill(COLOR_PALLET.card())
            .rounding(egui::Rounding::same(5.0))
            .inner_margin(Margin::symmetric(10., 10.))
            .outer_margin(Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 10.,
            });

        egui::containers::SidePanel::right("relics_requiremetns")
            .frame(relics_reqs)
            .resizable(false)
            .exact_width(card_width)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                self.requirements.show_ui(ui);
                if let Some(optimal_set) = self.optimal_set {
                    Self::show_optimal_set(ui, optimal_set)
                }
            });

        let center_panel = egui::containers::Frame::default().outer_margin(Margin {
            left: card_spacing,
            right: card_spacing,
            top: 0.,
            bottom: 10.,
        });

        egui::containers::CentralPanel::default()
            .frame(center_panel)
            .show_inside(ui, |ui| {
                let level_filter_card = egui::containers::Frame::default()
                    .fill(COLOR_PALLET.card())
                    .rounding(egui::Rounding::same(5.0))
                    .inner_margin(Margin::symmetric(10., 10.))
                    .outer_margin(Margin {
                        left: 0.,
                        right: 0.,
                        top: 0.,
                        bottom: 10.,
                    });

                egui::containers::TopBottomPanel::top("relics_levels_panel")
                    .frame(level_filter_card)
                    .show_inside(ui, |ui| self.relics_level_filter.show_ui(ui));

                let stats_filter_card = egui::containers::Frame::default()
                    .fill(COLOR_PALLET.card())
                    .inner_margin(Margin::symmetric(10., 10.))
                    .rounding(egui::Rounding::same(5.0));

                egui::containers::CentralPanel::default()
                    .frame(stats_filter_card)
                    .show_inside(ui, |ui| self.relics_stat_filter.show_ui(ui));
            });
    }

    fn show_optimal_set(ui: &mut Ui, relic_set: [Option<Relic>; 6]) {
        ui.label("Optimal Set:");
        for relic_op in relic_set {
            if let Some(relic) = relic_op {
                ui.label(relic.to_str());
            } else {
                ui.label("no piece here :(");
            }
        }
    }
}
