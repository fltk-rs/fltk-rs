#pragma once

enum BrowserType {
    BrowserType_Normal = 0,
    BrowserType_Select = 1,
    BrowserType_Hold = 2,
    BrowserType_Multi = 3,
};

enum BrowserScrollbar {
    BrowserScrollbar_None = 0,
    BrowserScrollbar_Horizontal = 1,
    BrowserScrollbar_Vertical = 2,
    BrowserScrollbar_Both = 3,
    BrowserScrollbar_AlwaysOn = 4,
    BrowserScrollbar_HorizontalAlways = 5,
    BrowserScrollbar_VerticalAlways = 6,
    BrowserScrollbar_BothAlways = 7,
};

enum FileType {
    FileType_Files = 0,
    FileType_Dirs,
};

enum ButtonType {
    ButtonType_Normal = 0,
    ButtonType_Toggle = 1,
    ButtonType_Radio = 102,
    ButtonType_Hidden = 3,
};

enum FileDialogType {
    FileDialogType_BrowseFile = 0,
    FileDialogType_BrowseDir,
    FileDialogType_BrowseMultiFile,
    FileDialogType_BrowseMultiDir,
    FileDialogType_BrowseSaveFile,
    FileDialogType_BrowseSaveDir,
};

enum FileDialogOptions {
    FileDialogOptions_NoOptions = 0,
    FileDialogOptions_SaveAsConfirm = 1,
    FileDialogOptions_NewFolder = 2,
    FileDialogOptions_Preview = 4,
    FileDialogOptions_UseFilterExt = 8,
};

enum BeepType {
    BeepType_Default = 0,
    BeepType_Message,
    BeepType_Error,
    BeepType_Question,
    BeepType_Password,
    BeepType_Notification,
};

enum FileChooserType {
    FileChooserType_Single = 0,
    FileChooserType_Multi = 1,
    FileChooserType_Create = 2,
    FileChooserType_Directory = 4,
};

enum LineStyle {
    LineStyle_Solid = 0,
    LineStyle_Dash,
    LineStyle_Dot,
    LineStyle_DashDot,
    LineStyle_DashDotDot,
    LineStyle_CapFlat = 100,
    LineStyle_CapRound = 200,
    LineStyle_CapSquare = 300,
    LineStyle_JoinMiter = 1000,
    LineStyle_JoinRound = 2000,
    LineStyle_JoinBevel = 3000,
};

enum LabelType {
    LabelType_Normal = 0,
    LabelType_None,
    LabelType_Shadow,
    LabelType_Engraved,
    LabelType_Embossed,
    LabelType_Multi,
    LabelType_Icon,
    LabelType_Image,
    LabelType_FreeType,
};

enum BoxType {
    BoxType_NoBox = 0,
    BoxType_FlatBox,
    BoxType_UpBox,
    BoxType_DownBox,
    BoxType_UpFrame,
    BoxType_DownFrame,
    BoxType_ThinUpBox,
    BoxType_ThinDownBox,
    BoxType_ThinUpFrame,
    BoxType_ThinDownFrame,
    BoxType_EngraveBox,
    BoxType_EmbossedBox,
    BoxType_EngravedFrame,
    BoxType_EmbossedFrame,
    BoxType_BorderBox,
    BoxType_ShadowBox,
    BoxType_BorderFrame,
    BoxType_ShadowFrame,
    BoxType_RoundedBox,
    BoxType_RShadowBox,
    BoxType_RoundedFrame,
    BoxType_RFlatBox,
    BoxType_RoundUpBox,
    BoxType_RoundDownBox,
    BoxType_DiamondUpBox,
    BoxType_DiamondDownBox,
    BoxType_OvalBox,
    BoxType_OShadowBox,
    BoxType_OvalFrame,
    BoxType_OFlatFrame,
    BoxType_PlasticUpBox,
    BoxType_PlasticDownBox,
    BoxType_PlasticUpFrame,
    BoxType_PlasticDownFrame,
    BoxType_PlasticThinUpBox,
    BoxType_PlasticThinDownBox,
    BoxType_PlasticRoundUpBox,
    BoxType_PlasticRoundDownBox,
    BoxType_GtkUpBox,
    BoxType_GtkDownBox,
    BoxType_GtkUpFrame,
    BoxType_GtkDownFrame,
    BoxType_GtkThinUpBox,
    BoxType_GtkThinDownBox,
    BoxType_GtkThinUpFrame,
    BoxType_GtkThinDownFrame,
    BoxType_GtkRoundUpFrame,
    BoxType_GtkRoundDownFrame,
    BoxType_GleamUpBox,
    BoxType_GleamDownBox,
    BoxType_GleamUpFrame,
    BoxType_GleamDownFrame,
    BoxType_GleamThinUpBox,
    BoxType_GleamThinDownBox,
    BoxType_GleamRoundUpBox,
    BoxType_GleamRoundDownBox,
    BoxType_FreeBoxType,
};

