#[derive(Debug, Clone, Copy, Display)]
pub enum Icons {
    #[fmt("fab fa-accessible-icon")]
    AccessibleIcon,
    #[fmt("fab fa-accusoft")]
    Accusoft,
    #[fmt("fab fa-acquisitions-incorporated")]
    AcquisitionsIncorporated,
    #[fmt("fas fa-ad")]
    Ad,
    #[fmt("fas fa-address-book")]
    AddressBookSolid,
    #[fmt("far fa-address-book")]
    AddressBookRegular,
    #[fmt("fas fa-address-card")]
    AddressCardSolid,
    #[fmt("far fa-address-card")]
    AddressCardRegular,
    #[fmt("fas fa-adjust")]
    Adjust,
    #[fmt("fab fa-adn")]
    Adn,
    #[fmt("fab fa-adversal")]
    Adversal,
    #[fmt("fab fa-affiliatetheme")]
    Affiliatetheme,
    #[fmt("fas fa-air-freshener")]
    AirFreshener,
    #[fmt("fab fa-airbnb")]
    Airbnb,
    #[fmt("fab fa-algolia")]
    Algolia,
    #[fmt("fas fa-align-center")]
    AlignCenter,
    #[fmt("fas fa-align-justify")]
    AlignJustify,
    #[fmt("fas fa-align-left")]
    AlignLeft,
    #[fmt("fas fa-align-right")]
    AlignRight,
    #[fmt("fab fa-alipay")]
    Alipay,
    #[fmt("fas fa-allergies")]
    Allergies,
    #[fmt("fab fa-amazon")]
    Amazon,
    #[fmt("fab fa-amazon-pay")]
    AmazonPay,
    #[fmt("fas fa-ambulance")]
    Ambulance,
    #[fmt("fas fa-american-sign-language-interpreting")]
    AmericanSignLanguageInterpreting,
    #[fmt("fab fa-amilia")]
    Amilia,
    #[fmt("fas fa-anchor")]
    Anchor,
    #[fmt("fab fa-android")]
    Android,
    #[fmt("fab fa-angellist")]
    Angellist,
    #[fmt("fas fa-angle-double-down")]
    AngleDoubleDown,
    #[fmt("fas fa-angle-double-left")]
    AngleDoubleLeft,
    #[fmt("fas fa-angle-double-right")]
    AngleDoubleRight,
    #[fmt("fas fa-angle-double-up")]
    AngleDoubleUp,
    #[fmt("fas fa-angle-down")]
    AngleDown,
    #[fmt("fas fa-angle-left")]
    AngleLeft,
    #[fmt("fas fa-angle-right")]
    AngleRight,
    #[fmt("fas fa-angle-up")]
    AngleUp,
    #[fmt("fas fa-angry")]
    AngrySolid,
    #[fmt("far fa-angry")]
    AngryRegular,
    #[fmt("fab fa-angrycreative")]
    Angrycreative,
    #[fmt("fab fa-angular")]
    Angular,
    #[fmt("fas fa-ankh")]
    Ankh,
    #[fmt("fab fa-app-store")]
    AppStore,
    #[fmt("fab fa-app-store-ios")]
    AppStoreIos,
    #[fmt("fab fa-apper")]
    Apper,
    #[fmt("fab fa-apple")]
    Apple,
    #[fmt("fas fa-apple-alt")]
    AppleAlt,
    #[fmt("fab fa-apple-pay")]
    ApplePay,
    #[fmt("fas fa-archive")]
    Archive,
    #[fmt("fas fa-archway")]
    Archway,
    #[fmt("fas fa-arrow-alt-circle-down")]
    ArrowAltCircleDownSolid,
    #[fmt("far fa-arrow-alt-circle-down")]
    ArrowAltCircleDownRegular,
    #[fmt("fas fa-arrow-alt-circle-left")]
    ArrowAltCircleLeftSolid,
    #[fmt("far fa-arrow-alt-circle-left")]
    ArrowAltCircleLeftRegular,
    #[fmt("fas fa-arrow-alt-circle-right")]
    ArrowAltCircleRightSolid,
    #[fmt("far fa-arrow-alt-circle-right")]
    ArrowAltCircleRightRegular,
    #[fmt("fas fa-arrow-alt-circle-up")]
    ArrowAltCircleUpSolid,
    #[fmt("far fa-arrow-alt-circle-up")]
    ArrowAltCircleUpRegular,
    #[fmt("fas fa-arrow-circle-down")]
    ArrowCircleDown,
    #[fmt("fas fa-arrow-circle-left")]
    ArrowCircleLeft,
    #[fmt("fas fa-arrow-circle-right")]
    ArrowCircleRight,
    #[fmt("fas fa-arrow-circle-up")]
    ArrowCircleUp,
    #[fmt("fas fa-arrow-down")]
    ArrowDown,
    #[fmt("fas fa-arrow-left")]
    ArrowLeft,
    #[fmt("fas fa-arrow-right")]
    ArrowRight,
    #[fmt("fas fa-arrow-up")]
    ArrowUp,
    #[fmt("fas fa-arrows-alt")]
    ArrowsAlt,
    #[fmt("fas fa-arrows-alt-h")]
    ArrowsAltH,
    #[fmt("fas fa-arrows-alt-v")]
    ArrowsAltV,
    #[fmt("fab fa-artstation")]
    Artstation,
    #[fmt("fas fa-assistive-listening-systems")]
    AssistiveListeningSystems,
    #[fmt("fas fa-asterisk")]
    Asterisk,
    #[fmt("fab fa-asymmetrik")]
    Asymmetrik,
    #[fmt("fas fa-at")]
    At,
    #[fmt("fas fa-atlas")]
    Atlas,
    #[fmt("fab fa-atlassian")]
    Atlassian,
    #[fmt("fas fa-atom")]
    Atom,
    #[fmt("fab fa-audible")]
    Audible,
    #[fmt("fas fa-audio-description")]
    AudioDescription,
    #[fmt("fab fa-autoprefixer")]
    Autoprefixer,
    #[fmt("fab fa-avianex")]
    Avianex,
    #[fmt("fab fa-aviato")]
    Aviato,
    #[fmt("fas fa-award")]
    Award,
    #[fmt("fab fa-aws")]
    Aws,
    #[fmt("fas fa-baby")]
    Baby,
    #[fmt("fas fa-baby-carriage")]
    BabyCarriage,
    #[fmt("fas fa-backspace")]
    Backspace,
    #[fmt("fas fa-backward")]
    Backward,
    #[fmt("fas fa-bacon")]
    Bacon,
    #[fmt("fas fa-bacteria")]
    Bacteria,
    #[fmt("fas fa-bacterium")]
    Bacterium,
    #[fmt("fas fa-bahai")]
    Bahai,
    #[fmt("fas fa-balance-scale")]
    BalanceScale,
    #[fmt("fas fa-balance-scale-left")]
    BalanceScaleLeft,
    #[fmt("fas fa-balance-scale-right")]
    BalanceScaleRight,
    #[fmt("fas fa-ban")]
    Ban,
    #[fmt("fas fa-band-aid")]
    BandAid,
    #[fmt("fab fa-bandcamp")]
    Bandcamp,
    #[fmt("fas fa-barcode")]
    Barcode,
    #[fmt("fas fa-bars")]
    Bars,
    #[fmt("fas fa-baseball-ball")]
    BaseballBall,
    #[fmt("fas fa-basketball-ball")]
    BasketballBall,
    #[fmt("fas fa-bath")]
    Bath,
    #[fmt("fas fa-battery-empty")]
    BatteryEmpty,
    #[fmt("fas fa-battery-full")]
    BatteryFull,
    #[fmt("fas fa-battery-half")]
    BatteryHalf,
    #[fmt("fas fa-battery-quarter")]
    BatteryQuarter,
    #[fmt("fas fa-battery-three-quarters")]
    BatteryThreeQuarters,
    #[fmt("fab fa-battle-net")]
    BattleNet,
    #[fmt("fas fa-bed")]
    Bed,
    #[fmt("fas fa-beer")]
    Beer,
    #[fmt("fab fa-behance")]
    Behance,
    #[fmt("fab fa-behance-square")]
    BehanceSquare,
    #[fmt("fas fa-bell")]
    BellSolid,
    #[fmt("far fa-bell")]
    BellRegular,
    #[fmt("fas fa-bell-slash")]
    BellSlashSolid,
    #[fmt("far fa-bell-slash")]
    BellSlashRegular,
    #[fmt("fas fa-bezier-curve")]
    BezierCurve,
    #[fmt("fas fa-bible")]
    Bible,
    #[fmt("fas fa-bicycle")]
    Bicycle,
    #[fmt("fas fa-biking")]
    Biking,
    #[fmt("fab fa-bimobject")]
    Bimobject,
    #[fmt("fas fa-binoculars")]
    Binoculars,
    #[fmt("fas fa-biohazard")]
    Biohazard,
    #[fmt("fas fa-birthday-cake")]
    BirthdayCake,
    #[fmt("fab fa-bitbucket")]
    Bitbucket,
    #[fmt("fab fa-bitcoin")]
    Bitcoin,
    #[fmt("fab fa-bity")]
    Bity,
    #[fmt("fab fa-black-tie")]
    BlackTie,
    #[fmt("fab fa-blackberry")]
    Blackberry,
    #[fmt("fas fa-blender")]
    Blender,
    #[fmt("fas fa-blender-phone")]
    BlenderPhone,
    #[fmt("fas fa-blind")]
    Blind,
    #[fmt("fas fa-blog")]
    Blog,
    #[fmt("fab fa-blogger")]
    Blogger,
    #[fmt("fab fa-blogger-b")]
    BloggerB,
    #[fmt("fab fa-bluetooth")]
    Bluetooth,
    #[fmt("fab fa-bluetooth-b")]
    BluetoothB,
    #[fmt("fas fa-bold")]
    Bold,
    #[fmt("fas fa-bolt")]
    Bolt,
    #[fmt("fas fa-bomb")]
    Bomb,
    #[fmt("fas fa-bone")]
    Bone,
    #[fmt("fas fa-bong")]
    Bong,
    #[fmt("fas fa-book")]
    Book,
    #[fmt("fas fa-book-dead")]
    BookDead,
    #[fmt("fas fa-book-medical")]
    BookMedical,
    #[fmt("fas fa-book-open")]
    BookOpen,
    #[fmt("fas fa-book-reader")]
    BookReader,
    #[fmt("fas fa-bookmark")]
    BookmarkSolid,
    #[fmt("far fa-bookmark")]
    BookmarkRegular,
    #[fmt("fab fa-bootstrap")]
    Bootstrap,
    #[fmt("fas fa-border-all")]
    BorderAll,
    #[fmt("fas fa-border-none")]
    BorderNone,
    #[fmt("fas fa-border-style")]
    BorderStyle,
    #[fmt("fas fa-bowling-ball")]
    BowlingBall,
    #[fmt("fas fa-box")]
    Box,
    #[fmt("fas fa-box-open")]
    BoxOpen,
    #[fmt("fas fa-box-tissue")]
    BoxTissue,
    #[fmt("fas fa-boxes")]
    Boxes,
    #[fmt("fas fa-braille")]
    Braille,
    #[fmt("fas fa-brain")]
    Brain,
    #[fmt("fas fa-bread-slice")]
    BreadSlice,
    #[fmt("fas fa-briefcase")]
    Briefcase,
    #[fmt("fas fa-briefcase-medical")]
    BriefcaseMedical,
    #[fmt("fas fa-broadcast-tower")]
    BroadcastTower,
    #[fmt("fas fa-broom")]
    Broom,
    #[fmt("fas fa-brush")]
    Brush,
    #[fmt("fab fa-btc")]
    Btc,
    #[fmt("fab fa-buffer")]
    Buffer,
    #[fmt("fas fa-bug")]
    Bug,
    #[fmt("fas fa-building")]
    BuildingSolid,
    #[fmt("far fa-building")]
    BuildingRegular,
    #[fmt("fas fa-bullhorn")]
    Bullhorn,
    #[fmt("fas fa-bullseye")]
    Bullseye,
    #[fmt("fas fa-burn")]
    Burn,
    #[fmt("fab fa-buromobelexperte")]
    Buromobelexperte,
    #[fmt("fas fa-bus")]
    Bus,
    #[fmt("fas fa-bus-alt")]
    BusAlt,
    #[fmt("fas fa-business-time")]
    BusinessTime,
    #[fmt("fab fa-buy-n-large")]
    BuyNLarge,
    #[fmt("fab fa-buysellads")]
    Buysellads,
    #[fmt("fas fa-calculator")]
    Calculator,
    #[fmt("fas fa-calendar")]
    CalendarSolid,
    #[fmt("far fa-calendar")]
    CalendarRegular,
    #[fmt("fas fa-calendar-alt")]
    CalendarAltSolid,
    #[fmt("far fa-calendar-alt")]
    CalendarAltRegular,
    #[fmt("fas fa-calendar-check")]
    CalendarCheckSolid,
    #[fmt("far fa-calendar-check")]
    CalendarCheckRegular,
    #[fmt("fas fa-calendar-day")]
    CalendarDay,
    #[fmt("fas fa-calendar-minus")]
    CalendarMinusSolid,
    #[fmt("far fa-calendar-minus")]
    CalendarMinusRegular,
    #[fmt("fas fa-calendar-plus")]
    CalendarPlusSolid,
    #[fmt("far fa-calendar-plus")]
    CalendarPlusRegular,
    #[fmt("fas fa-calendar-times")]
    CalendarTimesSolid,
    #[fmt("far fa-calendar-times")]
    CalendarTimesRegular,
    #[fmt("fas fa-calendar-week")]
    CalendarWeek,
    #[fmt("fas fa-camera")]
    Camera,
    #[fmt("fas fa-camera-retro")]
    CameraRetro,
    #[fmt("fas fa-campground")]
    Campground,
    #[fmt("fab fa-canadian-maple-leaf")]
    CanadianMapleLeaf,
    #[fmt("fas fa-candy-cane")]
    CandyCane,
    #[fmt("fas fa-cannabis")]
    Cannabis,
    #[fmt("fas fa-capsules")]
    Capsules,
    #[fmt("fas fa-car")]
    Car,
    #[fmt("fas fa-car-alt")]
    CarAlt,
    #[fmt("fas fa-car-battery")]
    CarBattery,
    #[fmt("fas fa-car-crash")]
    CarCrash,
    #[fmt("fas fa-car-side")]
    CarSide,
    #[fmt("fas fa-caravan")]
    Caravan,
    #[fmt("fas fa-caret-down")]
    CaretDown,
    #[fmt("fas fa-caret-left")]
    CaretLeft,
    #[fmt("fas fa-caret-right")]
    CaretRight,
    #[fmt("fas fa-caret-square-down")]
    CaretSquareDownSolid,
    #[fmt("far fa-caret-square-down")]
    CaretSquareDownRegular,
    #[fmt("fas fa-caret-square-left")]
    CaretSquareLeftSolid,
    #[fmt("far fa-caret-square-left")]
    CaretSquareLeftRegular,
    #[fmt("fas fa-caret-square-right")]
    CaretSquareRightSolid,
    #[fmt("far fa-caret-square-right")]
    CaretSquareRightRegular,
    #[fmt("fas fa-caret-square-up")]
    CaretSquareUpSolid,
    #[fmt("far fa-caret-square-up")]
    CaretSquareUpRegular,
    #[fmt("fas fa-caret-up")]
    CaretUp,
    #[fmt("fas fa-carrot")]
    Carrot,
    #[fmt("fas fa-cart-arrow-down")]
    CartArrowDown,
    #[fmt("fas fa-cart-plus")]
    CartPlus,
    #[fmt("fas fa-cash-register")]
    CashRegister,
    #[fmt("fas fa-cat")]
    Cat,
    #[fmt("fab fa-cc-amazon-pay")]
    CcAmazonPay,
    #[fmt("fab fa-cc-amex")]
    CcAmex,
    #[fmt("fab fa-cc-apple-pay")]
    CcApplePay,
    #[fmt("fab fa-cc-diners-club")]
    CcDinersClub,
    #[fmt("fab fa-cc-discover")]
    CcDiscover,
    #[fmt("fab fa-cc-jcb")]
    CcJcb,
    #[fmt("fab fa-cc-mastercard")]
    CcMastercard,
    #[fmt("fab fa-cc-paypal")]
    CcPaypal,
    #[fmt("fab fa-cc-stripe")]
    CcStripe,
    #[fmt("fab fa-cc-visa")]
    CcVisa,
    #[fmt("fab fa-centercode")]
    Centercode,
    #[fmt("fab fa-centos")]
    Centos,
    #[fmt("fas fa-certificate")]
    Certificate,
    #[fmt("fas fa-chair")]
    Chair,
    #[fmt("fas fa-chalkboard")]
    Chalkboard,
    #[fmt("fas fa-chalkboard-teacher")]
    ChalkboardTeacher,
    #[fmt("fas fa-charging-station")]
    ChargingStation,
    #[fmt("fas fa-chart-area")]
    ChartArea,
    #[fmt("fas fa-chart-bar")]
    ChartBarSolid,
    #[fmt("far fa-chart-bar")]
    ChartBarRegular,
    #[fmt("fas fa-chart-line")]
    ChartLine,
    #[fmt("fas fa-chart-pie")]
    ChartPie,
    #[fmt("fas fa-check")]
    Check,
    #[fmt("fas fa-check-circle")]
    CheckCircleSolid,
    #[fmt("far fa-check-circle")]
    CheckCircleRegular,
    #[fmt("fas fa-check-double")]
    CheckDouble,
    #[fmt("fas fa-check-square")]
    CheckSquareSolid,
    #[fmt("far fa-check-square")]
    CheckSquareRegular,
    #[fmt("fas fa-cheese")]
    Cheese,
    #[fmt("fas fa-chess")]
    Chess,
    #[fmt("fas fa-chess-bishop")]
    ChessBishop,
    #[fmt("fas fa-chess-board")]
    ChessBoard,
    #[fmt("fas fa-chess-king")]
    ChessKing,
    #[fmt("fas fa-chess-knight")]
    ChessKnight,
    #[fmt("fas fa-chess-pawn")]
    ChessPawn,
    #[fmt("fas fa-chess-queen")]
    ChessQueen,
    #[fmt("fas fa-chess-rook")]
    ChessRook,
    #[fmt("fas fa-chevron-circle-down")]
    ChevronCircleDown,
    #[fmt("fas fa-chevron-circle-left")]
    ChevronCircleLeft,
    #[fmt("fas fa-chevron-circle-right")]
    ChevronCircleRight,
    #[fmt("fas fa-chevron-circle-up")]
    ChevronCircleUp,
    #[fmt("fas fa-chevron-down")]
    ChevronDown,
    #[fmt("fas fa-chevron-left")]
    ChevronLeft,
    #[fmt("fas fa-chevron-right")]
    ChevronRight,
    #[fmt("fas fa-chevron-up")]
    ChevronUp,
    #[fmt("fas fa-child")]
    Child,
    #[fmt("fab fa-chrome")]
    Chrome,
    #[fmt("fab fa-chromecast")]
    Chromecast,
    #[fmt("fas fa-church")]
    Church,
    #[fmt("fas fa-circle")]
    CircleSolid,
    #[fmt("far fa-circle")]
    CircleRegular,
    #[fmt("fas fa-circle-notch")]
    CircleNotch,
    #[fmt("fas fa-city")]
    City,
    #[fmt("fas fa-clinic-medical")]
    ClinicMedical,
    #[fmt("fas fa-clipboard")]
    ClipboardSolid,
    #[fmt("far fa-clipboard")]
    ClipboardRegular,
    #[fmt("fas fa-clipboard-check")]
    ClipboardCheck,
    #[fmt("fas fa-clipboard-list")]
    ClipboardList,
    #[fmt("fas fa-clock")]
    ClockSolid,
    #[fmt("far fa-clock")]
    ClockRegular,
    #[fmt("fas fa-clone")]
    CloneSolid,
    #[fmt("far fa-clone")]
    CloneRegular,
    #[fmt("fas fa-closed-captioning")]
    ClosedCaptioningSolid,
    #[fmt("far fa-closed-captioning")]
    ClosedCaptioningRegular,
    #[fmt("fas fa-cloud")]
    Cloud,
    #[fmt("fas fa-cloud-download-alt")]
    CloudDownloadAlt,
    #[fmt("fas fa-cloud-meatball")]
    CloudMeatball,
    #[fmt("fas fa-cloud-moon")]
    CloudMoon,
    #[fmt("fas fa-cloud-moon-rain")]
    CloudMoonRain,
    #[fmt("fas fa-cloud-rain")]
    CloudRain,
    #[fmt("fas fa-cloud-showers-heavy")]
    CloudShowersHeavy,
    #[fmt("fas fa-cloud-sun")]
    CloudSun,
    #[fmt("fas fa-cloud-sun-rain")]
    CloudSunRain,
    #[fmt("fas fa-cloud-upload-alt")]
    CloudUploadAlt,
    #[fmt("fab fa-cloudflare")]
    Cloudflare,
    #[fmt("fab fa-cloudscale")]
    Cloudscale,
    #[fmt("fab fa-cloudsmith")]
    Cloudsmith,
    #[fmt("fab fa-cloudversify")]
    Cloudversify,
    #[fmt("fas fa-cocktail")]
    Cocktail,
    #[fmt("fas fa-code")]
    Code,
    #[fmt("fas fa-code-branch")]
    CodeBranch,
    #[fmt("fab fa-codepen")]
    Codepen,
    #[fmt("fab fa-codiepie")]
    Codiepie,
    #[fmt("fas fa-coffee")]
    Coffee,
    #[fmt("fas fa-cog")]
    Cog,
    #[fmt("fas fa-cogs")]
    Cogs,
    #[fmt("fas fa-coins")]
    Coins,
    #[fmt("fas fa-columns")]
    Columns,
    #[fmt("fas fa-comment")]
    CommentSolid,
    #[fmt("far fa-comment")]
    CommentRegular,
    #[fmt("fas fa-comment-alt")]
    CommentAltSolid,
    #[fmt("far fa-comment-alt")]
    CommentAltRegular,
    #[fmt("fas fa-comment-dollar")]
    CommentDollar,
    #[fmt("fas fa-comment-dots")]
    CommentDotsSolid,
    #[fmt("far fa-comment-dots")]
    CommentDotsRegular,
    #[fmt("fas fa-comment-medical")]
    CommentMedical,
    #[fmt("fas fa-comment-slash")]
    CommentSlash,
    #[fmt("fas fa-comments")]
    CommentsSolid,
    #[fmt("far fa-comments")]
    CommentsRegular,
    #[fmt("fas fa-comments-dollar")]
    CommentsDollar,
    #[fmt("fas fa-compact-disc")]
    CompactDisc,
    #[fmt("fas fa-compass")]
    CompassSolid,
    #[fmt("far fa-compass")]
    CompassRegular,
    #[fmt("fas fa-compress")]
    Compress,
    #[fmt("fas fa-compress-alt")]
    CompressAlt,
    #[fmt("fas fa-compress-arrows-alt")]
    CompressArrowsAlt,
    #[fmt("fas fa-concierge-bell")]
    ConciergeBell,
    #[fmt("fab fa-confluence")]
    Confluence,
    #[fmt("fab fa-connectdevelop")]
    Connectdevelop,
    #[fmt("fab fa-contao")]
    Contao,
    #[fmt("fas fa-cookie")]
    Cookie,
    #[fmt("fas fa-cookie-bite")]
    CookieBite,
    #[fmt("fas fa-copy")]
    CopySolid,
    #[fmt("far fa-copy")]
    CopyRegular,
    #[fmt("fas fa-copyright")]
    CopyrightSolid,
    #[fmt("far fa-copyright")]
    CopyrightRegular,
    #[fmt("fab fa-cotton-bureau")]
    CottonBureau,
    #[fmt("fas fa-couch")]
    Couch,
    #[fmt("fab fa-cpanel")]
    Cpanel,
    #[fmt("fab fa-creative-commons")]
    CreativeCommons,
    #[fmt("fab fa-creative-commons-by")]
    CreativeCommonsBy,
    #[fmt("fab fa-creative-commons-nc")]
    CreativeCommonsNc,
    #[fmt("fab fa-creative-commons-nc-eu")]
    CreativeCommonsNcEu,
    #[fmt("fab fa-creative-commons-nc-jp")]
    CreativeCommonsNcJp,
    #[fmt("fab fa-creative-commons-nd")]
    CreativeCommonsNd,
    #[fmt("fab fa-creative-commons-pd")]
    CreativeCommonsPd,
    #[fmt("fab fa-creative-commons-pd-alt")]
    CreativeCommonsPdAlt,
    #[fmt("fab fa-creative-commons-remix")]
    CreativeCommonsRemix,
    #[fmt("fab fa-creative-commons-sa")]
    CreativeCommonsSa,
    #[fmt("fab fa-creative-commons-sampling")]
    CreativeCommonsSampling,
    #[fmt("fab fa-creative-commons-sampling-plus")]
    CreativeCommonsSamplingPlus,
    #[fmt("fab fa-creative-commons-share")]
    CreativeCommonsShare,
    #[fmt("fab fa-creative-commons-zero")]
    CreativeCommonsZero,
    #[fmt("fas fa-credit-card")]
    CreditCardSolid,
    #[fmt("far fa-credit-card")]
    CreditCardRegular,
    #[fmt("fab fa-critical-role")]
    CriticalRole,
    #[fmt("fas fa-crop")]
    Crop,
    #[fmt("fas fa-crop-alt")]
    CropAlt,
    #[fmt("fas fa-cross")]
    Cross,
    #[fmt("fas fa-crosshairs")]
    Crosshairs,
    #[fmt("fas fa-crow")]
    Crow,
    #[fmt("fas fa-crown")]
    Crown,
    #[fmt("fas fa-crutch")]
    Crutch,
    #[fmt("fab fa-css3")]
    Css3,
    #[fmt("fab fa-css3-alt")]
    Css3Alt,
    #[fmt("fas fa-cube")]
    Cube,
    #[fmt("fas fa-cubes")]
    Cubes,
    #[fmt("fas fa-cut")]
    Cut,
    #[fmt("fab fa-cuttlefish")]
    Cuttlefish,
    #[fmt("fab fa-d-and-d")]
    DAndD,
    #[fmt("fab fa-d-and-d-beyond")]
    DAndDBeyond,
    #[fmt("fab fa-dailymotion")]
    Dailymotion,
    #[fmt("fab fa-dashcube")]
    Dashcube,
    #[fmt("fas fa-database")]
    Database,
    #[fmt("fas fa-deaf")]
    Deaf,
    #[fmt("fab fa-deezer")]
    Deezer,
    #[fmt("fab fa-delicious")]
    Delicious,
    #[fmt("fas fa-democrat")]
    Democrat,
    #[fmt("fab fa-deploydog")]
    Deploydog,
    #[fmt("fab fa-deskpro")]
    Deskpro,
    #[fmt("fas fa-desktop")]
    Desktop,
    #[fmt("fab fa-dev")]
    Dev,
    #[fmt("fab fa-deviantart")]
    Deviantart,
    #[fmt("fas fa-dharmachakra")]
    Dharmachakra,
    #[fmt("fab fa-dhl")]
    Dhl,
    #[fmt("fas fa-diagnoses")]
    Diagnoses,
    #[fmt("fab fa-diaspora")]
    Diaspora,
    #[fmt("fas fa-dice")]
    Dice,
    #[fmt("fas fa-dice-d20")]
    DiceD20,
    #[fmt("fas fa-dice-d6")]
    DiceD6,
    #[fmt("fas fa-dice-five")]
    DiceFive,
    #[fmt("fas fa-dice-four")]
    DiceFour,
    #[fmt("fas fa-dice-one")]
    DiceOne,
    #[fmt("fas fa-dice-six")]
    DiceSix,
    #[fmt("fas fa-dice-three")]
    DiceThree,
    #[fmt("fas fa-dice-two")]
    DiceTwo,
    #[fmt("fab fa-digg")]
    Digg,
    #[fmt("fab fa-digital-ocean")]
    DigitalOcean,
    #[fmt("fas fa-digital-tachograph")]
    DigitalTachograph,
    #[fmt("fas fa-directions")]
    Directions,
    #[fmt("fab fa-discord")]
    Discord,
    #[fmt("fab fa-discourse")]
    Discourse,
    #[fmt("fas fa-disease")]
    Disease,
    #[fmt("fas fa-divide")]
    Divide,
    #[fmt("fas fa-dizzy")]
    DizzySolid,
    #[fmt("far fa-dizzy")]
    DizzyRegular,
    #[fmt("fas fa-dna")]
    Dna,
    #[fmt("fab fa-dochub")]
    Dochub,
    #[fmt("fab fa-docker")]
    Docker,
    #[fmt("fas fa-dog")]
    Dog,
    #[fmt("fas fa-dollar-sign")]
    DollarSign,
    #[fmt("fas fa-dolly")]
    Dolly,
    #[fmt("fas fa-dolly-flatbed")]
    DollyFlatbed,
    #[fmt("fas fa-donate")]
    Donate,
    #[fmt("fas fa-door-closed")]
    DoorClosed,
    #[fmt("fas fa-door-open")]
    DoorOpen,
    #[fmt("fas fa-dot-circle")]
    DotCircleSolid,
    #[fmt("far fa-dot-circle")]
    DotCircleRegular,
    #[fmt("fas fa-dove")]
    Dove,
    #[fmt("fas fa-download")]
    Download,
    #[fmt("fab fa-draft2digital")]
    Draft2Digital,
    #[fmt("fas fa-drafting-compass")]
    DraftingCompass,
    #[fmt("fas fa-dragon")]
    Dragon,
    #[fmt("fas fa-draw-polygon")]
    DrawPolygon,
    #[fmt("fab fa-dribbble")]
    Dribbble,
    #[fmt("fab fa-dribbble-square")]
    DribbbleSquare,
    #[fmt("fab fa-dropbox")]
    Dropbox,
    #[fmt("fas fa-drum")]
    Drum,
    #[fmt("fas fa-drum-steelpan")]
    DrumSteelpan,
    #[fmt("fas fa-drumstick-bite")]
    DrumstickBite,
    #[fmt("fab fa-drupal")]
    Drupal,
    #[fmt("fas fa-dumbbell")]
    Dumbbell,
    #[fmt("fas fa-dumpster")]
    Dumpster,
    #[fmt("fas fa-dumpster-fire")]
    DumpsterFire,
    #[fmt("fas fa-dungeon")]
    Dungeon,
    #[fmt("fab fa-dyalog")]
    Dyalog,
    #[fmt("fab fa-earlybirds")]
    Earlybirds,
    #[fmt("fab fa-ebay")]
    Ebay,
    #[fmt("fab fa-edge")]
    Edge,
    #[fmt("fab fa-edge-legacy")]
    EdgeLegacy,
    #[fmt("fas fa-edit")]
    EditSolid,
    #[fmt("far fa-edit")]
    EditRegular,
    #[fmt("fas fa-egg")]
    Egg,
    #[fmt("fas fa-eject")]
    Eject,
    #[fmt("fab fa-elementor")]
    Elementor,
    #[fmt("fas fa-ellipsis-h")]
    EllipsisH,
    #[fmt("fas fa-ellipsis-v")]
    EllipsisV,
    #[fmt("fab fa-ello")]
    Ello,
    #[fmt("fab fa-ember")]
    Ember,
    #[fmt("fab fa-empire")]
    Empire,
    #[fmt("fas fa-envelope")]
    EnvelopeSolid,
    #[fmt("far fa-envelope")]
    EnvelopeRegular,
    #[fmt("fas fa-envelope-open")]
    EnvelopeOpenSolid,
    #[fmt("far fa-envelope-open")]
    EnvelopeOpenRegular,
    #[fmt("fas fa-envelope-open-text")]
    EnvelopeOpenText,
    #[fmt("fas fa-envelope-square")]
    EnvelopeSquare,
    #[fmt("fab fa-envira")]
    Envira,
    #[fmt("fas fa-equals")]
    Equals,
    #[fmt("fas fa-eraser")]
    Eraser,
    #[fmt("fab fa-erlang")]
    Erlang,
    #[fmt("fab fa-ethereum")]
    Ethereum,
    #[fmt("fas fa-ethernet")]
    Ethernet,
    #[fmt("fab fa-etsy")]
    Etsy,
    #[fmt("fas fa-euro-sign")]
    EuroSign,
    #[fmt("fab fa-evernote")]
    Evernote,
    #[fmt("fas fa-exchange-alt")]
    ExchangeAlt,
    #[fmt("fas fa-exclamation")]
    Exclamation,
    #[fmt("fas fa-exclamation-circle")]
    ExclamationCircle,
    #[fmt("fas fa-exclamation-triangle")]
    ExclamationTriangle,
    #[fmt("fas fa-expand")]
    Expand,
    #[fmt("fas fa-expand-alt")]
    ExpandAlt,
    #[fmt("fas fa-expand-arrows-alt")]
    ExpandArrowsAlt,
    #[fmt("fab fa-expeditedssl")]
    Expeditedssl,
    #[fmt("fas fa-external-link-alt")]
    ExternalLinkAlt,
    #[fmt("fas fa-external-link-square-alt")]
    ExternalLinkSquareAlt,
    #[fmt("fas fa-eye")]
    EyeSolid,
    #[fmt("far fa-eye")]
    EyeRegular,
    #[fmt("fas fa-eye-dropper")]
    EyeDropper,
    #[fmt("fas fa-eye-slash")]
    EyeSlashSolid,
    #[fmt("far fa-eye-slash")]
    EyeSlashRegular,
    #[fmt("fab fa-facebook")]
    Facebook,
    #[fmt("fab fa-facebook-f")]
    FacebookF,
    #[fmt("fab fa-facebook-messenger")]
    FacebookMessenger,
    #[fmt("fab fa-facebook-square")]
    FacebookSquare,
    #[fmt("fas fa-fan")]
    Fan,
    #[fmt("fab fa-fantasy-flight-games")]
    FantasyFlightGames,
    #[fmt("fas fa-fast-backward")]
    FastBackward,
    #[fmt("fas fa-fast-forward")]
    FastForward,
    #[fmt("fas fa-faucet")]
    Faucet,
    #[fmt("fas fa-fax")]
    Fax,
    #[fmt("fas fa-feather")]
    Feather,
    #[fmt("fas fa-feather-alt")]
    FeatherAlt,
    #[fmt("fab fa-fedex")]
    Fedex,
    #[fmt("fab fa-fedora")]
    Fedora,
    #[fmt("fas fa-female")]
    Female,
    #[fmt("fas fa-fighter-jet")]
    FighterJet,
    #[fmt("fab fa-figma")]
    Figma,
    #[fmt("fas fa-file")]
    FileSolid,
    #[fmt("far fa-file")]
    FileRegular,
    #[fmt("fas fa-file-alt")]
    FileAltSolid,
    #[fmt("far fa-file-alt")]
    FileAltRegular,
    #[fmt("fas fa-file-archive")]
    FileArchiveSolid,
    #[fmt("far fa-file-archive")]
    FileArchiveRegular,
    #[fmt("fas fa-file-audio")]
    FileAudioSolid,
    #[fmt("far fa-file-audio")]
    FileAudioRegular,
    #[fmt("fas fa-file-code")]
    FileCodeSolid,
    #[fmt("far fa-file-code")]
    FileCodeRegular,
    #[fmt("fas fa-file-contract")]
    FileContract,
    #[fmt("fas fa-file-csv")]
    FileCsv,
    #[fmt("fas fa-file-download")]
    FileDownload,
    #[fmt("fas fa-file-excel")]
    FileExcelSolid,
    #[fmt("far fa-file-excel")]
    FileExcelRegular,
    #[fmt("fas fa-file-export")]
    FileExport,
    #[fmt("fas fa-file-image")]
    FileImageSolid,
    #[fmt("far fa-file-image")]
    FileImageRegular,
    #[fmt("fas fa-file-import")]
    FileImport,
    #[fmt("fas fa-file-invoice")]
    FileInvoice,
    #[fmt("fas fa-file-invoice-dollar")]
    FileInvoiceDollar,
    #[fmt("fas fa-file-medical")]
    FileMedical,
    #[fmt("fas fa-file-medical-alt")]
    FileMedicalAlt,
    #[fmt("fas fa-file-pdf")]
    FilePdfSolid,
    #[fmt("far fa-file-pdf")]
    FilePdfRegular,
    #[fmt("fas fa-file-powerpoint")]
    FilePowerpointSolid,
    #[fmt("far fa-file-powerpoint")]
    FilePowerpointRegular,
    #[fmt("fas fa-file-prescription")]
    FilePrescription,
    #[fmt("fas fa-file-signature")]
    FileSignature,
    #[fmt("fas fa-file-upload")]
    FileUpload,
    #[fmt("fas fa-file-video")]
    FileVideoSolid,
    #[fmt("far fa-file-video")]
    FileVideoRegular,
    #[fmt("fas fa-file-word")]
    FileWordSolid,
    #[fmt("far fa-file-word")]
    FileWordRegular,
    #[fmt("fas fa-fill")]
    Fill,
    #[fmt("fas fa-fill-drip")]
    FillDrip,
    #[fmt("fas fa-film")]
    Film,
    #[fmt("fas fa-filter")]
    Filter,
    #[fmt("fas fa-fingerprint")]
    Fingerprint,
    #[fmt("fas fa-fire")]
    Fire,
    #[fmt("fas fa-fire-alt")]
    FireAlt,
    #[fmt("fas fa-fire-extinguisher")]
    FireExtinguisher,
    #[fmt("fab fa-firefox")]
    Firefox,
    #[fmt("fab fa-firefox-browser")]
    FirefoxBrowser,
    #[fmt("fas fa-first-aid")]
    FirstAid,
    #[fmt("fab fa-first-order")]
    FirstOrder,
    #[fmt("fab fa-first-order-alt")]
    FirstOrderAlt,
    #[fmt("fab fa-firstdraft")]
    Firstdraft,
    #[fmt("fas fa-fish")]
    Fish,
    #[fmt("fas fa-fist-raised")]
    FistRaised,
    #[fmt("fas fa-flag")]
    FlagSolid,
    #[fmt("far fa-flag")]
    FlagRegular,
    #[fmt("fas fa-flag-checkered")]
    FlagCheckered,
    #[fmt("fas fa-flag-usa")]
    FlagUsa,
    #[fmt("fas fa-flask")]
    Flask,
    #[fmt("fab fa-flickr")]
    Flickr,
    #[fmt("fab fa-flipboard")]
    Flipboard,
    #[fmt("fas fa-flushed")]
    FlushedSolid,
    #[fmt("far fa-flushed")]
    FlushedRegular,
    #[fmt("fab fa-fly")]
    Fly,
    #[fmt("fas fa-folder")]
    FolderSolid,
    #[fmt("far fa-folder")]
    FolderRegular,
    #[fmt("fas fa-folder-minus")]
    FolderMinus,
    #[fmt("fas fa-folder-open")]
    FolderOpenSolid,
    #[fmt("far fa-folder-open")]
    FolderOpenRegular,
    #[fmt("fas fa-folder-plus")]
    FolderPlus,
    #[fmt("fas fa-font")]
    Font,
    #[fmt("fab fa-font-awesome")]
    FontAwesome,
    #[fmt("fab fa-font-awesome-alt")]
    FontAwesomeAlt,
    #[fmt("fab fa-font-awesome-flag")]
    FontAwesomeFlag,
    #[fmt("far fa-font-awesome-logo-full")]
    FontAwesomeLogoFullRegular,
    #[fmt("fas fa-font-awesome-logo-full")]
    FontAwesomeLogoFullSolid,
    #[fmt("fab fa-font-awesome-logo-full")]
    FontAwesomeLogoFullBrand,
    #[fmt("fab fa-fonticons")]
    Fonticons,
    #[fmt("fab fa-fonticons-fi")]
    FonticonsFi,
    #[fmt("fas fa-football-ball")]
    FootballBall,
    #[fmt("fab fa-fort-awesome")]
    FortAwesome,
    #[fmt("fab fa-fort-awesome-alt")]
    FortAwesomeAlt,
    #[fmt("fab fa-forumbee")]
    Forumbee,
    #[fmt("fas fa-forward")]
    Forward,
    #[fmt("fab fa-foursquare")]
    Foursquare,
    #[fmt("fab fa-free-code-camp")]
    FreeCodeCamp,
    #[fmt("fab fa-freebsd")]
    Freebsd,
    #[fmt("fas fa-frog")]
    Frog,
    #[fmt("fas fa-frown")]
    FrownSolid,
    #[fmt("far fa-frown")]
    FrownRegular,
    #[fmt("fas fa-frown-open")]
    FrownOpenSolid,
    #[fmt("far fa-frown-open")]
    FrownOpenRegular,
    #[fmt("fab fa-fulcrum")]
    Fulcrum,
    #[fmt("fas fa-funnel-dollar")]
    FunnelDollar,
    #[fmt("fas fa-futbol")]
    FutbolSolid,
    #[fmt("far fa-futbol")]
    FutbolRegular,
    #[fmt("fab fa-galactic-republic")]
    GalacticRepublic,
    #[fmt("fab fa-galactic-senate")]
    GalacticSenate,
    #[fmt("fas fa-gamepad")]
    Gamepad,
    #[fmt("fas fa-gas-pump")]
    GasPump,
    #[fmt("fas fa-gavel")]
    Gavel,
    #[fmt("fas fa-gem")]
    GemSolid,
    #[fmt("far fa-gem")]
    GemRegular,
    #[fmt("fas fa-genderless")]
    Genderless,
    #[fmt("fab fa-get-pocket")]
    GetPocket,
    #[fmt("fab fa-gg")]
    Gg,
    #[fmt("fab fa-gg-circle")]
    GgCircle,
    #[fmt("fas fa-ghost")]
    Ghost,
    #[fmt("fas fa-gift")]
    Gift,
    #[fmt("fas fa-gifts")]
    Gifts,
    #[fmt("fab fa-git")]
    Git,
    #[fmt("fab fa-git-alt")]
    GitAlt,
    #[fmt("fab fa-git-square")]
    GitSquare,
    #[fmt("fab fa-github")]
    Github,
    #[fmt("fab fa-github-alt")]
    GithubAlt,
    #[fmt("fab fa-github-square")]
    GithubSquare,
    #[fmt("fab fa-gitkraken")]
    Gitkraken,
    #[fmt("fab fa-gitlab")]
    Gitlab,
    #[fmt("fab fa-gitter")]
    Gitter,
    #[fmt("fas fa-glass-cheers")]
    GlassCheers,
    #[fmt("fas fa-glass-martini")]
    GlassMartini,
    #[fmt("fas fa-glass-martini-alt")]
    GlassMartiniAlt,
    #[fmt("fas fa-glass-whiskey")]
    GlassWhiskey,
    #[fmt("fas fa-glasses")]
    Glasses,
    #[fmt("fab fa-glide")]
    Glide,
    #[fmt("fab fa-glide-g")]
    GlideG,
    #[fmt("fas fa-globe")]
    Globe,
    #[fmt("fas fa-globe-africa")]
    GlobeAfrica,
    #[fmt("fas fa-globe-americas")]
    GlobeAmericas,
    #[fmt("fas fa-globe-asia")]
    GlobeAsia,
    #[fmt("fas fa-globe-europe")]
    GlobeEurope,
    #[fmt("fab fa-gofore")]
    Gofore,
    #[fmt("fas fa-golf-ball")]
    GolfBall,
    #[fmt("fab fa-goodreads")]
    Goodreads,
    #[fmt("fab fa-goodreads-g")]
    GoodreadsG,
    #[fmt("fab fa-google")]
    Google,
    #[fmt("fab fa-google-drive")]
    GoogleDrive,
    #[fmt("fab fa-google-pay")]
    GooglePay,
    #[fmt("fab fa-google-play")]
    GooglePlay,
    #[fmt("fab fa-google-plus")]
    GooglePlus,
    #[fmt("fab fa-google-plus-g")]
    GooglePlusG,
    #[fmt("fab fa-google-plus-square")]
    GooglePlusSquare,
    #[fmt("fab fa-google-wallet")]
    GoogleWallet,
    #[fmt("fas fa-gopuram")]
    Gopuram,
    #[fmt("fas fa-graduation-cap")]
    GraduationCap,
    #[fmt("fab fa-gratipay")]
    Gratipay,
    #[fmt("fab fa-grav")]
    Grav,
    #[fmt("fas fa-greater-than")]
    GreaterThan,
    #[fmt("fas fa-greater-than-equal")]
    GreaterThanEqual,
    #[fmt("fas fa-grimace")]
    GrimaceSolid,
    #[fmt("far fa-grimace")]
    GrimaceRegular,
    #[fmt("fas fa-grin")]
    GrinSolid,
    #[fmt("far fa-grin")]
    GrinRegular,
    #[fmt("fas fa-grin-alt")]
    GrinAltSolid,
    #[fmt("far fa-grin-alt")]
    GrinAltRegular,
    #[fmt("fas fa-grin-beam")]
    GrinBeamSolid,
    #[fmt("far fa-grin-beam")]
    GrinBeamRegular,
    #[fmt("fas fa-grin-beam-sweat")]
    GrinBeamSweatSolid,
    #[fmt("far fa-grin-beam-sweat")]
    GrinBeamSweatRegular,
    #[fmt("fas fa-grin-hearts")]
    GrinHeartsSolid,
    #[fmt("far fa-grin-hearts")]
    GrinHeartsRegular,
    #[fmt("fas fa-grin-squint")]
    GrinSquintSolid,
    #[fmt("far fa-grin-squint")]
    GrinSquintRegular,
    #[fmt("fas fa-grin-squint-tears")]
    GrinSquintTearsSolid,
    #[fmt("far fa-grin-squint-tears")]
    GrinSquintTearsRegular,
    #[fmt("fas fa-grin-stars")]
    GrinStarsSolid,
    #[fmt("far fa-grin-stars")]
    GrinStarsRegular,
    #[fmt("fas fa-grin-tears")]
    GrinTearsSolid,
    #[fmt("far fa-grin-tears")]
    GrinTearsRegular,
    #[fmt("fas fa-grin-tongue")]
    GrinTongueSolid,
    #[fmt("far fa-grin-tongue")]
    GrinTongueRegular,
    #[fmt("fas fa-grin-tongue-squint")]
    GrinTongueSquintSolid,
    #[fmt("far fa-grin-tongue-squint")]
    GrinTongueSquintRegular,
    #[fmt("fas fa-grin-tongue-wink")]
    GrinTongueWinkSolid,
    #[fmt("far fa-grin-tongue-wink")]
    GrinTongueWinkRegular,
    #[fmt("fas fa-grin-wink")]
    GrinWinkSolid,
    #[fmt("far fa-grin-wink")]
    GrinWinkRegular,
    #[fmt("fas fa-grip-horizontal")]
    GripHorizontal,
    #[fmt("fas fa-grip-lines")]
    GripLines,
    #[fmt("fas fa-grip-lines-vertical")]
    GripLinesVertical,
    #[fmt("fas fa-grip-vertical")]
    GripVertical,
    #[fmt("fab fa-gripfire")]
    Gripfire,
    #[fmt("fab fa-grunt")]
    Grunt,
    #[fmt("fab fa-guilded")]
    Guilded,
    #[fmt("fas fa-guitar")]
    Guitar,
    #[fmt("fab fa-gulp")]
    Gulp,
    #[fmt("fas fa-h-square")]
    HSquare,
    #[fmt("fab fa-hacker-news")]
    HackerNews,
    #[fmt("fab fa-hacker-news-square")]
    HackerNewsSquare,
    #[fmt("fab fa-hackerrank")]
    Hackerrank,
    #[fmt("fas fa-hamburger")]
    Hamburger,
    #[fmt("fas fa-hammer")]
    Hammer,
    #[fmt("fas fa-hamsa")]
    Hamsa,
    #[fmt("fas fa-hand-holding")]
    HandHolding,
    #[fmt("fas fa-hand-holding-heart")]
    HandHoldingHeart,
    #[fmt("fas fa-hand-holding-medical")]
    HandHoldingMedical,
    #[fmt("fas fa-hand-holding-usd")]
    HandHoldingUsd,
    #[fmt("fas fa-hand-holding-water")]
    HandHoldingWater,
    #[fmt("fas fa-hand-lizard")]
    HandLizardSolid,
    #[fmt("far fa-hand-lizard")]
    HandLizardRegular,
    #[fmt("fas fa-hand-middle-finger")]
    HandMiddleFinger,
    #[fmt("fas fa-hand-paper")]
    HandPaperSolid,
    #[fmt("far fa-hand-paper")]
    HandPaperRegular,
    #[fmt("fas fa-hand-peace")]
    HandPeaceSolid,
    #[fmt("far fa-hand-peace")]
    HandPeaceRegular,
    #[fmt("fas fa-hand-point-down")]
    HandPointDownSolid,
    #[fmt("far fa-hand-point-down")]
    HandPointDownRegular,
    #[fmt("fas fa-hand-point-left")]
    HandPointLeftSolid,
    #[fmt("far fa-hand-point-left")]
    HandPointLeftRegular,
    #[fmt("fas fa-hand-point-right")]
    HandPointRightSolid,
    #[fmt("far fa-hand-point-right")]
    HandPointRightRegular,
    #[fmt("fas fa-hand-point-up")]
    HandPointUpSolid,
    #[fmt("far fa-hand-point-up")]
    HandPointUpRegular,
    #[fmt("fas fa-hand-pointer")]
    HandPointerSolid,
    #[fmt("far fa-hand-pointer")]
    HandPointerRegular,
    #[fmt("fas fa-hand-rock")]
    HandRockSolid,
    #[fmt("far fa-hand-rock")]
    HandRockRegular,
    #[fmt("fas fa-hand-scissors")]
    HandScissorsSolid,
    #[fmt("far fa-hand-scissors")]
    HandScissorsRegular,
    #[fmt("fas fa-hand-sparkles")]
    HandSparkles,
    #[fmt("fas fa-hand-spock")]
    HandSpockSolid,
    #[fmt("far fa-hand-spock")]
    HandSpockRegular,
    #[fmt("fas fa-hands")]
    Hands,
    #[fmt("fas fa-hands-helping")]
    HandsHelping,
    #[fmt("fas fa-hands-wash")]
    HandsWash,
    #[fmt("fas fa-handshake")]
    HandshakeSolid,
    #[fmt("far fa-handshake")]
    HandshakeRegular,
    #[fmt("fas fa-handshake-alt-slash")]
    HandshakeAltSlash,
    #[fmt("fas fa-handshake-slash")]
    HandshakeSlash,
    #[fmt("fas fa-hanukiah")]
    Hanukiah,
    #[fmt("fas fa-hard-hat")]
    HardHat,
    #[fmt("fas fa-hashtag")]
    Hashtag,
    #[fmt("fas fa-hat-cowboy")]
    HatCowboy,
    #[fmt("fas fa-hat-cowboy-side")]
    HatCowboySide,
    #[fmt("fas fa-hat-wizard")]
    HatWizard,
    #[fmt("fas fa-hdd")]
    HddSolid,
    #[fmt("far fa-hdd")]
    HddRegular,
    #[fmt("fas fa-head-side-cough")]
    HeadSideCough,
    #[fmt("fas fa-head-side-cough-slash")]
    HeadSideCoughSlash,
    #[fmt("fas fa-head-side-mask")]
    HeadSideMask,
    #[fmt("fas fa-head-side-virus")]
    HeadSideVirus,
    #[fmt("fas fa-heading")]
    Heading,
    #[fmt("fas fa-headphones")]
    Headphones,
    #[fmt("fas fa-headphones-alt")]
    HeadphonesAlt,
    #[fmt("fas fa-headset")]
    Headset,
    #[fmt("fas fa-heart")]
    HeartSolid,
    #[fmt("far fa-heart")]
    HeartRegular,
    #[fmt("fas fa-heart-broken")]
    HeartBroken,
    #[fmt("fas fa-heartbeat")]
    Heartbeat,
    #[fmt("fas fa-helicopter")]
    Helicopter,
    #[fmt("fas fa-highlighter")]
    Highlighter,
    #[fmt("fas fa-hiking")]
    Hiking,
    #[fmt("fas fa-hippo")]
    Hippo,
    #[fmt("fab fa-hips")]
    Hips,
    #[fmt("fab fa-hire-a-helper")]
    HireAHelper,
    #[fmt("fas fa-history")]
    History,
    #[fmt("fab fa-hive")]
    Hive,
    #[fmt("fas fa-hockey-puck")]
    HockeyPuck,
    #[fmt("fas fa-holly-berry")]
    HollyBerry,
    #[fmt("fas fa-home")]
    Home,
    #[fmt("fab fa-hooli")]
    Hooli,
    #[fmt("fab fa-hornbill")]
    Hornbill,
    #[fmt("fas fa-horse")]
    Horse,
    #[fmt("fas fa-horse-head")]
    HorseHead,
    #[fmt("fas fa-hospital")]
    HospitalSolid,
    #[fmt("far fa-hospital")]
    HospitalRegular,
    #[fmt("fas fa-hospital-alt")]
    HospitalAlt,
    #[fmt("fas fa-hospital-symbol")]
    HospitalSymbol,
    #[fmt("fas fa-hospital-user")]
    HospitalUser,
    #[fmt("fas fa-hot-tub")]
    HotTub,
    #[fmt("fas fa-hotdog")]
    Hotdog,
    #[fmt("fas fa-hotel")]
    Hotel,
    #[fmt("fab fa-hotjar")]
    Hotjar,
    #[fmt("fas fa-hourglass")]
    HourglassSolid,
    #[fmt("far fa-hourglass")]
    HourglassRegular,
    #[fmt("fas fa-hourglass-end")]
    HourglassEnd,
    #[fmt("fas fa-hourglass-half")]
    HourglassHalf,
    #[fmt("fas fa-hourglass-start")]
    HourglassStart,
    #[fmt("fas fa-house-damage")]
    HouseDamage,
    #[fmt("fas fa-house-user")]
    HouseUser,
    #[fmt("fab fa-houzz")]
    Houzz,
    #[fmt("fas fa-hryvnia")]
    Hryvnia,
    #[fmt("fab fa-html5")]
    Html5,
    #[fmt("fab fa-hubspot")]
    Hubspot,
    #[fmt("fas fa-i-cursor")]
    ICursor,
    #[fmt("fas fa-ice-cream")]
    IceCream,
    #[fmt("fas fa-icicles")]
    Icicles,
    #[fmt("fas fa-icons")]
    Icons,
    #[fmt("fas fa-id-badge")]
    IdBadgeSolid,
    #[fmt("far fa-id-badge")]
    IdBadgeRegular,
    #[fmt("fas fa-id-card")]
    IdCardSolid,
    #[fmt("far fa-id-card")]
    IdCardRegular,
    #[fmt("fas fa-id-card-alt")]
    IdCardAlt,
    #[fmt("fab fa-ideal")]
    Ideal,
    #[fmt("fas fa-igloo")]
    Igloo,
    #[fmt("fas fa-image")]
    ImageSolid,
    #[fmt("far fa-image")]
    ImageRegular,
    #[fmt("fas fa-images")]
    ImagesSolid,
    #[fmt("far fa-images")]
    ImagesRegular,
    #[fmt("fab fa-imdb")]
    Imdb,
    #[fmt("fas fa-inbox")]
    Inbox,
    #[fmt("fas fa-indent")]
    Indent,
    #[fmt("fas fa-industry")]
    Industry,
    #[fmt("fas fa-infinity")]
    Infinity,
    #[fmt("fas fa-info")]
    Info,
    #[fmt("fas fa-info-circle")]
    InfoCircle,
    #[fmt("fab fa-innosoft")]
    Innosoft,
    #[fmt("fab fa-instagram")]
    Instagram,
    #[fmt("fab fa-instagram-square")]
    InstagramSquare,
    #[fmt("fab fa-instalod")]
    Instalod,
    #[fmt("fab fa-intercom")]
    Intercom,
    #[fmt("fab fa-internet-explorer")]
    InternetExplorer,
    #[fmt("fab fa-invision")]
    Invision,
    #[fmt("fab fa-ioxhost")]
    Ioxhost,
    #[fmt("fas fa-italic")]
    Italic,
    #[fmt("fab fa-itch-io")]
    ItchIo,
    #[fmt("fab fa-itunes")]
    Itunes,
    #[fmt("fab fa-itunes-note")]
    ItunesNote,
    #[fmt("fab fa-java")]
    Java,
    #[fmt("fas fa-jedi")]
    Jedi,
    #[fmt("fab fa-jedi-order")]
    JediOrder,
    #[fmt("fab fa-jenkins")]
    Jenkins,
    #[fmt("fab fa-jira")]
    Jira,
    #[fmt("fab fa-joget")]
    Joget,
    #[fmt("fas fa-joint")]
    Joint,
    #[fmt("fab fa-joomla")]
    Joomla,
    #[fmt("fas fa-journal-whills")]
    JournalWhills,
    #[fmt("fab fa-js")]
    Js,
    #[fmt("fab fa-js-square")]
    JsSquare,
    #[fmt("fab fa-jsfiddle")]
    Jsfiddle,
    #[fmt("fas fa-kaaba")]
    Kaaba,
    #[fmt("fab fa-kaggle")]
    Kaggle,
    #[fmt("fas fa-key")]
    Key,
    #[fmt("fab fa-keybase")]
    Keybase,
    #[fmt("fas fa-keyboard")]
    KeyboardSolid,
    #[fmt("far fa-keyboard")]
    KeyboardRegular,
    #[fmt("fab fa-keycdn")]
    Keycdn,
    #[fmt("fas fa-khanda")]
    Khanda,
    #[fmt("fab fa-kickstarter")]
    Kickstarter,
    #[fmt("fab fa-kickstarter-k")]
    KickstarterK,
    #[fmt("fas fa-kiss")]
    KissSolid,
    #[fmt("far fa-kiss")]
    KissRegular,
    #[fmt("fas fa-kiss-beam")]
    KissBeamSolid,
    #[fmt("far fa-kiss-beam")]
    KissBeamRegular,
    #[fmt("fas fa-kiss-wink-heart")]
    KissWinkHeartSolid,
    #[fmt("far fa-kiss-wink-heart")]
    KissWinkHeartRegular,
    #[fmt("fas fa-kiwi-bird")]
    KiwiBird,
    #[fmt("fab fa-korvue")]
    Korvue,
    #[fmt("fas fa-landmark")]
    Landmark,
    #[fmt("fas fa-language")]
    Language,
    #[fmt("fas fa-laptop")]
    Laptop,
    #[fmt("fas fa-laptop-code")]
    LaptopCode,
    #[fmt("fas fa-laptop-house")]
    LaptopHouse,
    #[fmt("fas fa-laptop-medical")]
    LaptopMedical,
    #[fmt("fab fa-laravel")]
    Laravel,
    #[fmt("fab fa-lastfm")]
    Lastfm,
    #[fmt("fab fa-lastfm-square")]
    LastfmSquare,
    #[fmt("fas fa-laugh")]
    LaughSolid,
    #[fmt("far fa-laugh")]
    LaughRegular,
    #[fmt("fas fa-laugh-beam")]
    LaughBeamSolid,
    #[fmt("far fa-laugh-beam")]
    LaughBeamRegular,
    #[fmt("fas fa-laugh-squint")]
    LaughSquintSolid,
    #[fmt("far fa-laugh-squint")]
    LaughSquintRegular,
    #[fmt("fas fa-laugh-wink")]
    LaughWinkSolid,
    #[fmt("far fa-laugh-wink")]
    LaughWinkRegular,
    #[fmt("fas fa-layer-group")]
    LayerGroup,
    #[fmt("fas fa-leaf")]
    Leaf,
    #[fmt("fab fa-leanpub")]
    Leanpub,
    #[fmt("fas fa-lemon")]
    LemonSolid,
    #[fmt("far fa-lemon")]
    LemonRegular,
    #[fmt("fab fa-less")]
    Less,
    #[fmt("fas fa-less-than")]
    LessThan,
    #[fmt("fas fa-less-than-equal")]
    LessThanEqual,
    #[fmt("fas fa-level-down-alt")]
    LevelDownAlt,
    #[fmt("fas fa-level-up-alt")]
    LevelUpAlt,
    #[fmt("fas fa-life-ring")]
    LifeRingSolid,
    #[fmt("far fa-life-ring")]
    LifeRingRegular,
    #[fmt("fas fa-lightbulb")]
    LightbulbSolid,
    #[fmt("far fa-lightbulb")]
    LightbulbRegular,
    #[fmt("fab fa-line")]
    Line,
    #[fmt("fas fa-link")]
    Link,
    #[fmt("fab fa-linkedin")]
    Linkedin,
    #[fmt("fab fa-linkedin-in")]
    LinkedinIn,
    #[fmt("fab fa-linode")]
    Linode,
    #[fmt("fab fa-linux")]
    Linux,
    #[fmt("fas fa-lira-sign")]
    LiraSign,
    #[fmt("fas fa-list")]
    List,
    #[fmt("fas fa-list-alt")]
    ListAltSolid,
    #[fmt("far fa-list-alt")]
    ListAltRegular,
    #[fmt("fas fa-list-ol")]
    ListOl,
    #[fmt("fas fa-list-ul")]
    ListUl,
    #[fmt("fas fa-location-arrow")]
    LocationArrow,
    #[fmt("fas fa-lock")]
    Lock,
    #[fmt("fas fa-lock-open")]
    LockOpen,
    #[fmt("fas fa-long-arrow-alt-down")]
    LongArrowAltDown,
    #[fmt("fas fa-long-arrow-alt-left")]
    LongArrowAltLeft,
    #[fmt("fas fa-long-arrow-alt-right")]
    LongArrowAltRight,
    #[fmt("fas fa-long-arrow-alt-up")]
    LongArrowAltUp,
    #[fmt("fas fa-low-vision")]
    LowVision,
    #[fmt("fas fa-luggage-cart")]
    LuggageCart,
    #[fmt("fas fa-lungs")]
    Lungs,
    #[fmt("fas fa-lungs-virus")]
    LungsVirus,
    #[fmt("fab fa-lyft")]
    Lyft,
    #[fmt("fab fa-magento")]
    Magento,
    #[fmt("fas fa-magic")]
    Magic,
    #[fmt("fas fa-magnet")]
    Magnet,
    #[fmt("fas fa-mail-bulk")]
    MailBulk,
    #[fmt("fab fa-mailchimp")]
    Mailchimp,
    #[fmt("fas fa-male")]
    Male,
    #[fmt("fab fa-mandalorian")]
    Mandalorian,
    #[fmt("fas fa-map")]
    MapSolid,
    #[fmt("far fa-map")]
    MapRegular,
    #[fmt("fas fa-map-marked")]
    MapMarked,
    #[fmt("fas fa-map-marked-alt")]
    MapMarkedAlt,
    #[fmt("fas fa-map-marker")]
    MapMarker,
    #[fmt("fas fa-map-marker-alt")]
    MapMarkerAlt,
    #[fmt("fas fa-map-pin")]
    MapPin,
    #[fmt("fas fa-map-signs")]
    MapSigns,
    #[fmt("fab fa-markdown")]
    Markdown,
    #[fmt("fas fa-marker")]
    Marker,
    #[fmt("fas fa-mars")]
    Mars,
    #[fmt("fas fa-mars-double")]
    MarsDouble,
    #[fmt("fas fa-mars-stroke")]
    MarsStroke,
    #[fmt("fas fa-mars-stroke-h")]
    MarsStrokeH,
    #[fmt("fas fa-mars-stroke-v")]
    MarsStrokeV,
    #[fmt("fas fa-mask")]
    Mask,
    #[fmt("fab fa-mastodon")]
    Mastodon,
    #[fmt("fab fa-maxcdn")]
    Maxcdn,
    #[fmt("fab fa-mdb")]
    Mdb,
    #[fmt("fas fa-medal")]
    Medal,
    #[fmt("fab fa-medapps")]
    Medapps,
    #[fmt("fab fa-medium")]
    Medium,
    #[fmt("fab fa-medium-m")]
    MediumM,
    #[fmt("fas fa-medkit")]
    Medkit,
    #[fmt("fab fa-medrt")]
    Medrt,
    #[fmt("fab fa-meetup")]
    Meetup,
    #[fmt("fab fa-megaport")]
    Megaport,
    #[fmt("fas fa-meh")]
    MehSolid,
    #[fmt("far fa-meh")]
    MehRegular,
    #[fmt("fas fa-meh-blank")]
    MehBlankSolid,
    #[fmt("far fa-meh-blank")]
    MehBlankRegular,
    #[fmt("fas fa-meh-rolling-eyes")]
    MehRollingEyesSolid,
    #[fmt("far fa-meh-rolling-eyes")]
    MehRollingEyesRegular,
    #[fmt("fas fa-memory")]
    Memory,
    #[fmt("fab fa-mendeley")]
    Mendeley,
    #[fmt("fas fa-menorah")]
    Menorah,
    #[fmt("fas fa-mercury")]
    Mercury,
    #[fmt("fas fa-meteor")]
    Meteor,
    #[fmt("fab fa-microblog")]
    Microblog,
    #[fmt("fas fa-microchip")]
    Microchip,
    #[fmt("fas fa-microphone")]
    Microphone,
    #[fmt("fas fa-microphone-alt")]
    MicrophoneAlt,
    #[fmt("fas fa-microphone-alt-slash")]
    MicrophoneAltSlash,
    #[fmt("fas fa-microphone-slash")]
    MicrophoneSlash,
    #[fmt("fas fa-microscope")]
    Microscope,
    #[fmt("fab fa-microsoft")]
    Microsoft,
    #[fmt("fas fa-minus")]
    Minus,
    #[fmt("fas fa-minus-circle")]
    MinusCircle,
    #[fmt("fas fa-minus-square")]
    MinusSquareSolid,
    #[fmt("far fa-minus-square")]
    MinusSquareRegular,
    #[fmt("fas fa-mitten")]
    Mitten,
    #[fmt("fab fa-mix")]
    Mix,
    #[fmt("fab fa-mixcloud")]
    Mixcloud,
    #[fmt("fab fa-mixer")]
    Mixer,
    #[fmt("fab fa-mizuni")]
    Mizuni,
    #[fmt("fas fa-mobile")]
    Mobile,
    #[fmt("fas fa-mobile-alt")]
    MobileAlt,
    #[fmt("fab fa-modx")]
    Modx,
    #[fmt("fab fa-monero")]
    Monero,
    #[fmt("fas fa-money-bill")]
    MoneyBill,
    #[fmt("fas fa-money-bill-alt")]
    MoneyBillAltSolid,
    #[fmt("far fa-money-bill-alt")]
    MoneyBillAltRegular,
    #[fmt("fas fa-money-bill-wave")]
    MoneyBillWave,
    #[fmt("fas fa-money-bill-wave-alt")]
    MoneyBillWaveAlt,
    #[fmt("fas fa-money-check")]
    MoneyCheck,
    #[fmt("fas fa-money-check-alt")]
    MoneyCheckAlt,
    #[fmt("fas fa-monument")]
    Monument,
    #[fmt("fas fa-moon")]
    MoonSolid,
    #[fmt("far fa-moon")]
    MoonRegular,
    #[fmt("fas fa-mortar-pestle")]
    MortarPestle,
    #[fmt("fas fa-mosque")]
    Mosque,
    #[fmt("fas fa-motorcycle")]
    Motorcycle,
    #[fmt("fas fa-mountain")]
    Mountain,
    #[fmt("fas fa-mouse")]
    Mouse,
    #[fmt("fas fa-mouse-pointer")]
    MousePointer,
    #[fmt("fas fa-mug-hot")]
    MugHot,
    #[fmt("fas fa-music")]
    Music,
    #[fmt("fab fa-napster")]
    Napster,
    #[fmt("fab fa-neos")]
    Neos,
    #[fmt("fas fa-network-wired")]
    NetworkWired,
    #[fmt("fas fa-neuter")]
    Neuter,
    #[fmt("fas fa-newspaper")]
    NewspaperSolid,
    #[fmt("far fa-newspaper")]
    NewspaperRegular,
    #[fmt("fab fa-nimblr")]
    Nimblr,
    #[fmt("fab fa-node")]
    Node,
    #[fmt("fab fa-node-js")]
    NodeJs,
    #[fmt("fas fa-not-equal")]
    NotEqual,
    #[fmt("fas fa-notes-medical")]
    NotesMedical,
    #[fmt("fab fa-npm")]
    Npm,
    #[fmt("fab fa-ns8")]
    Ns8,
    #[fmt("fab fa-nutritionix")]
    Nutritionix,
    #[fmt("fas fa-object-group")]
    ObjectGroupSolid,
    #[fmt("far fa-object-group")]
    ObjectGroupRegular,
    #[fmt("fas fa-object-ungroup")]
    ObjectUngroupSolid,
    #[fmt("far fa-object-ungroup")]
    ObjectUngroupRegular,
    #[fmt("fab fa-octopus-deploy")]
    OctopusDeploy,
    #[fmt("fab fa-odnoklassniki")]
    Odnoklassniki,
    #[fmt("fab fa-odnoklassniki-square")]
    OdnoklassnikiSquare,
    #[fmt("fas fa-oil-can")]
    OilCan,
    #[fmt("fab fa-old-republic")]
    OldRepublic,
    #[fmt("fas fa-om")]
    Om,
    #[fmt("fab fa-opencart")]
    Opencart,
    #[fmt("fab fa-openid")]
    Openid,
    #[fmt("fab fa-opera")]
    Opera,
    #[fmt("fab fa-optin-monster")]
    OptinMonster,
    #[fmt("fab fa-orcid")]
    Orcid,
    #[fmt("fab fa-osi")]
    Osi,
    #[fmt("fas fa-otter")]
    Otter,
    #[fmt("fas fa-outdent")]
    Outdent,
    #[fmt("fab fa-page4")]
    Page4,
    #[fmt("fab fa-pagelines")]
    Pagelines,
    #[fmt("fas fa-pager")]
    Pager,
    #[fmt("fas fa-paint-brush")]
    PaintBrush,
    #[fmt("fas fa-paint-roller")]
    PaintRoller,
    #[fmt("fas fa-palette")]
    Palette,
    #[fmt("fab fa-palfed")]
    Palfed,
    #[fmt("fas fa-pallet")]
    Pallet,
    #[fmt("fas fa-paper-plane")]
    PaperPlaneSolid,
    #[fmt("far fa-paper-plane")]
    PaperPlaneRegular,
    #[fmt("fas fa-paperclip")]
    Paperclip,
    #[fmt("fas fa-parachute-box")]
    ParachuteBox,
    #[fmt("fas fa-paragraph")]
    Paragraph,
    #[fmt("fas fa-parking")]
    Parking,
    #[fmt("fas fa-passport")]
    Passport,
    #[fmt("fas fa-pastafarianism")]
    Pastafarianism,
    #[fmt("fas fa-paste")]
    Paste,
    #[fmt("fab fa-patreon")]
    Patreon,
    #[fmt("fas fa-pause")]
    Pause,
    #[fmt("fas fa-pause-circle")]
    PauseCircleSolid,
    #[fmt("far fa-pause-circle")]
    PauseCircleRegular,
    #[fmt("fas fa-paw")]
    Paw,
    #[fmt("fab fa-paypal")]
    Paypal,
    #[fmt("fas fa-peace")]
    Peace,
    #[fmt("fas fa-pen")]
    Pen,
    #[fmt("fas fa-pen-alt")]
    PenAlt,
    #[fmt("fas fa-pen-fancy")]
    PenFancy,
    #[fmt("fas fa-pen-nib")]
    PenNib,
    #[fmt("fas fa-pen-square")]
    PenSquare,
    #[fmt("fas fa-pencil-alt")]
    PencilAlt,
    #[fmt("fas fa-pencil-ruler")]
    PencilRuler,
    #[fmt("fab fa-penny-arcade")]
    PennyArcade,
    #[fmt("fas fa-people-arrows")]
    PeopleArrows,
    #[fmt("fas fa-people-carry")]
    PeopleCarry,
    #[fmt("fas fa-pepper-hot")]
    PepperHot,
    #[fmt("fab fa-perbyte")]
    Perbyte,
    #[fmt("fas fa-percent")]
    Percent,
    #[fmt("fas fa-percentage")]
    Percentage,
    #[fmt("fab fa-periscope")]
    Periscope,
    #[fmt("fas fa-person-booth")]
    PersonBooth,
    #[fmt("fab fa-phabricator")]
    Phabricator,
    #[fmt("fab fa-phoenix-framework")]
    PhoenixFramework,
    #[fmt("fab fa-phoenix-squadron")]
    PhoenixSquadron,
    #[fmt("fas fa-phone")]
    Phone,
    #[fmt("fas fa-phone-alt")]
    PhoneAlt,
    #[fmt("fas fa-phone-slash")]
    PhoneSlash,
    #[fmt("fas fa-phone-square")]
    PhoneSquare,
    #[fmt("fas fa-phone-square-alt")]
    PhoneSquareAlt,
    #[fmt("fas fa-phone-volume")]
    PhoneVolume,
    #[fmt("fas fa-photo-video")]
    PhotoVideo,
    #[fmt("fab fa-php")]
    Php,
    #[fmt("fab fa-pied-piper")]
    PiedPiper,
    #[fmt("fab fa-pied-piper-alt")]
    PiedPiperAlt,
    #[fmt("fab fa-pied-piper-hat")]
    PiedPiperHat,
    #[fmt("fab fa-pied-piper-pp")]
    PiedPiperPp,
    #[fmt("fab fa-pied-piper-square")]
    PiedPiperSquare,
    #[fmt("fas fa-piggy-bank")]
    PiggyBank,
    #[fmt("fas fa-pills")]
    Pills,
    #[fmt("fab fa-pinterest")]
    Pinterest,
    #[fmt("fab fa-pinterest-p")]
    PinterestP,
    #[fmt("fab fa-pinterest-square")]
    PinterestSquare,
    #[fmt("fas fa-pizza-slice")]
    PizzaSlice,
    #[fmt("fas fa-place-of-worship")]
    PlaceOfWorship,
    #[fmt("fas fa-plane")]
    Plane,
    #[fmt("fas fa-plane-arrival")]
    PlaneArrival,
    #[fmt("fas fa-plane-departure")]
    PlaneDeparture,
    #[fmt("fas fa-plane-slash")]
    PlaneSlash,
    #[fmt("fas fa-play")]
    Play,
    #[fmt("fas fa-play-circle")]
    PlayCircleSolid,
    #[fmt("far fa-play-circle")]
    PlayCircleRegular,
    #[fmt("fab fa-playstation")]
    Playstation,
    #[fmt("fas fa-plug")]
    Plug,
    #[fmt("fas fa-plus")]
    Plus,
    #[fmt("fas fa-plus-circle")]
    PlusCircle,
    #[fmt("fas fa-plus-square")]
    PlusSquareSolid,
    #[fmt("far fa-plus-square")]
    PlusSquareRegular,
    #[fmt("fas fa-podcast")]
    Podcast,
    #[fmt("fas fa-poll")]
    Poll,
    #[fmt("fas fa-poll-h")]
    PollH,
    #[fmt("fas fa-poo")]
    Poo,
    #[fmt("fas fa-poo-storm")]
    PooStorm,
    #[fmt("fas fa-poop")]
    Poop,
    #[fmt("fas fa-portrait")]
    Portrait,
    #[fmt("fas fa-pound-sign")]
    PoundSign,
    #[fmt("fas fa-power-off")]
    PowerOff,
    #[fmt("fas fa-pray")]
    Pray,
    #[fmt("fas fa-praying-hands")]
    PrayingHands,
    #[fmt("fas fa-prescription")]
    Prescription,
    #[fmt("fas fa-prescription-bottle")]
    PrescriptionBottle,
    #[fmt("fas fa-prescription-bottle-alt")]
    PrescriptionBottleAlt,
    #[fmt("fas fa-print")]
    Print,
    #[fmt("fas fa-procedures")]
    Procedures,
    #[fmt("fab fa-product-hunt")]
    ProductHunt,
    #[fmt("fas fa-project-diagram")]
    ProjectDiagram,
    #[fmt("fas fa-pump-medical")]
    PumpMedical,
    #[fmt("fas fa-pump-soap")]
    PumpSoap,
    #[fmt("fab fa-pushed")]
    Pushed,
    #[fmt("fas fa-puzzle-piece")]
    PuzzlePiece,
    #[fmt("fab fa-python")]
    Python,
    #[fmt("fab fa-qq")]
    Qq,
    #[fmt("fas fa-qrcode")]
    Qrcode,
    #[fmt("fas fa-question")]
    Question,
    #[fmt("fas fa-question-circle")]
    QuestionCircleSolid,
    #[fmt("far fa-question-circle")]
    QuestionCircleRegular,
    #[fmt("fas fa-quidditch")]
    Quidditch,
    #[fmt("fab fa-quinscape")]
    Quinscape,
    #[fmt("fab fa-quora")]
    Quora,
    #[fmt("fas fa-quote-left")]
    QuoteLeft,
    #[fmt("fas fa-quote-right")]
    QuoteRight,
    #[fmt("fas fa-quran")]
    Quran,
    #[fmt("fab fa-r-project")]
    RProject,
    #[fmt("fas fa-radiation")]
    Radiation,
    #[fmt("fas fa-radiation-alt")]
    RadiationAlt,
    #[fmt("fas fa-rainbow")]
    Rainbow,
    #[fmt("fas fa-random")]
    Random,
    #[fmt("fab fa-raspberry-pi")]
    RaspberryPi,
    #[fmt("fab fa-ravelry")]
    Ravelry,
    #[fmt("fab fa-react")]
    React,
    #[fmt("fab fa-reacteurope")]
    Reacteurope,
    #[fmt("fab fa-readme")]
    Readme,
    #[fmt("fab fa-rebel")]
    Rebel,
    #[fmt("fas fa-receipt")]
    Receipt,
    #[fmt("fas fa-record-vinyl")]
    RecordVinyl,
    #[fmt("fas fa-recycle")]
    Recycle,
    #[fmt("fab fa-red-river")]
    RedRiver,
    #[fmt("fab fa-reddit")]
    Reddit,
    #[fmt("fab fa-reddit-alien")]
    RedditAlien,
    #[fmt("fab fa-reddit-square")]
    RedditSquare,
    #[fmt("fab fa-redhat")]
    Redhat,
    #[fmt("fas fa-redo")]
    Redo,
    #[fmt("fas fa-redo-alt")]
    RedoAlt,
    #[fmt("fas fa-registered")]
    RegisteredSolid,
    #[fmt("far fa-registered")]
    RegisteredRegular,
    #[fmt("fas fa-remove-format")]
    RemoveFormat,
    #[fmt("fab fa-renren")]
    Renren,
    #[fmt("fas fa-reply")]
    Reply,
    #[fmt("fas fa-reply-all")]
    ReplyAll,
    #[fmt("fab fa-replyd")]
    Replyd,
    #[fmt("fas fa-republican")]
    Republican,
    #[fmt("fab fa-researchgate")]
    Researchgate,
    #[fmt("fab fa-resolving")]
    Resolving,
    #[fmt("fas fa-restroom")]
    Restroom,
    #[fmt("fas fa-retweet")]
    Retweet,
    #[fmt("fab fa-rev")]
    Rev,
    #[fmt("fas fa-ribbon")]
    Ribbon,
    #[fmt("fas fa-ring")]
    Ring,
    #[fmt("fas fa-road")]
    Road,
    #[fmt("fas fa-robot")]
    Robot,
    #[fmt("fas fa-rocket")]
    Rocket,
    #[fmt("fab fa-rocketchat")]
    Rocketchat,
    #[fmt("fab fa-rockrms")]
    Rockrms,
    #[fmt("fas fa-route")]
    Route,
    #[fmt("fas fa-rss")]
    Rss,
    #[fmt("fas fa-rss-square")]
    RssSquare,
    #[fmt("fas fa-ruble-sign")]
    RubleSign,
    #[fmt("fas fa-ruler")]
    Ruler,
    #[fmt("fas fa-ruler-combined")]
    RulerCombined,
    #[fmt("fas fa-ruler-horizontal")]
    RulerHorizontal,
    #[fmt("fas fa-ruler-vertical")]
    RulerVertical,
    #[fmt("fas fa-running")]
    Running,
    #[fmt("fas fa-rupee-sign")]
    RupeeSign,
    #[fmt("fab fa-rust")]
    Rust,
    #[fmt("fas fa-sad-cry")]
    SadCrySolid,
    #[fmt("far fa-sad-cry")]
    SadCryRegular,
    #[fmt("fas fa-sad-tear")]
    SadTearSolid,
    #[fmt("far fa-sad-tear")]
    SadTearRegular,
    #[fmt("fab fa-safari")]
    Safari,
    #[fmt("fab fa-salesforce")]
    Salesforce,
    #[fmt("fab fa-sass")]
    Sass,
    #[fmt("fas fa-satellite")]
    Satellite,
    #[fmt("fas fa-satellite-dish")]
    SatelliteDish,
    #[fmt("fas fa-save")]
    SaveSolid,
    #[fmt("far fa-save")]
    SaveRegular,
    #[fmt("fab fa-schlix")]
    Schlix,
    #[fmt("fas fa-school")]
    School,
    #[fmt("fas fa-screwdriver")]
    Screwdriver,
    #[fmt("fab fa-scribd")]
    Scribd,
    #[fmt("fas fa-scroll")]
    Scroll,
    #[fmt("fas fa-sd-card")]
    SdCard,
    #[fmt("fas fa-search")]
    Search,
    #[fmt("fas fa-search-dollar")]
    SearchDollar,
    #[fmt("fas fa-search-location")]
    SearchLocation,
    #[fmt("fas fa-search-minus")]
    SearchMinus,
    #[fmt("fas fa-search-plus")]
    SearchPlus,
    #[fmt("fab fa-searchengin")]
    Searchengin,
    #[fmt("fas fa-seedling")]
    Seedling,
    #[fmt("fab fa-sellcast")]
    Sellcast,
    #[fmt("fab fa-sellsy")]
    Sellsy,
    #[fmt("fas fa-server")]
    Server,
    #[fmt("fab fa-servicestack")]
    Servicestack,
    #[fmt("fas fa-shapes")]
    Shapes,
    #[fmt("fas fa-share")]
    Share,
    #[fmt("fas fa-share-alt")]
    ShareAlt,
    #[fmt("fas fa-share-alt-square")]
    ShareAltSquare,
    #[fmt("fas fa-share-square")]
    ShareSquareSolid,
    #[fmt("far fa-share-square")]
    ShareSquareRegular,
    #[fmt("fas fa-shekel-sign")]
    ShekelSign,
    #[fmt("fas fa-shield-alt")]
    ShieldAlt,
    #[fmt("fas fa-shield-virus")]
    ShieldVirus,
    #[fmt("fas fa-ship")]
    Ship,
    #[fmt("fas fa-shipping-fast")]
    ShippingFast,
    #[fmt("fab fa-shirtsinbulk")]
    Shirtsinbulk,
    #[fmt("fas fa-shoe-prints")]
    ShoePrints,
    #[fmt("fab fa-shopify")]
    Shopify,
    #[fmt("fas fa-shopping-bag")]
    ShoppingBag,
    #[fmt("fas fa-shopping-basket")]
    ShoppingBasket,
    #[fmt("fas fa-shopping-cart")]
    ShoppingCart,
    #[fmt("fab fa-shopware")]
    Shopware,
    #[fmt("fas fa-shower")]
    Shower,
    #[fmt("fas fa-shuttle-van")]
    ShuttleVan,
    #[fmt("fas fa-sign")]
    Sign,
    #[fmt("fas fa-sign-in-alt")]
    SignInAlt,
    #[fmt("fas fa-sign-language")]
    SignLanguage,
    #[fmt("fas fa-sign-out-alt")]
    SignOutAlt,
    #[fmt("fas fa-signal")]
    Signal,
    #[fmt("fas fa-signature")]
    Signature,
    #[fmt("fas fa-sim-card")]
    SimCard,
    #[fmt("fab fa-simplybuilt")]
    Simplybuilt,
    #[fmt("fas fa-sink")]
    Sink,
    #[fmt("fab fa-sistrix")]
    Sistrix,
    #[fmt("fas fa-sitemap")]
    Sitemap,
    #[fmt("fab fa-sith")]
    Sith,
    #[fmt("fas fa-skating")]
    Skating,
    #[fmt("fab fa-sketch")]
    Sketch,
    #[fmt("fas fa-skiing")]
    Skiing,
    #[fmt("fas fa-skiing-nordic")]
    SkiingNordic,
    #[fmt("fas fa-skull")]
    Skull,
    #[fmt("fas fa-skull-crossbones")]
    SkullCrossbones,
    #[fmt("fab fa-skyatlas")]
    Skyatlas,
    #[fmt("fab fa-skype")]
    Skype,
    #[fmt("fab fa-slack")]
    Slack,
    #[fmt("fab fa-slack-hash")]
    SlackHash,
    #[fmt("fas fa-slash")]
    Slash,
    #[fmt("fas fa-sleigh")]
    Sleigh,
    #[fmt("fas fa-sliders-h")]
    SlidersH,
    #[fmt("fab fa-slideshare")]
    Slideshare,
    #[fmt("fas fa-smile")]
    SmileSolid,
    #[fmt("far fa-smile")]
    SmileRegular,
    #[fmt("fas fa-smile-beam")]
    SmileBeamSolid,
    #[fmt("far fa-smile-beam")]
    SmileBeamRegular,
    #[fmt("fas fa-smile-wink")]
    SmileWinkSolid,
    #[fmt("far fa-smile-wink")]
    SmileWinkRegular,
    #[fmt("fas fa-smog")]
    Smog,
    #[fmt("fas fa-smoking")]
    Smoking,
    #[fmt("fas fa-smoking-ban")]
    SmokingBan,
    #[fmt("fas fa-sms")]
    Sms,
    #[fmt("fab fa-snapchat")]
    Snapchat,
    #[fmt("fab fa-snapchat-ghost")]
    SnapchatGhost,
    #[fmt("fab fa-snapchat-square")]
    SnapchatSquare,
    #[fmt("fas fa-snowboarding")]
    Snowboarding,
    #[fmt("fas fa-snowflake")]
    SnowflakeSolid,
    #[fmt("far fa-snowflake")]
    SnowflakeRegular,
    #[fmt("fas fa-snowman")]
    Snowman,
    #[fmt("fas fa-snowplow")]
    Snowplow,
    #[fmt("fas fa-soap")]
    Soap,
    #[fmt("fas fa-socks")]
    Socks,
    #[fmt("fas fa-solar-panel")]
    SolarPanel,
    #[fmt("fas fa-sort")]
    Sort,
    #[fmt("fas fa-sort-alpha-down")]
    SortAlphaDown,
    #[fmt("fas fa-sort-alpha-down-alt")]
    SortAlphaDownAlt,
    #[fmt("fas fa-sort-alpha-up")]
    SortAlphaUp,
    #[fmt("fas fa-sort-alpha-up-alt")]
    SortAlphaUpAlt,
    #[fmt("fas fa-sort-amount-down")]
    SortAmountDown,
    #[fmt("fas fa-sort-amount-down-alt")]
    SortAmountDownAlt,
    #[fmt("fas fa-sort-amount-up")]
    SortAmountUp,
    #[fmt("fas fa-sort-amount-up-alt")]
    SortAmountUpAlt,
    #[fmt("fas fa-sort-down")]
    SortDown,
    #[fmt("fas fa-sort-numeric-down")]
    SortNumericDown,
    #[fmt("fas fa-sort-numeric-down-alt")]
    SortNumericDownAlt,
    #[fmt("fas fa-sort-numeric-up")]
    SortNumericUp,
    #[fmt("fas fa-sort-numeric-up-alt")]
    SortNumericUpAlt,
    #[fmt("fas fa-sort-up")]
    SortUp,
    #[fmt("fab fa-soundcloud")]
    Soundcloud,
    #[fmt("fab fa-sourcetree")]
    Sourcetree,
    #[fmt("fas fa-spa")]
    Spa,
    #[fmt("fas fa-space-shuttle")]
    SpaceShuttle,
    #[fmt("fab fa-speakap")]
    Speakap,
    #[fmt("fab fa-speaker-deck")]
    SpeakerDeck,
    #[fmt("fas fa-spell-check")]
    SpellCheck,
    #[fmt("fas fa-spider")]
    Spider,
    #[fmt("fas fa-spinner")]
    Spinner,
    #[fmt("fas fa-splotch")]
    Splotch,
    #[fmt("fab fa-spotify")]
    Spotify,
    #[fmt("fas fa-spray-can")]
    SprayCan,
    #[fmt("fas fa-square")]
    SquareSolid,
    #[fmt("far fa-square")]
    SquareRegular,
    #[fmt("fas fa-square-full")]
    SquareFull,
    #[fmt("fas fa-square-root-alt")]
    SquareRootAlt,
    #[fmt("fab fa-squarespace")]
    Squarespace,
    #[fmt("fab fa-stack-exchange")]
    StackExchange,
    #[fmt("fab fa-stack-overflow")]
    StackOverflow,
    #[fmt("fab fa-stackpath")]
    Stackpath,
    #[fmt("fas fa-stamp")]
    Stamp,
    #[fmt("fas fa-star")]
    StarSolid,
    #[fmt("far fa-star")]
    StarRegular,
    #[fmt("fas fa-star-and-crescent")]
    StarAndCrescent,
    #[fmt("fas fa-star-half")]
    StarHalfSolid,
    #[fmt("far fa-star-half")]
    StarHalfRegular,
    #[fmt("fas fa-star-half-alt")]
    StarHalfAlt,
    #[fmt("fas fa-star-of-david")]
    StarOfDavid,
    #[fmt("fas fa-star-of-life")]
    StarOfLife,
    #[fmt("fab fa-staylinked")]
    Staylinked,
    #[fmt("fab fa-steam")]
    Steam,
    #[fmt("fab fa-steam-square")]
    SteamSquare,
    #[fmt("fab fa-steam-symbol")]
    SteamSymbol,
    #[fmt("fas fa-step-backward")]
    StepBackward,
    #[fmt("fas fa-step-forward")]
    StepForward,
    #[fmt("fas fa-stethoscope")]
    Stethoscope,
    #[fmt("fab fa-sticker-mule")]
    StickerMule,
    #[fmt("fas fa-sticky-note")]
    StickyNoteSolid,
    #[fmt("far fa-sticky-note")]
    StickyNoteRegular,
    #[fmt("fas fa-stop")]
    Stop,
    #[fmt("fas fa-stop-circle")]
    StopCircleSolid,
    #[fmt("far fa-stop-circle")]
    StopCircleRegular,
    #[fmt("fas fa-stopwatch")]
    Stopwatch,
    #[fmt("fas fa-stopwatch-20")]
    Stopwatch20,
    #[fmt("fas fa-store")]
    Store,
    #[fmt("fas fa-store-alt")]
    StoreAlt,
    #[fmt("fas fa-store-alt-slash")]
    StoreAltSlash,
    #[fmt("fas fa-store-slash")]
    StoreSlash,
    #[fmt("fab fa-strava")]
    Strava,
    #[fmt("fas fa-stream")]
    Stream,
    #[fmt("fas fa-street-view")]
    StreetView,
    #[fmt("fas fa-strikethrough")]
    Strikethrough,
    #[fmt("fab fa-stripe")]
    Stripe,
    #[fmt("fab fa-stripe-s")]
    StripeS,
    #[fmt("fas fa-stroopwafel")]
    Stroopwafel,
    #[fmt("fab fa-studiovinari")]
    Studiovinari,
    #[fmt("fab fa-stumbleupon")]
    Stumbleupon,
    #[fmt("fab fa-stumbleupon-circle")]
    StumbleuponCircle,
    #[fmt("fas fa-subscript")]
    Subscript,
    #[fmt("fas fa-subway")]
    Subway,
    #[fmt("fas fa-suitcase")]
    Suitcase,
    #[fmt("fas fa-suitcase-rolling")]
    SuitcaseRolling,
    #[fmt("fas fa-sun")]
    SunSolid,
    #[fmt("far fa-sun")]
    SunRegular,
    #[fmt("fab fa-superpowers")]
    Superpowers,
    #[fmt("fas fa-superscript")]
    Superscript,
    #[fmt("fab fa-supple")]
    Supple,
    #[fmt("fas fa-surprise")]
    SurpriseSolid,
    #[fmt("far fa-surprise")]
    SurpriseRegular,
    #[fmt("fab fa-suse")]
    Suse,
    #[fmt("fas fa-swatchbook")]
    Swatchbook,
    #[fmt("fab fa-swift")]
    Swift,
    #[fmt("fas fa-swimmer")]
    Swimmer,
    #[fmt("fas fa-swimming-pool")]
    SwimmingPool,
    #[fmt("fab fa-symfony")]
    Symfony,
    #[fmt("fas fa-synagogue")]
    Synagogue,
    #[fmt("fas fa-sync")]
    Sync,
    #[fmt("fas fa-sync-alt")]
    SyncAlt,
    #[fmt("fas fa-syringe")]
    Syringe,
    #[fmt("fas fa-table")]
    Table,
    #[fmt("fas fa-table-tennis")]
    TableTennis,
    #[fmt("fas fa-tablet")]
    Tablet,
    #[fmt("fas fa-tablet-alt")]
    TabletAlt,
    #[fmt("fas fa-tablets")]
    Tablets,
    #[fmt("fas fa-tachometer-alt")]
    TachometerAlt,
    #[fmt("fas fa-tag")]
    Tag,
    #[fmt("fas fa-tags")]
    Tags,
    #[fmt("fas fa-tape")]
    Tape,
    #[fmt("fas fa-tasks")]
    Tasks,
    #[fmt("fas fa-taxi")]
    Taxi,
    #[fmt("fab fa-teamspeak")]
    Teamspeak,
    #[fmt("fas fa-teeth")]
    Teeth,
    #[fmt("fas fa-teeth-open")]
    TeethOpen,
    #[fmt("fab fa-telegram")]
    Telegram,
    #[fmt("fab fa-telegram-plane")]
    TelegramPlane,
    #[fmt("fas fa-temperature-high")]
    TemperatureHigh,
    #[fmt("fas fa-temperature-low")]
    TemperatureLow,
    #[fmt("fab fa-tencent-weibo")]
    TencentWeibo,
    #[fmt("fas fa-tenge")]
    Tenge,
    #[fmt("fas fa-terminal")]
    Terminal,
    #[fmt("fas fa-text-height")]
    TextHeight,
    #[fmt("fas fa-text-width")]
    TextWidth,
    #[fmt("fas fa-th")]
    Th,
    #[fmt("fas fa-th-large")]
    ThLarge,
    #[fmt("fas fa-th-list")]
    ThList,
    #[fmt("fab fa-the-red-yeti")]
    TheRedYeti,
    #[fmt("fas fa-theater-masks")]
    TheaterMasks,
    #[fmt("fab fa-themeco")]
    Themeco,
    #[fmt("fab fa-themeisle")]
    Themeisle,
    #[fmt("fas fa-thermometer")]
    Thermometer,
    #[fmt("fas fa-thermometer-empty")]
    ThermometerEmpty,
    #[fmt("fas fa-thermometer-full")]
    ThermometerFull,
    #[fmt("fas fa-thermometer-half")]
    ThermometerHalf,
    #[fmt("fas fa-thermometer-quarter")]
    ThermometerQuarter,
    #[fmt("fas fa-thermometer-three-quarters")]
    ThermometerThreeQuarters,
    #[fmt("fab fa-think-peaks")]
    ThinkPeaks,
    #[fmt("fas fa-thumbs-down")]
    ThumbsDownSolid,
    #[fmt("far fa-thumbs-down")]
    ThumbsDownRegular,
    #[fmt("fas fa-thumbs-up")]
    ThumbsUpSolid,
    #[fmt("far fa-thumbs-up")]
    ThumbsUpRegular,
    #[fmt("fas fa-thumbtack")]
    Thumbtack,
    #[fmt("fas fa-ticket-alt")]
    TicketAlt,
    #[fmt("fab fa-tiktok")]
    Tiktok,
    #[fmt("fas fa-times")]
    Times,
    #[fmt("fas fa-times-circle")]
    TimesCircleSolid,
    #[fmt("far fa-times-circle")]
    TimesCircleRegular,
    #[fmt("fas fa-tint")]
    Tint,
    #[fmt("fas fa-tint-slash")]
    TintSlash,
    #[fmt("fas fa-tired")]
    TiredSolid,
    #[fmt("far fa-tired")]
    TiredRegular,
    #[fmt("fas fa-toggle-off")]
    ToggleOff,
    #[fmt("fas fa-toggle-on")]
    ToggleOn,
    #[fmt("fas fa-toilet")]
    Toilet,
    #[fmt("fas fa-toilet-paper")]
    ToiletPaper,
    #[fmt("fas fa-toilet-paper-slash")]
    ToiletPaperSlash,
    #[fmt("fas fa-toolbox")]
    Toolbox,
    #[fmt("fas fa-tools")]
    Tools,
    #[fmt("fas fa-tooth")]
    Tooth,
    #[fmt("fas fa-torah")]
    Torah,
    #[fmt("fas fa-torii-gate")]
    ToriiGate,
    #[fmt("fas fa-tractor")]
    Tractor,
    #[fmt("fab fa-trade-federation")]
    TradeFederation,
    #[fmt("fas fa-trademark")]
    Trademark,
    #[fmt("fas fa-traffic-light")]
    TrafficLight,
    #[fmt("fas fa-trailer")]
    Trailer,
    #[fmt("fas fa-train")]
    Train,
    #[fmt("fas fa-tram")]
    Tram,
    #[fmt("fas fa-transgender")]
    Transgender,
    #[fmt("fas fa-transgender-alt")]
    TransgenderAlt,
    #[fmt("fas fa-trash")]
    Trash,
    #[fmt("fas fa-trash-alt")]
    TrashAltSolid,
    #[fmt("far fa-trash-alt")]
    TrashAltRegular,
    #[fmt("fas fa-trash-restore")]
    TrashRestore,
    #[fmt("fas fa-trash-restore-alt")]
    TrashRestoreAlt,
    #[fmt("fas fa-tree")]
    Tree,
    #[fmt("fab fa-trello")]
    Trello,
    #[fmt("fab fa-tripadvisor")]
    Tripadvisor,
    #[fmt("fas fa-trophy")]
    Trophy,
    #[fmt("fas fa-truck")]
    Truck,
    #[fmt("fas fa-truck-loading")]
    TruckLoading,
    #[fmt("fas fa-truck-monster")]
    TruckMonster,
    #[fmt("fas fa-truck-moving")]
    TruckMoving,
    #[fmt("fas fa-truck-pickup")]
    TruckPickup,
    #[fmt("fas fa-tshirt")]
    Tshirt,
    #[fmt("fas fa-tty")]
    Tty,
    #[fmt("fab fa-tumblr")]
    Tumblr,
    #[fmt("fab fa-tumblr-square")]
    TumblrSquare,
    #[fmt("fas fa-tv")]
    Tv,
    #[fmt("fab fa-twitch")]
    Twitch,
    #[fmt("fab fa-twitter")]
    Twitter,
    #[fmt("fab fa-twitter-square")]
    TwitterSquare,
    #[fmt("fab fa-typo3")]
    Typo3,
    #[fmt("fab fa-uber")]
    Uber,
    #[fmt("fab fa-ubuntu")]
    Ubuntu,
    #[fmt("fab fa-uikit")]
    Uikit,
    #[fmt("fab fa-umbraco")]
    Umbraco,
    #[fmt("fas fa-umbrella")]
    Umbrella,
    #[fmt("fas fa-umbrella-beach")]
    UmbrellaBeach,
    #[fmt("fab fa-uncharted")]
    Uncharted,
    #[fmt("fas fa-underline")]
    Underline,
    #[fmt("fas fa-undo")]
    Undo,
    #[fmt("fas fa-undo-alt")]
    UndoAlt,
    #[fmt("fab fa-uniregistry")]
    Uniregistry,
    #[fmt("fab fa-unity")]
    Unity,
    #[fmt("fas fa-universal-access")]
    UniversalAccess,
    #[fmt("fas fa-university")]
    University,
    #[fmt("fas fa-unlink")]
    Unlink,
    #[fmt("fas fa-unlock")]
    Unlock,
    #[fmt("fas fa-unlock-alt")]
    UnlockAlt,
    #[fmt("fab fa-unsplash")]
    Unsplash,
    #[fmt("fab fa-untappd")]
    Untappd,
    #[fmt("fas fa-upload")]
    Upload,
    #[fmt("fab fa-ups")]
    Ups,
    #[fmt("fab fa-usb")]
    Usb,
    #[fmt("fas fa-user")]
    UserSolid,
    #[fmt("far fa-user")]
    UserRegular,
    #[fmt("fas fa-user-alt")]
    UserAlt,
    #[fmt("fas fa-user-alt-slash")]
    UserAltSlash,
    #[fmt("fas fa-user-astronaut")]
    UserAstronaut,
    #[fmt("fas fa-user-check")]
    UserCheck,
    #[fmt("fas fa-user-circle")]
    UserCircleSolid,
    #[fmt("far fa-user-circle")]
    UserCircleRegular,
    #[fmt("fas fa-user-clock")]
    UserClock,
    #[fmt("fas fa-user-cog")]
    UserCog,
    #[fmt("fas fa-user-edit")]
    UserEdit,
    #[fmt("fas fa-user-friends")]
    UserFriends,
    #[fmt("fas fa-user-graduate")]
    UserGraduate,
    #[fmt("fas fa-user-injured")]
    UserInjured,
    #[fmt("fas fa-user-lock")]
    UserLock,
    #[fmt("fas fa-user-md")]
    UserMd,
    #[fmt("fas fa-user-minus")]
    UserMinus,
    #[fmt("fas fa-user-ninja")]
    UserNinja,
    #[fmt("fas fa-user-nurse")]
    UserNurse,
    #[fmt("fas fa-user-plus")]
    UserPlus,
    #[fmt("fas fa-user-secret")]
    UserSecret,
    #[fmt("fas fa-user-shield")]
    UserShield,
    #[fmt("fas fa-user-slash")]
    UserSlash,
    #[fmt("fas fa-user-tag")]
    UserTag,
    #[fmt("fas fa-user-tie")]
    UserTie,
    #[fmt("fas fa-user-times")]
    UserTimes,
    #[fmt("fas fa-users")]
    Users,
    #[fmt("fas fa-users-cog")]
    UsersCog,
    #[fmt("fas fa-users-slash")]
    UsersSlash,
    #[fmt("fab fa-usps")]
    Usps,
    #[fmt("fab fa-ussunnah")]
    Ussunnah,
    #[fmt("fas fa-utensil-spoon")]
    UtensilSpoon,
    #[fmt("fas fa-utensils")]
    Utensils,
    #[fmt("fab fa-vaadin")]
    Vaadin,
    #[fmt("fas fa-vector-square")]
    VectorSquare,
    #[fmt("fas fa-venus")]
    Venus,
    #[fmt("fas fa-venus-double")]
    VenusDouble,
    #[fmt("fas fa-venus-mars")]
    VenusMars,
    #[fmt("fas fa-vest")]
    Vest,
    #[fmt("fas fa-vest-patches")]
    VestPatches,
    #[fmt("fab fa-viacoin")]
    Viacoin,
    #[fmt("fab fa-viadeo")]
    Viadeo,
    #[fmt("fab fa-viadeo-square")]
    ViadeoSquare,
    #[fmt("fas fa-vial")]
    Vial,
    #[fmt("fas fa-vials")]
    Vials,
    #[fmt("fab fa-viber")]
    Viber,
    #[fmt("fas fa-video")]
    Video,
    #[fmt("fas fa-video-slash")]
    VideoSlash,
    #[fmt("fas fa-vihara")]
    Vihara,
    #[fmt("fab fa-vimeo")]
    Vimeo,
    #[fmt("fab fa-vimeo-square")]
    VimeoSquare,
    #[fmt("fab fa-vimeo-v")]
    VimeoV,
    #[fmt("fab fa-vine")]
    Vine,
    #[fmt("fas fa-virus")]
    Virus,
    #[fmt("fas fa-virus-slash")]
    VirusSlash,
    #[fmt("fas fa-viruses")]
    Viruses,
    #[fmt("fab fa-vk")]
    Vk,
    #[fmt("fab fa-vnv")]
    Vnv,
    #[fmt("fas fa-voicemail")]
    Voicemail,
    #[fmt("fas fa-volleyball-ball")]
    VolleyballBall,
    #[fmt("fas fa-volume-down")]
    VolumeDown,
    #[fmt("fas fa-volume-mute")]
    VolumeMute,
    #[fmt("fas fa-volume-off")]
    VolumeOff,
    #[fmt("fas fa-volume-up")]
    VolumeUp,
    #[fmt("fas fa-vote-yea")]
    VoteYea,
    #[fmt("fas fa-vr-cardboard")]
    VrCardboard,
    #[fmt("fab fa-vuejs")]
    Vuejs,
    #[fmt("fas fa-walking")]
    Walking,
    #[fmt("fas fa-wallet")]
    Wallet,
    #[fmt("fas fa-warehouse")]
    Warehouse,
    #[fmt("fab fa-watchman-monitoring")]
    WatchmanMonitoring,
    #[fmt("fas fa-water")]
    Water,
    #[fmt("fas fa-wave-square")]
    WaveSquare,
    #[fmt("fab fa-waze")]
    Waze,
    #[fmt("fab fa-weebly")]
    Weebly,
    #[fmt("fab fa-weibo")]
    Weibo,
    #[fmt("fas fa-weight")]
    Weight,
    #[fmt("fas fa-weight-hanging")]
    WeightHanging,
    #[fmt("fab fa-weixin")]
    Weixin,
    #[fmt("fab fa-whatsapp")]
    Whatsapp,
    #[fmt("fab fa-whatsapp-square")]
    WhatsappSquare,
    #[fmt("fas fa-wheelchair")]
    Wheelchair,
    #[fmt("fab fa-whmcs")]
    Whmcs,
    #[fmt("fas fa-wifi")]
    Wifi,
    #[fmt("fab fa-wikipedia-w")]
    WikipediaW,
    #[fmt("fas fa-wind")]
    Wind,
    #[fmt("fas fa-window-close")]
    WindowCloseSolid,
    #[fmt("far fa-window-close")]
    WindowCloseRegular,
    #[fmt("fas fa-window-maximize")]
    WindowMaximizeSolid,
    #[fmt("far fa-window-maximize")]
    WindowMaximizeRegular,
    #[fmt("fas fa-window-minimize")]
    WindowMinimizeSolid,
    #[fmt("far fa-window-minimize")]
    WindowMinimizeRegular,
    #[fmt("fas fa-window-restore")]
    WindowRestoreSolid,
    #[fmt("far fa-window-restore")]
    WindowRestoreRegular,
    #[fmt("fab fa-windows")]
    Windows,
    #[fmt("fas fa-wine-bottle")]
    WineBottle,
    #[fmt("fas fa-wine-glass")]
    WineGlass,
    #[fmt("fas fa-wine-glass-alt")]
    WineGlassAlt,
    #[fmt("fab fa-wix")]
    Wix,
    #[fmt("fab fa-wizards-of-the-coast")]
    WizardsOfTheCoast,
    #[fmt("fab fa-wodu")]
    Wodu,
    #[fmt("fab fa-wolf-pack-battalion")]
    WolfPackBattalion,
    #[fmt("fas fa-won-sign")]
    WonSign,
    #[fmt("fab fa-wordpress")]
    Wordpress,
    #[fmt("fab fa-wordpress-simple")]
    WordpressSimple,
    #[fmt("fab fa-wpbeginner")]
    Wpbeginner,
    #[fmt("fab fa-wpexplorer")]
    Wpexplorer,
    #[fmt("fab fa-wpforms")]
    Wpforms,
    #[fmt("fab fa-wpressr")]
    Wpressr,
    #[fmt("fas fa-wrench")]
    Wrench,
    #[fmt("fas fa-x-ray")]
    XRay,
    #[fmt("fab fa-xbox")]
    Xbox,
    #[fmt("fab fa-xing")]
    Xing,
    #[fmt("fab fa-xing-square")]
    XingSquare,
    #[fmt("fab fa-y-combinator")]
    YCombinator,
    #[fmt("fab fa-yahoo")]
    Yahoo,
    #[fmt("fab fa-yammer")]
    Yammer,
    #[fmt("fab fa-yandex")]
    Yandex,
    #[fmt("fab fa-yandex-international")]
    YandexInternational,
    #[fmt("fab fa-yarn")]
    Yarn,
    #[fmt("fab fa-yelp")]
    Yelp,
    #[fmt("fas fa-yen-sign")]
    YenSign,
    #[fmt("fas fa-yin-yang")]
    YinYang,
    #[fmt("fab fa-yoast")]
    Yoast,
    #[fmt("fab fa-youtube")]
    Youtube,
    #[fmt("fab fa-youtube-square")]
    YoutubeSquare,
    #[fmt("fab fa-zhihu")]
    Zhihu,
}
