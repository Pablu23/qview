#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    OpenPopup,
    ClosePopup,
    Select,
    NextSearchResult,
    PrevSearchResult,
    ChangeTab,
}