enum Align {
    Align_Center = 0,
    Align_Top = 1,
    Align_Bottom = 2,
    Align_Left = 4,
    Align_Right = 8,
};

enum Font {
    Font_Helvetica = 0,
    Font_HelveticaBold = 1,
    Font_HelveticaItalic = 2,
    Font_HelveticaBoldItalic = 3,
    Font_Courier = 4,
    Font_CourierBold = 5,
    Font_CourierItalic = 6,
    Font_CourierBoldItalic = 7,
    Font_Times = 8,
    Font_TimesBold = 9,
    Font_TimesItalic = 10,
    Font_TimesBoldItalic = 11,
    Font_Symbol = 12,
    Font_Screen = 13,
    Font_ScreenBold = 14,
    Font_Zapfdingbats = 15,
};

enum Color {
    Color_ForeGround = 0,
    Color_BackGround = 7,
    Color_Inactive = 8,
    Color_Selection = 15,
    Color_Gray0 = 32,
    Color_Dark3 = 39,
    Color_Dark2 = 45,
    Color_Dark1 = 47,
    Color_FrameDefault = 49,
    Color_Light1 = 50,
    Color_Light2 = 52,
    Color_Light3 = 54,
    Color_Black = 56,
    Color_Red = 88,
    Color_Green = 63,
    Color_Yellow = 95,
    Color_Blue = 216,
    Color_Magenta = 248,
    Color_Cyan = 223,
    Color_DarkRed = 72,
    Color_DarkGreen = 60,
    Color_DarkYellow = 76,
    Color_DarkBlue = 136,
    Color_DarkMagenta = 152,
    Color_DarkCyan = 140,
    Color_White = 255,
};

enum Event {
    Event_None = 0,
    Event_Push,
    Event_Released,
    Event_Enter,
    Event_Leave,
    Event_Drag,
    Event_Focus,
    Event_Unfocus,
    Event_KeyDown,
    Event_KeyUp,
    Event_Close,
    Event_Move,
    Event_Shortcut,
    Event_Deactivate,
    Event_Activate,
    Event_Hide,
    Event_Show,
    Event_Paste,
    Event_SelectionClear,
    Event_MouseWheel,
    Event_DndEnter,
    Event_DndDrag,
    Event_DndLeave,
    Event_DndRelease,
    Event_ScreenConfigChanged,
    Event_Fullscreen,
    Event_ZoomGesture,
    Event_ZoomEvent,
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
    Key_CapsLock = 0xffe5,
    Key_MetaL = 0xffe7,
    Key_MetaR = 0xffe8,
    Key_AltL = 0xffe9,
    Key_AltR = 0xffea,
    Key_Delete = 0xffff,
};

