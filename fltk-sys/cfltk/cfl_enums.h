#pragma once

enum Label {
    Label_Normal = 0,
    Label_None,
    Label_Shadow,
    Label_Engraved,
    Label_Embossed,
    Label_Multi,
    Label_Icon,
    Label_Image,
    Label_FreeType,
};

enum Frame {
    NoBox = 0,
    FlatBox,
    UpBox,
    DownBox,
    UpFrame,
    DownFrame,
    ThinUpBox,
    ThinDownBox,
    ThinUpFrame,
    ThinDownFrame,
    EngraveBox,
    EmbossedBox,
    EngravedFrame,
    EmbossedFrame,
    BorderBox,
    ShadowBox,
    BorderFrame,
    ShadowFrame,
    RoundedBox,
    RShadowBox,
    RoundedFrame,
    RFlatBox,
    RoundUpBox,
    RoundDownBox,
    DiamondUpBox,
    DiamondDownBox,
    OvalBox,
    OShadowBox,
    OvalFrame,
    OFlatFrame,
    PlasticUpBox,
    PlasticDownBox,
    PlasticUpFrame,
    PlasticDownFrame,
    PlasticThinUpBox,
    PlasticThinDownBox,
    PlasticRoundUpBox,
    PlasticRoundDownBox,
    GtkUpBox,
    GtkDownBox,
    GtkUpFrame,
    GtkDownFrame,
    GtkThinUpBox,
    GtkThinDownBox,
    GtkThinUpFrame,
    GtkThinDownFrame,
    GtkRoundUpFrame,
    GtkRoundDownFrame,
    GleamUpBox,
    GleamDownBox,
    GleamUpFrame,
    GleamDownFrame,
    GleamThinUpBox,
    GleamThinDownBox,
    GleamRoundUpBox,
    GleamRoundDownBox,
    FreeBox,
};

enum Align {
    Align_Center = 0,
    Align_Top = 1,
    Align_Bottom = 2,
    Align_Left = 4,
    Align_Right = 8,
};

enum Font {
    Helvetica = 0,
    HelveticaBold = 1,
    HelveticaItalic = 2,
    HelveticaBoldItalic = 3,
    Courier = 4,
    CourierBold = 5,
    CourierItalic = 6,
    CourierBoldItalic = 7,
    Times = 8,
    TimesBold = 9,
    TimesItalic = 10,
    TimesBoldItalic = 11,
    Symbol = 12,
    Screen = 13,
    ScreenBold = 14,
    Zapfdingbats = 15,
    Freefont = 16,
};

enum Color {
    ForeGround = 0,
    BackGround = 7,
    Inactive = 8,
    Selection = 15,
    Gray0 = 32,
    Dark3 = 39,
    Dark2 = 45,
    Dark1 = 47,
    FrameDefault = 49,
    Light1 = 50,
    Light2 = 52,
    Light3 = 54,
    Black = 56,
    Red = 88,
    Green = 63,
    Yellow = 95,
    Blue = 216,
    Magenta = 248,
    Cyan = 223,
    DarkRed = 72,
    DarkGreen = 60,
    DarkYellow = 76,
    DarkBlue = 136,
    DarkMagenta = 152,
    DarkCyan = 140,
    White = 255,
};

enum Event {
    NoEvent = 0,
    PushEvent,
    ReleasedEvent,
    EnterEvent,
    LeaveEvent,
    DragEvent,
    FocusEvent,
    UnfocusEvent,
    KeyDownEvent,
    KeyUpEvent,
    CloseEvent,
    MoveEvent,
    ShortcutEvent,
    DeactivateEvent,
    ActivateEvent,
    HideEvent,
    ShowEvent,
    PasteEvent,
    SelectionClearEvent,
    MouseWheelEvent,
    DndEnterEvent,
    DndDragEvent,
    DndLeaveEvent,
    DndReleaseEvent,
    ScreenConfigChangedEvent,
    FullscreenEvent,
    ZoomGestureEvent,
    ZoomEventEvent,
};

enum Key {
    Key_None = 0,
    Key_Button = 0xfee8,
    Key_BackSpace = 0xff08,
    Key_Tab = 0xff09,
    Key_IsoKey = 0xff0c,
    Key_Enter = 0xff0d,
    Key_Pause = 0xff13,
    Key_ScrollLock = 0xff14,
    Key_Escape = 0xff1b,
    Key_Kana = 0xff2e,
    Key_Eisu = 0xff2f,
    Key_Yen = 0xff30,
    Key_JISUnderscore = 0xff31,
    Key_Home = 0xff50,
    Key_Left = 0xff51,
    Key_Up = 0xff52,
    Key_Right = 0xff53,
    Key_Down = 0xff54,
    Key_PageUp = 0xff55,
    Key_PageDown = 0xff56,
    Key_End = 0xff57,
    Key_Print = 0xff61,
    Key_Insert = 0xff63,
    Key_Menu = 0xff67,
    Key_Help = 0xff68,
    Key_NumLock = 0xff7f,
    Key_KP = 0xff80,
    Key_KPEnter = 0xff8d,
    Key_KPLast = 0xffbd,
    Key_FLast = 0xffe0,
    Key_ShiftL = 0xffe1,
    Key_ShiftR = 0xffe2,
    Key_ControlL = 0xffe3,
    Key_ControlR = 0xffe4,
    CapsLock = 0xffe5,
    Key_MetaL = 0xffe7,
    Key_MetaR = 0xffe8,
    Key_AltL = 0xffe9,
    Key_AltR = 0xffea,
    Key_Delete = 0xffff,
};

