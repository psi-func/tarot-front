use bevy::prelude::*;
use bevy_simple_text_input::{TextInputBundle, TextInputPlugin, TextInputSubmitEvent};

const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TextInputPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup)
        .add_systems(Update, listener)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

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

fn listener(mut events: EventReader<TextInputSubmitEvent>) {
    todo!("Send message to back");

    
    
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
    }
}