enum CallbackTrigger {
    CallbackTrigger_Never = 0,
    CallbackTrigger_Changed = 1,
    CallbackTrigger_NotChanged = 2,
    CallbackTrigger_Release = 4,
    CallbackTrigger_ReleaseAlways = 6,
    CallbackTrigger_EnterKey = 8,
    CallbackTrigger_EnterKeyAlways = 10,
    CallbackTrigger_EnterKeyChanged = 11,
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

enum Mode {
    Mode_Rgb = 0,
    Mode_Index = 1,
    Mode_Double = 2,
    Mode_Accum = 4,
    Mode_Alpha = 8,
    Mode_Depth = 16,
    Mode_Stencil = 32,
    Mode_Rgb8 = 64,
    Mode_MultiSample = 128,
    Mode_Stereo = 256,
    Mode_FakeSingle = 512,
    Mode_Opengl3 = 1024,
};

enum PackType {
    PackType_Vertical = 0,
    PackType_Horizontal = 1,
};

enum ScrollType {
    ScrollType_None = 0,
    ScrollType_Horizontal = 1,
    ScrollType_Vertical = 2,
    ScrollType_Both = 3,
    ScrollType_AlwaysOn = 4,
    ScrollType_HorizontalAlways = 5,
    ScrollType_VerticalAlways = 6,
    ScrollType_BothAlways = 7,
};

enum InputType {
    InputType_Normal = 0,
    InputType_Float = 1,
    InputType_Int = 2,
    InputType_Multiline = 4,
    InputType_Secret = 5,
    InputType_Input = 7,
    InputType_Readonly = 8,
    InputType_Wrap = 16,
};

enum MenuFlag {
    MenuFlag_Normal = 0,
    MenuFlag_Inactive = 1,
    MenuFlag_Toggle = 2,
    MenuFlag_Value = 4,
    MenuFlag_Radio = 8,
    MenuFlag_Invisible = 0x10,
    MenuFlag_SubmenuPointer = 0x20,
    MenuFlag_Submenu = 0x40,
    MenuFlag_MenuDivider = 0x80,
    MenuFlag_MenuHorizontal = 0x100,
};

enum ChartType {
    ChartType_Bar = 0,
    ChartType_HorizontalBar = 1,
    ChartType_Line = 2,
    ChartType_Fill = 3,
    ChartType_Spike = 4,
    ChartType_Pie = 5,
    ChartType_SpecialPie = 6,
};

enum ClockType {
    ClockType_Square = 0,
    ClockType_Round = 1,
};

enum TableContext {
    TableContext_None = 0,
    TableContext_StartPage = 0x01,
    TableContext_EndPage = 0x02,
    TableContext_RowHeader = 0x04,
    TableContext_ColHeader = 0x08,
    TableContext_Cell = 0x10,
    TableContext_Table = 0x20,
    TableContext_RcResize = 0x40,
};

enum TableRowSelectMode {
    TableRowSelectMode_SelectNone,
    TableRowSelectMode_SelectSingle,
    TableRowSelectMode_SelectMulti,
};

enum TreeSort {
    TreeSort_None = 0,
    TreeSort_Ascending = 1,
    TreeSort_Descending = 2,
};

enum TreeConnectorStyle {
    TreeConnectorStyle_None = 0,
    TreeConnectorStyle_Dotted = 1,
    TreeConnectorStyle_Solid = 2,
};

enum TreeSelect {
    TreeSelect_None = 0,
    TreeSelect_Single = 1,
    TreeSelect_Multi = 2,
    TreeSelect_SingleDraggable = 3,
};

enum TreeItemSelect {
    TreeItemSelect_Deselect = 0,
    TreeItemSelect_Select = 1,
    TreeItemSelect_Toggle = 2,
};

enum TreeReason {
    TreeReason_None = 0,
    TreeReason_Selected,
    TreeReason_Deselected,
    TreeReason_Reselected,
    TreeReason_Opened,
    TreeReason_Closed,
    TreeReason_Dragged,
};

enum TreeItemReselectMode {
    TreeItemReselectMode_Once = 0,
    TreeItemReselectMode_Always,
};

enum TreeItemDrawMode {
    TreeItemDrawMode_Default = 0,
    TreeItemDrawMode_LabelAndWidget = 1,
    TreeItemDrawMode_HeightFromWidget = 2,
};

enum SliderType {
    SliderType_Vertical = 0,
    SliderType_Horizontal = 1,
    SliderType_VerticalFill = 2,
    SliderType_HorizontalFill = 3,
    SliderType_VerticalNice = 4,
    SliderType_HorizontalNice = 5,
};

enum DialType {
    DialType_Normal = 0,
    DialType_Line = 1,
    DialType_Fill = 2,
};

enum CounterType {
    CounterType_Normal = 0,
    CounterType_Simple = 1,
};

enum ScrollbarType {
    ScrollbarType_Vertical = 0,
    ScrollbarType_Horizontal = 1,
    ScrollbarType_VerticalFill = 2,
    ScrollbarType_HorizontalFill = 3,
    ScrollbarType_VerticalNice = 4,
    ScrollbarType_HorizontalNice = 5,
};

enum WindowType {
    WindowType_Normal = 240,
    WindowType_Double = 241,
};

enum WrapMode {
    WrapNone,
    WrapAtColumn,
    WrapAtPixel,
    WrapAtBounds,
};

enum DragType {
    DragNone = -2,
    DragStartDnd = -1,
    DragChar = 0,
    DragWord = 1,
    DragLine = 2,
};