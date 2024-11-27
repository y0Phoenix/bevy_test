use bevy::prelude::*;
use double_dot_state::prelude::{DoubleStates, AppExt};

/// Reference the `State and UI.uxf` file and open with UMLet or VSCode for more state info
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, DoubleStates)]
pub enum GameState {
    #[default]
    /// linear LoadingUI
    #[linear(LoadingUI)]
    LoadingAssets,
    /// linear MainMenu
    #[linear(MainMenu)]
    LoadingUI,
    /// arbitrary LoadingWorld, OptionsMenu
    #[arbitrary(LoadingWorld, OptionsMenu)]
    MainMenu,
    /// arbitrary OptionsMenu, ExitToMain, InGame
    #[arbitrary(OptionsMenu, ExitToMain, InGame)]
    PauseMenu,
    /// arbitrary PauseMenu, MainMenu
    #[arbitrary(PauseMenu, MainMenu)]
    OptionsMenu,
    /// linear LoadingTextures
    #[linear(LoadingTextures)]
    LoadingWorld,
    /// linear InGame
    #[linear(InGame)]
    LoadingTextures,
    /// arbitrary PauseMenu, linear LevelChange
    #[arbitrary(PauseMenu)]
    #[linear(LevelChange)]
    InGame,
    /// linear InGame
    #[linear(InGame)]
    LevelChange,
    ExitToMain,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_double_state::<GameState>()
            // .add_startup_system(print_name)
        ;
    }
}