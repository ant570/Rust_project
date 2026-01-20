use crate::States;

//Definicja stanów gry
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    StartMenu, //Główne menu
    Playing,       //Gra trwa
    HowToPlay1,    //Instrukcja (po wejściu z StartMenu)
    HowToPlay2,    //Instrukcja (po wejściu z PauseMenu)
    Paused,        //Okno zapauzowania
    Finished,      //Okno końca gry
    SettingsPause, //Ustawienia (po wejściu z StartMenu)
    SettingsStart, //ustawienia (po wejściu z PauseMenu)
}
