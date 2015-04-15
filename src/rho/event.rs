pub enum Modifier
{
    NONE = 0,
    ALT,
    META,
    SHIFT,
    CONTROL,
}

struct InputEvent {
    character: Option<char>,
    modifier: Option<Modifier>,
}
