use std::{collections::{BTreeMap, HashMap, HashSet}};

use bevy::{diagnostic::{DiagnosticPath, DiagnosticsStore}, prelude::*, text::FontSmoothing};


#[macro_export]
macro_rules! overlay_set {
    ($name:ident, $value:ident) => {
        #[cfg(feature = "debug_overlay")]
        {
            overlay            
        }
    }
}

#[macro_export]
macro_rules! overlay {
    () => {
        #[cfg(feature = "debug_overlay")]
        {
            overlay: bevy::prelude::EventWriter<$crate::debug_overlay::DebugOverlayEvent>
        }
    };
}

pub struct DebugOverlayPlugin{
    pub enabled: bool,
    pub supress_default: bool,
}

impl DebugOverlayPlugin {
    pub fn enabled() -> Self {
        Self {enabled: true, supress_default: false}
    }
    pub fn supress_default(mut self) -> Self {
        self.supress_default = true;
        self
    }
}

impl Default for DebugOverlayPlugin {
    fn default() -> Self {
        Self {enabled: false, supress_default: false}
    }
}

impl Plugin for DebugOverlayPlugin {
    fn build(&self, _app: &mut App) {
        #[cfg(feature = "debug_overlay")]
        {
            let sd = self.supress_default;
            _app
                .insert_resource(DebugOverlay::enabled(self.enabled))
                .add_systems(PreStartup, 
                    move |cmd: Commands, asset_server: Res<AssetServer>, overlay: ResMut<DebugOverlay>, events: EventWriter<DebugOverlayEvent>| init(sd, cmd, asset_server, overlay, events));
            if self.supress_default {
                _app.add_systems(PreUpdate, debug_overlay_tick);
            } else {
                _app.add_systems(PreUpdate, (default_sustem_tick, debug_overlay_tick).chain());
            }
            _app.add_event::<DebugOverlayEvent>();
        }
    }
}

#[derive(Event)]
pub enum DebugOverlayEvent{
    Set{key: String, val: DebugRecord},
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum OverlayAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    TopCenter,
    BottomCenter,
    LeftCenter,
    RightCenter,

    // Follow entity on world
    Dynamic(Entity),
    // Fills entire screen
    Fill,
}

impl OverlayAnchor {
    pub fn is_bottom(&self) -> bool {
        matches!(self, OverlayAnchor::BottomLeft | OverlayAnchor::BottomRight | OverlayAnchor::BottomCenter)
    }
    pub fn is_right(&self) -> bool {
        matches!(self, OverlayAnchor::TopRight | OverlayAnchor::BottomRight | OverlayAnchor::RightCenter)
    }
    pub fn is_left(&self) -> bool {
        matches!(self, OverlayAnchor::TopLeft | OverlayAnchor::BottomLeft | OverlayAnchor::LeftCenter)
    }
    pub fn is_top(&self) -> bool {
        matches!(self, OverlayAnchor::TopLeft | OverlayAnchor::TopRight | OverlayAnchor::TopCenter)
    }
    
    pub fn justify_content(&self) -> JustifyContent {
        if self.is_bottom() {
            JustifyContent::End
        } else {
            JustifyContent::Start
        }
    }

    pub fn flex_direction(&self) -> FlexDirection {
        if self.is_bottom() {
            FlexDirection::ColumnReverse
        } else {
            FlexDirection::Column
        }
    }

    pub fn is_center_horizontal(&self) -> bool {
        matches!(self, OverlayAnchor::Center | OverlayAnchor::TopCenter | OverlayAnchor::BottomCenter)
    }

    pub fn is_center_vertical(&self) -> bool {
        matches!(self, OverlayAnchor::Center | OverlayAnchor::LeftCenter | OverlayAnchor::RightCenter)
    }

    fn align_items(&self) -> AlignItems {
        if self.is_bottom() {
            AlignItems::End
        } else if self.is_center_horizontal() {
            AlignItems::Center
        } else {
            AlignItems::Start
        }
    }
    fn align_self(&self) -> AlignSelf {
        if self.is_bottom() {
            AlignSelf::End
        } else if self.is_center_horizontal() {
            AlignSelf::Center
        } else {
            AlignSelf::Start
        }
    }


    pub fn to_node(&self) -> Node {
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: self.justify_content(),
            flex_direction: self.flex_direction(),
            align_items: self.align_items(),
            align_self: self.align_self(),
            position_type: PositionType::Absolute,
            ..default()
        }
    }
}


