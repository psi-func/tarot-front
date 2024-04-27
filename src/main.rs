use bevy::prelude::*;
use bevy_simple_text_input::{TextInputBundle, TextInputPlugin, TextInputSubmitEvent};
use bevy_wasm_tasks::{WASMTasksPlugin, WASMTasksRuntime};

const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

use tarot_front::connector::*;
use tarot_front::resources::{card::CardId, deck::Deck};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WASMTasksPlugin)
        .add_plugins(TextInputPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup)
        .add_systems(Update, listener)
        .add_systems(Update, click_deck)
        .add_systems(Update, add_explanation)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct GreetWindow;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: TEXT_COLOR.into(),
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GreetWindow)
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section("Задай Таро свой вопрос", text_style.clone())
                    .with_justify(JustifyText::Center),

                style: Style {
                    width: Val::Percent(50.0),
                    padding: UiRect::left(Val::Percent(10.0)),
                    margin: UiRect::all(Val::Px(20.)),
                    ..Default::default()
                },
                ..Default::default()
            },));

            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),

                        ..Default::default()
                    },
                    border_color: BORDER_COLOR_ACTIVE.into(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..Default::default()
                },
                TextInputBundle::default().with_text_style(text_style.clone()),
            ));
        });
}

#[derive(Component)]
struct MainDeck;

#[derive(Resource)]
struct DeckHolder {
    card_container: Entity,
    deck_parent: Entity,
    deck: Deck,
    style: Style,
    texture_atlas: Handle<TextureAtlasLayout>,
    texture_handle: Handle<Image>,
}

impl DeckHolder {
    fn spawn_tarot(&mut self, commands: &mut Commands) -> u16 {
        let mut texture_atlas: TextureAtlas = self.texture_atlas.clone().into();

        let cards = self.deck.get_cards(1);
        let card = cards.iter().nth(0).unwrap();
        info!("{:?}", card);

        let card_id: u8 = card.clone().into();
        texture_atlas.index = card_id as usize;

        let new_tarot_card = commands
            .spawn(AtlasImageBundle {
                style: self.style.clone(),
                texture_atlas: texture_atlas,
                image: UiImage::new(self.texture_handle.clone()),
                ..default()
            })
            .id();

        let mut parent = commands.entity(self.card_container);
        parent.add_child(new_tarot_card);
        card_id as u16
    }
}

fn listener(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    greet: Query<Entity, With<GreetWindow>>,
    mut events: EventReader<TextInputSubmitEvent>,
    runtime: ResMut<WASMTasksRuntime>,
) {
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
        commands.entity(greet.single()).despawn_recursive();

        let texture_handle = asset_server.load("tarot/classic.png");
        let texture_atlas =
            TextureAtlasLayout::from_grid(Vec2::new(240.0, 400.0), 8, 10, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let mut card_container: Option<Entity> = None;
        let mut deck_parent: Option<Entity> = None;

        let style = Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Percent(5.0),
            ..Default::default()
        };

        let style_card = Style {
            width: Val::Px(153.6),
            height: Val::Px(256.),
            margin: UiRect::axes(Val::Px(20.), Val::Auto),
            ..Default::default()
        };

        commands
            .spawn(NodeBundle {
                style: style.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                deck_parent = Some(
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(40.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|deck_par| {
                            let mut texture_atlas: TextureAtlas =
                                texture_atlas_handle.clone().into();
                            texture_atlas.index = 78;

                            deck_par
                                .spawn(AtlasImageBundle {
                                    style: style_card.clone(),
                                    texture_atlas: texture_atlas,
                                    image: UiImage::new(texture_handle.clone()),
                                    ..default()
                                })
                                .insert(MainDeck);
                        })
                        .id(),
                );

                card_container = Some(
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(40.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .id(),
                );
            });

        runtime.spawn_background_task(move |mut ctx| async move {
            let tarot_cards = post_get_cards().await.unwrap();

            let cards: Vec<_> = tarot_cards
                .cards
                .into_iter()
                .map(|el| CardId::try_from(el).unwrap())
                .collect();

            ctx.run_on_main_thread(move |ctx| {
                let world = ctx.world;
                world.insert_resource(DeckHolder {
                    style: style_card.clone(),
                    texture_handle,
                    deck_parent: deck_parent.unwrap(),
                    card_container: card_container.unwrap(),
                    texture_atlas: texture_atlas_handle,
                    deck: Deck { cards },
                });
            })
            .await;
        });
    }
}

#[derive(Resource)]
struct NNPredict {
    exp: String,
}

fn click_deck(
    mut commands: Commands,
    meshes: Query<(Entity, &Transform), With<MainDeck>>,
    window: Query<&Window>,
    event_mouse: Res<ButtonInput<MouseButton>>,
    mut deck_holder: Option<ResMut<DeckHolder>>,
    runtime: ResMut<WASMTasksRuntime>,
    mut opened_cards: Local<Vec<u16>>,
) {
    if deck_holder.is_none() {
        return;
    }

    if event_mouse.just_pressed(MouseButton::Left) {
        if opened_cards.len() >= 5 {
            return;
        }
        let coords = window.single().cursor_position();
        let (en, tr) = meshes.single();
        // TODO: check collision
        info!("mouse: {}, tr: {}", coords.unwrap(), tr.translation);

        let spawned_card = deck_holder
            .as_deref_mut()
            .unwrap()
            .spawn_tarot(&mut commands);
        opened_cards.push(spawned_card);

        if opened_cards.len() == 5 {
            // remove card
            commands
                .entity(deck_holder.as_deref().unwrap().deck_parent)
                .despawn_descendants();

            let opened_cards = opened_cards.clone();

            runtime.spawn_background_task(move |mut ctx| async move {
                let explanation = post_get_explanation(opened_cards).await.unwrap();

                ctx.run_on_main_thread(move |ctx| {
                    let world = ctx.world;
                    world.insert_resource(NNPredict {
                        exp: explanation.exp,
                    });
                })
                .await;
            });
        }
    }
}

fn add_explanation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    nn_exp: Option<Res<NNPredict>>,
    deck_holder: Option<Res<DeckHolder>>,
) {
    if nn_exp.is_none() || deck_holder.is_none() {
        return;
    }

    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: TEXT_COLOR.into(),
    };

    let text = nn_exp.unwrap().exp.clone();

    let res_entity = commands
        .spawn(TextBundle {
            text: Text::from_section(text.as_str(), text_style.clone())
                .with_justify(JustifyText::Center),

            style: Style {
                width: Val::Percent(90.0),
                padding: UiRect::left(Val::Percent(10.0)),
                margin: UiRect::all(Val::Px(20.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .entity(deck_holder.as_deref().unwrap().deck_parent)
        .add_child(res_entity);
}
