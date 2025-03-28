use beatsaver_api::models::{
    enums::Characteristic,
    map::{Map, MapDifficulty},
};
use log::info;
use poise::serenity_prelude::{
    Colour, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption,
};

pub struct MapEmbed {
    pub map: Map,
    pub selected_index: usize,
    pub options: Vec<CreateSelectMenuOption>,
}

/// Creates a list of the available embeds.
fn get_map_diffs_list(map: &Map) -> Vec<CreateSelectMenuOption> {
    let mut map_diffs: Vec<CreateSelectMenuOption> =
        vec![CreateSelectMenuOption::new("Metadata", "0").default_selection(true)];

    map_diffs.extend(map.versions[0].diffs.iter().enumerate().map(|(idx, diff)| {
        CreateSelectMenuOption::new(
            format!("{} {}", diff.characteristic.name(), diff.difficulty),
            (idx + 1).to_string(),
        )
    }));

    info!("Difficulties: {:?}", map_diffs);

    map_diffs
}

impl MapEmbed {
    pub fn new(map: Map) -> Self {
        let options = get_map_diffs_list(&map);

        Self {
            map,
            selected_index: 0, // Metadata
            options,
        }
    }

    pub fn set_index(&mut self, new_index: &str) {
        self.selected_index = str::parse::<usize>(new_index).unwrap_or_default();
        self.set_new_default();
    }

    pub fn build_embeds(&self) -> Vec<CreateEmbed> {
        match self.selected_index {
            0 => vec![self.create_map_metadata_embed()],
            idx => vec![
                self.create_map_metadata_embed(),
                self.create_map_diff_embed(&self.map.versions[0].diffs[idx - 1]),
            ],
        }
    }

    pub fn build_embed_components(&mut self) -> Vec<CreateActionRow> {
        vec![
            CreateActionRow::SelectMenu(
                CreateSelectMenu::new(
                    "diffsel",
                    CreateSelectMenuKind::String {
                        options: self.options.clone(),
                    },
                )
                .placeholder("Select Difficulty"),
            ),
            CreateActionRow::Buttons(vec![
                CreateButton::new_link(&self.map.versions[0].download_url)
                    .label("Download")
                    .emoji('⬇'),
                CreateButton::new_link(format!(
                    "https://allpoland.github.io/ArcViewer/?id={}",
                    &self.map.id
                ))
                .label("Preview")
                .emoji('⏯'),
            ]),
        ]
    }

    fn set_new_default(&mut self) {
        for idx in 0..self.options.len() {
            self.options[idx] = self.options[idx]
                .clone()
                .default_selection(idx == self.selected_index);
        }
    }

    // MARK: Embed creator functions

    /// Creates the general map info embed.
    fn create_base_embed(&self) -> CreateEmbed {
        let embed: CreateEmbed = CreateEmbed::new()
            .title(&self.map.name)
            .url(format!("https://beatsaver.com/maps/{}", self.map.id))
            .description(&self.map.description)
            .thumbnail(&self.map.versions[0].cover_url)
            .footer(CreateEmbedFooter::new(format!("!bsr {}", &self.map.id)))
            .timestamp(self.map.uploaded);

        embed
    }

    /// Creates the general metadata of the map embed.
    fn create_map_metadata_embed(&self) -> CreateEmbed {
        let embed = self.create_base_embed();
        embed
            .field("Mapper(s)", &self.map.metadata.level_author_name, false)
            .field("Artist(s)", &self.map.metadata.song_author_name, false)
            .fields([
                ("BPM", &self.map.metadata.bpm.to_string(), true),
                (
                    "Length",
                    &format!(
                        "{}:{:0>2}",
                        (self.map.metadata.duration / 60) % 60,
                        self.map.metadata.duration % 60
                    ),
                    true,
                ),
                (
                    "Rating",
                    &format!(
                        "▲ {} / ▼ {} ({:.2}%)",
                        self.map.stats.upvotes,
                        self.map.stats.downvotes,
                        self.map.stats.score * 100.0
                    ),
                    true,
                ),
            ])
            .colour(self.get_embed_colour())
    }