pub fn default_sustem_tick(
    mut events: EventWriter<DebugOverlayEvent>,
    diagnostics_store: Option<Res<DiagnosticsStore>>,
){
    let Some(diagnostics_store) = diagnostics_store else {
        error_once!("Cant find DiagnosticsStore! make shure ure are add FrameTimeDiagnosticsPlugin!");
        return;
    };

    let fps = diagnostics_store.get(&DiagnosticPath::const_new("fps"));
    if let Some(fps) = fps {
        let smoothed = fps.smoothed().unwrap_or(0.);
        let avg = fps.average().unwrap_or(0.);
        let get_fps_color = |x: f64| -> Color {
            if x > 59. {
                Color::srgba(0., 1., 0., 1.)
            } else if x > 49. {
                Color::srgba(0.2, 0.8, 0., 1.)
            } else if x > 39. {
                Color::srgba(0.4, 0.6, 0., 1.)
            } else if x > 10. {
                Color::srgba(0.6, 0.4, 0., 1.)
            } else if x > 5. {
                Color::srgba(0.8, 0.2, 0., 1.)
            } else {
                Color::srgba(0.1, 0., 0., 1.)
            }
        };
        events.write(DebugOverlayEvent::Set {
            key: "FPS_AVG".to_string(),
            val: DebugRecord {
                record_type: DebugRecordType::Text {
                    text: vec![
                        (Color::srgba(0.5, 0.5, 0.5, 1.), format!("avg. ")),
                        (get_fps_color(avg), format!("{:.1}", avg)),
                        (Color::srgba(0.9, 0.9, 0.9, 1.), format!(" fps")),
                    ]
                },
                anchor: OverlayAnchor::BottomRight,
                layer: 0
            }
        });
        events.write(DebugOverlayEvent::Set {
            key: "FPS_SM".to_string(),
            val: DebugRecord {
                record_type: DebugRecordType::Text {
                    text: vec![
                        (get_fps_color(smoothed), format!("{:.1}", smoothed)),
                        (Color::srgba(0.9, 0.9, 0.9, 1.), format!(" fps")),
                    ]
                },
                anchor: OverlayAnchor::BottomRight,
                layer: 0
            }
        });
    }
}

pub fn debug_overlay_tick(
    inputs: Res<ButtonInput<KeyCode>>,
    mut events: EventReader<DebugOverlayEvent>,
    mut root: Single<&mut Transform, With<DebugOverlayRoot>>,
    mut overlay: ResMut<DebugOverlay>,
    mut cmd: Commands,
){
    if inputs.pressed(KeyCode::ControlLeft)
    && inputs.pressed(KeyCode::ShiftLeft) {
        let d = inputs.just_pressed(KeyCode::Equal) as i32 - 
        inputs.just_pressed(KeyCode::Minus) as i32;
        root.scale += 0.1 * d as f32;
    }
    for e in events.read() {
        match e {
            DebugOverlayEvent::Set{key, val} => {
                overlay.set(&mut cmd, key, val);
            }
            _ => {unimplemented!()}
        }
    }
}

#[derive(Clone)]
pub enum DebugRecordType {
    Text {
        text: Vec<(Color, String)>,
    },
    Graph {
        data: Vec<f32>,
        size: usize,
        last_index: usize,
        max: f32,
        min: f32,
        marks: Vec<(f32, String)>,
    },
    Image {
        handle: Handle<Image>,
        size: Option<Vec2>,
        flip_x: bool,
        flip_y: bool,
        rect: Option<Rect>,
        label: Option<String>,
    }
}

impl DebugRecordType {
    fn to_node_bundle(&self, cmd: &mut Commands, font: Handle<Font>) -> Entity {
        match self {
            DebugRecordType::Text { text } => {
                cmd.spawn((
                    TextFont { font: font.clone(), font_smoothing: FontSmoothing::None, ..default() },
                    Text::default(),
                )).with_children(|cmd|{
                    for (color, text) in text {
                        cmd.spawn((
                            TextFont { font: font.clone(), font_smoothing: FontSmoothing::None, ..default() },
                            TextSpan::new(text.clone()),
                            TextColor::from(*color),
                        ));
                    }
                }).id()
            }
            _ => unimplemented!()
        }
    } 
    fn update(&mut self, cmd: &mut Commands, record: &Self, entity: Entity) {
        match self {
            DebugRecordType::Text { text } => {
                let DebugRecordType::Text{text: new_text} = record else {warn!("Wrong record type passed!");return};
                let diff = text.len() as i32 - new_text.len() as i32;
                if diff >= 0 {
                    // cmd.entity(entity).chil
                }
            }
            _ => unimplemented!()
        }
    }
}

#[derive(Clone)]
pub struct DebugRecord {
    record_type: DebugRecordType,
    anchor: OverlayAnchor,
    layer: i8,
}

pub struct StoredDebugRecord {
    record: DebugRecord,
    entity: Entity,
}



#[derive(Component)]
pub struct DebugOverlayRoot;