enum Shortcut {
    Shortcut_None = 0,
    Shortcut_Shift = 0x00010000,
    Shortcut_CapsLock = 0x00020000,
    Shortcut_Ctrl = 0x00040000,
    Shortcut_Alt = 0x00080000,
};

enum CallbackTrigger {
    Trigger_Never = 0,
    Trigger_Changed = 1,
    Trigger_NotChanged = 2,
    Trigger_Release = 4,
    Trigger_ReleaseAlways = 6,
    Trigger_EnterKey = 8,
    Trigger_EnterKeyAlways = 10,
    Trigger_EnterKeyChanged = 11,
};

enum TextCursor {
    TextCursor_Normal,
    TextCursor_Caret,
    TextCursor_Dim,
    TextCursor_Block,
    TextCursor_Heavy,
    TextCursor_Simple,
};

enum Cursor {
    Cursor_Default = 0,
    Cursor_Arrow = 35,
    Cursor_Cross = 66,
    Cursor_Wait = 76,
    Cursor_Insert = 77,
    Cursor_Hand = 31,
    Cursor_Help = 47,
    Cursor_Move = 27,
    Cursor_NS = 78,
    Cursor_WE = 79,
    Cursor_NWSE = 80,
    Cursor_NESW = 81,
    Cursor_N = 70,
    Cursor_NE = 69,
    Cursor_E = 49,
    Cursor_SE = 8,
    Cursor_S = 9,
    Cursor_SW = 7,
    Cursor_W = 36,
    Cursor_NW = 68,
    Cursor_None = 255,
};

enum Chart {
    Chart_Bar = 0,
    Chart_HorizontalBar = 1,
    Chart_Line = 2,
    Chart_Fill = 3,
    Chart_Spike = 4,
    Chart_Pie = 5,
    Chart_SpecialPie = 6,
};

enum Clock {
    Square = 0,
    Round = 1,
};

enum LineStyle {
    SolidLine = 0,
    DashLine,
    DotLine,
    DashDotLine,
    DashDotDotLine,
    CapFlat = 100,
    CapRound = 200,
    CapSquare = 300,
    JoinMiter = 1000,
    JoinRound = 2000,
    JoinBevel = 3000,
};

enum Mode {
    Rgb = 0,
    Index = 1,
    Double = 2,
    Accum = 4,
    Alpha = 8,
    Depth = 16,
    Stencil = 32,
    Rgb8 = 64,
    MultiSample = 128,
    Stereo = 256,
    FakeSingle = 512, // Fake single buffered windows using double-buffer
    Opengl3 = 1024,
};

enum Browser {
    Normal = 0,
    Select = 1,
    Hold = 2,
    MultiBrowser = 3,
};

enum Button {
    NormalButton = 0,
    Toggle = 1,
    Radio = 102,
    Hidden = 3,
};

enum FileDialog {
    BrowseFile = 0,
    BrowseDir,
    BrowseMultiFile,
    BrowseMultiDir,
    BrowseSaveFile,
    BrowseSaveDir,
};

enum FileDialogOptions {
    NoOptions = 0,
    SaveAsConfirm = 1,
    NewFolder = 2,
    Preview = 4,
    UseFilterExt = 8,
};

enum Beep {
    Default = 0,
    Message,
    Error,
    Question,
    Password,
    Notification,
};

enum Pack {
    VerticalPack = 0,
    HorizontalPack = 1,
};

enum Scroll {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
    AlwaysOn = 4,
    HorizontalAlways = 5,
    VerticalAlways = 6,
    BothAlways = 7,
};

enum Input {
    NormalInput = 0,
    Float = 1,
    Int = 2,
    Multiline = 4,
    Secret = 5,
    Input = 7,
    Readonly = 8,
    Wrap = 16,
};

enum MenuFlag {
    NormalMenu = 0,
    InactiveMenu = 1,
    ToggleMenu = 2,
    ValueMenu = 4,
    RadioMenu = 8,
    InvisibleMenu = 0x10,
    SubmenuPointerMenu = 0x20,
    Submenu = 0x40,
    MenuDivider = 0x80,
    MenuHorizontal = 0x100,
};

enum TableContext {
    TableContextNone = 0,
    StartPage = 0x01,
    EndPage = 0x02,
    RowHeader = 0x04,
    ColHeader = 0x08,
    Cell = 0x10,
    Table = 0x20,
    RcResize = 0x40,
};

enum TableRowSelectMode {
    SelectNone,
    SelectSingle,
    SelectMulti,
};

enum TreeSort {
    Sort_None = 0,
    Ascending = 1,
    Descending = 2,
};

enum TreeConnectorStyle {
    Connector_None = 0,
    Dotted = 1,
    Solid = 2,
};

enum TreeSelect {
    TreeSelect_None = 0,
    Single = 1,
    Multi = 2,
    SingleDraggable = 3,
};

enum TreeItemSelect {
    Deselect = 0,
    ItemSelect = 1,
    ItemToggle = 2,
};

enum TreeReason {
    ReasonNone = 0,
    Selected,
    Deselected,
    Reselected,
    Opened,
    Closed,
    Dragged,
};

enum TreeItemReselectMode {
    Once = 0,
    Always,
};

enum TreeItemDrawMode {
    DrawModeDefault = 0,
    LabelAndWidget = 1,
    HeightFromWidget = 2,
};

enum Window {
    NormalWindow = 240,
    DoubleWindow = 241,
};

enum Slider {
    VerticalSlider = 0,
    HorizontalSlider = 1,
    VerticalFill = 2,
    HorizontalFill = 3,
    VerticalNice = 4,
    HorizontalNice = 5,
};

enum Dial {
    NormalDial = 0,
    Line = 1,
    Fill = 2,
};

enum Counter {
    NormalCounter = 0,
    Simple = 1,
};
