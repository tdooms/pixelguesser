use derive_more::Display;

#[derive(Debug, Clone, Copy, Display)]
pub enum Icons {
    #[display(fmt = "fab fa-accessible-icon")]
    AccessibleIcon,
    #[display(fmt = "fab fa-accusoft")]
    Accusoft,
    #[display(fmt = "fab fa-acquisitions-incorporated")]
    AcquisitionsIncorporated,
    #[display(fmt = "fas fa-ad")]
    Ad,
    #[display(fmt = "fas fa-address-book")]
    AddressBookSolid,
    #[display(fmt = "far fa-address-book")]
    AddressBookRegular,
    #[display(fmt = "fas fa-address-card")]
    AddressCardSolid,
    #[display(fmt = "far fa-address-card")]
    AddressCardRegular,
    #[display(fmt = "fas fa-adjust")]
    Adjust,
    #[display(fmt = "fab fa-adn")]
    Adn,
    #[display(fmt = "fab fa-adversal")]
    Adversal,
    #[display(fmt = "fab fa-affiliatetheme")]
    Affiliatetheme,
    #[display(fmt = "fas fa-air-freshener")]
    AirFreshener,
    #[display(fmt = "fab fa-airbnb")]
    Airbnb,
    #[display(fmt = "fab fa-algolia")]
    Algolia,
    #[display(fmt = "fas fa-align-center")]
    AlignCenter,
    #[display(fmt = "fas fa-align-justify")]
    AlignJustify,
    #[display(fmt = "fas fa-align-left")]
    AlignLeft,
    #[display(fmt = "fas fa-align-right")]
    AlignRight,
    #[display(fmt = "fab fa-alipay")]
    Alipay,
    #[display(fmt = "fas fa-allergies")]
    Allergies,
    #[display(fmt = "fab fa-amazon")]
    Amazon,
    #[display(fmt = "fab fa-amazon-pay")]
    AmazonPay,
    #[display(fmt = "fas fa-ambulance")]
    Ambulance,
    #[display(fmt = "fas fa-american-sign-language-interpreting")]
    AmericanSignLanguageInterpreting,
    #[display(fmt = "fab fa-amilia")]
    Amilia,
    #[display(fmt = "fas fa-anchor")]
    Anchor,
    #[display(fmt = "fab fa-android")]
    Android,
    #[display(fmt = "fab fa-angellist")]
    Angellist,
    #[display(fmt = "fas fa-angle-double-down")]
    AngleDoubleDown,
    #[display(fmt = "fas fa-angle-double-left")]
    AngleDoubleLeft,
    #[display(fmt = "fas fa-angle-double-right")]
    AngleDoubleRight,
    #[display(fmt = "fas fa-angle-double-up")]
    AngleDoubleUp,
    #[display(fmt = "fas fa-angle-down")]
    AngleDown,
    #[display(fmt = "fas fa-angle-left")]
    AngleLeft,
    #[display(fmt = "fas fa-angle-right")]
    AngleRight,
    #[display(fmt = "fas fa-angle-up")]
    AngleUp,
    #[display(fmt = "fas fa-angry")]
    AngrySolid,
    #[display(fmt = "far fa-angry")]
    AngryRegular,
    #[display(fmt = "fab fa-angrycreative")]
    Angrycreative,
    #[display(fmt = "fab fa-angular")]
    Angular,
    #[display(fmt = "fas fa-ankh")]
    Ankh,
    #[display(fmt = "fab fa-app-store")]
    AppStore,
    #[display(fmt = "fab fa-app-store-ios")]
    AppStoreIos,
    #[display(fmt = "fab fa-apper")]
    Apper,
    #[display(fmt = "fab fa-apple")]
    Apple,
    #[display(fmt = "fas fa-apple-alt")]
    AppleAlt,
    #[display(fmt = "fab fa-apple-pay")]
    ApplePay,
    #[display(fmt = "fas fa-archive")]
    Archive,
    #[display(fmt = "fas fa-archway")]
    Archway,
    #[display(fmt = "fas fa-arrow-alt-circle-down")]
    ArrowAltCircleDownSolid,
    #[display(fmt = "far fa-arrow-alt-circle-down")]
    ArrowAltCircleDownRegular,
    #[display(fmt = "fas fa-arrow-alt-circle-left")]
    ArrowAltCircleLeftSolid,
    #[display(fmt = "far fa-arrow-alt-circle-left")]
    ArrowAltCircleLeftRegular,
    #[display(fmt = "fas fa-arrow-alt-circle-right")]
    ArrowAltCircleRightSolid,
    #[display(fmt = "far fa-arrow-alt-circle-right")]
    ArrowAltCircleRightRegular,
    #[display(fmt = "fas fa-arrow-alt-circle-up")]
    ArrowAltCircleUpSolid,
    #[display(fmt = "far fa-arrow-alt-circle-up")]
    ArrowAltCircleUpRegular,
    #[display(fmt = "fas fa-arrow-circle-down")]
    ArrowCircleDown,
    #[display(fmt = "fas fa-arrow-circle-left")]
    ArrowCircleLeft,
    #[display(fmt = "fas fa-arrow-circle-right")]
    ArrowCircleRight,
    #[display(fmt = "fas fa-arrow-circle-up")]
    ArrowCircleUp,
    #[display(fmt = "fas fa-arrow-down")]
    ArrowDown,
    #[display(fmt = "fas fa-arrow-left")]
    ArrowLeft,
    #[display(fmt = "fas fa-arrow-right")]
    ArrowRight,
    #[display(fmt = "fas fa-arrow-up")]
    ArrowUp,
    #[display(fmt = "fas fa-arrows-alt")]
    ArrowsAlt,
    #[display(fmt = "fas fa-arrows-alt-h")]
    ArrowsAltH,
    #[display(fmt = "fas fa-arrows-alt-v")]
    ArrowsAltV,
    #[display(fmt = "fab fa-artstation")]
    Artstation,
    #[display(fmt = "fas fa-assistive-listening-systems")]
    AssistiveListeningSystems,
    #[display(fmt = "fas fa-asterisk")]
    Asterisk,
    #[display(fmt = "fab fa-asymmetrik")]
    Asymmetrik,
    #[display(fmt = "fas fa-at")]
    At,
    #[display(fmt = "fas fa-atlas")]
    Atlas,
    #[display(fmt = "fab fa-atlassian")]
    Atlassian,
    #[display(fmt = "fas fa-atom")]
    Atom,
    #[display(fmt = "fab fa-audible")]
    Audible,
    #[display(fmt = "fas fa-audio-description")]
    AudioDescription,
    #[display(fmt = "fab fa-autoprefixer")]
    Autoprefixer,
    #[display(fmt = "fab fa-avianex")]
    Avianex,
    #[display(fmt = "fab fa-aviato")]
    Aviato,
    #[display(fmt = "fas fa-award")]
    Award,
    #[display(fmt = "fab fa-aws")]
    Aws,
    #[display(fmt = "fas fa-baby")]
    Baby,
    #[display(fmt = "fas fa-baby-carriage")]
    BabyCarriage,
    #[display(fmt = "fas fa-backspace")]
    Backspace,
    #[display(fmt = "fas fa-backward")]
    Backward,
    #[display(fmt = "fas fa-bacon")]
    Bacon,
    #[display(fmt = "fas fa-bacteria")]
    Bacteria,
    #[display(fmt = "fas fa-bacterium")]
    Bacterium,
    #[display(fmt = "fas fa-bahai")]
    Bahai,
    #[display(fmt = "fas fa-balance-scale")]
    BalanceScale,
    #[display(fmt = "fas fa-balance-scale-left")]
    BalanceScaleLeft,
    #[display(fmt = "fas fa-balance-scale-right")]
    BalanceScaleRight,
    #[display(fmt = "fas fa-ban")]
    Ban,
    #[display(fmt = "fas fa-band-aid")]
    BandAid,
    #[display(fmt = "fab fa-bandcamp")]
    Bandcamp,
    #[display(fmt = "fas fa-barcode")]
    Barcode,
    #[display(fmt = "fas fa-bars")]
    Bars,
    #[display(fmt = "fas fa-baseball-ball")]
    BaseballBall,
    #[display(fmt = "fas fa-basketball-ball")]
    BasketballBall,
    #[display(fmt = "fas fa-bath")]
    Bath,
    #[display(fmt = "fas fa-battery-empty")]
    BatteryEmpty,
    #[display(fmt = "fas fa-battery-full")]
    BatteryFull,
    #[display(fmt = "fas fa-battery-half")]
    BatteryHalf,
    #[display(fmt = "fas fa-battery-quarter")]
    BatteryQuarter,
    #[display(fmt = "fas fa-battery-three-quarters")]
    BatteryThreeQuarters,
    #[display(fmt = "fab fa-battle-net")]
    BattleNet,
    #[display(fmt = "fas fa-bed")]
    Bed,
    #[display(fmt = "fas fa-beer")]
    Beer,
    #[display(fmt = "fab fa-behance")]
    Behance,
    #[display(fmt = "fab fa-behance-square")]
    BehanceSquare,
    #[display(fmt = "fas fa-bell")]
    BellSolid,
    #[display(fmt = "far fa-bell")]
    BellRegular,
    #[display(fmt = "fas fa-bell-slash")]
    BellSlashSolid,
    #[display(fmt = "far fa-bell-slash")]
    BellSlashRegular,
    #[display(fmt = "fas fa-bezier-curve")]
    BezierCurve,
    #[display(fmt = "fas fa-bible")]
    Bible,
    #[display(fmt = "fas fa-bicycle")]
    Bicycle,
    #[display(fmt = "fas fa-biking")]
    Biking,
    #[display(fmt = "fab fa-bimobject")]
    Bimobject,
    #[display(fmt = "fas fa-binoculars")]
    Binoculars,
    #[display(fmt = "fas fa-biohazard")]
    Biohazard,
    #[display(fmt = "fas fa-birthday-cake")]
    BirthdayCake,
    #[display(fmt = "fab fa-bitbucket")]
    Bitbucket,
    #[display(fmt = "fab fa-bitcoin")]
    Bitcoin,
    #[display(fmt = "fab fa-bity")]
    Bity,
    #[display(fmt = "fab fa-black-tie")]
    BlackTie,
    #[display(fmt = "fab fa-blackberry")]
    Blackberry,
    #[display(fmt = "fas fa-blender")]
    Blender,
    #[display(fmt = "fas fa-blender-phone")]
    BlenderPhone,
    #[display(fmt = "fas fa-blind")]
    Blind,
    #[display(fmt = "fas fa-blog")]
    Blog,
    #[display(fmt = "fab fa-blogger")]
    Blogger,
    #[display(fmt = "fab fa-blogger-b")]
    BloggerB,
    #[display(fmt = "fab fa-bluetooth")]
    Bluetooth,
    #[display(fmt = "fab fa-bluetooth-b")]
    BluetoothB,
    #[display(fmt = "fas fa-bold")]
    Bold,
    #[display(fmt = "fas fa-bolt")]
    Bolt,
    #[display(fmt = "fas fa-bomb")]
    Bomb,
    #[display(fmt = "fas fa-bone")]
    Bone,
    #[display(fmt = "fas fa-bong")]
    Bong,
    #[display(fmt = "fas fa-book")]
    Book,
    #[display(fmt = "fas fa-book-dead")]
    BookDead,
    #[display(fmt = "fas fa-book-medical")]
    BookMedical,
    #[display(fmt = "fas fa-book-open")]
    BookOpen,
    #[display(fmt = "fas fa-book-reader")]
    BookReader,
    #[display(fmt = "fas fa-bookmark")]
    BookmarkSolid,
    #[display(fmt = "far fa-bookmark")]
    BookmarkRegular,
    #[display(fmt = "fab fa-bootstrap")]
    Bootstrap,
    #[display(fmt = "fas fa-border-all")]
    BorderAll,
    #[display(fmt = "fas fa-border-none")]
    BorderNone,
    #[display(fmt = "fas fa-border-style")]
    BorderStyle,
    #[display(fmt = "fas fa-bowling-ball")]
    BowlingBall,
    #[display(fmt = "fas fa-box")]
    Box,
    #[display(fmt = "fas fa-box-open")]
    BoxOpen,
    #[display(fmt = "fas fa-box-tissue")]
    BoxTissue,
    #[display(fmt = "fas fa-boxes")]
    Boxes,
    #[display(fmt = "fas fa-braille")]
    Braille,
    #[display(fmt = "fas fa-brain")]
    Brain,
    #[display(fmt = "fas fa-bread-slice")]
    BreadSlice,
    #[display(fmt = "fas fa-briefcase")]
    Briefcase,
    #[display(fmt = "fas fa-briefcase-medical")]
    BriefcaseMedical,
    #[display(fmt = "fas fa-broadcast-tower")]
    BroadcastTower,
    #[display(fmt = "fas fa-broom")]
    Broom,
    #[display(fmt = "fas fa-brush")]
    Brush,
    #[display(fmt = "fab fa-btc")]
    Btc,
    #[display(fmt = "fab fa-buffer")]
    Buffer,
    #[display(fmt = "fas fa-bug")]
    Bug,
    #[display(fmt = "fas fa-building")]
    BuildingSolid,
    #[display(fmt = "far fa-building")]
    BuildingRegular,
    #[display(fmt = "fas fa-bullhorn")]
    Bullhorn,
    #[display(fmt = "fas fa-bullseye")]
    Bullseye,
    #[display(fmt = "fas fa-burn")]
    Burn,
    #[display(fmt = "fab fa-buromobelexperte")]
    Buromobelexperte,
    #[display(fmt = "fas fa-bus")]
    Bus,
    #[display(fmt = "fas fa-bus-alt")]
    BusAlt,
    #[display(fmt = "fas fa-business-time")]
    BusinessTime,
    #[display(fmt = "fab fa-buy-n-large")]
    BuyNLarge,
    #[display(fmt = "fab fa-buysellads")]
    Buysellads,
    #[display(fmt = "fas fa-calculator")]
    Calculator,
    #[display(fmt = "fas fa-calendar")]
    CalendarSolid,
    #[display(fmt = "far fa-calendar")]
    CalendarRegular,
    #[display(fmt = "fas fa-calendar-alt")]
    CalendarAltSolid,
    #[display(fmt = "far fa-calendar-alt")]
    CalendarAltRegular,
    #[display(fmt = "fas fa-calendar-check")]
    CalendarCheckSolid,
    #[display(fmt = "far fa-calendar-check")]
    CalendarCheckRegular,
    #[display(fmt = "fas fa-calendar-day")]
    CalendarDay,
    #[display(fmt = "fas fa-calendar-minus")]
    CalendarMinusSolid,
    #[display(fmt = "far fa-calendar-minus")]
    CalendarMinusRegular,
    #[display(fmt = "fas fa-calendar-plus")]
    CalendarPlusSolid,
    #[display(fmt = "far fa-calendar-plus")]
    CalendarPlusRegular,
    #[display(fmt = "fas fa-calendar-times")]
    CalendarTimesSolid,
    #[display(fmt = "far fa-calendar-times")]
    CalendarTimesRegular,
    #[display(fmt = "fas fa-calendar-week")]
    CalendarWeek,
    #[display(fmt = "fas fa-camera")]
    Camera,
    #[display(fmt = "fas fa-camera-retro")]
    CameraRetro,
    #[display(fmt = "fas fa-campground")]
    Campground,
    #[display(fmt = "fab fa-canadian-maple-leaf")]
    CanadianMapleLeaf,
    #[display(fmt = "fas fa-candy-cane")]
    CandyCane,
    #[display(fmt = "fas fa-cannabis")]
    Cannabis,
    #[display(fmt = "fas fa-capsules")]
    Capsules,
    #[display(fmt = "fas fa-car")]
    Car,
    #[display(fmt = "fas fa-car-alt")]
    CarAlt,
    #[display(fmt = "fas fa-car-battery")]
    CarBattery,
    #[display(fmt = "fas fa-car-crash")]
    CarCrash,
    #[display(fmt = "fas fa-car-side")]
    CarSide,
    #[display(fmt = "fas fa-caravan")]
    Caravan,
    #[display(fmt = "fas fa-caret-down")]
    CaretDown,
    #[display(fmt = "fas fa-caret-left")]
    CaretLeft,
    #[display(fmt = "fas fa-caret-right")]
    CaretRight,
    #[display(fmt = "fas fa-caret-square-down")]
    CaretSquareDownSolid,
    #[display(fmt = "far fa-caret-square-down")]
    CaretSquareDownRegular,
    #[display(fmt = "fas fa-caret-square-left")]
    CaretSquareLeftSolid,
    #[display(fmt = "far fa-caret-square-left")]
    CaretSquareLeftRegular,
    #[display(fmt = "fas fa-caret-square-right")]
    CaretSquareRightSolid,
    #[display(fmt = "far fa-caret-square-right")]
    CaretSquareRightRegular,
    #[display(fmt = "fas fa-caret-square-up")]
    CaretSquareUpSolid,
    #[display(fmt = "far fa-caret-square-up")]
    CaretSquareUpRegular,
    #[display(fmt = "fas fa-caret-up")]
    CaretUp,
    #[display(fmt = "fas fa-carrot")]
    Carrot,
    #[display(fmt = "fas fa-cart-arrow-down")]
    CartArrowDown,
    #[display(fmt = "fas fa-cart-plus")]
    CartPlus,
    #[display(fmt = "fas fa-cash-register")]
    CashRegister,
    #[display(fmt = "fas fa-cat")]
    Cat,
    #[display(fmt = "fab fa-cc-amazon-pay")]
    CcAmazonPay,
    #[display(fmt = "fab fa-cc-amex")]
    CcAmex,
    #[display(fmt = "fab fa-cc-apple-pay")]
    CcApplePay,
    #[display(fmt = "fab fa-cc-diners-club")]
    CcDinersClub,
    #[display(fmt = "fab fa-cc-discover")]
    CcDiscover,
    #[display(fmt = "fab fa-cc-jcb")]
    CcJcb,
    #[display(fmt = "fab fa-cc-mastercard")]
    CcMastercard,
    #[display(fmt = "fab fa-cc-paypal")]
    CcPaypal,
    #[display(fmt = "fab fa-cc-stripe")]
    CcStripe,
    #[display(fmt = "fab fa-cc-visa")]
    CcVisa,
    #[display(fmt = "fab fa-centercode")]
    Centercode,
    #[display(fmt = "fab fa-centos")]
    Centos,
    #[display(fmt = "fas fa-certificate")]
    Certificate,
    #[display(fmt = "fas fa-chair")]
    Chair,
    #[display(fmt = "fas fa-chalkboard")]
    Chalkboard,
    #[display(fmt = "fas fa-chalkboard-teacher")]
    ChalkboardTeacher,
    #[display(fmt = "fas fa-charging-station")]
    ChargingStation,
    #[display(fmt = "fas fa-chart-area")]
    ChartArea,
    #[display(fmt = "fas fa-chart-bar")]
    ChartBarSolid,
    #[display(fmt = "far fa-chart-bar")]
    ChartBarRegular,
    #[display(fmt = "fas fa-chart-line")]
    ChartLine,
    #[display(fmt = "fas fa-chart-pie")]
    ChartPie,
    #[display(fmt = "fas fa-check")]
    Check,
    #[display(fmt = "fas fa-check-circle")]
    CheckCircleSolid,
    #[display(fmt = "far fa-check-circle")]
    CheckCircleRegular,
    #[display(fmt = "fas fa-check-double")]
    CheckDouble,
    #[display(fmt = "fas fa-check-square")]
    CheckSquareSolid,
    #[display(fmt = "far fa-check-square")]
    CheckSquareRegular,
    #[display(fmt = "fas fa-cheese")]
    Cheese,
    #[display(fmt = "fas fa-chess")]
    Chess,
    #[display(fmt = "fas fa-chess-bishop")]
    ChessBishop,
    #[display(fmt = "fas fa-chess-board")]
    ChessBoard,
    #[display(fmt = "fas fa-chess-king")]
    ChessKing,
    #[display(fmt = "fas fa-chess-knight")]
    ChessKnight,
    #[display(fmt = "fas fa-chess-pawn")]
    ChessPawn,
    #[display(fmt = "fas fa-chess-queen")]
    ChessQueen,
    #[display(fmt = "fas fa-chess-rook")]
    ChessRook,
    #[display(fmt = "fas fa-chevron-circle-down")]
    ChevronCircleDown,
    #[display(fmt = "fas fa-chevron-circle-left")]
    ChevronCircleLeft,
    #[display(fmt = "fas fa-chevron-circle-right")]
    ChevronCircleRight,
    #[display(fmt = "fas fa-chevron-circle-up")]
    ChevronCircleUp,
    #[display(fmt = "fas fa-chevron-down")]
    ChevronDown,
    #[display(fmt = "fas fa-chevron-left")]
    ChevronLeft,
    #[display(fmt = "fas fa-chevron-right")]
    ChevronRight,
    #[display(fmt = "fas fa-chevron-up")]
    ChevronUp,
    #[display(fmt = "fas fa-child")]
    Child,
    #[display(fmt = "fab fa-chrome")]
    Chrome,
    #[display(fmt = "fab fa-chromecast")]
    Chromecast,
    #[display(fmt = "fas fa-church")]
    Church,
    #[display(fmt = "fas fa-circle")]
    CircleSolid,
    #[display(fmt = "far fa-circle")]
    CircleRegular,
    #[display(fmt = "fas fa-circle-notch")]
    CircleNotch,
    #[display(fmt = "fas fa-city")]
    City,
    #[display(fmt = "fas fa-clinic-medical")]
    ClinicMedical,
    #[display(fmt = "fas fa-clipboard")]
    ClipboardSolid,
    #[display(fmt = "far fa-clipboard")]
    ClipboardRegular,
    #[display(fmt = "fas fa-clipboard-check")]
    ClipboardCheck,
    #[display(fmt = "fas fa-clipboard-list")]
    ClipboardList,
    #[display(fmt = "fas fa-clock")]
    ClockSolid,
    #[display(fmt = "far fa-clock")]
    ClockRegular,
    #[display(fmt = "fas fa-clone")]
    CloneSolid,
    #[display(fmt = "far fa-clone")]
    CloneRegular,
    #[display(fmt = "fas fa-closed-captioning")]
    ClosedCaptioningSolid,
    #[display(fmt = "far fa-closed-captioning")]
    ClosedCaptioningRegular,
    #[display(fmt = "fas fa-cloud")]
    Cloud,
    #[display(fmt = "fas fa-cloud-download-alt")]
    CloudDownloadAlt,
    #[display(fmt = "fas fa-cloud-meatball")]
    CloudMeatball,
    #[display(fmt = "fas fa-cloud-moon")]
    CloudMoon,
    #[display(fmt = "fas fa-cloud-moon-rain")]
    CloudMoonRain,
    #[display(fmt = "fas fa-cloud-rain")]
    CloudRain,
    #[display(fmt = "fas fa-cloud-showers-heavy")]
    CloudShowersHeavy,
    #[display(fmt = "fas fa-cloud-sun")]
    CloudSun,
    #[display(fmt = "fas fa-cloud-sun-rain")]
    CloudSunRain,
    #[display(fmt = "fas fa-cloud-upload-alt")]
    CloudUploadAlt,
    #[display(fmt = "fab fa-cloudflare")]
    Cloudflare,
    #[display(fmt = "fab fa-cloudscale")]
    Cloudscale,
    #[display(fmt = "fab fa-cloudsmith")]
    Cloudsmith,
    #[display(fmt = "fab fa-cloudversify")]
    Cloudversify,
    #[display(fmt = "fas fa-cocktail")]
    Cocktail,
    #[display(fmt = "fas fa-code")]
    Code,
    #[display(fmt = "fas fa-code-branch")]
    CodeBranch,
    #[display(fmt = "fab fa-codepen")]
    Codepen,
    #[display(fmt = "fab fa-codiepie")]
    Codiepie,
    #[display(fmt = "fas fa-coffee")]
    Coffee,
    #[display(fmt = "fas fa-cog")]
    Cog,
    #[display(fmt = "fas fa-cogs")]
    Cogs,
    #[display(fmt = "fas fa-coins")]
    Coins,
    #[display(fmt = "fas fa-columns")]
    Columns,
    #[display(fmt = "fas fa-comment")]
    CommentSolid,
    #[display(fmt = "far fa-comment")]
    CommentRegular,
    #[display(fmt = "fas fa-comment-alt")]
    CommentAltSolid,
    #[display(fmt = "far fa-comment-alt")]
    CommentAltRegular,
    #[display(fmt = "fas fa-comment-dollar")]
    CommentDollar,
    #[display(fmt = "fas fa-comment-dots")]
    CommentDotsSolid,
    #[display(fmt = "far fa-comment-dots")]
    CommentDotsRegular,
    #[display(fmt = "fas fa-comment-medical")]
    CommentMedical,
    #[display(fmt = "fas fa-comment-slash")]
    CommentSlash,
    #[display(fmt = "fas fa-comments")]
    CommentsSolid,
    #[display(fmt = "far fa-comments")]
    CommentsRegular,
    #[display(fmt = "fas fa-comments-dollar")]
    CommentsDollar,
    #[display(fmt = "fas fa-compact-disc")]
    CompactDisc,
    #[display(fmt = "fas fa-compass")]
    CompassSolid,
    #[display(fmt = "far fa-compass")]
    CompassRegular,
    #[display(fmt = "fas fa-compress")]
    Compress,
    #[display(fmt = "fas fa-compress-alt")]
    CompressAlt,
    #[display(fmt = "fas fa-compress-arrows-alt")]
    CompressArrowsAlt,
    #[display(fmt = "fas fa-concierge-bell")]
    ConciergeBell,
    #[display(fmt = "fab fa-confluence")]
    Confluence,
    #[display(fmt = "fab fa-connectdevelop")]
    Connectdevelop,
    #[display(fmt = "fab fa-contao")]
    Contao,
    #[display(fmt = "fas fa-cookie")]
    Cookie,
    #[display(fmt = "fas fa-cookie-bite")]
    CookieBite,
    #[display(fmt = "fas fa-copy")]
    CopySolid,
    #[display(fmt = "far fa-copy")]
    CopyRegular,
    #[display(fmt = "fas fa-copyright")]
    CopyrightSolid,
    #[display(fmt = "far fa-copyright")]
    CopyrightRegular,
    #[display(fmt = "fab fa-cotton-bureau")]
    CottonBureau,
    #[display(fmt = "fas fa-couch")]
    Couch,
    #[display(fmt = "fab fa-cpanel")]
    Cpanel,
    #[display(fmt = "fab fa-creative-commons")]
    CreativeCommons,
    #[display(fmt = "fab fa-creative-commons-by")]
    CreativeCommonsBy,
    #[display(fmt = "fab fa-creative-commons-nc")]
    CreativeCommonsNc,
    #[display(fmt = "fab fa-creative-commons-nc-eu")]
    CreativeCommonsNcEu,
    #[display(fmt = "fab fa-creative-commons-nc-jp")]
    CreativeCommonsNcJp,
    #[display(fmt = "fab fa-creative-commons-nd")]
    CreativeCommonsNd,
    #[display(fmt = "fab fa-creative-commons-pd")]
    CreativeCommonsPd,
    #[display(fmt = "fab fa-creative-commons-pd-alt")]
    CreativeCommonsPdAlt,
    #[display(fmt = "fab fa-creative-commons-remix")]
    CreativeCommonsRemix,
    #[display(fmt = "fab fa-creative-commons-sa")]
    CreativeCommonsSa,
    #[display(fmt = "fab fa-creative-commons-sampling")]
    CreativeCommonsSampling,
    #[display(fmt = "fab fa-creative-commons-sampling-plus")]
    CreativeCommonsSamplingPlus,
    #[display(fmt = "fab fa-creative-commons-share")]
    CreativeCommonsShare,
    #[display(fmt = "fab fa-creative-commons-zero")]
    CreativeCommonsZero,
    #[display(fmt = "fas fa-credit-card")]
    CreditCardSolid,
    #[display(fmt = "far fa-credit-card")]
    CreditCardRegular,
    #[display(fmt = "fab fa-critical-role")]
    CriticalRole,
    #[display(fmt = "fas fa-crop")]
    Crop,
    #[display(fmt = "fas fa-crop-alt")]
    CropAlt,
    #[display(fmt = "fas fa-cross")]
    Cross,
    #[display(fmt = "fas fa-crosshairs")]
    Crosshairs,
    #[display(fmt = "fas fa-crow")]
    Crow,
    #[display(fmt = "fas fa-crown")]
    Crown,
    #[display(fmt = "fas fa-crutch")]
    Crutch,
    #[display(fmt = "fab fa-css3")]
    Css3,
    #[display(fmt = "fab fa-css3-alt")]
    Css3Alt,
    #[display(fmt = "fas fa-cube")]
    Cube,
    #[display(fmt = "fas fa-cubes")]
    Cubes,
    #[display(fmt = "fas fa-cut")]
    Cut,
    #[display(fmt = "fab fa-cuttlefish")]
    Cuttlefish,
    #[display(fmt = "fab fa-d-and-d")]
    DAndD,
    #[display(fmt = "fab fa-d-and-d-beyond")]
    DAndDBeyond,
    #[display(fmt = "fab fa-dailymotion")]
    Dailymotion,
    #[display(fmt = "fab fa-dashcube")]
    Dashcube,
    #[display(fmt = "fas fa-database")]
    Database,
    #[display(fmt = "fas fa-deaf")]
    Deaf,
    #[display(fmt = "fab fa-deezer")]
    Deezer,
    #[display(fmt = "fab fa-delicious")]
    Delicious,
    #[display(fmt = "fas fa-democrat")]
    Democrat,
    #[display(fmt = "fab fa-deploydog")]
    Deploydog,
    #[display(fmt = "fab fa-deskpro")]
    Deskpro,
    #[display(fmt = "fas fa-desktop")]
    Desktop,
    #[display(fmt = "fab fa-dev")]
    Dev,
    #[display(fmt = "fab fa-deviantart")]
    Deviantart,
    #[display(fmt = "fas fa-dharmachakra")]
    Dharmachakra,
    #[display(fmt = "fab fa-dhl")]
    Dhl,
    #[display(fmt = "fas fa-diagnoses")]
    Diagnoses,
    #[display(fmt = "fab fa-diaspora")]
    Diaspora,
    #[display(fmt = "fas fa-dice")]
    Dice,
    #[display(fmt = "fas fa-dice-d20")]
    DiceD20,
    #[display(fmt = "fas fa-dice-d6")]
    DiceD6,
    #[display(fmt = "fas fa-dice-five")]
    DiceFive,
    #[display(fmt = "fas fa-dice-four")]
    DiceFour,
    #[display(fmt = "fas fa-dice-one")]
    DiceOne,
    #[display(fmt = "fas fa-dice-six")]
    DiceSix,
    #[display(fmt = "fas fa-dice-three")]
    DiceThree,
    #[display(fmt = "fas fa-dice-two")]
    DiceTwo,
    #[display(fmt = "fab fa-digg")]
    Digg,
    #[display(fmt = "fab fa-digital-ocean")]
    DigitalOcean,
    #[display(fmt = "fas fa-digital-tachograph")]
    DigitalTachograph,
    #[display(fmt = "fas fa-directions")]
    Directions,
    #[display(fmt = "fab fa-discord")]
    Discord,
    #[display(fmt = "fab fa-discourse")]
    Discourse,
    #[display(fmt = "fas fa-disease")]
    Disease,
    #[display(fmt = "fas fa-divide")]
    Divide,
    #[display(fmt = "fas fa-dizzy")]
    DizzySolid,
    #[display(fmt = "far fa-dizzy")]
    DizzyRegular,
    #[display(fmt = "fas fa-dna")]
    Dna,
    #[display(fmt = "fab fa-dochub")]
    Dochub,
    #[display(fmt = "fab fa-docker")]
    Docker,
    #[display(fmt = "fas fa-dog")]
    Dog,
    #[display(fmt = "fas fa-dollar-sign")]
    DollarSign,
    #[display(fmt = "fas fa-dolly")]
    Dolly,
    #[display(fmt = "fas fa-dolly-flatbed")]
    DollyFlatbed,
    #[display(fmt = "fas fa-donate")]
    Donate,
    #[display(fmt = "fas fa-door-closed")]
    DoorClosed,
    #[display(fmt = "fas fa-door-open")]
    DoorOpen,
    #[display(fmt = "fas fa-dot-circle")]
    DotCircleSolid,
    #[display(fmt = "far fa-dot-circle")]
    DotCircleRegular,
    #[display(fmt = "fas fa-dove")]
    Dove,
    #[display(fmt = "fas fa-download")]
    Download,
    #[display(fmt = "fab fa-draft2digital")]
    Draft2Digital,
    #[display(fmt = "fas fa-drafting-compass")]
    DraftingCompass,
    #[display(fmt = "fas fa-dragon")]
    Dragon,
    #[display(fmt = "fas fa-draw-polygon")]
    DrawPolygon,
    #[display(fmt = "fab fa-dribbble")]
    Dribbble,
    #[display(fmt = "fab fa-dribbble-square")]
    DribbbleSquare,
    #[display(fmt = "fab fa-dropbox")]
    Dropbox,
    #[display(fmt = "fas fa-drum")]
    Drum,
    #[display(fmt = "fas fa-drum-steelpan")]
    DrumSteelpan,
    #[display(fmt = "fas fa-drumstick-bite")]
    DrumstickBite,
    #[display(fmt = "fab fa-drupal")]
    Drupal,
    #[display(fmt = "fas fa-dumbbell")]
    Dumbbell,
    #[display(fmt = "fas fa-dumpster")]
    Dumpster,
    #[display(fmt = "fas fa-dumpster-fire")]
    DumpsterFire,
    #[display(fmt = "fas fa-dungeon")]
    Dungeon,
    #[display(fmt = "fab fa-dyalog")]
    Dyalog,
    #[display(fmt = "fab fa-earlybirds")]
    Earlybirds,
    #[display(fmt = "fab fa-ebay")]
    Ebay,
    #[display(fmt = "fab fa-edge")]
    Edge,
    #[display(fmt = "fab fa-edge-legacy")]
    EdgeLegacy,
    #[display(fmt = "fas fa-edit")]
    EditSolid,
    #[display(fmt = "far fa-edit")]
    EditRegular,
    #[display(fmt = "fas fa-egg")]
    Egg,
    #[display(fmt = "fas fa-eject")]
    Eject,
    #[display(fmt = "fab fa-elementor")]
    Elementor,
    #[display(fmt = "fas fa-ellipsis-h")]
    EllipsisH,
    #[display(fmt = "fas fa-ellipsis-v")]
    EllipsisV,
    #[display(fmt = "fab fa-ello")]
    Ello,
    #[display(fmt = "fab fa-ember")]
    Ember,
    #[display(fmt = "fab fa-empire")]
    Empire,
    #[display(fmt = "fas fa-envelope")]
    EnvelopeSolid,
    #[display(fmt = "far fa-envelope")]
    EnvelopeRegular,
    #[display(fmt = "fas fa-envelope-open")]
    EnvelopeOpenSolid,
    #[display(fmt = "far fa-envelope-open")]
    EnvelopeOpenRegular,
    #[display(fmt = "fas fa-envelope-open-text")]
    EnvelopeOpenText,
    #[display(fmt = "fas fa-envelope-square")]
    EnvelopeSquare,
    #[display(fmt = "fab fa-envira")]
    Envira,
    #[display(fmt = "fas fa-equals")]
    Equals,
    #[display(fmt = "fas fa-eraser")]
    Eraser,
    #[display(fmt = "fab fa-erlang")]
    Erlang,
    #[display(fmt = "fab fa-ethereum")]
    Ethereum,
    #[display(fmt = "fas fa-ethernet")]
    Ethernet,
    #[display(fmt = "fab fa-etsy")]
    Etsy,
    #[display(fmt = "fas fa-euro-sign")]
    EuroSign,
    #[display(fmt = "fab fa-evernote")]
    Evernote,
    #[display(fmt = "fas fa-exchange-alt")]
    ExchangeAlt,
    #[display(fmt = "fas fa-exclamation")]
    Exclamation,
    #[display(fmt = "fas fa-exclamation-circle")]
    ExclamationCircle,
    #[display(fmt = "fas fa-exclamation-triangle")]
    ExclamationTriangle,
    #[display(fmt = "fas fa-expand")]
    Expand,
    #[display(fmt = "fas fa-expand-alt")]
    ExpandAlt,
    #[display(fmt = "fas fa-expand-arrows-alt")]
    ExpandArrowsAlt,
    #[display(fmt = "fab fa-expeditedssl")]
    Expeditedssl,
    #[display(fmt = "fas fa-external-link-alt")]
    ExternalLinkAlt,
    #[display(fmt = "fas fa-external-link-square-alt")]
    ExternalLinkSquareAlt,
    #[display(fmt = "fas fa-eye")]
    EyeSolid,
    #[display(fmt = "far fa-eye")]
    EyeRegular,
    #[display(fmt = "fas fa-eye-dropper")]
    EyeDropper,
    #[display(fmt = "fas fa-eye-slash")]
    EyeSlashSolid,
    #[display(fmt = "far fa-eye-slash")]
    EyeSlashRegular,
    #[display(fmt = "fab fa-facebook")]
    Facebook,
    #[display(fmt = "fab fa-facebook-f")]
    FacebookF,
    #[display(fmt = "fab fa-facebook-messenger")]
    FacebookMessenger,
    #[display(fmt = "fab fa-facebook-square")]
    FacebookSquare,
    #[display(fmt = "fas fa-fan")]
    Fan,
    #[display(fmt = "fab fa-fantasy-flight-games")]
    FantasyFlightGames,
    #[display(fmt = "fas fa-fast-backward")]
    FastBackward,
    #[display(fmt = "fas fa-fast-forward")]
    FastForward,
    #[display(fmt = "fas fa-faucet")]
    Faucet,
    #[display(fmt = "fas fa-fax")]
    Fax,
    #[display(fmt = "fas fa-feather")]
    Feather,
    #[display(fmt = "fas fa-feather-alt")]
    FeatherAlt,
    #[display(fmt = "fab fa-fedex")]
    Fedex,
    #[display(fmt = "fab fa-fedora")]
    Fedora,
    #[display(fmt = "fas fa-female")]
    Female,
    #[display(fmt = "fas fa-fighter-jet")]
    FighterJet,
    #[display(fmt = "fab fa-figma")]
    Figma,
    #[display(fmt = "fas fa-file")]
    FileSolid,
    #[display(fmt = "far fa-file")]
    FileRegular,
    #[display(fmt = "fas fa-file-alt")]
    FileAltSolid,
    #[display(fmt = "far fa-file-alt")]
    FileAltRegular,
    #[display(fmt = "fas fa-file-archive")]
    FileArchiveSolid,
    #[display(fmt = "far fa-file-archive")]
    FileArchiveRegular,
    #[display(fmt = "fas fa-file-audio")]
    FileAudioSolid,
    #[display(fmt = "far fa-file-audio")]
    FileAudioRegular,
    #[display(fmt = "fas fa-file-code")]
    FileCodeSolid,
    #[display(fmt = "far fa-file-code")]
    FileCodeRegular,
    #[display(fmt = "fas fa-file-contract")]
    FileContract,
    #[display(fmt = "fas fa-file-csv")]
    FileCsv,
    #[display(fmt = "fas fa-file-download")]
    FileDownload,
    #[display(fmt = "fas fa-file-excel")]
    FileExcelSolid,
    #[display(fmt = "far fa-file-excel")]
    FileExcelRegular,
    #[display(fmt = "fas fa-file-export")]
    FileExport,
    #[display(fmt = "fas fa-file-image")]
    FileImageSolid,
    #[display(fmt = "far fa-file-image")]
    FileImageRegular,
    #[display(fmt = "fas fa-file-import")]
    FileImport,
    #[display(fmt = "fas fa-file-invoice")]
    FileInvoice,
    #[display(fmt = "fas fa-file-invoice-dollar")]
    FileInvoiceDollar,
    #[display(fmt = "fas fa-file-medical")]
    FileMedical,
    #[display(fmt = "fas fa-file-medical-alt")]
    FileMedicalAlt,
    #[display(fmt = "fas fa-file-pdf")]
    FilePdfSolid,
    #[display(fmt = "far fa-file-pdf")]
    FilePdfRegular,
    #[display(fmt = "fas fa-file-powerpoint")]
    FilePowerpointSolid,
    #[display(fmt = "far fa-file-powerpoint")]
    FilePowerpointRegular,
    #[display(fmt = "fas fa-file-prescription")]
    FilePrescription,
    #[display(fmt = "fas fa-file-signature")]
    FileSignature,
    #[display(fmt = "fas fa-file-upload")]
    FileUpload,
    #[display(fmt = "fas fa-file-video")]
    FileVideoSolid,
    #[display(fmt = "far fa-file-video")]
    FileVideoRegular,
    #[display(fmt = "fas fa-file-word")]
    FileWordSolid,
    #[display(fmt = "far fa-file-word")]
    FileWordRegular,
    #[display(fmt = "fas fa-fill")]
    Fill,
    #[display(fmt = "fas fa-fill-drip")]
    FillDrip,
    #[display(fmt = "fas fa-film")]
    Film,
    #[display(fmt = "fas fa-filter")]
    Filter,
    #[display(fmt = "fas fa-fingerprint")]
    Fingerprint,
    #[display(fmt = "fas fa-fire")]
    Fire,
    #[display(fmt = "fas fa-fire-alt")]
    FireAlt,
    #[display(fmt = "fas fa-fire-extinguisher")]
    FireExtinguisher,
    #[display(fmt = "fab fa-firefox")]
    Firefox,
    #[display(fmt = "fab fa-firefox-browser")]
    FirefoxBrowser,
    #[display(fmt = "fas fa-first-aid")]
    FirstAid,
    #[display(fmt = "fab fa-first-order")]
    FirstOrder,
    #[display(fmt = "fab fa-first-order-alt")]
    FirstOrderAlt,
    #[display(fmt = "fab fa-firstdraft")]
    Firstdraft,
    #[display(fmt = "fas fa-fish")]
    Fish,
    #[display(fmt = "fas fa-fist-raised")]
    FistRaised,
    #[display(fmt = "fas fa-flag")]
    FlagSolid,
    #[display(fmt = "far fa-flag")]
    FlagRegular,
    #[display(fmt = "fas fa-flag-checkered")]
    FlagCheckered,
    #[display(fmt = "fas fa-flag-usa")]
    FlagUsa,
    #[display(fmt = "fas fa-flask")]
    Flask,
    #[display(fmt = "fab fa-flickr")]
    Flickr,
    #[display(fmt = "fab fa-flipboard")]
    Flipboard,
    #[display(fmt = "fas fa-flushed")]
    FlushedSolid,
    #[display(fmt = "far fa-flushed")]
    FlushedRegular,
    #[display(fmt = "fab fa-fly")]
    Fly,
    #[display(fmt = "fas fa-folder")]
    FolderSolid,
    #[display(fmt = "far fa-folder")]
    FolderRegular,
    #[display(fmt = "fas fa-folder-minus")]
    FolderMinus,
    #[display(fmt = "fas fa-folder-open")]
    FolderOpenSolid,
    #[display(fmt = "far fa-folder-open")]
    FolderOpenRegular,
    #[display(fmt = "fas fa-folder-plus")]
    FolderPlus,
    #[display(fmt = "fas fa-font")]
    Font,
    #[display(fmt = "fab fa-font-awesome")]
    FontAwesome,
    #[display(fmt = "fab fa-font-awesome-alt")]
    FontAwesomeAlt,
    #[display(fmt = "fab fa-font-awesome-flag")]
    FontAwesomeFlag,
    #[display(fmt = "far fa-font-awesome-logo-full")]
    FontAwesomeLogoFullRegular,
    #[display(fmt = "fas fa-font-awesome-logo-full")]
    FontAwesomeLogoFullSolid,
    #[display(fmt = "fab fa-font-awesome-logo-full")]
    FontAwesomeLogoFullBrand,
    #[display(fmt = "fab fa-fonticons")]
    Fonticons,
    #[display(fmt = "fab fa-fonticons-fi")]
    FonticonsFi,
    #[display(fmt = "fas fa-football-ball")]
    FootballBall,
    #[display(fmt = "fab fa-fort-awesome")]
    FortAwesome,
    #[display(fmt = "fab fa-fort-awesome-alt")]
    FortAwesomeAlt,
    #[display(fmt = "fab fa-forumbee")]
    Forumbee,
    #[display(fmt = "fas fa-forward")]
    Forward,
    #[display(fmt = "fab fa-foursquare")]
    Foursquare,
    #[display(fmt = "fab fa-free-code-camp")]
    FreeCodeCamp,
    #[display(fmt = "fab fa-freebsd")]
    Freebsd,
    #[display(fmt = "fas fa-frog")]
    Frog,
    #[display(fmt = "fas fa-frown")]
    FrownSolid,
    #[display(fmt = "far fa-frown")]
    FrownRegular,
    #[display(fmt = "fas fa-frown-open")]
    FrownOpenSolid,
    #[display(fmt = "far fa-frown-open")]
    FrownOpenRegular,
    #[display(fmt = "fab fa-fulcrum")]
    Fulcrum,
    #[display(fmt = "fas fa-funnel-dollar")]
    FunnelDollar,
    #[display(fmt = "fas fa-futbol")]
    FutbolSolid,
    #[display(fmt = "far fa-futbol")]
    FutbolRegular,
    #[display(fmt = "fab fa-galactic-republic")]
    GalacticRepublic,
    #[display(fmt = "fab fa-galactic-senate")]
    GalacticSenate,
    #[display(fmt = "fas fa-gamepad")]
    Gamepad,
    #[display(fmt = "fas fa-gas-pump")]
    GasPump,
    #[display(fmt = "fas fa-gavel")]
    Gavel,
    #[display(fmt = "fas fa-gem")]
    GemSolid,
    #[display(fmt = "far fa-gem")]
    GemRegular,
    #[display(fmt = "fas fa-genderless")]
    Genderless,
    #[display(fmt = "fab fa-get-pocket")]
    GetPocket,
    #[display(fmt = "fab fa-gg")]
    Gg,
    #[display(fmt = "fab fa-gg-circle")]
    GgCircle,
    #[display(fmt = "fas fa-ghost")]
    Ghost,
    #[display(fmt = "fas fa-gift")]
    Gift,
    #[display(fmt = "fas fa-gifts")]
    Gifts,
    #[display(fmt = "fab fa-git")]
    Git,
    #[display(fmt = "fab fa-git-alt")]
    GitAlt,
    #[display(fmt = "fab fa-git-square")]
    GitSquare,
    #[display(fmt = "fab fa-github")]
    Github,
    #[display(fmt = "fab fa-github-alt")]
    GithubAlt,
    #[display(fmt = "fab fa-github-square")]
    GithubSquare,
    #[display(fmt = "fab fa-gitkraken")]
    Gitkraken,
    #[display(fmt = "fab fa-gitlab")]
    Gitlab,
    #[display(fmt = "fab fa-gitter")]
    Gitter,
    #[display(fmt = "fas fa-glass-cheers")]
    GlassCheers,
    #[display(fmt = "fas fa-glass-martini")]
    GlassMartini,
    #[display(fmt = "fas fa-glass-martini-alt")]
    GlassMartiniAlt,
    #[display(fmt = "fas fa-glass-whiskey")]
    GlassWhiskey,
    #[display(fmt = "fas fa-glasses")]
    Glasses,
    #[display(fmt = "fab fa-glide")]
    Glide,
    #[display(fmt = "fab fa-glide-g")]
    GlideG,
    #[display(fmt = "fas fa-globe")]
    Globe,
    #[display(fmt = "fas fa-globe-africa")]
    GlobeAfrica,
    #[display(fmt = "fas fa-globe-americas")]
    GlobeAmericas,
    #[display(fmt = "fas fa-globe-asia")]
    GlobeAsia,
    #[display(fmt = "fas fa-globe-europe")]
    GlobeEurope,
    #[display(fmt = "fab fa-gofore")]
    Gofore,
    #[display(fmt = "fas fa-golf-ball")]
    GolfBall,
    #[display(fmt = "fab fa-goodreads")]
    Goodreads,
    #[display(fmt = "fab fa-goodreads-g")]
    GoodreadsG,
    #[display(fmt = "fab fa-google")]
    Google,
    #[display(fmt = "fab fa-google-drive")]
    GoogleDrive,
    #[display(fmt = "fab fa-google-pay")]
    GooglePay,
    #[display(fmt = "fab fa-google-play")]
    GooglePlay,
    #[display(fmt = "fab fa-google-plus")]
    GooglePlus,
    #[display(fmt = "fab fa-google-plus-g")]
    GooglePlusG,
    #[display(fmt = "fab fa-google-plus-square")]
    GooglePlusSquare,
    #[display(fmt = "fab fa-google-wallet")]
    GoogleWallet,
    #[display(fmt = "fas fa-gopuram")]
    Gopuram,
    #[display(fmt = "fas fa-graduation-cap")]
    GraduationCap,
    #[display(fmt = "fab fa-gratipay")]
    Gratipay,
    #[display(fmt = "fab fa-grav")]
    Grav,
    #[display(fmt = "fas fa-greater-than")]
    GreaterThan,
    #[display(fmt = "fas fa-greater-than-equal")]
    GreaterThanEqual,
    #[display(fmt = "fas fa-grimace")]
    GrimaceSolid,
    #[display(fmt = "far fa-grimace")]
    GrimaceRegular,
    #[display(fmt = "fas fa-grin")]
    GrinSolid,
    #[display(fmt = "far fa-grin")]
    GrinRegular,
    #[display(fmt = "fas fa-grin-alt")]
    GrinAltSolid,
    #[display(fmt = "far fa-grin-alt")]
    GrinAltRegular,
    #[display(fmt = "fas fa-grin-beam")]
    GrinBeamSolid,
    #[display(fmt = "far fa-grin-beam")]
    GrinBeamRegular,
    #[display(fmt = "fas fa-grin-beam-sweat")]
    GrinBeamSweatSolid,
    #[display(fmt = "far fa-grin-beam-sweat")]
    GrinBeamSweatRegular,
    #[display(fmt = "fas fa-grin-hearts")]
    GrinHeartsSolid,
    #[display(fmt = "far fa-grin-hearts")]
    GrinHeartsRegular,
    #[display(fmt = "fas fa-grin-squint")]
    GrinSquintSolid,
    #[display(fmt = "far fa-grin-squint")]
    GrinSquintRegular,
    #[display(fmt = "fas fa-grin-squint-tears")]
    GrinSquintTearsSolid,
    #[display(fmt = "far fa-grin-squint-tears")]
    GrinSquintTearsRegular,
    #[display(fmt = "fas fa-grin-stars")]
    GrinStarsSolid,
    #[display(fmt = "far fa-grin-stars")]
    GrinStarsRegular,
    #[display(fmt = "fas fa-grin-tears")]
    GrinTearsSolid,
    #[display(fmt = "far fa-grin-tears")]
    GrinTearsRegular,
    #[display(fmt = "fas fa-grin-tongue")]
    GrinTongueSolid,
    #[display(fmt = "far fa-grin-tongue")]
    GrinTongueRegular,
    #[display(fmt = "fas fa-grin-tongue-squint")]
    GrinTongueSquintSolid,
    #[display(fmt = "far fa-grin-tongue-squint")]
    GrinTongueSquintRegular,
    #[display(fmt = "fas fa-grin-tongue-wink")]
    GrinTongueWinkSolid,
    #[display(fmt = "far fa-grin-tongue-wink")]
    GrinTongueWinkRegular,
    #[display(fmt = "fas fa-grin-wink")]
    GrinWinkSolid,
    #[display(fmt = "far fa-grin-wink")]
    GrinWinkRegular,
    #[display(fmt = "fas fa-grip-horizontal")]
    GripHorizontal,
    #[display(fmt = "fas fa-grip-lines")]
    GripLines,
    #[display(fmt = "fas fa-grip-lines-vertical")]
    GripLinesVertical,
    #[display(fmt = "fas fa-grip-vertical")]
    GripVertical,
    #[display(fmt = "fab fa-gripfire")]
    Gripfire,
    #[display(fmt = "fab fa-grunt")]
    Grunt,
    #[display(fmt = "fab fa-guilded")]
    Guilded,
    #[display(fmt = "fas fa-guitar")]
    Guitar,
    #[display(fmt = "fab fa-gulp")]
    Gulp,
    #[display(fmt = "fas fa-h-square")]
    HSquare,
    #[display(fmt = "fab fa-hacker-news")]
    HackerNews,
    #[display(fmt = "fab fa-hacker-news-square")]
    HackerNewsSquare,
    #[display(fmt = "fab fa-hackerrank")]
    Hackerrank,
    #[display(fmt = "fas fa-hamburger")]
    Hamburger,
    #[display(fmt = "fas fa-hammer")]
    Hammer,
    #[display(fmt = "fas fa-hamsa")]
    Hamsa,
    #[display(fmt = "fas fa-hand-holding")]
    HandHolding,
    #[display(fmt = "fas fa-hand-holding-heart")]
    HandHoldingHeart,
    #[display(fmt = "fas fa-hand-holding-medical")]
    HandHoldingMedical,
    #[display(fmt = "fas fa-hand-holding-usd")]
    HandHoldingUsd,
    #[display(fmt = "fas fa-hand-holding-water")]
    HandHoldingWater,
    #[display(fmt = "fas fa-hand-lizard")]
    HandLizardSolid,
    #[display(fmt = "far fa-hand-lizard")]
    HandLizardRegular,
    #[display(fmt = "fas fa-hand-middle-finger")]
    HandMiddleFinger,
    #[display(fmt = "fas fa-hand-paper")]
    HandPaperSolid,
    #[display(fmt = "far fa-hand-paper")]
    HandPaperRegular,
    #[display(fmt = "fas fa-hand-peace")]
    HandPeaceSolid,
    #[display(fmt = "far fa-hand-peace")]
    HandPeaceRegular,
    #[display(fmt = "fas fa-hand-point-down")]
    HandPointDownSolid,
    #[display(fmt = "far fa-hand-point-down")]
    HandPointDownRegular,
    #[display(fmt = "fas fa-hand-point-left")]
    HandPointLeftSolid,
    #[display(fmt = "far fa-hand-point-left")]
    HandPointLeftRegular,
    #[display(fmt = "fas fa-hand-point-right")]
    HandPointRightSolid,
    #[display(fmt = "far fa-hand-point-right")]
    HandPointRightRegular,
    #[display(fmt = "fas fa-hand-point-up")]
    HandPointUpSolid,
    #[display(fmt = "far fa-hand-point-up")]
    HandPointUpRegular,
    #[display(fmt = "fas fa-hand-pointer")]
    HandPointerSolid,
    #[display(fmt = "far fa-hand-pointer")]
    HandPointerRegular,
    #[display(fmt = "fas fa-hand-rock")]
    HandRockSolid,
    #[display(fmt = "far fa-hand-rock")]
    HandRockRegular,
    #[display(fmt = "fas fa-hand-scissors")]
    HandScissorsSolid,
    #[display(fmt = "far fa-hand-scissors")]
    HandScissorsRegular,
    #[display(fmt = "fas fa-hand-sparkles")]
    HandSparkles,
    #[display(fmt = "fas fa-hand-spock")]
    HandSpockSolid,
    #[display(fmt = "far fa-hand-spock")]
    HandSpockRegular,
    #[display(fmt = "fas fa-hands")]
    Hands,
    #[display(fmt = "fas fa-hands-helping")]
    HandsHelping,
    #[display(fmt = "fas fa-hands-wash")]
    HandsWash,
    #[display(fmt = "fas fa-handshake")]
    HandshakeSolid,
    #[display(fmt = "far fa-handshake")]
    HandshakeRegular,
    #[display(fmt = "fas fa-handshake-alt-slash")]
    HandshakeAltSlash,
    #[display(fmt = "fas fa-handshake-slash")]
    HandshakeSlash,
    #[display(fmt = "fas fa-hanukiah")]
    Hanukiah,
    #[display(fmt = "fas fa-hard-hat")]
    HardHat,
    #[display(fmt = "fas fa-hashtag")]
    Hashtag,
    #[display(fmt = "fas fa-hat-cowboy")]
    HatCowboy,
    #[display(fmt = "fas fa-hat-cowboy-side")]
    HatCowboySide,
    #[display(fmt = "fas fa-hat-wizard")]
    HatWizard,
    #[display(fmt = "fas fa-hdd")]
    HddSolid,
    #[display(fmt = "far fa-hdd")]
    HddRegular,
    #[display(fmt = "fas fa-head-side-cough")]
    HeadSideCough,
    #[display(fmt = "fas fa-head-side-cough-slash")]
    HeadSideCoughSlash,
    #[display(fmt = "fas fa-head-side-mask")]
    HeadSideMask,
    #[display(fmt = "fas fa-head-side-virus")]
    HeadSideVirus,
    #[display(fmt = "fas fa-heading")]
    Heading,
    #[display(fmt = "fas fa-headphones")]
    Headphones,
    #[display(fmt = "fas fa-headphones-alt")]
    HeadphonesAlt,
    #[display(fmt = "fas fa-headset")]
    Headset,
    #[display(fmt = "fas fa-heart")]
    HeartSolid,
    #[display(fmt = "far fa-heart")]
    HeartRegular,
    #[display(fmt = "fas fa-heart-broken")]
    HeartBroken,
    #[display(fmt = "fas fa-heartbeat")]
    Heartbeat,
    #[display(fmt = "fas fa-helicopter")]
    Helicopter,
    #[display(fmt = "fas fa-highlighter")]
    Highlighter,
    #[display(fmt = "fas fa-hiking")]
    Hiking,
    #[display(fmt = "fas fa-hippo")]
    Hippo,
    #[display(fmt = "fab fa-hips")]
    Hips,
    #[display(fmt = "fab fa-hire-a-helper")]
    HireAHelper,
    #[display(fmt = "fas fa-history")]
    History,
    #[display(fmt = "fab fa-hive")]
    Hive,
    #[display(fmt = "fas fa-hockey-puck")]
    HockeyPuck,
    #[display(fmt = "fas fa-holly-berry")]
    HollyBerry,
    #[display(fmt = "fas fa-home")]
    Home,
    #[display(fmt = "fab fa-hooli")]
    Hooli,
    #[display(fmt = "fab fa-hornbill")]
    Hornbill,
    #[display(fmt = "fas fa-horse")]
    Horse,
    #[display(fmt = "fas fa-horse-head")]
    HorseHead,
    #[display(fmt = "fas fa-hospital")]
    HospitalSolid,
    #[display(fmt = "far fa-hospital")]
    HospitalRegular,
    #[display(fmt = "fas fa-hospital-alt")]
    HospitalAlt,
    #[display(fmt = "fas fa-hospital-symbol")]
    HospitalSymbol,
    #[display(fmt = "fas fa-hospital-user")]
    HospitalUser,
    #[display(fmt = "fas fa-hot-tub")]
    HotTub,
    #[display(fmt = "fas fa-hotdog")]
    Hotdog,
    #[display(fmt = "fas fa-hotel")]
    Hotel,
    #[display(fmt = "fab fa-hotjar")]
    Hotjar,
    #[display(fmt = "fas fa-hourglass")]
    HourglassSolid,
    #[display(fmt = "far fa-hourglass")]
    HourglassRegular,
    #[display(fmt = "fas fa-hourglass-end")]
    HourglassEnd,
    #[display(fmt = "fas fa-hourglass-half")]
    HourglassHalf,
    #[display(fmt = "fas fa-hourglass-start")]
    HourglassStart,
    #[display(fmt = "fas fa-house-damage")]
    HouseDamage,
    #[display(fmt = "fas fa-house-user")]
    HouseUser,
    #[display(fmt = "fab fa-houzz")]
    Houzz,
    #[display(fmt = "fas fa-hryvnia")]
    Hryvnia,
    #[display(fmt = "fab fa-html5")]
    Html5,
    #[display(fmt = "fab fa-hubspot")]
    Hubspot,
    #[display(fmt = "fas fa-i-cursor")]
    ICursor,
    #[display(fmt = "fas fa-ice-cream")]
    IceCream,
    #[display(fmt = "fas fa-icicles")]
    Icicles,
    #[display(fmt = "fas fa-icons")]
    Icons,
    #[display(fmt = "fas fa-id-badge")]
    IdBadgeSolid,
    #[display(fmt = "far fa-id-badge")]
    IdBadgeRegular,
    #[display(fmt = "fas fa-id-card")]
    IdCardSolid,
    #[display(fmt = "far fa-id-card")]
    IdCardRegular,
    #[display(fmt = "fas fa-id-card-alt")]
    IdCardAlt,
    #[display(fmt = "fab fa-ideal")]
    Ideal,
    #[display(fmt = "fas fa-igloo")]
    Igloo,
    #[display(fmt = "fas fa-image")]
    ImageSolid,
    #[display(fmt = "far fa-image")]
    ImageRegular,
    #[display(fmt = "fas fa-images")]
    ImagesSolid,
    #[display(fmt = "far fa-images")]
    ImagesRegular,
    #[display(fmt = "fab fa-imdb")]
    Imdb,
    #[display(fmt = "fas fa-inbox")]
    Inbox,
    #[display(fmt = "fas fa-indent")]
    Indent,
    #[display(fmt = "fas fa-industry")]
    Industry,
    #[display(fmt = "fas fa-infinity")]
    Infinity,
    #[display(fmt = "fas fa-info")]
    Info,
    #[display(fmt = "fas fa-info-circle")]
    InfoCircle,
    #[display(fmt = "fab fa-innosoft")]
    Innosoft,
    #[display(fmt = "fab fa-instagram")]
    Instagram,
    #[display(fmt = "fab fa-instagram-square")]
    InstagramSquare,
    #[display(fmt = "fab fa-instalod")]
    Instalod,
    #[display(fmt = "fab fa-intercom")]
    Intercom,
    #[display(fmt = "fab fa-internet-explorer")]
    InternetExplorer,
    #[display(fmt = "fab fa-invision")]
    Invision,
    #[display(fmt = "fab fa-ioxhost")]
    Ioxhost,
    #[display(fmt = "fas fa-italic")]
    Italic,
    #[display(fmt = "fab fa-itch-io")]
    ItchIo,
    #[display(fmt = "fab fa-itunes")]
    Itunes,
    #[display(fmt = "fab fa-itunes-note")]
    ItunesNote,
    #[display(fmt = "fab fa-java")]
    Java,
    #[display(fmt = "fas fa-jedi")]
    Jedi,
    #[display(fmt = "fab fa-jedi-order")]
    JediOrder,
    #[display(fmt = "fab fa-jenkins")]
    Jenkins,
    #[display(fmt = "fab fa-jira")]
    Jira,
    #[display(fmt = "fab fa-joget")]
    Joget,
    #[display(fmt = "fas fa-joint")]
    Joint,
    #[display(fmt = "fab fa-joomla")]
    Joomla,
    #[display(fmt = "fas fa-journal-whills")]
    JournalWhills,
    #[display(fmt = "fab fa-js")]
    Js,
    #[display(fmt = "fab fa-js-square")]
    JsSquare,
    #[display(fmt = "fab fa-jsfiddle")]
    Jsfiddle,
    #[display(fmt = "fas fa-kaaba")]
    Kaaba,
    #[display(fmt = "fab fa-kaggle")]
    Kaggle,
    #[display(fmt = "fas fa-key")]
    Key,
    #[display(fmt = "fab fa-keybase")]
    Keybase,
    #[display(fmt = "fas fa-keyboard")]
    KeyboardSolid,
    #[display(fmt = "far fa-keyboard")]
    KeyboardRegular,
    #[display(fmt = "fab fa-keycdn")]
    Keycdn,
    #[display(fmt = "fas fa-khanda")]
    Khanda,
    #[display(fmt = "fab fa-kickstarter")]
    Kickstarter,
    #[display(fmt = "fab fa-kickstarter-k")]
    KickstarterK,
    #[display(fmt = "fas fa-kiss")]
    KissSolid,
    #[display(fmt = "far fa-kiss")]
    KissRegular,
    #[display(fmt = "fas fa-kiss-beam")]
    KissBeamSolid,
    #[display(fmt = "far fa-kiss-beam")]
    KissBeamRegular,
    #[display(fmt = "fas fa-kiss-wink-heart")]
    KissWinkHeartSolid,
    #[display(fmt = "far fa-kiss-wink-heart")]
    KissWinkHeartRegular,
    #[display(fmt = "fas fa-kiwi-bird")]
    KiwiBird,
    #[display(fmt = "fab fa-korvue")]
    Korvue,
    #[display(fmt = "fas fa-landmark")]
    Landmark,
    #[display(fmt = "fas fa-language")]
    Language,
    #[display(fmt = "fas fa-laptop")]
    Laptop,
    #[display(fmt = "fas fa-laptop-code")]
    LaptopCode,
    #[display(fmt = "fas fa-laptop-house")]
    LaptopHouse,
    #[display(fmt = "fas fa-laptop-medical")]
    LaptopMedical,
    #[display(fmt = "fab fa-laravel")]
    Laravel,
    #[display(fmt = "fab fa-lastfm")]
    Lastfm,
    #[display(fmt = "fab fa-lastfm-square")]
    LastfmSquare,
    #[display(fmt = "fas fa-laugh")]
    LaughSolid,
    #[display(fmt = "far fa-laugh")]
    LaughRegular,
    #[display(fmt = "fas fa-laugh-beam")]
    LaughBeamSolid,
    #[display(fmt = "far fa-laugh-beam")]
    LaughBeamRegular,
    #[display(fmt = "fas fa-laugh-squint")]
    LaughSquintSolid,
    #[display(fmt = "far fa-laugh-squint")]
    LaughSquintRegular,
    #[display(fmt = "fas fa-laugh-wink")]
    LaughWinkSolid,
    #[display(fmt = "far fa-laugh-wink")]
    LaughWinkRegular,
    #[display(fmt = "fas fa-layer-group")]
    LayerGroup,
    #[display(fmt = "fas fa-leaf")]
    Leaf,
    #[display(fmt = "fab fa-leanpub")]
    Leanpub,
    #[display(fmt = "fas fa-lemon")]
    LemonSolid,
    #[display(fmt = "far fa-lemon")]
    LemonRegular,
    #[display(fmt = "fab fa-less")]
    Less,
    #[display(fmt = "fas fa-less-than")]
    LessThan,
    #[display(fmt = "fas fa-less-than-equal")]
    LessThanEqual,
    #[display(fmt = "fas fa-level-down-alt")]
    LevelDownAlt,
    #[display(fmt = "fas fa-level-up-alt")]
    LevelUpAlt,
    #[display(fmt = "fas fa-life-ring")]
    LifeRingSolid,
    #[display(fmt = "far fa-life-ring")]
    LifeRingRegular,
    #[display(fmt = "fas fa-lightbulb")]
    LightbulbSolid,
    #[display(fmt = "far fa-lightbulb")]
    LightbulbRegular,
    #[display(fmt = "fab fa-line")]
    Line,
    #[display(fmt = "fas fa-link")]
    Link,
    #[display(fmt = "fab fa-linkedin")]
    Linkedin,
    #[display(fmt = "fab fa-linkedin-in")]
    LinkedinIn,
    #[display(fmt = "fab fa-linode")]
    Linode,
    #[display(fmt = "fab fa-linux")]
    Linux,
    #[display(fmt = "fas fa-lira-sign")]
    LiraSign,
    #[display(fmt = "fas fa-list")]
    List,
    #[display(fmt = "fas fa-list-alt")]
    ListAltSolid,
    #[display(fmt = "far fa-list-alt")]
    ListAltRegular,
    #[display(fmt = "fas fa-list-ol")]
    ListOl,
    #[display(fmt = "fas fa-list-ul")]
    ListUl,
    #[display(fmt = "fas fa-location-arrow")]
    LocationArrow,
    #[display(fmt = "fas fa-lock")]
    Lock,
    #[display(fmt = "fas fa-lock-open")]
    LockOpen,
    #[display(fmt = "fas fa-long-arrow-alt-down")]
    LongArrowAltDown,
    #[display(fmt = "fas fa-long-arrow-alt-left")]
    LongArrowAltLeft,
    #[display(fmt = "fas fa-long-arrow-alt-right")]
    LongArrowAltRight,
    #[display(fmt = "fas fa-long-arrow-alt-up")]
    LongArrowAltUp,
    #[display(fmt = "fas fa-low-vision")]
    LowVision,
    #[display(fmt = "fas fa-luggage-cart")]
    LuggageCart,
    #[display(fmt = "fas fa-lungs")]
    Lungs,
    #[display(fmt = "fas fa-lungs-virus")]
    LungsVirus,
    #[display(fmt = "fab fa-lyft")]
    Lyft,
    #[display(fmt = "fab fa-magento")]
    Magento,
    #[display(fmt = "fas fa-magic")]
    Magic,
    #[display(fmt = "fas fa-magnet")]
    Magnet,
    #[display(fmt = "fas fa-mail-bulk")]
    MailBulk,
    #[display(fmt = "fab fa-mailchimp")]
    Mailchimp,
    #[display(fmt = "fas fa-male")]
    Male,
    #[display(fmt = "fab fa-mandalorian")]
    Mandalorian,
    #[display(fmt = "fas fa-map")]
    MapSolid,
    #[display(fmt = "far fa-map")]
    MapRegular,
    #[display(fmt = "fas fa-map-marked")]
    MapMarked,
    #[display(fmt = "fas fa-map-marked-alt")]
    MapMarkedAlt,
    #[display(fmt = "fas fa-map-marker")]
    MapMarker,
    #[display(fmt = "fas fa-map-marker-alt")]
    MapMarkerAlt,
    #[display(fmt = "fas fa-map-pin")]
    MapPin,
    #[display(fmt = "fas fa-map-signs")]
    MapSigns,
    #[display(fmt = "fab fa-markdown")]
    Markdown,
    #[display(fmt = "fas fa-marker")]
    Marker,
    #[display(fmt = "fas fa-mars")]
    Mars,
    #[display(fmt = "fas fa-mars-double")]
    MarsDouble,
    #[display(fmt = "fas fa-mars-stroke")]
    MarsStroke,
    #[display(fmt = "fas fa-mars-stroke-h")]
    MarsStrokeH,
    #[display(fmt = "fas fa-mars-stroke-v")]
    MarsStrokeV,
    #[display(fmt = "fas fa-mask")]
    Mask,
    #[display(fmt = "fab fa-mastodon")]
    Mastodon,
    #[display(fmt = "fab fa-maxcdn")]
    Maxcdn,
    #[display(fmt = "fab fa-mdb")]
    Mdb,
    #[display(fmt = "fas fa-medal")]
    Medal,
    #[display(fmt = "fab fa-medapps")]
    Medapps,
    #[display(fmt = "fab fa-medium")]
    Medium,
    #[display(fmt = "fab fa-medium-m")]
    MediumM,
    #[display(fmt = "fas fa-medkit")]
    Medkit,
    #[display(fmt = "fab fa-medrt")]
    Medrt,
    #[display(fmt = "fab fa-meetup")]
    Meetup,
    #[display(fmt = "fab fa-megaport")]
    Megaport,
    #[display(fmt = "fas fa-meh")]
    MehSolid,
    #[display(fmt = "far fa-meh")]
    MehRegular,
    #[display(fmt = "fas fa-meh-blank")]
    MehBlankSolid,
    #[display(fmt = "far fa-meh-blank")]
    MehBlankRegular,
    #[display(fmt = "fas fa-meh-rolling-eyes")]
    MehRollingEyesSolid,
    #[display(fmt = "far fa-meh-rolling-eyes")]
    MehRollingEyesRegular,
    #[display(fmt = "fas fa-memory")]
    Memory,
    #[display(fmt = "fab fa-mendeley")]
    Mendeley,
    #[display(fmt = "fas fa-menorah")]
    Menorah,
    #[display(fmt = "fas fa-mercury")]
    Mercury,
    #[display(fmt = "fas fa-meteor")]
    Meteor,
    #[display(fmt = "fab fa-microblog")]
    Microblog,
    #[display(fmt = "fas fa-microchip")]
    Microchip,
    #[display(fmt = "fas fa-microphone")]
    Microphone,
    #[display(fmt = "fas fa-microphone-alt")]
    MicrophoneAlt,
    #[display(fmt = "fas fa-microphone-alt-slash")]
    MicrophoneAltSlash,
    #[display(fmt = "fas fa-microphone-slash")]
    MicrophoneSlash,
    #[display(fmt = "fas fa-microscope")]
    Microscope,
    #[display(fmt = "fab fa-microsoft")]
    Microsoft,
    #[display(fmt = "fas fa-minus")]
    Minus,
    #[display(fmt = "fas fa-minus-circle")]
    MinusCircle,
    #[display(fmt = "fas fa-minus-square")]
    MinusSquareSolid,
    #[display(fmt = "far fa-minus-square")]
    MinusSquareRegular,
    #[display(fmt = "fas fa-mitten")]
    Mitten,
    #[display(fmt = "fab fa-mix")]
    Mix,
    #[display(fmt = "fab fa-mixcloud")]
    Mixcloud,
    #[display(fmt = "fab fa-mixer")]
    Mixer,
    #[display(fmt = "fab fa-mizuni")]
    Mizuni,
    #[display(fmt = "fas fa-mobile")]
    Mobile,
    #[display(fmt = "fas fa-mobile-alt")]
    MobileAlt,
    #[display(fmt = "fab fa-modx")]
    Modx,
    #[display(fmt = "fab fa-monero")]
    Monero,
    #[display(fmt = "fas fa-money-bill")]
    MoneyBill,
    #[display(fmt = "fas fa-money-bill-alt")]
    MoneyBillAltSolid,
    #[display(fmt = "far fa-money-bill-alt")]
    MoneyBillAltRegular,
    #[display(fmt = "fas fa-money-bill-wave")]
    MoneyBillWave,
    #[display(fmt = "fas fa-money-bill-wave-alt")]
    MoneyBillWaveAlt,
    #[display(fmt = "fas fa-money-check")]
    MoneyCheck,
    #[display(fmt = "fas fa-money-check-alt")]
    MoneyCheckAlt,
    #[display(fmt = "fas fa-monument")]
    Monument,
    #[display(fmt = "fas fa-moon")]
    MoonSolid,
    #[display(fmt = "far fa-moon")]
    MoonRegular,
    #[display(fmt = "fas fa-mortar-pestle")]
    MortarPestle,
    #[display(fmt = "fas fa-mosque")]
    Mosque,
    #[display(fmt = "fas fa-motorcycle")]
    Motorcycle,
    #[display(fmt = "fas fa-mountain")]
    Mountain,
    #[display(fmt = "fas fa-mouse")]
    Mouse,
    #[display(fmt = "fas fa-mouse-pointer")]
    MousePointer,
    #[display(fmt = "fas fa-mug-hot")]
    MugHot,
    #[display(fmt = "fas fa-music")]
    Music,
    #[display(fmt = "fab fa-napster")]
    Napster,
    #[display(fmt = "fab fa-neos")]
    Neos,
    #[display(fmt = "fas fa-network-wired")]
    NetworkWired,
    #[display(fmt = "fas fa-neuter")]
    Neuter,
    #[display(fmt = "fas fa-newspaper")]
    NewspaperSolid,
    #[display(fmt = "far fa-newspaper")]
    NewspaperRegular,
    #[display(fmt = "fab fa-nimblr")]
    Nimblr,
    #[display(fmt = "fab fa-node")]
    Node,
    #[display(fmt = "fab fa-node-js")]
    NodeJs,
    #[display(fmt = "fas fa-not-equal")]
    NotEqual,
    #[display(fmt = "fas fa-notes-medical")]
    NotesMedical,
    #[display(fmt = "fab fa-npm")]
    Npm,
    #[display(fmt = "fab fa-ns8")]
    Ns8,
    #[display(fmt = "fab fa-nutritionix")]
    Nutritionix,
    #[display(fmt = "fas fa-object-group")]
    ObjectGroupSolid,
    #[display(fmt = "far fa-object-group")]
    ObjectGroupRegular,
    #[display(fmt = "fas fa-object-ungroup")]
    ObjectUngroupSolid,
    #[display(fmt = "far fa-object-ungroup")]
    ObjectUngroupRegular,
    #[display(fmt = "fab fa-octopus-deploy")]
    OctopusDeploy,
    #[display(fmt = "fab fa-odnoklassniki")]
    Odnoklassniki,
    #[display(fmt = "fab fa-odnoklassniki-square")]
    OdnoklassnikiSquare,
    #[display(fmt = "fas fa-oil-can")]
    OilCan,
    #[display(fmt = "fab fa-old-republic")]
    OldRepublic,
    #[display(fmt = "fas fa-om")]
    Om,
    #[display(fmt = "fab fa-opencart")]
    Opencart,
    #[display(fmt = "fab fa-openid")]
    Openid,
    #[display(fmt = "fab fa-opera")]
    Opera,
    #[display(fmt = "fab fa-optin-monster")]
    OptinMonster,
    #[display(fmt = "fab fa-orcid")]
    Orcid,
    #[display(fmt = "fab fa-osi")]
    Osi,
    #[display(fmt = "fas fa-otter")]
    Otter,
    #[display(fmt = "fas fa-outdent")]
    Outdent,
    #[display(fmt = "fab fa-page4")]
    Page4,
    #[display(fmt = "fab fa-pagelines")]
    Pagelines,
    #[display(fmt = "fas fa-pager")]
    Pager,
    #[display(fmt = "fas fa-paint-brush")]
    PaintBrush,
    #[display(fmt = "fas fa-paint-roller")]
    PaintRoller,
    #[display(fmt = "fas fa-palette")]
    Palette,
    #[display(fmt = "fab fa-palfed")]
    Palfed,
    #[display(fmt = "fas fa-pallet")]
    Pallet,
    #[display(fmt = "fas fa-paper-plane")]
    PaperPlaneSolid,
    #[display(fmt = "far fa-paper-plane")]
    PaperPlaneRegular,
    #[display(fmt = "fas fa-paperclip")]
    Paperclip,
    #[display(fmt = "fas fa-parachute-box")]
    ParachuteBox,
    #[display(fmt = "fas fa-paragraph")]
    Paragraph,
    #[display(fmt = "fas fa-parking")]
    Parking,
    #[display(fmt = "fas fa-passport")]
    Passport,
    #[display(fmt = "fas fa-pastafarianism")]
    Pastafarianism,
    #[display(fmt = "fas fa-paste")]
    Paste,
    #[display(fmt = "fab fa-patreon")]
    Patreon,
    #[display(fmt = "fas fa-pause")]
    Pause,
    #[display(fmt = "fas fa-pause-circle")]
    PauseCircleSolid,
    #[display(fmt = "far fa-pause-circle")]
    PauseCircleRegular,
    #[display(fmt = "fas fa-paw")]
    Paw,
    #[display(fmt = "fab fa-paypal")]
    Paypal,
    #[display(fmt = "fas fa-peace")]
    Peace,
    #[display(fmt = "fas fa-pen")]
    Pen,
    #[display(fmt = "fas fa-pen-alt")]
    PenAlt,
    #[display(fmt = "fas fa-pen-fancy")]
    PenFancy,
    #[display(fmt = "fas fa-pen-nib")]
    PenNib,
    #[display(fmt = "fas fa-pen-square")]
    PenSquare,
    #[display(fmt = "fas fa-pencil-alt")]
    PencilAlt,
    #[display(fmt = "fas fa-pencil-ruler")]
    PencilRuler,
    #[display(fmt = "fab fa-penny-arcade")]
    PennyArcade,
    #[display(fmt = "fas fa-people-arrows")]
    PeopleArrows,
    #[display(fmt = "fas fa-people-carry")]
    PeopleCarry,
    #[display(fmt = "fas fa-pepper-hot")]
    PepperHot,
    #[display(fmt = "fab fa-perbyte")]
    Perbyte,
    #[display(fmt = "fas fa-percent")]
    Percent,
    #[display(fmt = "fas fa-percentage")]
    Percentage,
    #[display(fmt = "fab fa-periscope")]
    Periscope,
    #[display(fmt = "fas fa-person-booth")]
    PersonBooth,
    #[display(fmt = "fab fa-phabricator")]
    Phabricator,
    #[display(fmt = "fab fa-phoenix-framework")]
    PhoenixFramework,
    #[display(fmt = "fab fa-phoenix-squadron")]
    PhoenixSquadron,
    #[display(fmt = "fas fa-phone")]
    Phone,
    #[display(fmt = "fas fa-phone-alt")]
    PhoneAlt,
    #[display(fmt = "fas fa-phone-slash")]
    PhoneSlash,
    #[display(fmt = "fas fa-phone-square")]
    PhoneSquare,
    #[display(fmt = "fas fa-phone-square-alt")]
    PhoneSquareAlt,
    #[display(fmt = "fas fa-phone-volume")]
    PhoneVolume,
    #[display(fmt = "fas fa-photo-video")]
    PhotoVideo,
    #[display(fmt = "fab fa-php")]
    Php,
    #[display(fmt = "fab fa-pied-piper")]
    PiedPiper,
    #[display(fmt = "fab fa-pied-piper-alt")]
    PiedPiperAlt,
    #[display(fmt = "fab fa-pied-piper-hat")]
    PiedPiperHat,
    #[display(fmt = "fab fa-pied-piper-pp")]
    PiedPiperPp,
    #[display(fmt = "fab fa-pied-piper-square")]
    PiedPiperSquare,
    #[display(fmt = "fas fa-piggy-bank")]
    PiggyBank,
    #[display(fmt = "fas fa-pills")]
    Pills,
    #[display(fmt = "fab fa-pinterest")]
    Pinterest,
    #[display(fmt = "fab fa-pinterest-p")]
    PinterestP,
    #[display(fmt = "fab fa-pinterest-square")]
    PinterestSquare,
    #[display(fmt = "fas fa-pizza-slice")]
    PizzaSlice,
    #[display(fmt = "fas fa-place-of-worship")]
    PlaceOfWorship,
    #[display(fmt = "fas fa-plane")]
    Plane,
    #[display(fmt = "fas fa-plane-arrival")]
    PlaneArrival,
    #[display(fmt = "fas fa-plane-departure")]
    PlaneDeparture,
    #[display(fmt = "fas fa-plane-slash")]
    PlaneSlash,
    #[display(fmt = "fas fa-play")]
    Play,
    #[display(fmt = "fas fa-play-circle")]
    PlayCircleSolid,
    #[display(fmt = "far fa-play-circle")]
    PlayCircleRegular,
    #[display(fmt = "fab fa-playstation")]
    Playstation,
    #[display(fmt = "fas fa-plug")]
    Plug,
    #[display(fmt = "fas fa-plus")]
    Plus,
    #[display(fmt = "fas fa-plus-circle")]
    PlusCircle,
    #[display(fmt = "fas fa-plus-square")]
    PlusSquareSolid,
    #[display(fmt = "far fa-plus-square")]
    PlusSquareRegular,
    #[display(fmt = "fas fa-podcast")]
    Podcast,
    #[display(fmt = "fas fa-poll")]
    Poll,
    #[display(fmt = "fas fa-poll-h")]
    PollH,
    #[display(fmt = "fas fa-poo")]
    Poo,
    #[display(fmt = "fas fa-poo-storm")]
    PooStorm,
    #[display(fmt = "fas fa-poop")]
    Poop,
    #[display(fmt = "fas fa-portrait")]
    Portrait,
    #[display(fmt = "fas fa-pound-sign")]
    PoundSign,
    #[display(fmt = "fas fa-power-off")]
    PowerOff,
    #[display(fmt = "fas fa-pray")]
    Pray,
    #[display(fmt = "fas fa-praying-hands")]
    PrayingHands,
    #[display(fmt = "fas fa-prescription")]
    Prescription,
    #[display(fmt = "fas fa-prescription-bottle")]
    PrescriptionBottle,
    #[display(fmt = "fas fa-prescription-bottle-alt")]
    PrescriptionBottleAlt,
    #[display(fmt = "fas fa-print")]
    Print,
    #[display(fmt = "fas fa-procedures")]
    Procedures,
    #[display(fmt = "fab fa-product-hunt")]
    ProductHunt,
    #[display(fmt = "fas fa-project-diagram")]
    ProjectDiagram,
    #[display(fmt = "fas fa-pump-medical")]
    PumpMedical,
    #[display(fmt = "fas fa-pump-soap")]
    PumpSoap,
    #[display(fmt = "fab fa-pushed")]
    Pushed,
    #[display(fmt = "fas fa-puzzle-piece")]
    PuzzlePiece,
    #[display(fmt = "fab fa-python")]
    Python,
    #[display(fmt = "fab fa-qq")]
    Qq,
    #[display(fmt = "fas fa-qrcode")]
    Qrcode,
    #[display(fmt = "fas fa-question")]
    Question,
    #[display(fmt = "fas fa-question-circle")]
    QuestionCircleSolid,
    #[display(fmt = "far fa-question-circle")]
    QuestionCircleRegular,
    #[display(fmt = "fas fa-quidditch")]
    Quidditch,
    #[display(fmt = "fab fa-quinscape")]
    Quinscape,
    #[display(fmt = "fab fa-quora")]
    Quora,
    #[display(fmt = "fas fa-quote-left")]
    QuoteLeft,
    #[display(fmt = "fas fa-quote-right")]
    QuoteRight,
    #[display(fmt = "fas fa-quran")]
    Quran,
    #[display(fmt = "fab fa-r-project")]
    RProject,
    #[display(fmt = "fas fa-radiation")]
    Radiation,
    #[display(fmt = "fas fa-radiation-alt")]
    RadiationAlt,
    #[display(fmt = "fas fa-rainbow")]
    Rainbow,
    #[display(fmt = "fas fa-random")]
    Random,
    #[display(fmt = "fab fa-raspberry-pi")]
    RaspberryPi,
    #[display(fmt = "fab fa-ravelry")]
    Ravelry,
    #[display(fmt = "fab fa-react")]
    React,
    #[display(fmt = "fab fa-reacteurope")]
    Reacteurope,
    #[display(fmt = "fab fa-readme")]
    Readme,
    #[display(fmt = "fab fa-rebel")]
    Rebel,
    #[display(fmt = "fas fa-receipt")]
    Receipt,
    #[display(fmt = "fas fa-record-vinyl")]
    RecordVinyl,
    #[display(fmt = "fas fa-recycle")]
    Recycle,
    #[display(fmt = "fab fa-red-river")]
    RedRiver,
    #[display(fmt = "fab fa-reddit")]
    Reddit,
    #[display(fmt = "fab fa-reddit-alien")]
    RedditAlien,
    #[display(fmt = "fab fa-reddit-square")]
    RedditSquare,
    #[display(fmt = "fab fa-redhat")]
    Redhat,
    #[display(fmt = "fas fa-redo")]
    Redo,
    #[display(fmt = "fas fa-redo-alt")]
    RedoAlt,
    #[display(fmt = "fas fa-registered")]
    RegisteredSolid,
    #[display(fmt = "far fa-registered")]
    RegisteredRegular,
    #[display(fmt = "fas fa-remove-format")]
    RemoveFormat,
    #[display(fmt = "fab fa-renren")]
    Renren,
    #[display(fmt = "fas fa-reply")]
    Reply,
    #[display(fmt = "fas fa-reply-all")]
    ReplyAll,
    #[display(fmt = "fab fa-replyd")]
    Replyd,
    #[display(fmt = "fas fa-republican")]
    Republican,
    #[display(fmt = "fab fa-researchgate")]
    Researchgate,
    #[display(fmt = "fab fa-resolving")]
    Resolving,
    #[display(fmt = "fas fa-restroom")]
    Restroom,
    #[display(fmt = "fas fa-retweet")]
    Retweet,
    #[display(fmt = "fab fa-rev")]
    Rev,
    #[display(fmt = "fas fa-ribbon")]
    Ribbon,
    #[display(fmt = "fas fa-ring")]
    Ring,
    #[display(fmt = "fas fa-road")]
    Road,
    #[display(fmt = "fas fa-robot")]
    Robot,
    #[display(fmt = "fas fa-rocket")]
    Rocket,
    #[display(fmt = "fab fa-rocketchat")]
    Rocketchat,
    #[display(fmt = "fab fa-rockrms")]
    Rockrms,
    #[display(fmt = "fas fa-route")]
    Route,
    #[display(fmt = "fas fa-rss")]
    Rss,
    #[display(fmt = "fas fa-rss-square")]
    RssSquare,
    #[display(fmt = "fas fa-ruble-sign")]
    RubleSign,
    #[display(fmt = "fas fa-ruler")]
    Ruler,
    #[display(fmt = "fas fa-ruler-combined")]
    RulerCombined,
    #[display(fmt = "fas fa-ruler-horizontal")]
    RulerHorizontal,
    #[display(fmt = "fas fa-ruler-vertical")]
    RulerVertical,
    #[display(fmt = "fas fa-running")]
    Running,
    #[display(fmt = "fas fa-rupee-sign")]
    RupeeSign,
    #[display(fmt = "fab fa-rust")]
    Rust,
    #[display(fmt = "fas fa-sad-cry")]
    SadCrySolid,
    #[display(fmt = "far fa-sad-cry")]
    SadCryRegular,
    #[display(fmt = "fas fa-sad-tear")]
    SadTearSolid,
    #[display(fmt = "far fa-sad-tear")]
    SadTearRegular,
    #[display(fmt = "fab fa-safari")]
    Safari,
    #[display(fmt = "fab fa-salesforce")]
    Salesforce,
    #[display(fmt = "fab fa-sass")]
    Sass,
    #[display(fmt = "fas fa-satellite")]
    Satellite,
    #[display(fmt = "fas fa-satellite-dish")]
    SatelliteDish,
    #[display(fmt = "fas fa-save")]
    SaveSolid,
    #[display(fmt = "far fa-save")]
    SaveRegular,
    #[display(fmt = "fab fa-schlix")]
    Schlix,
    #[display(fmt = "fas fa-school")]
    School,
    #[display(fmt = "fas fa-screwdriver")]
    Screwdriver,
    #[display(fmt = "fab fa-scribd")]
    Scribd,
    #[display(fmt = "fas fa-scroll")]
    Scroll,
    #[display(fmt = "fas fa-sd-card")]
    SdCard,
    #[display(fmt = "fas fa-search")]
    Search,
    #[display(fmt = "fas fa-search-dollar")]
    SearchDollar,
    #[display(fmt = "fas fa-search-location")]
    SearchLocation,
    #[display(fmt = "fas fa-search-minus")]
    SearchMinus,
    #[display(fmt = "fas fa-search-plus")]
    SearchPlus,
    #[display(fmt = "fab fa-searchengin")]
    Searchengin,
    #[display(fmt = "fas fa-seedling")]
    Seedling,
    #[display(fmt = "fab fa-sellcast")]
    Sellcast,
    #[display(fmt = "fab fa-sellsy")]
    Sellsy,
    #[display(fmt = "fas fa-server")]
    Server,
    #[display(fmt = "fab fa-servicestack")]
    Servicestack,
    #[display(fmt = "fas fa-shapes")]
    Shapes,
    #[display(fmt = "fas fa-share")]
    Share,
    #[display(fmt = "fas fa-share-alt")]
    ShareAlt,
    #[display(fmt = "fas fa-share-alt-square")]
    ShareAltSquare,
    #[display(fmt = "fas fa-share-square")]
    ShareSquareSolid,
    #[display(fmt = "far fa-share-square")]
    ShareSquareRegular,
    #[display(fmt = "fas fa-shekel-sign")]
    ShekelSign,
    #[display(fmt = "fas fa-shield-alt")]
    ShieldAlt,
    #[display(fmt = "fas fa-shield-virus")]
    ShieldVirus,
    #[display(fmt = "fas fa-ship")]
    Ship,
    #[display(fmt = "fas fa-shipping-fast")]
    ShippingFast,
    #[display(fmt = "fab fa-shirtsinbulk")]
    Shirtsinbulk,
    #[display(fmt = "fas fa-shoe-prints")]
    ShoePrints,
    #[display(fmt = "fab fa-shopify")]
    Shopify,
    #[display(fmt = "fas fa-shopping-bag")]
    ShoppingBag,
    #[display(fmt = "fas fa-shopping-basket")]
    ShoppingBasket,
    #[display(fmt = "fas fa-shopping-cart")]
    ShoppingCart,
    #[display(fmt = "fab fa-shopware")]
    Shopware,
    #[display(fmt = "fas fa-shower")]
    Shower,
    #[display(fmt = "fas fa-shuttle-van")]
    ShuttleVan,
    #[display(fmt = "fas fa-sign")]
    Sign,
    #[display(fmt = "fas fa-sign-in-alt")]
    SignInAlt,
    #[display(fmt = "fas fa-sign-language")]
    SignLanguage,
    #[display(fmt = "fas fa-sign-out-alt")]
    SignOutAlt,
    #[display(fmt = "fas fa-signal")]
    Signal,
    #[display(fmt = "fas fa-signature")]
    Signature,
    #[display(fmt = "fas fa-sim-card")]
    SimCard,
    #[display(fmt = "fab fa-simplybuilt")]
    Simplybuilt,
    #[display(fmt = "fas fa-sink")]
    Sink,
    #[display(fmt = "fab fa-sistrix")]
    Sistrix,
    #[display(fmt = "fas fa-sitemap")]
    Sitemap,
    #[display(fmt = "fab fa-sith")]
    Sith,
    #[display(fmt = "fas fa-skating")]
    Skating,
    #[display(fmt = "fab fa-sketch")]
    Sketch,
    #[display(fmt = "fas fa-skiing")]
    Skiing,
    #[display(fmt = "fas fa-skiing-nordic")]
    SkiingNordic,
    #[display(fmt = "fas fa-skull")]
    Skull,
    #[display(fmt = "fas fa-skull-crossbones")]
    SkullCrossbones,
    #[display(fmt = "fab fa-skyatlas")]
    Skyatlas,
    #[display(fmt = "fab fa-skype")]
    Skype,
    #[display(fmt = "fab fa-slack")]
    Slack,
    #[display(fmt = "fab fa-slack-hash")]
    SlackHash,
    #[display(fmt = "fas fa-slash")]
    Slash,
    #[display(fmt = "fas fa-sleigh")]
    Sleigh,
    #[display(fmt = "fas fa-sliders-h")]
    SlidersH,
    #[display(fmt = "fab fa-slideshare")]
    Slideshare,
    #[display(fmt = "fas fa-smile")]
    SmileSolid,
    #[display(fmt = "far fa-smile")]
    SmileRegular,
    #[display(fmt = "fas fa-smile-beam")]
    SmileBeamSolid,
    #[display(fmt = "far fa-smile-beam")]
    SmileBeamRegular,
    #[display(fmt = "fas fa-smile-wink")]
    SmileWinkSolid,
    #[display(fmt = "far fa-smile-wink")]
    SmileWinkRegular,
    #[display(fmt = "fas fa-smog")]
    Smog,
    #[display(fmt = "fas fa-smoking")]
    Smoking,
    #[display(fmt = "fas fa-smoking-ban")]
    SmokingBan,
    #[display(fmt = "fas fa-sms")]
    Sms,
    #[display(fmt = "fab fa-snapchat")]
    Snapchat,
    #[display(fmt = "fab fa-snapchat-ghost")]
    SnapchatGhost,
    #[display(fmt = "fab fa-snapchat-square")]
    SnapchatSquare,
    #[display(fmt = "fas fa-snowboarding")]
    Snowboarding,
    #[display(fmt = "fas fa-snowflake")]
    SnowflakeSolid,
    #[display(fmt = "far fa-snowflake")]
    SnowflakeRegular,
    #[display(fmt = "fas fa-snowman")]
    Snowman,
    #[display(fmt = "fas fa-snowplow")]
    Snowplow,
    #[display(fmt = "fas fa-soap")]
    Soap,
    #[display(fmt = "fas fa-socks")]
    Socks,
    #[display(fmt = "fas fa-solar-panel")]
    SolarPanel,
    #[display(fmt = "fas fa-sort")]
    Sort,
    #[display(fmt = "fas fa-sort-alpha-down")]
    SortAlphaDown,
    #[display(fmt = "fas fa-sort-alpha-down-alt")]
    SortAlphaDownAlt,
    #[display(fmt = "fas fa-sort-alpha-up")]
    SortAlphaUp,
    #[display(fmt = "fas fa-sort-alpha-up-alt")]
    SortAlphaUpAlt,
    #[display(fmt = "fas fa-sort-amount-down")]
    SortAmountDown,
    #[display(fmt = "fas fa-sort-amount-down-alt")]
    SortAmountDownAlt,
    #[display(fmt = "fas fa-sort-amount-up")]
    SortAmountUp,
    #[display(fmt = "fas fa-sort-amount-up-alt")]
    SortAmountUpAlt,
    #[display(fmt = "fas fa-sort-down")]
    SortDown,
    #[display(fmt = "fas fa-sort-numeric-down")]
    SortNumericDown,
    #[display(fmt = "fas fa-sort-numeric-down-alt")]
    SortNumericDownAlt,
    #[display(fmt = "fas fa-sort-numeric-up")]
    SortNumericUp,
    #[display(fmt = "fas fa-sort-numeric-up-alt")]
    SortNumericUpAlt,
    #[display(fmt = "fas fa-sort-up")]
    SortUp,
    #[display(fmt = "fab fa-soundcloud")]
    Soundcloud,
    #[display(fmt = "fab fa-sourcetree")]
    Sourcetree,
    #[display(fmt = "fas fa-spa")]
    Spa,
    #[display(fmt = "fas fa-space-shuttle")]
    SpaceShuttle,
    #[display(fmt = "fab fa-speakap")]
    Speakap,
    #[display(fmt = "fab fa-speaker-deck")]
    SpeakerDeck,
    #[display(fmt = "fas fa-spell-check")]
    SpellCheck,
    #[display(fmt = "fas fa-spider")]
    Spider,
    #[display(fmt = "fas fa-spinner")]
    Spinner,
    #[display(fmt = "fas fa-splotch")]
    Splotch,
    #[display(fmt = "fab fa-spotify")]
    Spotify,
    #[display(fmt = "fas fa-spray-can")]
    SprayCan,
    #[display(fmt = "fas fa-square")]
    SquareSolid,
    #[display(fmt = "far fa-square")]
    SquareRegular,
    #[display(fmt = "fas fa-square-full")]
    SquareFull,
    #[display(fmt = "fas fa-square-root-alt")]
    SquareRootAlt,
    #[display(fmt = "fab fa-squarespace")]
    Squarespace,
    #[display(fmt = "fab fa-stack-exchange")]
    StackExchange,
    #[display(fmt = "fab fa-stack-overflow")]
    StackOverflow,
    #[display(fmt = "fab fa-stackpath")]
    Stackpath,
    #[display(fmt = "fas fa-stamp")]
    Stamp,
    #[display(fmt = "fas fa-star")]
    StarSolid,
    #[display(fmt = "far fa-star")]
    StarRegular,
    #[display(fmt = "fas fa-star-and-crescent")]
    StarAndCrescent,
    #[display(fmt = "fas fa-star-half")]
    StarHalfSolid,
    #[display(fmt = "far fa-star-half")]
    StarHalfRegular,
    #[display(fmt = "fas fa-star-half-alt")]
    StarHalfAlt,
    #[display(fmt = "fas fa-star-of-david")]
    StarOfDavid,
    #[display(fmt = "fas fa-star-of-life")]
    StarOfLife,
    #[display(fmt = "fab fa-staylinked")]
    Staylinked,
    #[display(fmt = "fab fa-steam")]
    Steam,
    #[display(fmt = "fab fa-steam-square")]
    SteamSquare,
    #[display(fmt = "fab fa-steam-symbol")]
    SteamSymbol,
    #[display(fmt = "fas fa-step-backward")]
    StepBackward,
    #[display(fmt = "fas fa-step-forward")]
    StepForward,
    #[display(fmt = "fas fa-stethoscope")]
    Stethoscope,
    #[display(fmt = "fab fa-sticker-mule")]
    StickerMule,
    #[display(fmt = "fas fa-sticky-note")]
    StickyNoteSolid,
    #[display(fmt = "far fa-sticky-note")]
    StickyNoteRegular,
    #[display(fmt = "fas fa-stop")]
    Stop,
    #[display(fmt = "fas fa-stop-circle")]
    StopCircleSolid,
    #[display(fmt = "far fa-stop-circle")]
    StopCircleRegular,
    #[display(fmt = "fas fa-stopwatch")]
    Stopwatch,
    #[display(fmt = "fas fa-stopwatch-20")]
    Stopwatch20,
    #[display(fmt = "fas fa-store")]
    Store,
    #[display(fmt = "fas fa-store-alt")]
    StoreAlt,
    #[display(fmt = "fas fa-store-alt-slash")]
    StoreAltSlash,
    #[display(fmt = "fas fa-store-slash")]
    StoreSlash,
    #[display(fmt = "fab fa-strava")]
    Strava,
    #[display(fmt = "fas fa-stream")]
    Stream,
    #[display(fmt = "fas fa-street-view")]
    StreetView,
    #[display(fmt = "fas fa-strikethrough")]
    Strikethrough,
    #[display(fmt = "fab fa-stripe")]
    Stripe,
    #[display(fmt = "fab fa-stripe-s")]
    StripeS,
    #[display(fmt = "fas fa-stroopwafel")]
    Stroopwafel,
    #[display(fmt = "fab fa-studiovinari")]
    Studiovinari,
    #[display(fmt = "fab fa-stumbleupon")]
    Stumbleupon,
    #[display(fmt = "fab fa-stumbleupon-circle")]
    StumbleuponCircle,
    #[display(fmt = "fas fa-subscript")]
    Subscript,
    #[display(fmt = "fas fa-subway")]
    Subway,
    #[display(fmt = "fas fa-suitcase")]
    Suitcase,
    #[display(fmt = "fas fa-suitcase-rolling")]
    SuitcaseRolling,
    #[display(fmt = "fas fa-sun")]
    SunSolid,
    #[display(fmt = "far fa-sun")]
    SunRegular,
    #[display(fmt = "fab fa-superpowers")]
    Superpowers,
    #[display(fmt = "fas fa-superscript")]
    Superscript,
    #[display(fmt = "fab fa-supple")]
    Supple,
    #[display(fmt = "fas fa-surprise")]
    SurpriseSolid,
    #[display(fmt = "far fa-surprise")]
    SurpriseRegular,
    #[display(fmt = "fab fa-suse")]
    Suse,
    #[display(fmt = "fas fa-swatchbook")]
    Swatchbook,
    #[display(fmt = "fab fa-swift")]
    Swift,
    #[display(fmt = "fas fa-swimmer")]
    Swimmer,
    #[display(fmt = "fas fa-swimming-pool")]
    SwimmingPool,
    #[display(fmt = "fab fa-symfony")]
    Symfony,
    #[display(fmt = "fas fa-synagogue")]
    Synagogue,
    #[display(fmt = "fas fa-sync")]
    Sync,
    #[display(fmt = "fas fa-sync-alt")]
    SyncAlt,
    #[display(fmt = "fas fa-syringe")]
    Syringe,
    #[display(fmt = "fas fa-table")]
    Table,
    #[display(fmt = "fas fa-table-tennis")]
    TableTennis,
    #[display(fmt = "fas fa-tablet")]
    Tablet,
    #[display(fmt = "fas fa-tablet-alt")]
    TabletAlt,
    #[display(fmt = "fas fa-tablets")]
    Tablets,
    #[display(fmt = "fas fa-tachometer-alt")]
    TachometerAlt,
    #[display(fmt = "fas fa-tag")]
    Tag,
    #[display(fmt = "fas fa-tags")]
    Tags,
    #[display(fmt = "fas fa-tape")]
    Tape,
    #[display(fmt = "fas fa-tasks")]
    Tasks,
    #[display(fmt = "fas fa-taxi")]
    Taxi,
    #[display(fmt = "fab fa-teamspeak")]
    Teamspeak,
    #[display(fmt = "fas fa-teeth")]
    Teeth,
    #[display(fmt = "fas fa-teeth-open")]
    TeethOpen,
    #[display(fmt = "fab fa-telegram")]
    Telegram,
    #[display(fmt = "fab fa-telegram-plane")]
    TelegramPlane,
    #[display(fmt = "fas fa-temperature-high")]
    TemperatureHigh,
    #[display(fmt = "fas fa-temperature-low")]
    TemperatureLow,
    #[display(fmt = "fab fa-tencent-weibo")]
    TencentWeibo,
    #[display(fmt = "fas fa-tenge")]
    Tenge,
    #[display(fmt = "fas fa-terminal")]
    Terminal,
    #[display(fmt = "fas fa-text-height")]
    TextHeight,
    #[display(fmt = "fas fa-text-width")]
    TextWidth,
    #[display(fmt = "fas fa-th")]
    Th,
    #[display(fmt = "fas fa-th-large")]
    ThLarge,
    #[display(fmt = "fas fa-th-list")]
    ThList,
    #[display(fmt = "fab fa-the-red-yeti")]
    TheRedYeti,
    #[display(fmt = "fas fa-theater-masks")]
    TheaterMasks,
    #[display(fmt = "fab fa-themeco")]
    Themeco,
    #[display(fmt = "fab fa-themeisle")]
    Themeisle,
    #[display(fmt = "fas fa-thermometer")]
    Thermometer,
    #[display(fmt = "fas fa-thermometer-empty")]
    ThermometerEmpty,
    #[display(fmt = "fas fa-thermometer-full")]
    ThermometerFull,
    #[display(fmt = "fas fa-thermometer-half")]
    ThermometerHalf,
    #[display(fmt = "fas fa-thermometer-quarter")]
    ThermometerQuarter,
    #[display(fmt = "fas fa-thermometer-three-quarters")]
    ThermometerThreeQuarters,
    #[display(fmt = "fab fa-think-peaks")]
    ThinkPeaks,
    #[display(fmt = "fas fa-thumbs-down")]
    ThumbsDownSolid,
    #[display(fmt = "far fa-thumbs-down")]
    ThumbsDownRegular,
    #[display(fmt = "fas fa-thumbs-up")]
    ThumbsUpSolid,
    #[display(fmt = "far fa-thumbs-up")]
    ThumbsUpRegular,
    #[display(fmt = "fas fa-thumbtack")]
    Thumbtack,
    #[display(fmt = "fas fa-ticket-alt")]
    TicketAlt,
    #[display(fmt = "fab fa-tiktok")]
    Tiktok,
    #[display(fmt = "fas fa-times")]
    Times,
    #[display(fmt = "fas fa-times-circle")]
    TimesCircleSolid,
    #[display(fmt = "far fa-times-circle")]
    TimesCircleRegular,
    #[display(fmt = "fas fa-tint")]
    Tint,
    #[display(fmt = "fas fa-tint-slash")]
    TintSlash,
    #[display(fmt = "fas fa-tired")]
    TiredSolid,
    #[display(fmt = "far fa-tired")]
    TiredRegular,
    #[display(fmt = "fas fa-toggle-off")]
    ToggleOff,
    #[display(fmt = "fas fa-toggle-on")]
    ToggleOn,
    #[display(fmt = "fas fa-toilet")]
    Toilet,
    #[display(fmt = "fas fa-toilet-paper")]
    ToiletPaper,
    #[display(fmt = "fas fa-toilet-paper-slash")]
    ToiletPaperSlash,
    #[display(fmt = "fas fa-toolbox")]
    Toolbox,
    #[display(fmt = "fas fa-tools")]
    Tools,
    #[display(fmt = "fas fa-tooth")]
    Tooth,
    #[display(fmt = "fas fa-torah")]
    Torah,
    #[display(fmt = "fas fa-torii-gate")]
    ToriiGate,
    #[display(fmt = "fas fa-tractor")]
    Tractor,
    #[display(fmt = "fab fa-trade-federation")]
    TradeFederation,
    #[display(fmt = "fas fa-trademark")]
    Trademark,
    #[display(fmt = "fas fa-traffic-light")]
    TrafficLight,
    #[display(fmt = "fas fa-trailer")]
    Trailer,
    #[display(fmt = "fas fa-train")]
    Train,
    #[display(fmt = "fas fa-tram")]
    Tram,
    #[display(fmt = "fas fa-transgender")]
    Transgender,
    #[display(fmt = "fas fa-transgender-alt")]
    TransgenderAlt,
    #[display(fmt = "fas fa-trash")]
    Trash,
    #[display(fmt = "fas fa-trash-alt")]
    TrashAltSolid,
    #[display(fmt = "far fa-trash-alt")]
    TrashAltRegular,
    #[display(fmt = "fas fa-trash-restore")]
    TrashRestore,
    #[display(fmt = "fas fa-trash-restore-alt")]
    TrashRestoreAlt,
    #[display(fmt = "fas fa-tree")]
    Tree,
    #[display(fmt = "fab fa-trello")]
    Trello,
    #[display(fmt = "fab fa-tripadvisor")]
    Tripadvisor,
    #[display(fmt = "fas fa-trophy")]
    Trophy,
    #[display(fmt = "fas fa-truck")]
    Truck,
    #[display(fmt = "fas fa-truck-loading")]
    TruckLoading,
    #[display(fmt = "fas fa-truck-monster")]
    TruckMonster,
    #[display(fmt = "fas fa-truck-moving")]
    TruckMoving,
    #[display(fmt = "fas fa-truck-pickup")]
    TruckPickup,
    #[display(fmt = "fas fa-tshirt")]
    Tshirt,
    #[display(fmt = "fas fa-tty")]
    Tty,
    #[display(fmt = "fab fa-tumblr")]
    Tumblr,
    #[display(fmt = "fab fa-tumblr-square")]
    TumblrSquare,
    #[display(fmt = "fas fa-tv")]
    Tv,
    #[display(fmt = "fab fa-twitch")]
    Twitch,
    #[display(fmt = "fab fa-twitter")]
    Twitter,
    #[display(fmt = "fab fa-twitter-square")]
    TwitterSquare,
    #[display(fmt = "fab fa-typo3")]
    Typo3,
    #[display(fmt = "fab fa-uber")]
    Uber,
    #[display(fmt = "fab fa-ubuntu")]
    Ubuntu,
    #[display(fmt = "fab fa-uikit")]
    Uikit,
    #[display(fmt = "fab fa-umbraco")]
    Umbraco,
    #[display(fmt = "fas fa-umbrella")]
    Umbrella,
    #[display(fmt = "fas fa-umbrella-beach")]
    UmbrellaBeach,
    #[display(fmt = "fab fa-uncharted")]
    Uncharted,
    #[display(fmt = "fas fa-underline")]
    Underline,
    #[display(fmt = "fas fa-undo")]
    Undo,
    #[display(fmt = "fas fa-undo-alt")]
    UndoAlt,
    #[display(fmt = "fab fa-uniregistry")]
    Uniregistry,
    #[display(fmt = "fab fa-unity")]
    Unity,
    #[display(fmt = "fas fa-universal-access")]
    UniversalAccess,
    #[display(fmt = "fas fa-university")]
    University,
    #[display(fmt = "fas fa-unlink")]
    Unlink,
    #[display(fmt = "fas fa-unlock")]
    Unlock,
    #[display(fmt = "fas fa-unlock-alt")]
    UnlockAlt,
    #[display(fmt = "fab fa-unsplash")]
    Unsplash,
    #[display(fmt = "fab fa-untappd")]
    Untappd,
    #[display(fmt = "fas fa-upload")]
    Upload,
    #[display(fmt = "fab fa-ups")]
    Ups,
    #[display(fmt = "fab fa-usb")]
    Usb,
    #[display(fmt = "fas fa-user")]
    UserSolid,
    #[display(fmt = "far fa-user")]
    UserRegular,
    #[display(fmt = "fas fa-user-alt")]
    UserAlt,
    #[display(fmt = "fas fa-user-alt-slash")]
    UserAltSlash,
    #[display(fmt = "fas fa-user-astronaut")]
    UserAstronaut,
    #[display(fmt = "fas fa-user-check")]
    UserCheck,
    #[display(fmt = "fas fa-user-circle")]
    UserCircleSolid,
    #[display(fmt = "far fa-user-circle")]
    UserCircleRegular,
    #[display(fmt = "fas fa-user-clock")]
    UserClock,
    #[display(fmt = "fas fa-user-cog")]
    UserCog,
    #[display(fmt = "fas fa-user-edit")]
    UserEdit,
    #[display(fmt = "fas fa-user-friends")]
    UserFriends,
    #[display(fmt = "fas fa-user-graduate")]
    UserGraduate,
    #[display(fmt = "fas fa-user-injured")]
    UserInjured,
    #[display(fmt = "fas fa-user-lock")]
    UserLock,
    #[display(fmt = "fas fa-user-md")]
    UserMd,
    #[display(fmt = "fas fa-user-minus")]
    UserMinus,
    #[display(fmt = "fas fa-user-ninja")]
    UserNinja,
    #[display(fmt = "fas fa-user-nurse")]
    UserNurse,
    #[display(fmt = "fas fa-user-plus")]
    UserPlus,
    #[display(fmt = "fas fa-user-secret")]
    UserSecret,
    #[display(fmt = "fas fa-user-shield")]
    UserShield,
    #[display(fmt = "fas fa-user-slash")]
    UserSlash,
    #[display(fmt = "fas fa-user-tag")]
    UserTag,
    #[display(fmt = "fas fa-user-tie")]
    UserTie,
    #[display(fmt = "fas fa-user-times")]
    UserTimes,
    #[display(fmt = "fas fa-users")]
    Users,
    #[display(fmt = "fas fa-users-cog")]
    UsersCog,
    #[display(fmt = "fas fa-users-slash")]
    UsersSlash,
    #[display(fmt = "fab fa-usps")]
    Usps,
    #[display(fmt = "fab fa-ussunnah")]
    Ussunnah,
    #[display(fmt = "fas fa-utensil-spoon")]
    UtensilSpoon,
    #[display(fmt = "fas fa-utensils")]
    Utensils,
    #[display(fmt = "fab fa-vaadin")]
    Vaadin,
    #[display(fmt = "fas fa-vector-square")]
    VectorSquare,
    #[display(fmt = "fas fa-venus")]
    Venus,
    #[display(fmt = "fas fa-venus-double")]
    VenusDouble,
    #[display(fmt = "fas fa-venus-mars")]
    VenusMars,
    #[display(fmt = "fas fa-vest")]
    Vest,
    #[display(fmt = "fas fa-vest-patches")]
    VestPatches,
    #[display(fmt = "fab fa-viacoin")]
    Viacoin,
    #[display(fmt = "fab fa-viadeo")]
    Viadeo,
    #[display(fmt = "fab fa-viadeo-square")]
    ViadeoSquare,
    #[display(fmt = "fas fa-vial")]
    Vial,
    #[display(fmt = "fas fa-vials")]
    Vials,
    #[display(fmt = "fab fa-viber")]
    Viber,
    #[display(fmt = "fas fa-video")]
    Video,
    #[display(fmt = "fas fa-video-slash")]
    VideoSlash,
    #[display(fmt = "fas fa-vihara")]
    Vihara,
    #[display(fmt = "fab fa-vimeo")]
    Vimeo,
    #[display(fmt = "fab fa-vimeo-square")]
    VimeoSquare,
    #[display(fmt = "fab fa-vimeo-v")]
    VimeoV,
    #[display(fmt = "fab fa-vine")]
    Vine,
    #[display(fmt = "fas fa-virus")]
    Virus,
    #[display(fmt = "fas fa-virus-slash")]
    VirusSlash,
    #[display(fmt = "fas fa-viruses")]
    Viruses,
    #[display(fmt = "fab fa-vk")]
    Vk,
    #[display(fmt = "fab fa-vnv")]
    Vnv,
    #[display(fmt = "fas fa-voicemail")]
    Voicemail,
    #[display(fmt = "fas fa-volleyball-ball")]
    VolleyballBall,
    #[display(fmt = "fas fa-volume-down")]
    VolumeDown,
    #[display(fmt = "fas fa-volume-mute")]
    VolumeMute,
    #[display(fmt = "fas fa-volume-off")]
    VolumeOff,
    #[display(fmt = "fas fa-volume-up")]
    VolumeUp,
    #[display(fmt = "fas fa-vote-yea")]
    VoteYea,
    #[display(fmt = "fas fa-vr-cardboard")]
    VrCardboard,
    #[display(fmt = "fab fa-vuejs")]
    Vuejs,
    #[display(fmt = "fas fa-walking")]
    Walking,
    #[display(fmt = "fas fa-wallet")]
    Wallet,
    #[display(fmt = "fas fa-warehouse")]
    Warehouse,
    #[display(fmt = "fab fa-watchman-monitoring")]
    WatchmanMonitoring,
    #[display(fmt = "fas fa-water")]
    Water,
    #[display(fmt = "fas fa-wave-square")]
    WaveSquare,
    #[display(fmt = "fab fa-waze")]
    Waze,
    #[display(fmt = "fab fa-weebly")]
    Weebly,
    #[display(fmt = "fab fa-weibo")]
    Weibo,
    #[display(fmt = "fas fa-weight")]
    Weight,
    #[display(fmt = "fas fa-weight-hanging")]
    WeightHanging,
    #[display(fmt = "fab fa-weixin")]
    Weixin,
    #[display(fmt = "fab fa-whatsapp")]
    Whatsapp,
    #[display(fmt = "fab fa-whatsapp-square")]
    WhatsappSquare,
    #[display(fmt = "fas fa-wheelchair")]
    Wheelchair,
    #[display(fmt = "fab fa-whmcs")]
    Whmcs,
    #[display(fmt = "fas fa-wifi")]
    Wifi,
    #[display(fmt = "fab fa-wikipedia-w")]
    WikipediaW,
    #[display(fmt = "fas fa-wind")]
    Wind,
    #[display(fmt = "fas fa-window-close")]
    WindowCloseSolid,
    #[display(fmt = "far fa-window-close")]
    WindowCloseRegular,
    #[display(fmt = "fas fa-window-maximize")]
    WindowMaximizeSolid,
    #[display(fmt = "far fa-window-maximize")]
    WindowMaximizeRegular,
    #[display(fmt = "fas fa-window-minimize")]
    WindowMinimizeSolid,
    #[display(fmt = "far fa-window-minimize")]
    WindowMinimizeRegular,
    #[display(fmt = "fas fa-window-restore")]
    WindowRestoreSolid,
    #[display(fmt = "far fa-window-restore")]
    WindowRestoreRegular,
    #[display(fmt = "fab fa-windows")]
    Windows,
    #[display(fmt = "fas fa-wine-bottle")]
    WineBottle,
    #[display(fmt = "fas fa-wine-glass")]
    WineGlass,
    #[display(fmt = "fas fa-wine-glass-alt")]
    WineGlassAlt,
    #[display(fmt = "fab fa-wix")]
    Wix,
    #[display(fmt = "fab fa-wizards-of-the-coast")]
    WizardsOfTheCoast,
    #[display(fmt = "fab fa-wodu")]
    Wodu,
    #[display(fmt = "fab fa-wolf-pack-battalion")]
    WolfPackBattalion,
    #[display(fmt = "fas fa-won-sign")]
    WonSign,
    #[display(fmt = "fab fa-wordpress")]
    Wordpress,
    #[display(fmt = "fab fa-wordpress-simple")]
    WordpressSimple,
    #[display(fmt = "fab fa-wpbeginner")]
    Wpbeginner,
    #[display(fmt = "fab fa-wpexplorer")]
    Wpexplorer,
    #[display(fmt = "fab fa-wpforms")]
    Wpforms,
    #[display(fmt = "fab fa-wpressr")]
    Wpressr,
    #[display(fmt = "fas fa-wrench")]
    Wrench,
    #[display(fmt = "fas fa-x-ray")]
    XRay,
    #[display(fmt = "fab fa-xbox")]
    Xbox,
    #[display(fmt = "fab fa-xing")]
    Xing,
    #[display(fmt = "fab fa-xing-square")]
    XingSquare,
    #[display(fmt = "fab fa-y-combinator")]
    YCombinator,
    #[display(fmt = "fab fa-yahoo")]
    Yahoo,
    #[display(fmt = "fab fa-yammer")]
    Yammer,
    #[display(fmt = "fab fa-yandex")]
    Yandex,
    #[display(fmt = "fab fa-yandex-international")]
    YandexInternational,
    #[display(fmt = "fab fa-yarn")]
    Yarn,
    #[display(fmt = "fab fa-yelp")]
    Yelp,
    #[display(fmt = "fas fa-yen-sign")]
    YenSign,
    #[display(fmt = "fas fa-yin-yang")]
    YinYang,
    #[display(fmt = "fab fa-yoast")]
    Yoast,
    #[display(fmt = "fab fa-youtube")]
    Youtube,
    #[display(fmt = "fab fa-youtube-square")]
    YoutubeSquare,
    #[display(fmt = "fab fa-zhihu")]
    Zhihu,
}