pub fn init(
    supress_default: bool,
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut overlay: ResMut<DebugOverlay>,
    mut events: EventWriter<DebugOverlayEvent>,
){
    overlay.font = asset_server.load("fonts/orp_regular.ttf");
    overlay.root = cmd.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            position_type: PositionType::Absolute,
            ..default()
        },
        DebugOverlayRoot,
        Name::new("DebugRoot")
    )).id();
    
    if !supress_default {
        events.write(DebugOverlayEvent::Set {
            key: "PKGINFO".to_string(),
            val: DebugRecord {
                record_type: DebugRecordType::Text {
                    text: vec![
                        (Color::srgba(0.5, 0.5, 0.5, 1.), format!("{} v{}", NAME, VERSION)),
                    ]
                },
                anchor: OverlayAnchor::BottomRight,
                layer: 0
            }
        });
        events.write(DebugOverlayEvent::Set {
            key: "FPS_AVG".to_string(),
            val: DebugRecord {
                record_type: DebugRecordType::Text {
                    text: vec![
                        (Color::srgba(0.5, 0.5, 0.5, 1.), format!("avg. 0 fps")),
                    ]
                },
                anchor: OverlayAnchor::BottomRight,
                layer: 0
            }
        });
        events.write(DebugOverlayEvent::Set {
            key: "FPS_SM".to_string(),
            val: DebugRecord {
                record_type: DebugRecordType::Text {
                    text: vec![
                        (Color::srgba(0.5, 0.5, 0.5, 1.), format!("0 fps")),
                    ]
                },
                anchor: OverlayAnchor::BottomRight,
                layer: 0
            }
        });
    }
}




const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub struct LayerRecord {
    entity: Entity,
    anchors: HashMap<OverlayAnchor, Entity>,
}

impl LayerRecord {
    pub fn from_entity(e: Entity) -> Self {
        Self {
            entity: e,
            anchors: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub struct DebugOverlay{
    root: Entity,
    layers: BTreeMap<i8, LayerRecord>,
    visible: bool,
    records: HashMap<String, StoredDebugRecord>,
    enabled_layers: HashSet<i8>,
    font: Handle<Font>,
    bg_color: Color,
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self {
            root: Entity::PLACEHOLDER,
            font: Handle::default(),
            layers: BTreeMap::new(),
            visible: false,
            records: HashMap::new(),
            enabled_layers: HashSet::new(),
            bg_color: Color::srgba(0.1, 0.1, 0.1, 0.8),
        }
    }
}

impl DebugOverlay {
    fn enabled(enabled: bool) -> Self {
        Self {
            visible: enabled,
            ..default()
        }
    }
    fn layer_create_or_get<'a, 'b>(&'a mut self, cmd: &mut Commands, layer: i8) -> &'b mut LayerRecord where 'a : 'b {
        self.layers.entry(layer).or_insert_with(|| {
            let e = cmd.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                Name::new(format!("Layer {}", layer)),
            )).id();
            cmd.entity(self.root).add_child(e);
            LayerRecord::from_entity(e)
        })
    }

    fn anchor_create_or_get(&mut self, cmd: &mut Commands, layer: i8, anchor: OverlayAnchor) -> &Entity {
        let layer = self.layer_create_or_get(cmd, layer);
        let overlay_anchor = anchor.clone();
        layer.anchors.entry(anchor).or_insert_with(|| {
            let e = cmd.spawn((
                Name::new(format!("Anchor {overlay_anchor:?}")),
                overlay_anchor.to_node(),
            )).id();
            cmd.entity(layer.entity).add_child(e);
            e
        })
    }
    

    fn record_create_or_update(&mut self, cmd: &mut Commands, key: &str, record: &DebugRecord) {
        let anchor = *self.anchor_create_or_get(cmd, record.layer, record.anchor.clone());
        if let Some(stored_record) = self.records.get_mut(key) {
            stored_record.record.record_type.update(cmd, &record.record_type, stored_record.entity);
        } else {
            let e = record.record_type.to_node_bundle(cmd, self.font.clone());
            let e = cmd.entity(e).insert(
                BackgroundColor(self.bg_color.clone()),
            ).id();
            cmd.entity(anchor).add_child(e);
            StoredDebugRecord {
                record: record.clone(),
                entity: e,
            };
            self.records.insert(key.to_owned(), StoredDebugRecord{record: record.clone(), entity: e});
        };


        // let v = self.records.entry(key.to_owned()).or_insert_with(|| {
        //         let e = cmd.spawn(record.record_type.to_node_bundle()).id();
        //         cmd.entity(anchor).add_child(e);
        //         StoredDebugRecord {
        //             record: record.clone(),
        //             entity: e,
        //         }
        //     }
        // );
        // v
    }

    fn set(&mut self, cmd: &mut Commands, key: &str, val: &DebugRecord){
        // let DebugRecord{layer, record_type: record, anchor} = val;
        self.record_create_or_update(cmd, key, val);
        // let record;
    }
}