    /// Creates the embed representing data for one difficulty.
    fn create_map_diff_embed(&self, diff: &MapDifficulty) -> CreateEmbed {
        let mut embed = CreateEmbed::new()
            .title(if let Some(label) = &diff.label {
                label
            } else {
                &diff.difficulty
            })
            .thumbnail(self.get_characteristic_thumbnail(&diff.characteristic));

        embed = embed.field(
            "Characteristic/Difficulty",
            format!("{} {}", diff.characteristic.name(), diff.difficulty),
            false,
        );

        if let Some(scoresaber_stars) = diff.ss_stars {
            embed = embed.field("ScoreSaber Stars", format!("{:.2}", scoresaber_stars), true);
        }
        if let Some(beatleader_stars) = diff.bl_stars {
            embed = embed.field("BeatLeader Stars", format!("{:.2}", beatleader_stars), true);
        }

        embed
            .field("", "", false)
            .fields(vec![
                ("Notes", diff.notes.to_string(), true),
                ("Bombs", diff.bombs.to_string(), true),
                ("Walls", diff.obstacles.to_string(), true),
                ("NJS", diff.njs.to_string(), true),
                ("NPS", diff.nps.to_string(), true),
                ("Lights", diff.events.to_string(), true),
            ])
            .colour(self.get_diff_colour(&diff.difficulty))
    }

    // MARK: Embed colour functions

    /// Adds a colour to the map metadata embed.
    fn get_embed_colour(&self) -> Colour {
        if self.map.ss_ranked || self.map.bl_ranked {
            Colour::from_rgb(243, 156, 18)
        } else if self.map.curated_at.is_some() {
            Colour::from_rgb(0, 188, 140)
        } else if self.map.uploader.verified_mapper {
            Colour::from_rgb(118, 70, 175)
        } else {
            Colour::from_rgb(68, 68, 68)
        }
    }

    /// Adds a difficulty colour to the map difficulty embed.
    fn get_diff_colour(&self, diff_name: &str) -> Colour {
        match diff_name {
            "ExpertPlus" => Colour::from_rgb(166, 149, 255),
            "Expert" => Colour::from_rgb(255, 149, 166),
            "Hard" => Colour::from_rgb(255, 183, 77),
            "Normal" => Colour::from_rgb(0, 238, 255),
            "Easy" => Colour::from_rgb(129, 199, 132),
            _ => unreachable!(),
        }
    }

    /// Gets a thumbnail of the difficulty's characteristic.
    fn get_characteristic_thumbnail(&self, characteristic: &Characteristic) -> String {
        match characteristic {
            Characteristic::Standard => "https://raw.githubusercontent.com/mercurialworld/mafuyu/refs/heads/main/assets/Standard.png".into(),
            Characteristic::OneSaber => "https://raw.githubusercontent.com/mercurialworld/mafuyu/refs/heads/main/assets/OneSaber.png".into(),
            Characteristic::NoArrows => "https://raw.githubusercontent.com/mercurialworld/mafuyu/refs/heads/main/assets/NoArrows.png".into(),
            Characteristic::Rotation90Degrees => "https://raw.githubusercontent.com/mercurialworld/mafuyu/refs/heads/main/assets/90Degree.png".into(),
            Characteristic::Rotation360Degrees => "https://raw.githubusercontent.com/mercurialworld/mafuyu/refs/heads/main/assets/360Degree.png".into(),
            Characteristic::Lightshow => "https://raw.githubusercontent.com/mercurialworld/mafuyu/refs/heads/main/assets/Lightshow.png".into(),
            Characteristic::Lawless => "https://raw.githubusercontent.com/mercurialworld/mafuyu/refs/heads/main/assets/Lawless.png".into(),
            Characteristic::Legacy => "https://raw.githubusercontent.com/mercurialworld/mafuyu/refs/heads/main/assets/Legacy.png".into(),
        }
    }
}
