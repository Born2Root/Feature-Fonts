use font_kit::font::Font;
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

fn main() -> std::io::Result<()> {
    let mut output = String::from("feature calt {");

    let font_path = "./Roboto+SegMDL2.ttf";

    let mut file = File::open(font_path)?;
    let mut font_data = Vec::new();
    file.read_to_end(&mut font_data)?;

    let font = Font::from_path(Path::new(font_path), 0).unwrap();

    let face = match ttf_parser::Face::parse(&font_data, 0) {
        Ok(f) => f,
        Err(e) => {
            eprint!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let emojis = vec![
        ("GlobalNavigationButton", '\u{E700}'),
        ("Wifi", '\u{E701}'),
        ("Bluetooth", '\u{E702}'),
        ("Connect", '\u{E703}'),
        ("InternetSharing", '\u{E704}'),
        ("VPN", '\u{E705}'),
        ("Brightness", '\u{E706}'),
        ("MapPin", '\u{E707}'),
        ("QuietHours", '\u{E708}'),
        ("Airplane", '\u{E709}'),
        ("Tablet", '\u{E70A}'),
        ("QuickNote", '\u{E70B}'),
        ("RememberedDevice", '\u{E70C}'),
        ("ChevronDown", '\u{E70D}'),
        ("ChevronUp", '\u{E70E}'),
        ("Edit", '\u{E70F}'),
        ("Add", '\u{E710}'),
        ("Cancel", '\u{E711}'),
        ("More", '\u{E712}'),
        ("Setting", '\u{E713}'),
        ("Video", '\u{E714}'),
        ("Mail", '\u{E715}'),
        ("People", '\u{E716}'),
        ("Phone", '\u{E717}'),
        ("Pin", '\u{E718}'),
        ("Shop", '\u{E719}'),
        ("Stop", '\u{E71A}'),
        ("Link", '\u{E71B}'),
        ("Filter", '\u{E71C}'),
        ("AllApps", '\u{E71D}'),
        ("Zoom", '\u{E71E}'),
        ("ZoomOut", '\u{E71F}'),
        ("Microphone", '\u{E720}'),
        ("Search", '\u{E721}'),
        ("Camera", '\u{E722}'),
        ("Attach", '\u{E723}'),
        ("Send", '\u{E724}'),
        ("SendFill", '\u{E725}'),
        ("WalkSolid", '\u{E726}'),
        ("InPrivate", '\u{E727}'),
        ("FavoriteList", '\u{E728}'),
        ("PageSolid", '\u{E729}'),
        ("Forward", '\u{E72A}'),
        ("Back", '\u{E72B}'),
        ("Refresh", '\u{E72C}'),
        ("Share", '\u{E72D}'),
        ("Lock", '\u{E72E}'),
        ("ReportHacked", '\u{E730}'),
        ("EMI", '\u{E731}'),
        ("FavoriteStar", '\u{E734}'),
        ("FavoriteStarFill", '\u{E735}'),
        ("ReadingMode", '\u{E736}'),
        ("Favicon", '\u{E737}'),
        ("Remove", '\u{E738}'),
        ("Checkbox", '\u{E739}'),
        ("CheckboxComposite", '\u{E73A}'),
        ("CheckboxFill", '\u{E73B}'),
        ("CheckboxIndeterminate", '\u{E73C}'),
        ("CheckboxCompositeReversed", '\u{E73D}'),
        ("CheckMark", '\u{E73E}'),
        ("BackToWindow", '\u{E73F}'),
        ("FullScreen", '\u{E740}'),
        ("ResizeTouchLarger", '\u{E741}'),
        ("ResizeTouchSmaller", '\u{E742}'),
        ("ResizeMouseSmall", '\u{E743}'),
        ("ResizeMouseMedium", '\u{E744}'),
        ("ResizeMouseWide", '\u{E745}'),
        ("ResizeMouseTall", '\u{E746}'),
        ("ResizeMouseLarge", '\u{E747}'),
        ("SwitchUser", '\u{E748}'),
        ("Print", '\u{E749}'),
        ("Up", '\u{E74A}'),
        ("Down", '\u{E74B}'),
        ("OEM", '\u{E74C}'),
        ("Delete", '\u{E74D}'),
        ("Save", '\u{E74E}'),
        ("Mute", '\u{E74F}'),
        ("BackSpaceQWERTY", '\u{E750}'),
        ("ReturnKey", '\u{E751}'),
        ("UpArrowShiftKey", '\u{E752}'),
        ("Cloud", '\u{E753}'),
        ("Flashlight", '\u{E754}'),
        ("RotationLock", '\u{E755}'),
        ("CommandPrompt", '\u{E756}'),
        ("SIPMove", '\u{E759}'),
        ("SIPUndock", '\u{E75A}'),
        ("SIPRedock", '\u{E75B}'),
        ("EraseTool", '\u{E75C}'),
        ("UnderscoreSpace", '\u{E75D}'),
        ("GripperTool", '\u{E75E}'),
        ("Dialpad", '\u{E75F}'),
        ("PageLeft", '\u{E760}'),
        ("PageRight", '\u{E761}'),
        ("MultiSelect", '\u{E762}'),
        ("KeyboardLeftHanded", '\u{E763}'),
        ("KeyboardRightHanded", '\u{E764}'),
        ("KeyboardClassic", '\u{E765}'),
        ("KeyboardSplit", '\u{E766}'),
        ("Volume", '\u{E767}'),
        ("Play", '\u{E768}'),
        ("Pause", '\u{E769}'),
        ("ChevronLeft", '\u{E76B}'),
        ("ChevronRight", '\u{E76C}'),
        ("InkingTool", '\u{E76D}'),
        ("Emoji2", '\u{E76E}'),
        ("GripperBarHorizontal", '\u{E76F}'),
        ("System", '\u{E770}'),
        ("Personalize", '\u{E771}'),
        ("Devices", '\u{E772}'),
        ("SearchAndApps", '\u{E773}'),
        ("Globe", '\u{E774}'),
        ("TimeLanguage", '\u{E775}'),
        ("EaseOfAccess", '\u{E776}'),
        ("UpdateRestore", '\u{E777}'),
        ("HangUp", '\u{E778}'),
        ("ContactInfo", '\u{E779}'),
        ("Unpin", '\u{E77A}'),
        ("Contact", '\u{E77B}'),
        ("Memo", '\u{E77C}'),
        ("IncomingCall", '\u{E77E}'),
        ("Paste", '\u{E77F}'),
        ("PhoneBook", '\u{E780}'),
        ("LEDLight", '\u{E781}'),
        ("Error", '\u{E783}'),
        ("GripperBarVertical", '\u{E784}'),
        ("Unlock", '\u{E785}'),
        ("Slideshow", '\u{E786}'),
        ("Calendar", '\u{E787}'),
        ("GripperResize", '\u{E788}'),
        ("Megaphone", '\u{E789}'),
        ("Trim", '\u{E78A}'),
        ("NewWindow", '\u{E78B}'),
        ("SaveLocal", '\u{E78C}'),
        ("Color", '\u{E790}'),
        ("DataSense", '\u{E791}'),
        ("SaveAs", '\u{E792}'),
        ("Light", '\u{E793}'),
        ("AspectRatio", '\u{E799}'),
        ("DataSenseBar", '\u{E7A5}'),
        ("Redo", '\u{E7A6}'),
        ("Undo", '\u{E7A7}'),
        ("Crop", '\u{E7A8}'),
        ("OpenWith", '\u{E7AC}'),
        ("Rotate", '\u{E7AD}'),
        ("RedEye", '\u{E7B3}'),
        ("SetlockScreen", '\u{E7B5}'),
        ("MapPin2", '\u{E7B7}'),
        ("Package", '\u{E7B8}'),
        ("Warning", '\u{E7BA}'),
        ("ReadingList", '\u{E7BC}'),
        ("Education", '\u{E7BE}'),
        ("ShoppingCart", '\u{E7BF}'),
        ("Train", '\u{E7C0}'),
        ("Flag", '\u{E7C1}'),
        ("Page", '\u{E7C3}'),
        ("TaskView", '\u{E7C4}'),
        ("BrowsePhotos", '\u{E7C5}'),
        ("HalfStarLeft", '\u{E7C6}'),
        ("HalfStarRight", '\u{E7C7}'),
        ("Record", '\u{E7C8}'),
        ("TouchPointer", '\u{E7C9}'),
        ("LangJPN", '\u{E7DE}'),
        ("Ferry", '\u{E7E3}'),
        ("Highlight", '\u{E7E6}'),
        ("ActionCenterNotification", '\u{E7E7}'),
        ("PowerButton", '\u{E7E8}'),
        ("ResizeTouchNarrower", '\u{E7EA}'),
        ("ResizeTouchShorter", '\u{E7EB}'),
        ("DrivingMode", '\u{E7EC}'),
        ("RingerSilent", '\u{E7ED}'),
        ("OtherUser", '\u{E7EE}'),
        ("Admin", '\u{E7EF}'),
        ("CC", '\u{E7F0}'),
        ("SDCard", '\u{E7F1}'),
        ("CallForwarding", '\u{E7F2}'),
        ("SettingsDisplaySound", '\u{E7F3}'),
        ("TVMonitor", '\u{E7F4}'),
        ("Speakers", '\u{E7F5}'),
        ("Headphone", '\u{E7F6}'),
        ("DeviceLaptopPic", '\u{E7F7}'),
        ("DeviceLaptopNoPic", '\u{E7F8}'),
        ("DeviceMonitorRightPic", '\u{E7F9}'),
        ("DeviceMonitorLeftPic", '\u{E7FA}'),
        ("DeviceMonitorNoPic", '\u{E7FB}'),
        ("Game", '\u{E7FC}'),
        ("HorizontalTabKey", '\u{E7FD}'),
        ("StreetsideSplitMinimize", '\u{E802}'),
        ("StreetsideSplitExpand", '\u{E803}'),
        ("Car", '\u{E804}'),
        ("Walk", '\u{E805}'),
        ("Bus", '\u{E806}'),
        ("TiltUp", '\u{E809}'),
        ("TiltDown", '\u{E80A}'),
        ("CallControl", '\u{E80B}'),
        ("RotateMapRight", '\u{E80C}'),
        ("RotateMapLeft", '\u{E80D}'),
        ("Home", '\u{E80F}'),
        ("ParkingLocation", '\u{E811}'),
        ("MapCompassTop", '\u{E812}'),
        ("MapCompassBottom", '\u{E813}'),
        ("IncidentTriangle", '\u{E814}'),
        ("Touch", '\u{E815}'),
        ("MapDirections", '\u{E816}'),
        ("StartPoint", '\u{E819}'),
        ("StopPoint", '\u{E81A}'),
        ("EndPoint", '\u{E81B}'),
        ("History", '\u{E81C}'),
        ("Location", '\u{E81D}'),
        ("MapLayers", '\u{E81E}'),
        ("Accident", '\u{E81F}'),
        ("Work", '\u{E821}'),
        ("Construction", '\u{E822}'),
        ("Recent", '\u{E823}'),
        ("Bank", '\u{E825}'),
        ("DownloadMap", '\u{E826}'),
        ("InkingToolFill2", '\u{E829}'),
        ("HighlightFill2", '\u{E82A}'),
        ("EraseToolFill", '\u{E82B}'),
        ("EraseToolFill2", '\u{E82C}'),
        ("Dictionary", '\u{E82D}'),
        ("DictionaryAdd", '\u{E82E}'),
        ("ToolTip", '\u{E82F}'),
        ("ChromeBack", '\u{E830}'),
        ("ProvisioningPackage", '\u{E835}'),
        ("AddRemoteDevice", '\u{E836}'),
        ("FolderOpen", '\u{E838}'),
        ("Ethernet", '\u{E839}'),
        ("ShareBroadband", '\u{E83A}'),
        ("DirectAccess", '\u{E83B}'),
        ("DialUp", '\u{E83C}'),
        ("DefenderApp", '\u{E83D}'),
        ("BatteryCharging9", '\u{E83E}'),
        ("Battery10", '\u{E83F}'),
        ("Pinned", '\u{E840}'),
        ("PinFill", '\u{E841}'),
        ("PinnedFill", '\u{E842}'),
        ("PeriodKey", '\u{E843}'),
        ("PuncKey", '\u{E844}'),
        ("RevToggleKey", '\u{E845}'),
        ("RightArrowKeyTime1", '\u{E846}'),
        ("RightArrowKeyTime2", '\u{E847}'),
        ("LeftQuote", '\u{E848}'),
        ("RightQuote", '\u{E849}'),
        ("DownShiftKey", '\u{E84A}'),
        ("UpShiftKey", '\u{E84B}'),
        ("PuncKey0", '\u{E84C}'),
        ("PuncKeyLeftBottom", '\u{E84D}'),
        ("RightArrowKeyTime3", '\u{E84E}'),
        ("RightArrowKeyTime4", '\u{E84F}'),
        ("Battery0", '\u{E850}'),
        ("Battery1", '\u{E851}'),
        ("Battery2", '\u{E852}'),
        ("Battery3", '\u{E853}'),
        ("Battery4", '\u{E854}'),
        ("Battery5", '\u{E855}'),
        ("Battery6", '\u{E856}'),
        ("Battery7", '\u{E857}'),
        ("Battery8", '\u{E858}'),
        ("Battery9", '\u{E859}'),
        ("BatteryCharging0", '\u{E85A}'),
        ("BatteryCharging1", '\u{E85B}'),
        ("BatteryCharging2", '\u{E85C}'),
        ("BatteryCharging3", '\u{E85D}'),
        ("BatteryCharging4", '\u{E85E}'),
        ("BatteryCharging5", '\u{E85F}'),
        ("BatteryCharging6", '\u{E860}'),
        ("BatteryCharging7", '\u{E861}'),
        ("BatteryCharging8", '\u{E862}'),
        ("BatterySaver0", '\u{E863}'),
        ("BatterySaver1", '\u{E864}'),
        ("BatterySaver2", '\u{E865}'),
        ("BatterySaver3", '\u{E866}'),
        ("BatterySaver4", '\u{E867}'),
        ("BatterySaver5", '\u{E868}'),
        ("BatterySaver6", '\u{E869}'),
        ("BatterySaver7", '\u{E86A}'),
        ("BatterySaver8", '\u{E86B}'),
        ("SignalBars1", '\u{E86C}'),
        ("SignalBars2", '\u{E86D}'),
        ("SignalBars3", '\u{E86E}'),
        ("SignalBars4", '\u{E86F}'),
        ("SignalBars5", '\u{E870}'),
        ("SignalNotConnected", '\u{E871}'),
        ("Wifi1", '\u{E872}'),
        ("Wifi2", '\u{E873}'),
        ("Wifi3", '\u{E874}'),
        ("MobSIMLock", '\u{E875}'),
        ("MobSIMMissing", '\u{E876}'),
        ("Vibrate", '\u{E877}'),
        ("RoamingInternational", '\u{E878}'),
        ("RoamingDomestic", '\u{E879}'),
        ("CallForwardInternational", '\u{E87A}'),
        ("CallForwardRoaming", '\u{E87B}'),
        ("JpnRomanji", '\u{E87C}'),
        ("JpnRomanjiLock", '\u{E87D}'),
        ("JpnRomanjiShift", '\u{E87E}'),
        ("JpnRomanjiShiftLock", '\u{E87F}'),
        ("StatusDataTransfer", '\u{E880}'),
        ("StatusDataTransferVPN", '\u{E881}'),
        ("StatusDualSIM2", '\u{E882}'),
        ("StatusDualSIM2VPN", '\u{E883}'),
        ("StatusDualSIM1", '\u{E884}'),
        ("StatusDualSIM1VPN", '\u{E885}'),
        ("StatusSGLTE", '\u{E886}'),
        ("StatusSGLTECell", '\u{E887}'),
        ("StatusSGLTEDataVPN", '\u{E888}'),
        ("StatusVPN", '\u{E889}'),
        ("WifiHotspot", '\u{E88A}'),
        ("LanguageKor", '\u{E88B}'),
        ("LanguageCht", '\u{E88C}'),
        ("LanguageChs", '\u{E88D}'),
        ("USB", '\u{E88E}'),
        ("InkingToolFill", '\u{E88F}'),
        ("View", '\u{E890}'),
        ("HighlightFill", '\u{E891}'),
        ("Previous", '\u{E892}'),
        ("Next", '\u{E893}'),
        ("Clear", '\u{E894}'),
        ("Sync", '\u{E895}'),
        ("Download", '\u{E896}'),
        ("Help", '\u{E897}'),
        ("Upload", '\u{E898}'),
        ("Emoji", '\u{E899}'),
        ("TwoPage", '\u{E89A}'),
        ("LeaveChat", '\u{E89B}'),
        ("MailForward", '\u{E89C}'),
        ("RotateCamera", '\u{E89E}'),
        ("ClosePane", '\u{E89F}'),
        ("OpenPane", '\u{E8A0}'),
        ("PreviewLink", '\u{E8A1}'),
        ("AttachCamera", '\u{E8A2}'),
        ("ZoomIn", '\u{E8A3}'),
        ("Bookmarks", '\u{E8A4}'),
        ("Document", '\u{E8A5}'),
        ("ProtectedDocument", '\u{E8A6}'),
        ("OpenInNewWindow", '\u{E8A7}'),
        ("MailFill", '\u{E8A8}'),
        ("ViewAll", '\u{E8A9}'),
        ("VideoChat", '\u{E8AA}'),
        ("Switch", '\u{E8AB}'),
        ("Rename", '\u{E8AC}'),
        ("Go", '\u{E8AD}'),
        ("SurfaceHub", '\u{E8AE}'),
        ("Remote", '\u{E8AF}'),
        ("Click", '\u{E8B0}'),
        ("Shuffle", '\u{E8B1}'),
        ("Movies", '\u{E8B2}'),
        ("SelectAll", '\u{E8B3}'),
        ("Orientation", '\u{E8B4}'),
        ("Import", '\u{E8B5}'),
        ("ImportAll", '\u{E8B6}'),
        ("Folder", '\u{E8B7}'),
        ("Webcam", '\u{E8B8}'),
        ("Picture", '\u{E8B9}'),
        ("Caption", '\u{E8BA}'),
        ("ChromeClose", '\u{E8BB}'),
        ("ShowResults", '\u{E8BC}'),
        ("Message", '\u{E8BD}'),
        ("Leaf", '\u{E8BE}'),
        ("CalendarDay", '\u{E8BF}'),
        ("CalendarWeek", '\u{E8C0}'),
        ("Characters", '\u{E8C1}'),
        ("MailReplyAll", '\u{E8C2}'),
        ("Read", '\u{E8C3}'),
        ("ShowBcc", '\u{E8C4}'),
        ("HideBcc", '\u{E8C5}'),
        ("Cut", '\u{E8C6}'),
        ("PaymentCard", '\u{E8C7}'),
        ("Copy", '\u{E8C8}'),
        ("Important", '\u{E8C9}'),
        ("MailReply", '\u{E8CA}'),
        ("Sort", '\u{E8CB}'),
        ("MobileTablet", '\u{E8CC}'),
        ("DisconnectDrive", '\u{E8CD}'),
        ("MapDrive", '\u{E8CE}'),
        ("ContactPresence", '\u{E8CF}'),
        ("Priority", '\u{E8D0}'),
        ("GotoToday", '\u{E8D1}'),
        ("Font", '\u{E8D2}'),
        ("FontColor", '\u{E8D3}'),
        ("Contact2", '\u{E8D4}'),
        ("FolderFill", '\u{E8D5}'),
        ("Audio", '\u{E8D6}'),
        ("Permissions", '\u{E8D7}'),
        ("DisableUpdates", '\u{E8D8}'),
        ("Unfavorite", '\u{E8D9}'),
        ("OpenLocal", '\u{E8DA}'),
        ("Italic", '\u{E8DB}'),
        ("Underline", '\u{E8DC}'),
        ("Bold", '\u{E8DD}'),
        ("MoveToFolder", '\u{E8DE}'),
        ("LikeDislike", '\u{E8DF}'),
        ("Dislike", '\u{E8E0}'),
        ("Like", '\u{E8E1}'),
        ("AlignRight", '\u{E8E2}'),
        ("AlignCenter", '\u{E8E3}'),
        ("AlignLeft", '\u{E8E4}'),
        ("OpenFile", '\u{E8E5}'),
        ("ClearSelection", '\u{E8E6}'),
        ("FontDecrease", '\u{E8E7}'),
        ("FontIncrease", '\u{E8E8}'),
        ("FontSize", '\u{E8E9}'),
        ("CellPhone", '\u{E8EA}'),
        ("Reshare", '\u{E8EB}'),
        ("Tag", '\u{E8EC}'),
        ("RepeatOne", '\u{E8ED}'),
        ("RepeatAll", '\u{E8EE}'),
        ("Calculator", '\u{E8EF}'),
        ("Directions", '\u{E8F0}'),
        ("Library", '\u{E8F1}'),
        ("ChatBubbles", '\u{E8F2}'),
        ("PostUpdate", '\u{E8F3}'),
        ("NewFolder", '\u{E8F4}'),
        ("CalendarReply", '\u{E8F5}'),
        ("UnsyncFolder", '\u{E8F6}'),
        ("SyncFolder", '\u{E8F7}'),
        ("BlockContact", '\u{E8F8}'),
        ("SwitchApps", '\u{E8F9}'),
        ("AddFriend", '\u{E8FA}'),
        ("Accept", '\u{E8FB}'),
        ("GoToStart", '\u{E8FC}'),
        ("BulletedList", '\u{E8FD}'),
        ("Scan", '\u{E8FE}'),
        ("Preview", '\u{E8FF}'),
        ("Group", '\u{E902}'),
        ("ZeroBars", '\u{E904}'),
        ("OneBar", '\u{E905}'),
        ("TwoBars", '\u{E906}'),
        ("ThreeBars", '\u{E907}'),
        ("FourBars", '\u{E908}'),
        ("World", '\u{E909}'),
        ("Comment", '\u{E90A}'),
        ("MusicInfo", '\u{E90B}'),
        ("DockLeft", '\u{E90C}'),
        ("DockRight", '\u{E90D}'),
        ("DockBottom", '\u{E90E}'),
        ("Repair", '\u{E90F}'),
        ("Accounts", '\u{E910}'),
        ("DullSound", '\u{E911}'),
        ("Manage", '\u{E912}'),
        ("Street", '\u{E913}'),
        ("Printer3D", '\u{E914}'),
        ("RadioBullet", '\u{E915}'),
        ("Stopwatch", '\u{E916}'),
        ("Photo", '\u{E91B}'),
        ("ActionCenter", '\u{E91C}'),
        ("FullCircleMask", '\u{E91F}'),
        ("ChromeMinimize", '\u{E921}'),
        ("ChromeMaximize", '\u{E922}'),
        ("ChromeRestore", '\u{E923}'),
        ("Annotation", '\u{E924}'),
        ("BackSpaceQWERTYSm", '\u{E925}'),
        ("BackSpaceQWERTYMd", '\u{E926}'),
        ("Swipe", '\u{E927}'),
        ("Fingerprint", '\u{E928}'),
        ("Handwriting", '\u{E929}'),
        ("ChromeBackToWindow", '\u{E92C}'),
        ("ChromeFullScreen", '\u{E92D}'),
        ("KeyboardStandard", '\u{E92E}'),
        ("KeyboardDismiss", '\u{E92F}'),
        ("Completed", '\u{E930}'),
        ("ChromeAnnotate", '\u{E931}'),
        ("Label", '\u{E932}'),
        ("IBeam", '\u{E933}'),
        ("IBeamOutline", '\u{E934}'),
        ("FlickDown", '\u{E935}'),
        ("FlickUp", '\u{E936}'),
        ("FlickLeft", '\u{E937}'),
        ("FlickRight", '\u{E938}'),
        ("FeedbackApp", '\u{E939}'),
        ("MusicAlbum", '\u{E93C}'),
        ("Streaming", '\u{E93E}'),
        ("Code", '\u{E943}'),
        ("ReturnToWindow", '\u{E944}'),
        ("LightningBolt", '\u{E945}'),
        ("Info", '\u{E946}'),
        ("CalculatorMultiply", '\u{E947}'),
        ("CalculatorAddition", '\u{E948}'),
        ("CalculatorSubtract", '\u{E949}'),
        ("CalculatorDivide", '\u{E94A}'),
        ("CalculatorSquareroot", '\u{E94B}'),
        ("CalculatorPercentage", '\u{E94C}'),
        ("CalculatorNegate", '\u{E94D}'),
        ("CalculatorEqualTo", '\u{E94E}'),
        ("CalculatorBackspace", '\u{E94F}'),
        ("Component", '\u{E950}'),
        ("DMC", '\u{E951}'),
        ("Dock", '\u{E952}'),
        ("MultimediaDMS", '\u{E953}'),
        ("MultimediaDVR", '\u{E954}'),
        ("MultimediaPMP", '\u{E955}'),
        ("PrintfaxPrinterFile", '\u{E956}'),
        ("Sensor", '\u{E957}'),
        ("StorageOptical", '\u{E958}'),
        ("Communications", '\u{E95A}'),
        ("Headset", '\u{E95B}'),
        ("Projector", '\u{E95D}'),
        ("Health", '\u{E95E}'),
        ("Wire", '\u{E95F}'),
        ("Webcam2", '\u{E960}'),
        ("Input", '\u{E961}'),
        ("Mouse", '\u{E962}'),
        ("Smartcard", '\u{E963}'),
        ("SmartcardVirtual", '\u{E964}'),
        ("MediaStorageTower", '\u{E965}'),
        ("ReturnKeySm", '\u{E966}'),
        ("GameConsole", '\u{E967}'),
        ("Network", '\u{E968}'),
        ("StorageNetworkWireless", '\u{E969}'),
        ("StorageTape", '\u{E96A}'),
        ("ChevronUpSmall", '\u{E96D}'),
        ("ChevronDownSmall", '\u{E96E}'),
        ("ChevronLeftSmall", '\u{E96F}'),
        ("ChevronRightSmall", '\u{E970}'),
        ("ChevronUpMed", '\u{E971}'),
        ("ChevronDownMed", '\u{E972}'),
        ("ChevronLeftMed", '\u{E973}'),
        ("ChevronRightMed", '\u{E974}'),
        ("Devices2", '\u{E975}'),
        ("ExpandTile", '\u{E976}'),
        ("PC1", '\u{E977}'),
        ("PresenceChicklet", '\u{E978}'),
        ("PresenceChickletVideo", '\u{E979}'),
        ("Reply", '\u{E97A}'),
        ("SetTile", '\u{E97B}'),
        ("Type", '\u{E97C}'),
        ("Korean", '\u{E97D}'),
        ("HalfAlpha", '\u{E97E}'),
        ("FullAlpha", '\u{E97F}'),
        ("Key12On", '\u{E980}'),
        ("ChineseChangjie", '\u{E981}'),
        ("QWERTYOn", '\u{E982}'),
        ("QWERTYOff", '\u{E983}'),
        ("ChineseQuick", '\u{E984}'),
        ("Japanese", '\u{E985}'),
        ("FullHiragana", '\u{E986}'),
        ("FullKatakana", '\u{E987}'),
        ("HalfKatakana", '\u{E988}'),
        ("ChineseBoPoMoFo", '\u{E989}'),
        ("ChinesePinyin", '\u{E98A}'),
        ("ConstructionCone", '\u{E98F}'),
        ("XboxOneConsole", '\u{E990}'),
        ("Volume0", '\u{E992}'),
        ("Volume1", '\u{E993}'),
        ("Volume2", '\u{E994}'),
        ("Volume3", '\u{E995}'),
        ("BatteryUnknown", '\u{E996}'),
        ("WifiAttentionOverlay", '\u{E998}'),
        ("Robot", '\u{E99A}'),
        ("TapAndSend", '\u{E9A1}'),
        ("FitPage", '\u{E9A6}'),
        ("PasswordKeyShow", '\u{E9A8}'),
        ("PasswordKeyHide", '\u{E9A9}'),
        ("BidiLtr", '\u{E9AA}'),
        ("BidiRtl", '\u{E9AB}'),
        ("ForwardSm", '\u{E9AC}'),
        ("CommaKey", '\u{E9AD}'),
        ("DashKey", '\u{E9AE}'),
        ("DullSoundKey", '\u{E9AF}'),
        ("HalfDullSound", '\u{E9B0}'),
        ("RightDoubleQuote", '\u{E9B1}'),
        ("LeftDoubleQuote", '\u{E9B2}'),
        ("PuncKeyRightBottom", '\u{E9B3}'),
        ("PuncKey1", '\u{E9B4}'),
        ("PuncKey2", '\u{E9B5}'),
        ("PuncKey3", '\u{E9B6}'),
        ("PuncKey4", '\u{E9B7}'),
        ("PuncKey5", '\u{E9B8}'),
        ("PuncKey6", '\u{E9B9}'),
        ("PuncKey9", '\u{E9BA}'),
        ("PuncKey7", '\u{E9BB}'),
        ("PuncKey8", '\u{E9BC}'),
        ("Frigid", '\u{E9CA}'),
        ("Unknown", '\u{E9CE}'),
        ("AreaChart", '\u{E9D2}'),
        ("CheckList", '\u{E9D5}'),
        ("Diagnostic", '\u{E9D9}'),
        ("Equalizer", '\u{E9E9}'),
        ("Process", '\u{E9F3}'),
        ("Processing", '\u{E9F5}'),
        ("ReportDocument", '\u{E9F9}'),
    ];

    for emoji in emojis {
        let glyph_id_emoji = font.glyph_for_char(emoji.1);

        if glyph_id_emoji.is_none() {
            println!("no glyph_id for {}: {}", emoji.0, emoji.1);
            continue;
        }
        let glyph_id_emoji = glyph_id_emoji.unwrap();

        let emoji_name;
        if let Some(name) = face.glyph_name(ttf_parser::GlyphId(glyph_id_emoji as u16)) {
            emoji_name = name.to_string();
        } else {
            println!("No glyph names found.");
            continue;
        }
        output.push_str(&format!("\n  # replace {}", emoji.0));

        output.push_str(&format!(
            "\n  sub colon' {}colon' by {};\n",
            emoji
                .0
                .chars()
                .map(|c| {
                    let glyph_id = font.glyph_for_char(c).unwrap();

                    if let Some(name) = face.glyph_name(ttf_parser::GlyphId(glyph_id as u16)) {
                        return format!("{name}' ");
                    } else {
                        println!("No glyph names found.");
                    }
                    String::new()
                })
                .collect::<String>(),
            emoji_name
        ));
    }

    output.push_str(&format!("\n}} calt;"));

    fs::write("substitute.fea", output)?;

    Ok(())
}
