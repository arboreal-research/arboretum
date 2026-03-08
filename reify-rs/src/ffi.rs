use std::ffi::{c_char, CStr};

use serde::{Deserialize, Serialize};

pub enum FfiMessage {
    Finalize,
    Record(Record),
}

//// BEGIN ARBORETUM GENERATED CODE ////
#[derive(Debug, Serialize, Deserialize)]
pub enum Record {
  Record0(Record0),
  Record1(Record1),
  Record2(Record2),
  Record3(Record3),
  Record4(Record4),
  Record5(Record5),
  Record6(Record6),
  Record7(Record7),
  Record8(Record8),
  Record9(Record9),
  Record10(Record10),
  Record11(Record11),
  Record12(Record12),
  Record13(Record13),
  Record14(Record14),
  Record15(Record15),
  Record16(Record16),
  Record17(Record17),
  Record18(Record18),
  Record19(Record19),
  Record20(Record20),
  Record21(Record21),
  Record22(Record22),
  Record23(Record23),
  Record24(Record24),
  Record25(Record25),
  Record26(Record26),
  Record27(Record27),
  Record28(Record28),
  Record29(Record29),
  Record30(Record30),
  Record31(Record31),
  Record32(Record32),
  Record33(Record33),
  Record34(Record34),
  Record35(Record35),
  Record36(Record36),
  Record37(Record37),
  Record38(Record38),
  Record39(Record39),
  Record40(Record40),
  Record41(Record41),
  Record42(Record42),
  Record43(Record43),
  Record44(Record44),
  Record45(Record45),
  Record46(Record46),
  Record47(Record47),
  Record48(Record48),
  Record49(Record49),
  Record50(Record50),
  Record51(Record51),
  Record52(Record52),
  Record53(Record53),
  Record54(Record54),
  Record55(Record55),
  Record56(Record56),
  Record57(Record57),
  Record58(Record58),
  Record59(Record59),
  Record60(Record60),
  Record61(Record61),
  Record62(Record62),
  Record63(Record63),
  Record64(Record64),
  Record65(Record65),
  Record66(Record66),
  Record67(Record67),
  Record68(Record68),
  Record69(Record69),
  Record70(Record70),
  Record71(Record71),
  Record72(Record72),
  Record73(Record73),
  Record74(Record74),
  Record75(Record75),
  Record76(Record76),
  Record77(Record77),
  Record78(Record78),
  Record79(Record79),
  Record80(Record80),
  Record81(Record81),
  Record82(Record82),
  Record83(Record83),
  Record84(Record84),
  Record85(Record85),
  Record86(Record86),
  Record87(Record87),
  Record88(Record88),
  Record89(Record89),
  Record90(Record90),
  Record91(Record91),
  Record92(Record92),
  Record93(Record93),
  Record94(Record94),
  Record95(Record95),
  Record96(Record96),
  Record97(Record97),
  Record98(Record98),
  Record99(Record99),
  Record100(Record100),
  Record101(Record101),
  Record102(Record102),
  Record103(Record103),
  Record104(Record104),
  Record105(Record105),
  Record106(Record106),
  Record107(Record107),
  Record108(Record108),
  Record109(Record109),
  Record110(Record110),
  Record111(Record111),
  Record112(Record112),
  Record113(Record113),
  Record114(Record114),
  Record115(Record115),
  Record116(Record116),
  Record117(Record117),
  Record118(Record118),
  Record119(Record119),
  Record120(Record120),
  Record121(Record121),
  Record122(Record122),
  Record123(Record123),
  Record124(Record124),
  Record125(Record125),
  Record126(Record126),
  Record127(Record127),
  Record128(Record128),
  Record129(Record129),
  Record130(Record130),
  Record131(Record131),
  Record132(Record132),
  Record133(Record133),
  Record134(Record134),
  Record135(Record135),
  Record136(Record136),
  Record137(Record137),
  Record138(Record138),
  Record139(Record139),
  Record140(Record140),
  Record141(Record141),
  Record142(Record142),
  Record143(Record143),
  Record144(Record144),
  Record145(Record145),
  Record146(Record146),
  Record147(Record147),
  Record148(Record148),
  Record149(Record149),
  Record150(Record150),
  Record151(Record151),
  Record152(Record152),
  Record153(Record153),
  Record154(Record154),
  Record155(Record155),
  Record156(Record156),
  Record157(Record157),
  Record158(Record158),
  Record159(Record159),
  Record160(Record160),
  Record161(Record161),
  Record162(Record162),
  Record163(Record163),
  Record164(Record164),
  Record165(Record165),
  Record166(Record166),
  Record167(Record167),
  Record168(Record168),
  Record169(Record169),
  Record170(Record170),
  Record171(Record171),
  Record172(Record172),
  Record173(Record173),
  Record174(Record174),
  Record175(Record175),
  Record176(Record176),
  Record177(Record177),
  Record178(Record178),
  Record179(Record179),
  Record180(Record180),
  Record181(Record181),
  Record182(Record182),
  Record183(Record183),
  Record184(Record184),
  Record185(Record185),
  Record186(Record186),
  Record187(Record187),
  Record188(Record188),
  Record189(Record189),
  Record190(Record190),
  Record191(Record191),
  Record192(Record192),
  Record193(Record193),
  Record194(Record194),
  Record195(Record195),
  Record196(Record196),
  Record197(Record197),
  Record198(Record198),
  Record199(Record199),
  Record200(Record200),
  Record201(Record201),
  Record202(Record202),
  Record203(Record203),
  Record204(Record204),
  Record205(Record205),
  Record206(Record206),
  Record207(Record207),
  Record208(Record208),
  Record209(Record209),
  Record210(Record210),
  Record211(Record211),
  Record212(Record212),
  Record213(Record213),
  Record214(Record214),
  Record215(Record215),
  Record216(Record216),
  Record217(Record217),
  Record218(Record218),
  Record219(Record219),
  Record220(Record220),
  Record221(Record221),
  Record222(Record222),
  Record223(Record223),
  Record224(Record224),
  Record225(Record225),
  Record226(Record226),
  Record227(Record227),
  Record228(Record228),
  Record229(Record229),
  Record230(Record230),
  Record231(Record231),
  Record232(Record232),
  Record233(Record233),
  Record234(Record234),
  Record235(Record235),
  Record236(Record236),
  Record237(Record237),
  Record238(Record238),
  Record239(Record239),
  Record240(Record240),
  Record241(Record241),
  Record242(Record242),
  Record243(Record243),
  Record244(Record244),
  Record245(Record245),
  Record246(Record246),
  Record247(Record247),
  Record248(Record248),
  Record249(Record249),
  Record250(Record250),
  Record251(Record251),
  Record252(Record252),
  Record253(Record253),
  Record254(Record254),
  Record255(Record255),
  Record256(Record256),
  Record257(Record257),
  Record258(Record258),
  Record259(Record259),
  Record260(Record260),
  Record261(Record261),
  Record262(Record262),
  Record263(Record263),
  Record264(Record264),
  Record265(Record265),
  Record266(Record266),
  Record267(Record267),
  Record268(Record268),
  Record269(Record269),
  Record270(Record270),
  Record271(Record271),
  Record272(Record272),
  Record273(Record273),
  Record274(Record274),
  Record275(Record275),
  Record276(Record276),
  Record277(Record277),
  Record278(Record278),
  Record279(Record279),
  Record280(Record280),
  Record281(Record281),
  Record282(Record282),
  Record283(Record283),
  Record284(Record284),
  Record285(Record285),
  Record286(Record286),
  Record287(Record287),
  Record288(Record288),
  Record289(Record289),
  Record290(Record290),
  Record291(Record291),
  Record292(Record292),
  Record293(Record293),
  Record294(Record294),
  Record295(Record295),
  Record296(Record296),
  Record297(Record297),
  Record298(Record298),
  Record299(Record299),
  Record300(Record300),
  Record301(Record301),
  Record302(Record302),
  Record303(Record303),
  Record304(Record304),
  Record305(Record305),
  Record306(Record306),
  Record307(Record307),
  Record308(Record308),
  Record309(Record309),
  Record310(Record310),
  Record311(Record311),
  Record312(Record312),
  Record313(Record313),
  Record314(Record314),
  Record315(Record315),
  Record316(Record316),
  Record317(Record317),
  Record318(Record318),
  Record319(Record319),
  Record320(Record320),
  Record321(Record321),
  Record322(Record322),
  Record323(Record323),
  Record324(Record324),
  Record325(Record325),
  Record326(Record326),
  Record327(Record327),
  Record328(Record328),
  Record329(Record329),
  Record330(Record330),
  Record331(Record331),
  Record332(Record332),
  Record333(Record333),
  Record334(Record334),
  Record335(Record335),
  Record336(Record336),
  Record337(Record337),
  Record338(Record338),
  Record339(Record339),
  Record340(Record340),
  Record341(Record341),
  Record342(Record342),
  Record343(Record343),
  Record344(Record344),
  Record345(Record345),
  Record346(Record346),
  Record347(Record347),
  Record348(Record348),
  Record349(Record349),
  Record350(Record350),
  Record351(Record351),
  Record352(Record352),
  Record353(Record353),
  Record354(Record354),
}

// Using thread-local storage for FFI compatibility
use std::cell::RefCell;

thread_local!(static RECORD_QUEUE_0: RefCell<Vec<Record0>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_1: RefCell<Vec<Record1>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_2: RefCell<Vec<Record2>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_3: RefCell<Vec<Record3>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_4: RefCell<Vec<Record4>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_5: RefCell<Vec<Record5>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_6: RefCell<Vec<Record6>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_7: RefCell<Vec<Record7>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_8: RefCell<Vec<Record8>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_9: RefCell<Vec<Record9>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_10: RefCell<Vec<Record10>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_11: RefCell<Vec<Record11>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_12: RefCell<Vec<Record12>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_13: RefCell<Vec<Record13>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_14: RefCell<Vec<Record14>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_15: RefCell<Vec<Record15>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_16: RefCell<Vec<Record16>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_17: RefCell<Vec<Record17>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_18: RefCell<Vec<Record18>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_19: RefCell<Vec<Record19>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_20: RefCell<Vec<Record20>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_21: RefCell<Vec<Record21>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_22: RefCell<Vec<Record22>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_23: RefCell<Vec<Record23>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_24: RefCell<Vec<Record24>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_25: RefCell<Vec<Record25>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_26: RefCell<Vec<Record26>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_27: RefCell<Vec<Record27>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_28: RefCell<Vec<Record28>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_29: RefCell<Vec<Record29>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_30: RefCell<Vec<Record30>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_31: RefCell<Vec<Record31>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_32: RefCell<Vec<Record32>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_33: RefCell<Vec<Record33>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_34: RefCell<Vec<Record34>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_35: RefCell<Vec<Record35>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_36: RefCell<Vec<Record36>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_37: RefCell<Vec<Record37>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_38: RefCell<Vec<Record38>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_39: RefCell<Vec<Record39>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_40: RefCell<Vec<Record40>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_41: RefCell<Vec<Record41>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_42: RefCell<Vec<Record42>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_43: RefCell<Vec<Record43>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_44: RefCell<Vec<Record44>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_45: RefCell<Vec<Record45>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_46: RefCell<Vec<Record46>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_47: RefCell<Vec<Record47>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_48: RefCell<Vec<Record48>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_49: RefCell<Vec<Record49>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_50: RefCell<Vec<Record50>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_51: RefCell<Vec<Record51>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_52: RefCell<Vec<Record52>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_53: RefCell<Vec<Record53>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_54: RefCell<Vec<Record54>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_55: RefCell<Vec<Record55>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_56: RefCell<Vec<Record56>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_57: RefCell<Vec<Record57>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_58: RefCell<Vec<Record58>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_59: RefCell<Vec<Record59>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_60: RefCell<Vec<Record60>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_61: RefCell<Vec<Record61>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_62: RefCell<Vec<Record62>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_63: RefCell<Vec<Record63>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_64: RefCell<Vec<Record64>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_65: RefCell<Vec<Record65>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_66: RefCell<Vec<Record66>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_67: RefCell<Vec<Record67>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_68: RefCell<Vec<Record68>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_69: RefCell<Vec<Record69>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_70: RefCell<Vec<Record70>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_71: RefCell<Vec<Record71>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_72: RefCell<Vec<Record72>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_73: RefCell<Vec<Record73>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_74: RefCell<Vec<Record74>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_75: RefCell<Vec<Record75>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_76: RefCell<Vec<Record76>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_77: RefCell<Vec<Record77>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_78: RefCell<Vec<Record78>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_79: RefCell<Vec<Record79>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_80: RefCell<Vec<Record80>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_81: RefCell<Vec<Record81>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_82: RefCell<Vec<Record82>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_83: RefCell<Vec<Record83>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_84: RefCell<Vec<Record84>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_85: RefCell<Vec<Record85>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_86: RefCell<Vec<Record86>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_87: RefCell<Vec<Record87>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_88: RefCell<Vec<Record88>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_89: RefCell<Vec<Record89>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_90: RefCell<Vec<Record90>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_91: RefCell<Vec<Record91>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_92: RefCell<Vec<Record92>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_93: RefCell<Vec<Record93>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_94: RefCell<Vec<Record94>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_95: RefCell<Vec<Record95>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_96: RefCell<Vec<Record96>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_97: RefCell<Vec<Record97>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_98: RefCell<Vec<Record98>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_99: RefCell<Vec<Record99>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_100: RefCell<Vec<Record100>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_101: RefCell<Vec<Record101>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_102: RefCell<Vec<Record102>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_103: RefCell<Vec<Record103>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_104: RefCell<Vec<Record104>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_105: RefCell<Vec<Record105>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_106: RefCell<Vec<Record106>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_107: RefCell<Vec<Record107>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_108: RefCell<Vec<Record108>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_109: RefCell<Vec<Record109>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_110: RefCell<Vec<Record110>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_111: RefCell<Vec<Record111>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_112: RefCell<Vec<Record112>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_113: RefCell<Vec<Record113>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_114: RefCell<Vec<Record114>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_115: RefCell<Vec<Record115>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_116: RefCell<Vec<Record116>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_117: RefCell<Vec<Record117>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_118: RefCell<Vec<Record118>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_119: RefCell<Vec<Record119>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_120: RefCell<Vec<Record120>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_121: RefCell<Vec<Record121>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_122: RefCell<Vec<Record122>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_123: RefCell<Vec<Record123>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_124: RefCell<Vec<Record124>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_125: RefCell<Vec<Record125>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_126: RefCell<Vec<Record126>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_127: RefCell<Vec<Record127>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_128: RefCell<Vec<Record128>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_129: RefCell<Vec<Record129>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_130: RefCell<Vec<Record130>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_131: RefCell<Vec<Record131>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_132: RefCell<Vec<Record132>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_133: RefCell<Vec<Record133>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_134: RefCell<Vec<Record134>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_135: RefCell<Vec<Record135>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_136: RefCell<Vec<Record136>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_137: RefCell<Vec<Record137>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_138: RefCell<Vec<Record138>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_139: RefCell<Vec<Record139>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_140: RefCell<Vec<Record140>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_141: RefCell<Vec<Record141>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_142: RefCell<Vec<Record142>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_143: RefCell<Vec<Record143>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_144: RefCell<Vec<Record144>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_145: RefCell<Vec<Record145>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_146: RefCell<Vec<Record146>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_147: RefCell<Vec<Record147>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_148: RefCell<Vec<Record148>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_149: RefCell<Vec<Record149>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_150: RefCell<Vec<Record150>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_151: RefCell<Vec<Record151>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_152: RefCell<Vec<Record152>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_153: RefCell<Vec<Record153>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_154: RefCell<Vec<Record154>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_155: RefCell<Vec<Record155>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_156: RefCell<Vec<Record156>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_157: RefCell<Vec<Record157>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_158: RefCell<Vec<Record158>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_159: RefCell<Vec<Record159>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_160: RefCell<Vec<Record160>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_161: RefCell<Vec<Record161>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_162: RefCell<Vec<Record162>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_163: RefCell<Vec<Record163>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_164: RefCell<Vec<Record164>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_165: RefCell<Vec<Record165>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_166: RefCell<Vec<Record166>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_167: RefCell<Vec<Record167>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_168: RefCell<Vec<Record168>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_169: RefCell<Vec<Record169>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_170: RefCell<Vec<Record170>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_171: RefCell<Vec<Record171>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_172: RefCell<Vec<Record172>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_173: RefCell<Vec<Record173>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_174: RefCell<Vec<Record174>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_175: RefCell<Vec<Record175>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_176: RefCell<Vec<Record176>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_177: RefCell<Vec<Record177>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_178: RefCell<Vec<Record178>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_179: RefCell<Vec<Record179>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_180: RefCell<Vec<Record180>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_181: RefCell<Vec<Record181>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_182: RefCell<Vec<Record182>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_183: RefCell<Vec<Record183>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_184: RefCell<Vec<Record184>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_185: RefCell<Vec<Record185>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_186: RefCell<Vec<Record186>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_187: RefCell<Vec<Record187>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_188: RefCell<Vec<Record188>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_189: RefCell<Vec<Record189>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_190: RefCell<Vec<Record190>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_191: RefCell<Vec<Record191>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_192: RefCell<Vec<Record192>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_193: RefCell<Vec<Record193>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_194: RefCell<Vec<Record194>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_195: RefCell<Vec<Record195>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_196: RefCell<Vec<Record196>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_197: RefCell<Vec<Record197>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_198: RefCell<Vec<Record198>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_199: RefCell<Vec<Record199>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_200: RefCell<Vec<Record200>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_201: RefCell<Vec<Record201>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_202: RefCell<Vec<Record202>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_203: RefCell<Vec<Record203>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_204: RefCell<Vec<Record204>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_205: RefCell<Vec<Record205>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_206: RefCell<Vec<Record206>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_207: RefCell<Vec<Record207>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_208: RefCell<Vec<Record208>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_209: RefCell<Vec<Record209>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_210: RefCell<Vec<Record210>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_211: RefCell<Vec<Record211>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_212: RefCell<Vec<Record212>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_213: RefCell<Vec<Record213>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_214: RefCell<Vec<Record214>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_215: RefCell<Vec<Record215>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_216: RefCell<Vec<Record216>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_217: RefCell<Vec<Record217>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_218: RefCell<Vec<Record218>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_219: RefCell<Vec<Record219>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_220: RefCell<Vec<Record220>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_221: RefCell<Vec<Record221>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_222: RefCell<Vec<Record222>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_223: RefCell<Vec<Record223>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_224: RefCell<Vec<Record224>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_225: RefCell<Vec<Record225>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_226: RefCell<Vec<Record226>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_227: RefCell<Vec<Record227>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_228: RefCell<Vec<Record228>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_229: RefCell<Vec<Record229>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_230: RefCell<Vec<Record230>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_231: RefCell<Vec<Record231>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_232: RefCell<Vec<Record232>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_233: RefCell<Vec<Record233>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_234: RefCell<Vec<Record234>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_235: RefCell<Vec<Record235>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_236: RefCell<Vec<Record236>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_237: RefCell<Vec<Record237>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_238: RefCell<Vec<Record238>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_239: RefCell<Vec<Record239>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_240: RefCell<Vec<Record240>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_241: RefCell<Vec<Record241>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_242: RefCell<Vec<Record242>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_243: RefCell<Vec<Record243>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_244: RefCell<Vec<Record244>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_245: RefCell<Vec<Record245>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_246: RefCell<Vec<Record246>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_247: RefCell<Vec<Record247>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_248: RefCell<Vec<Record248>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_249: RefCell<Vec<Record249>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_250: RefCell<Vec<Record250>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_251: RefCell<Vec<Record251>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_252: RefCell<Vec<Record252>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_253: RefCell<Vec<Record253>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_254: RefCell<Vec<Record254>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_255: RefCell<Vec<Record255>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_256: RefCell<Vec<Record256>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_257: RefCell<Vec<Record257>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_258: RefCell<Vec<Record258>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_259: RefCell<Vec<Record259>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_260: RefCell<Vec<Record260>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_261: RefCell<Vec<Record261>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_262: RefCell<Vec<Record262>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_263: RefCell<Vec<Record263>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_264: RefCell<Vec<Record264>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_265: RefCell<Vec<Record265>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_266: RefCell<Vec<Record266>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_267: RefCell<Vec<Record267>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_268: RefCell<Vec<Record268>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_269: RefCell<Vec<Record269>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_270: RefCell<Vec<Record270>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_271: RefCell<Vec<Record271>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_272: RefCell<Vec<Record272>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_273: RefCell<Vec<Record273>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_274: RefCell<Vec<Record274>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_275: RefCell<Vec<Record275>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_276: RefCell<Vec<Record276>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_277: RefCell<Vec<Record277>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_278: RefCell<Vec<Record278>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_279: RefCell<Vec<Record279>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_280: RefCell<Vec<Record280>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_281: RefCell<Vec<Record281>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_282: RefCell<Vec<Record282>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_283: RefCell<Vec<Record283>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_284: RefCell<Vec<Record284>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_285: RefCell<Vec<Record285>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_286: RefCell<Vec<Record286>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_287: RefCell<Vec<Record287>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_288: RefCell<Vec<Record288>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_289: RefCell<Vec<Record289>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_290: RefCell<Vec<Record290>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_291: RefCell<Vec<Record291>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_292: RefCell<Vec<Record292>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_293: RefCell<Vec<Record293>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_294: RefCell<Vec<Record294>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_295: RefCell<Vec<Record295>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_296: RefCell<Vec<Record296>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_297: RefCell<Vec<Record297>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_298: RefCell<Vec<Record298>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_299: RefCell<Vec<Record299>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_300: RefCell<Vec<Record300>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_301: RefCell<Vec<Record301>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_302: RefCell<Vec<Record302>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_303: RefCell<Vec<Record303>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_304: RefCell<Vec<Record304>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_305: RefCell<Vec<Record305>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_306: RefCell<Vec<Record306>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_307: RefCell<Vec<Record307>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_308: RefCell<Vec<Record308>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_309: RefCell<Vec<Record309>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_310: RefCell<Vec<Record310>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_311: RefCell<Vec<Record311>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_312: RefCell<Vec<Record312>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_313: RefCell<Vec<Record313>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_314: RefCell<Vec<Record314>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_315: RefCell<Vec<Record315>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_316: RefCell<Vec<Record316>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_317: RefCell<Vec<Record317>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_318: RefCell<Vec<Record318>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_319: RefCell<Vec<Record319>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_320: RefCell<Vec<Record320>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_321: RefCell<Vec<Record321>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_322: RefCell<Vec<Record322>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_323: RefCell<Vec<Record323>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_324: RefCell<Vec<Record324>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_325: RefCell<Vec<Record325>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_326: RefCell<Vec<Record326>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_327: RefCell<Vec<Record327>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_328: RefCell<Vec<Record328>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_329: RefCell<Vec<Record329>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_330: RefCell<Vec<Record330>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_331: RefCell<Vec<Record331>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_332: RefCell<Vec<Record332>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_333: RefCell<Vec<Record333>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_334: RefCell<Vec<Record334>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_335: RefCell<Vec<Record335>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_336: RefCell<Vec<Record336>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_337: RefCell<Vec<Record337>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_338: RefCell<Vec<Record338>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_339: RefCell<Vec<Record339>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_340: RefCell<Vec<Record340>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_341: RefCell<Vec<Record341>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_342: RefCell<Vec<Record342>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_343: RefCell<Vec<Record343>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_344: RefCell<Vec<Record344>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_345: RefCell<Vec<Record345>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_346: RefCell<Vec<Record346>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_347: RefCell<Vec<Record347>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_348: RefCell<Vec<Record348>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_349: RefCell<Vec<Record349>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_350: RefCell<Vec<Record350>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_351: RefCell<Vec<Record351>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_352: RefCell<Vec<Record352>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_353: RefCell<Vec<Record353>> = RefCell::new(Vec::new()));
thread_local!(static RECORD_QUEUE_354: RefCell<Vec<Record354>> = RefCell::new(Vec::new()));

#[no_mangle]
pub extern "C" fn queue_record(record: Record) {
  match record {
    Record::Record0(r) => {
      RECORD_QUEUE_0.with(|tb: &RefCell<Vec<Record0>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record1(r) => {
      RECORD_QUEUE_1.with(|tb: &RefCell<Vec<Record1>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record2(r) => {
      RECORD_QUEUE_2.with(|tb: &RefCell<Vec<Record2>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record3(r) => {
      RECORD_QUEUE_3.with(|tb: &RefCell<Vec<Record3>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record4(r) => {
      RECORD_QUEUE_4.with(|tb: &RefCell<Vec<Record4>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record5(r) => {
      RECORD_QUEUE_5.with(|tb: &RefCell<Vec<Record5>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record6(r) => {
      RECORD_QUEUE_6.with(|tb: &RefCell<Vec<Record6>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record7(r) => {
      RECORD_QUEUE_7.with(|tb: &RefCell<Vec<Record7>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record8(r) => {
      RECORD_QUEUE_8.with(|tb: &RefCell<Vec<Record8>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record9(r) => {
      RECORD_QUEUE_9.with(|tb: &RefCell<Vec<Record9>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record10(r) => {
      RECORD_QUEUE_10.with(|tb: &RefCell<Vec<Record10>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record11(r) => {
      RECORD_QUEUE_11.with(|tb: &RefCell<Vec<Record11>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record12(r) => {
      RECORD_QUEUE_12.with(|tb: &RefCell<Vec<Record12>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record13(r) => {
      RECORD_QUEUE_13.with(|tb: &RefCell<Vec<Record13>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record14(r) => {
      RECORD_QUEUE_14.with(|tb: &RefCell<Vec<Record14>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record15(r) => {
      RECORD_QUEUE_15.with(|tb: &RefCell<Vec<Record15>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record16(r) => {
      RECORD_QUEUE_16.with(|tb: &RefCell<Vec<Record16>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record17(r) => {
      RECORD_QUEUE_17.with(|tb: &RefCell<Vec<Record17>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record18(r) => {
      RECORD_QUEUE_18.with(|tb: &RefCell<Vec<Record18>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record19(r) => {
      RECORD_QUEUE_19.with(|tb: &RefCell<Vec<Record19>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record20(r) => {
      RECORD_QUEUE_20.with(|tb: &RefCell<Vec<Record20>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record21(r) => {
      RECORD_QUEUE_21.with(|tb: &RefCell<Vec<Record21>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record22(r) => {
      RECORD_QUEUE_22.with(|tb: &RefCell<Vec<Record22>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record23(r) => {
      RECORD_QUEUE_23.with(|tb: &RefCell<Vec<Record23>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record24(r) => {
      RECORD_QUEUE_24.with(|tb: &RefCell<Vec<Record24>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record25(r) => {
      RECORD_QUEUE_25.with(|tb: &RefCell<Vec<Record25>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record26(r) => {
      RECORD_QUEUE_26.with(|tb: &RefCell<Vec<Record26>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record27(r) => {
      RECORD_QUEUE_27.with(|tb: &RefCell<Vec<Record27>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record28(r) => {
      RECORD_QUEUE_28.with(|tb: &RefCell<Vec<Record28>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record29(r) => {
      RECORD_QUEUE_29.with(|tb: &RefCell<Vec<Record29>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record30(r) => {
      RECORD_QUEUE_30.with(|tb: &RefCell<Vec<Record30>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record31(r) => {
      RECORD_QUEUE_31.with(|tb: &RefCell<Vec<Record31>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record32(r) => {
      RECORD_QUEUE_32.with(|tb: &RefCell<Vec<Record32>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record33(r) => {
      RECORD_QUEUE_33.with(|tb: &RefCell<Vec<Record33>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record34(r) => {
      RECORD_QUEUE_34.with(|tb: &RefCell<Vec<Record34>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record35(r) => {
      RECORD_QUEUE_35.with(|tb: &RefCell<Vec<Record35>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record36(r) => {
      RECORD_QUEUE_36.with(|tb: &RefCell<Vec<Record36>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record37(r) => {
      RECORD_QUEUE_37.with(|tb: &RefCell<Vec<Record37>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record38(r) => {
      RECORD_QUEUE_38.with(|tb: &RefCell<Vec<Record38>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record39(r) => {
      RECORD_QUEUE_39.with(|tb: &RefCell<Vec<Record39>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record40(r) => {
      RECORD_QUEUE_40.with(|tb: &RefCell<Vec<Record40>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record41(r) => {
      RECORD_QUEUE_41.with(|tb: &RefCell<Vec<Record41>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record42(r) => {
      RECORD_QUEUE_42.with(|tb: &RefCell<Vec<Record42>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record43(r) => {
      RECORD_QUEUE_43.with(|tb: &RefCell<Vec<Record43>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record44(r) => {
      RECORD_QUEUE_44.with(|tb: &RefCell<Vec<Record44>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record45(r) => {
      RECORD_QUEUE_45.with(|tb: &RefCell<Vec<Record45>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record46(r) => {
      RECORD_QUEUE_46.with(|tb: &RefCell<Vec<Record46>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record47(r) => {
      RECORD_QUEUE_47.with(|tb: &RefCell<Vec<Record47>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record48(r) => {
      RECORD_QUEUE_48.with(|tb: &RefCell<Vec<Record48>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record49(r) => {
      RECORD_QUEUE_49.with(|tb: &RefCell<Vec<Record49>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record50(r) => {
      RECORD_QUEUE_50.with(|tb: &RefCell<Vec<Record50>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record51(r) => {
      RECORD_QUEUE_51.with(|tb: &RefCell<Vec<Record51>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record52(r) => {
      RECORD_QUEUE_52.with(|tb: &RefCell<Vec<Record52>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record53(r) => {
      RECORD_QUEUE_53.with(|tb: &RefCell<Vec<Record53>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record54(r) => {
      RECORD_QUEUE_54.with(|tb: &RefCell<Vec<Record54>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record55(r) => {
      RECORD_QUEUE_55.with(|tb: &RefCell<Vec<Record55>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record56(r) => {
      RECORD_QUEUE_56.with(|tb: &RefCell<Vec<Record56>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record57(r) => {
      RECORD_QUEUE_57.with(|tb: &RefCell<Vec<Record57>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record58(r) => {
      RECORD_QUEUE_58.with(|tb: &RefCell<Vec<Record58>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record59(r) => {
      RECORD_QUEUE_59.with(|tb: &RefCell<Vec<Record59>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record60(r) => {
      RECORD_QUEUE_60.with(|tb: &RefCell<Vec<Record60>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record61(r) => {
      RECORD_QUEUE_61.with(|tb: &RefCell<Vec<Record61>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record62(r) => {
      RECORD_QUEUE_62.with(|tb: &RefCell<Vec<Record62>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record63(r) => {
      RECORD_QUEUE_63.with(|tb: &RefCell<Vec<Record63>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record64(r) => {
      RECORD_QUEUE_64.with(|tb: &RefCell<Vec<Record64>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record65(r) => {
      RECORD_QUEUE_65.with(|tb: &RefCell<Vec<Record65>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record66(r) => {
      RECORD_QUEUE_66.with(|tb: &RefCell<Vec<Record66>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record67(r) => {
      RECORD_QUEUE_67.with(|tb: &RefCell<Vec<Record67>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record68(r) => {
      RECORD_QUEUE_68.with(|tb: &RefCell<Vec<Record68>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record69(r) => {
      RECORD_QUEUE_69.with(|tb: &RefCell<Vec<Record69>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record70(r) => {
      RECORD_QUEUE_70.with(|tb: &RefCell<Vec<Record70>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record71(r) => {
      RECORD_QUEUE_71.with(|tb: &RefCell<Vec<Record71>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record72(r) => {
      RECORD_QUEUE_72.with(|tb: &RefCell<Vec<Record72>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record73(r) => {
      RECORD_QUEUE_73.with(|tb: &RefCell<Vec<Record73>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record74(r) => {
      RECORD_QUEUE_74.with(|tb: &RefCell<Vec<Record74>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record75(r) => {
      RECORD_QUEUE_75.with(|tb: &RefCell<Vec<Record75>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record76(r) => {
      RECORD_QUEUE_76.with(|tb: &RefCell<Vec<Record76>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record77(r) => {
      RECORD_QUEUE_77.with(|tb: &RefCell<Vec<Record77>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record78(r) => {
      RECORD_QUEUE_78.with(|tb: &RefCell<Vec<Record78>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record79(r) => {
      RECORD_QUEUE_79.with(|tb: &RefCell<Vec<Record79>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record80(r) => {
      RECORD_QUEUE_80.with(|tb: &RefCell<Vec<Record80>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record81(r) => {
      RECORD_QUEUE_81.with(|tb: &RefCell<Vec<Record81>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record82(r) => {
      RECORD_QUEUE_82.with(|tb: &RefCell<Vec<Record82>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record83(r) => {
      RECORD_QUEUE_83.with(|tb: &RefCell<Vec<Record83>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record84(r) => {
      RECORD_QUEUE_84.with(|tb: &RefCell<Vec<Record84>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record85(r) => {
      RECORD_QUEUE_85.with(|tb: &RefCell<Vec<Record85>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record86(r) => {
      RECORD_QUEUE_86.with(|tb: &RefCell<Vec<Record86>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record87(r) => {
      RECORD_QUEUE_87.with(|tb: &RefCell<Vec<Record87>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record88(r) => {
      RECORD_QUEUE_88.with(|tb: &RefCell<Vec<Record88>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record89(r) => {
      RECORD_QUEUE_89.with(|tb: &RefCell<Vec<Record89>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record90(r) => {
      RECORD_QUEUE_90.with(|tb: &RefCell<Vec<Record90>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record91(r) => {
      RECORD_QUEUE_91.with(|tb: &RefCell<Vec<Record91>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record92(r) => {
      RECORD_QUEUE_92.with(|tb: &RefCell<Vec<Record92>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record93(r) => {
      RECORD_QUEUE_93.with(|tb: &RefCell<Vec<Record93>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record94(r) => {
      RECORD_QUEUE_94.with(|tb: &RefCell<Vec<Record94>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record95(r) => {
      RECORD_QUEUE_95.with(|tb: &RefCell<Vec<Record95>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record96(r) => {
      RECORD_QUEUE_96.with(|tb: &RefCell<Vec<Record96>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record97(r) => {
      RECORD_QUEUE_97.with(|tb: &RefCell<Vec<Record97>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record98(r) => {
      RECORD_QUEUE_98.with(|tb: &RefCell<Vec<Record98>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record99(r) => {
      RECORD_QUEUE_99.with(|tb: &RefCell<Vec<Record99>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record100(r) => {
      RECORD_QUEUE_100.with(|tb: &RefCell<Vec<Record100>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record101(r) => {
      RECORD_QUEUE_101.with(|tb: &RefCell<Vec<Record101>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record102(r) => {
      RECORD_QUEUE_102.with(|tb: &RefCell<Vec<Record102>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record103(r) => {
      RECORD_QUEUE_103.with(|tb: &RefCell<Vec<Record103>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record104(r) => {
      RECORD_QUEUE_104.with(|tb: &RefCell<Vec<Record104>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record105(r) => {
      RECORD_QUEUE_105.with(|tb: &RefCell<Vec<Record105>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record106(r) => {
      RECORD_QUEUE_106.with(|tb: &RefCell<Vec<Record106>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record107(r) => {
      RECORD_QUEUE_107.with(|tb: &RefCell<Vec<Record107>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record108(r) => {
      RECORD_QUEUE_108.with(|tb: &RefCell<Vec<Record108>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record109(r) => {
      RECORD_QUEUE_109.with(|tb: &RefCell<Vec<Record109>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record110(r) => {
      RECORD_QUEUE_110.with(|tb: &RefCell<Vec<Record110>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record111(r) => {
      RECORD_QUEUE_111.with(|tb: &RefCell<Vec<Record111>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record112(r) => {
      RECORD_QUEUE_112.with(|tb: &RefCell<Vec<Record112>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record113(r) => {
      RECORD_QUEUE_113.with(|tb: &RefCell<Vec<Record113>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record114(r) => {
      RECORD_QUEUE_114.with(|tb: &RefCell<Vec<Record114>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record115(r) => {
      RECORD_QUEUE_115.with(|tb: &RefCell<Vec<Record115>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record116(r) => {
      RECORD_QUEUE_116.with(|tb: &RefCell<Vec<Record116>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record117(r) => {
      RECORD_QUEUE_117.with(|tb: &RefCell<Vec<Record117>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record118(r) => {
      RECORD_QUEUE_118.with(|tb: &RefCell<Vec<Record118>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record119(r) => {
      RECORD_QUEUE_119.with(|tb: &RefCell<Vec<Record119>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record120(r) => {
      RECORD_QUEUE_120.with(|tb: &RefCell<Vec<Record120>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record121(r) => {
      RECORD_QUEUE_121.with(|tb: &RefCell<Vec<Record121>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record122(r) => {
      RECORD_QUEUE_122.with(|tb: &RefCell<Vec<Record122>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record123(r) => {
      RECORD_QUEUE_123.with(|tb: &RefCell<Vec<Record123>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record124(r) => {
      RECORD_QUEUE_124.with(|tb: &RefCell<Vec<Record124>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record125(r) => {
      RECORD_QUEUE_125.with(|tb: &RefCell<Vec<Record125>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record126(r) => {
      RECORD_QUEUE_126.with(|tb: &RefCell<Vec<Record126>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record127(r) => {
      RECORD_QUEUE_127.with(|tb: &RefCell<Vec<Record127>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record128(r) => {
      RECORD_QUEUE_128.with(|tb: &RefCell<Vec<Record128>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record129(r) => {
      RECORD_QUEUE_129.with(|tb: &RefCell<Vec<Record129>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record130(r) => {
      RECORD_QUEUE_130.with(|tb: &RefCell<Vec<Record130>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record131(r) => {
      RECORD_QUEUE_131.with(|tb: &RefCell<Vec<Record131>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record132(r) => {
      RECORD_QUEUE_132.with(|tb: &RefCell<Vec<Record132>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record133(r) => {
      RECORD_QUEUE_133.with(|tb: &RefCell<Vec<Record133>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record134(r) => {
      RECORD_QUEUE_134.with(|tb: &RefCell<Vec<Record134>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record135(r) => {
      RECORD_QUEUE_135.with(|tb: &RefCell<Vec<Record135>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record136(r) => {
      RECORD_QUEUE_136.with(|tb: &RefCell<Vec<Record136>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record137(r) => {
      RECORD_QUEUE_137.with(|tb: &RefCell<Vec<Record137>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record138(r) => {
      RECORD_QUEUE_138.with(|tb: &RefCell<Vec<Record138>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record139(r) => {
      RECORD_QUEUE_139.with(|tb: &RefCell<Vec<Record139>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record140(r) => {
      RECORD_QUEUE_140.with(|tb: &RefCell<Vec<Record140>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record141(r) => {
      RECORD_QUEUE_141.with(|tb: &RefCell<Vec<Record141>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record142(r) => {
      RECORD_QUEUE_142.with(|tb: &RefCell<Vec<Record142>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record143(r) => {
      RECORD_QUEUE_143.with(|tb: &RefCell<Vec<Record143>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record144(r) => {
      RECORD_QUEUE_144.with(|tb: &RefCell<Vec<Record144>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record145(r) => {
      RECORD_QUEUE_145.with(|tb: &RefCell<Vec<Record145>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record146(r) => {
      RECORD_QUEUE_146.with(|tb: &RefCell<Vec<Record146>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record147(r) => {
      RECORD_QUEUE_147.with(|tb: &RefCell<Vec<Record147>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record148(r) => {
      RECORD_QUEUE_148.with(|tb: &RefCell<Vec<Record148>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record149(r) => {
      RECORD_QUEUE_149.with(|tb: &RefCell<Vec<Record149>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record150(r) => {
      RECORD_QUEUE_150.with(|tb: &RefCell<Vec<Record150>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record151(r) => {
      RECORD_QUEUE_151.with(|tb: &RefCell<Vec<Record151>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record152(r) => {
      RECORD_QUEUE_152.with(|tb: &RefCell<Vec<Record152>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record153(r) => {
      RECORD_QUEUE_153.with(|tb: &RefCell<Vec<Record153>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record154(r) => {
      RECORD_QUEUE_154.with(|tb: &RefCell<Vec<Record154>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record155(r) => {
      RECORD_QUEUE_155.with(|tb: &RefCell<Vec<Record155>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record156(r) => {
      RECORD_QUEUE_156.with(|tb: &RefCell<Vec<Record156>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record157(r) => {
      RECORD_QUEUE_157.with(|tb: &RefCell<Vec<Record157>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record158(r) => {
      RECORD_QUEUE_158.with(|tb: &RefCell<Vec<Record158>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record159(r) => {
      RECORD_QUEUE_159.with(|tb: &RefCell<Vec<Record159>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record160(r) => {
      RECORD_QUEUE_160.with(|tb: &RefCell<Vec<Record160>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record161(r) => {
      RECORD_QUEUE_161.with(|tb: &RefCell<Vec<Record161>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record162(r) => {
      RECORD_QUEUE_162.with(|tb: &RefCell<Vec<Record162>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record163(r) => {
      RECORD_QUEUE_163.with(|tb: &RefCell<Vec<Record163>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record164(r) => {
      RECORD_QUEUE_164.with(|tb: &RefCell<Vec<Record164>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record165(r) => {
      RECORD_QUEUE_165.with(|tb: &RefCell<Vec<Record165>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record166(r) => {
      RECORD_QUEUE_166.with(|tb: &RefCell<Vec<Record166>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record167(r) => {
      RECORD_QUEUE_167.with(|tb: &RefCell<Vec<Record167>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record168(r) => {
      RECORD_QUEUE_168.with(|tb: &RefCell<Vec<Record168>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record169(r) => {
      RECORD_QUEUE_169.with(|tb: &RefCell<Vec<Record169>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record170(r) => {
      RECORD_QUEUE_170.with(|tb: &RefCell<Vec<Record170>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record171(r) => {
      RECORD_QUEUE_171.with(|tb: &RefCell<Vec<Record171>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record172(r) => {
      RECORD_QUEUE_172.with(|tb: &RefCell<Vec<Record172>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record173(r) => {
      RECORD_QUEUE_173.with(|tb: &RefCell<Vec<Record173>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record174(r) => {
      RECORD_QUEUE_174.with(|tb: &RefCell<Vec<Record174>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record175(r) => {
      RECORD_QUEUE_175.with(|tb: &RefCell<Vec<Record175>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record176(r) => {
      RECORD_QUEUE_176.with(|tb: &RefCell<Vec<Record176>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record177(r) => {
      RECORD_QUEUE_177.with(|tb: &RefCell<Vec<Record177>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record178(r) => {
      RECORD_QUEUE_178.with(|tb: &RefCell<Vec<Record178>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record179(r) => {
      RECORD_QUEUE_179.with(|tb: &RefCell<Vec<Record179>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record180(r) => {
      RECORD_QUEUE_180.with(|tb: &RefCell<Vec<Record180>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record181(r) => {
      RECORD_QUEUE_181.with(|tb: &RefCell<Vec<Record181>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record182(r) => {
      RECORD_QUEUE_182.with(|tb: &RefCell<Vec<Record182>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record183(r) => {
      RECORD_QUEUE_183.with(|tb: &RefCell<Vec<Record183>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record184(r) => {
      RECORD_QUEUE_184.with(|tb: &RefCell<Vec<Record184>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record185(r) => {
      RECORD_QUEUE_185.with(|tb: &RefCell<Vec<Record185>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record186(r) => {
      RECORD_QUEUE_186.with(|tb: &RefCell<Vec<Record186>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record187(r) => {
      RECORD_QUEUE_187.with(|tb: &RefCell<Vec<Record187>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record188(r) => {
      RECORD_QUEUE_188.with(|tb: &RefCell<Vec<Record188>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record189(r) => {
      RECORD_QUEUE_189.with(|tb: &RefCell<Vec<Record189>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record190(r) => {
      RECORD_QUEUE_190.with(|tb: &RefCell<Vec<Record190>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record191(r) => {
      RECORD_QUEUE_191.with(|tb: &RefCell<Vec<Record191>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record192(r) => {
      RECORD_QUEUE_192.with(|tb: &RefCell<Vec<Record192>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record193(r) => {
      RECORD_QUEUE_193.with(|tb: &RefCell<Vec<Record193>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record194(r) => {
      RECORD_QUEUE_194.with(|tb: &RefCell<Vec<Record194>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record195(r) => {
      RECORD_QUEUE_195.with(|tb: &RefCell<Vec<Record195>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record196(r) => {
      RECORD_QUEUE_196.with(|tb: &RefCell<Vec<Record196>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record197(r) => {
      RECORD_QUEUE_197.with(|tb: &RefCell<Vec<Record197>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record198(r) => {
      RECORD_QUEUE_198.with(|tb: &RefCell<Vec<Record198>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record199(r) => {
      RECORD_QUEUE_199.with(|tb: &RefCell<Vec<Record199>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record200(r) => {
      RECORD_QUEUE_200.with(|tb: &RefCell<Vec<Record200>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record201(r) => {
      RECORD_QUEUE_201.with(|tb: &RefCell<Vec<Record201>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record202(r) => {
      RECORD_QUEUE_202.with(|tb: &RefCell<Vec<Record202>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record203(r) => {
      RECORD_QUEUE_203.with(|tb: &RefCell<Vec<Record203>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record204(r) => {
      RECORD_QUEUE_204.with(|tb: &RefCell<Vec<Record204>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record205(r) => {
      RECORD_QUEUE_205.with(|tb: &RefCell<Vec<Record205>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record206(r) => {
      RECORD_QUEUE_206.with(|tb: &RefCell<Vec<Record206>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record207(r) => {
      RECORD_QUEUE_207.with(|tb: &RefCell<Vec<Record207>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record208(r) => {
      RECORD_QUEUE_208.with(|tb: &RefCell<Vec<Record208>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record209(r) => {
      RECORD_QUEUE_209.with(|tb: &RefCell<Vec<Record209>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record210(r) => {
      RECORD_QUEUE_210.with(|tb: &RefCell<Vec<Record210>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record211(r) => {
      RECORD_QUEUE_211.with(|tb: &RefCell<Vec<Record211>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record212(r) => {
      RECORD_QUEUE_212.with(|tb: &RefCell<Vec<Record212>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record213(r) => {
      RECORD_QUEUE_213.with(|tb: &RefCell<Vec<Record213>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record214(r) => {
      RECORD_QUEUE_214.with(|tb: &RefCell<Vec<Record214>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record215(r) => {
      RECORD_QUEUE_215.with(|tb: &RefCell<Vec<Record215>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record216(r) => {
      RECORD_QUEUE_216.with(|tb: &RefCell<Vec<Record216>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record217(r) => {
      RECORD_QUEUE_217.with(|tb: &RefCell<Vec<Record217>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record218(r) => {
      RECORD_QUEUE_218.with(|tb: &RefCell<Vec<Record218>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record219(r) => {
      RECORD_QUEUE_219.with(|tb: &RefCell<Vec<Record219>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record220(r) => {
      RECORD_QUEUE_220.with(|tb: &RefCell<Vec<Record220>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record221(r) => {
      RECORD_QUEUE_221.with(|tb: &RefCell<Vec<Record221>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record222(r) => {
      RECORD_QUEUE_222.with(|tb: &RefCell<Vec<Record222>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record223(r) => {
      RECORD_QUEUE_223.with(|tb: &RefCell<Vec<Record223>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record224(r) => {
      RECORD_QUEUE_224.with(|tb: &RefCell<Vec<Record224>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record225(r) => {
      RECORD_QUEUE_225.with(|tb: &RefCell<Vec<Record225>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record226(r) => {
      RECORD_QUEUE_226.with(|tb: &RefCell<Vec<Record226>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record227(r) => {
      RECORD_QUEUE_227.with(|tb: &RefCell<Vec<Record227>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record228(r) => {
      RECORD_QUEUE_228.with(|tb: &RefCell<Vec<Record228>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record229(r) => {
      RECORD_QUEUE_229.with(|tb: &RefCell<Vec<Record229>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record230(r) => {
      RECORD_QUEUE_230.with(|tb: &RefCell<Vec<Record230>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record231(r) => {
      RECORD_QUEUE_231.with(|tb: &RefCell<Vec<Record231>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record232(r) => {
      RECORD_QUEUE_232.with(|tb: &RefCell<Vec<Record232>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record233(r) => {
      RECORD_QUEUE_233.with(|tb: &RefCell<Vec<Record233>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record234(r) => {
      RECORD_QUEUE_234.with(|tb: &RefCell<Vec<Record234>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record235(r) => {
      RECORD_QUEUE_235.with(|tb: &RefCell<Vec<Record235>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record236(r) => {
      RECORD_QUEUE_236.with(|tb: &RefCell<Vec<Record236>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record237(r) => {
      RECORD_QUEUE_237.with(|tb: &RefCell<Vec<Record237>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record238(r) => {
      RECORD_QUEUE_238.with(|tb: &RefCell<Vec<Record238>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record239(r) => {
      RECORD_QUEUE_239.with(|tb: &RefCell<Vec<Record239>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record240(r) => {
      RECORD_QUEUE_240.with(|tb: &RefCell<Vec<Record240>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record241(r) => {
      RECORD_QUEUE_241.with(|tb: &RefCell<Vec<Record241>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record242(r) => {
      RECORD_QUEUE_242.with(|tb: &RefCell<Vec<Record242>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record243(r) => {
      RECORD_QUEUE_243.with(|tb: &RefCell<Vec<Record243>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record244(r) => {
      RECORD_QUEUE_244.with(|tb: &RefCell<Vec<Record244>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record245(r) => {
      RECORD_QUEUE_245.with(|tb: &RefCell<Vec<Record245>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record246(r) => {
      RECORD_QUEUE_246.with(|tb: &RefCell<Vec<Record246>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record247(r) => {
      RECORD_QUEUE_247.with(|tb: &RefCell<Vec<Record247>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record248(r) => {
      RECORD_QUEUE_248.with(|tb: &RefCell<Vec<Record248>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record249(r) => {
      RECORD_QUEUE_249.with(|tb: &RefCell<Vec<Record249>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record250(r) => {
      RECORD_QUEUE_250.with(|tb: &RefCell<Vec<Record250>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record251(r) => {
      RECORD_QUEUE_251.with(|tb: &RefCell<Vec<Record251>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record252(r) => {
      RECORD_QUEUE_252.with(|tb: &RefCell<Vec<Record252>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record253(r) => {
      RECORD_QUEUE_253.with(|tb: &RefCell<Vec<Record253>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record254(r) => {
      RECORD_QUEUE_254.with(|tb: &RefCell<Vec<Record254>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record255(r) => {
      RECORD_QUEUE_255.with(|tb: &RefCell<Vec<Record255>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record256(r) => {
      RECORD_QUEUE_256.with(|tb: &RefCell<Vec<Record256>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record257(r) => {
      RECORD_QUEUE_257.with(|tb: &RefCell<Vec<Record257>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record258(r) => {
      RECORD_QUEUE_258.with(|tb: &RefCell<Vec<Record258>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record259(r) => {
      RECORD_QUEUE_259.with(|tb: &RefCell<Vec<Record259>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record260(r) => {
      RECORD_QUEUE_260.with(|tb: &RefCell<Vec<Record260>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record261(r) => {
      RECORD_QUEUE_261.with(|tb: &RefCell<Vec<Record261>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record262(r) => {
      RECORD_QUEUE_262.with(|tb: &RefCell<Vec<Record262>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record263(r) => {
      RECORD_QUEUE_263.with(|tb: &RefCell<Vec<Record263>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record264(r) => {
      RECORD_QUEUE_264.with(|tb: &RefCell<Vec<Record264>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record265(r) => {
      RECORD_QUEUE_265.with(|tb: &RefCell<Vec<Record265>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record266(r) => {
      RECORD_QUEUE_266.with(|tb: &RefCell<Vec<Record266>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record267(r) => {
      RECORD_QUEUE_267.with(|tb: &RefCell<Vec<Record267>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record268(r) => {
      RECORD_QUEUE_268.with(|tb: &RefCell<Vec<Record268>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record269(r) => {
      RECORD_QUEUE_269.with(|tb: &RefCell<Vec<Record269>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record270(r) => {
      RECORD_QUEUE_270.with(|tb: &RefCell<Vec<Record270>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record271(r) => {
      RECORD_QUEUE_271.with(|tb: &RefCell<Vec<Record271>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record272(r) => {
      RECORD_QUEUE_272.with(|tb: &RefCell<Vec<Record272>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record273(r) => {
      RECORD_QUEUE_273.with(|tb: &RefCell<Vec<Record273>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record274(r) => {
      RECORD_QUEUE_274.with(|tb: &RefCell<Vec<Record274>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record275(r) => {
      RECORD_QUEUE_275.with(|tb: &RefCell<Vec<Record275>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record276(r) => {
      RECORD_QUEUE_276.with(|tb: &RefCell<Vec<Record276>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record277(r) => {
      RECORD_QUEUE_277.with(|tb: &RefCell<Vec<Record277>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record278(r) => {
      RECORD_QUEUE_278.with(|tb: &RefCell<Vec<Record278>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record279(r) => {
      RECORD_QUEUE_279.with(|tb: &RefCell<Vec<Record279>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record280(r) => {
      RECORD_QUEUE_280.with(|tb: &RefCell<Vec<Record280>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record281(r) => {
      RECORD_QUEUE_281.with(|tb: &RefCell<Vec<Record281>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record282(r) => {
      RECORD_QUEUE_282.with(|tb: &RefCell<Vec<Record282>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record283(r) => {
      RECORD_QUEUE_283.with(|tb: &RefCell<Vec<Record283>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record284(r) => {
      RECORD_QUEUE_284.with(|tb: &RefCell<Vec<Record284>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record285(r) => {
      RECORD_QUEUE_285.with(|tb: &RefCell<Vec<Record285>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record286(r) => {
      RECORD_QUEUE_286.with(|tb: &RefCell<Vec<Record286>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record287(r) => {
      RECORD_QUEUE_287.with(|tb: &RefCell<Vec<Record287>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record288(r) => {
      RECORD_QUEUE_288.with(|tb: &RefCell<Vec<Record288>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record289(r) => {
      RECORD_QUEUE_289.with(|tb: &RefCell<Vec<Record289>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record290(r) => {
      RECORD_QUEUE_290.with(|tb: &RefCell<Vec<Record290>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record291(r) => {
      RECORD_QUEUE_291.with(|tb: &RefCell<Vec<Record291>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record292(r) => {
      RECORD_QUEUE_292.with(|tb: &RefCell<Vec<Record292>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record293(r) => {
      RECORD_QUEUE_293.with(|tb: &RefCell<Vec<Record293>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record294(r) => {
      RECORD_QUEUE_294.with(|tb: &RefCell<Vec<Record294>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record295(r) => {
      RECORD_QUEUE_295.with(|tb: &RefCell<Vec<Record295>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record296(r) => {
      RECORD_QUEUE_296.with(|tb: &RefCell<Vec<Record296>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record297(r) => {
      RECORD_QUEUE_297.with(|tb: &RefCell<Vec<Record297>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record298(r) => {
      RECORD_QUEUE_298.with(|tb: &RefCell<Vec<Record298>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record299(r) => {
      RECORD_QUEUE_299.with(|tb: &RefCell<Vec<Record299>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record300(r) => {
      RECORD_QUEUE_300.with(|tb: &RefCell<Vec<Record300>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record301(r) => {
      RECORD_QUEUE_301.with(|tb: &RefCell<Vec<Record301>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record302(r) => {
      RECORD_QUEUE_302.with(|tb: &RefCell<Vec<Record302>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record303(r) => {
      RECORD_QUEUE_303.with(|tb: &RefCell<Vec<Record303>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record304(r) => {
      RECORD_QUEUE_304.with(|tb: &RefCell<Vec<Record304>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record305(r) => {
      RECORD_QUEUE_305.with(|tb: &RefCell<Vec<Record305>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record306(r) => {
      RECORD_QUEUE_306.with(|tb: &RefCell<Vec<Record306>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record307(r) => {
      RECORD_QUEUE_307.with(|tb: &RefCell<Vec<Record307>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record308(r) => {
      RECORD_QUEUE_308.with(|tb: &RefCell<Vec<Record308>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record309(r) => {
      RECORD_QUEUE_309.with(|tb: &RefCell<Vec<Record309>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record310(r) => {
      RECORD_QUEUE_310.with(|tb: &RefCell<Vec<Record310>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record311(r) => {
      RECORD_QUEUE_311.with(|tb: &RefCell<Vec<Record311>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record312(r) => {
      RECORD_QUEUE_312.with(|tb: &RefCell<Vec<Record312>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record313(r) => {
      RECORD_QUEUE_313.with(|tb: &RefCell<Vec<Record313>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record314(r) => {
      RECORD_QUEUE_314.with(|tb: &RefCell<Vec<Record314>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record315(r) => {
      RECORD_QUEUE_315.with(|tb: &RefCell<Vec<Record315>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record316(r) => {
      RECORD_QUEUE_316.with(|tb: &RefCell<Vec<Record316>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record317(r) => {
      RECORD_QUEUE_317.with(|tb: &RefCell<Vec<Record317>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record318(r) => {
      RECORD_QUEUE_318.with(|tb: &RefCell<Vec<Record318>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record319(r) => {
      RECORD_QUEUE_319.with(|tb: &RefCell<Vec<Record319>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record320(r) => {
      RECORD_QUEUE_320.with(|tb: &RefCell<Vec<Record320>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record321(r) => {
      RECORD_QUEUE_321.with(|tb: &RefCell<Vec<Record321>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record322(r) => {
      RECORD_QUEUE_322.with(|tb: &RefCell<Vec<Record322>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record323(r) => {
      RECORD_QUEUE_323.with(|tb: &RefCell<Vec<Record323>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record324(r) => {
      RECORD_QUEUE_324.with(|tb: &RefCell<Vec<Record324>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record325(r) => {
      RECORD_QUEUE_325.with(|tb: &RefCell<Vec<Record325>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record326(r) => {
      RECORD_QUEUE_326.with(|tb: &RefCell<Vec<Record326>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record327(r) => {
      RECORD_QUEUE_327.with(|tb: &RefCell<Vec<Record327>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record328(r) => {
      RECORD_QUEUE_328.with(|tb: &RefCell<Vec<Record328>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record329(r) => {
      RECORD_QUEUE_329.with(|tb: &RefCell<Vec<Record329>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record330(r) => {
      RECORD_QUEUE_330.with(|tb: &RefCell<Vec<Record330>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record331(r) => {
      RECORD_QUEUE_331.with(|tb: &RefCell<Vec<Record331>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record332(r) => {
      RECORD_QUEUE_332.with(|tb: &RefCell<Vec<Record332>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record333(r) => {
      RECORD_QUEUE_333.with(|tb: &RefCell<Vec<Record333>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record334(r) => {
      RECORD_QUEUE_334.with(|tb: &RefCell<Vec<Record334>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record335(r) => {
      RECORD_QUEUE_335.with(|tb: &RefCell<Vec<Record335>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record336(r) => {
      RECORD_QUEUE_336.with(|tb: &RefCell<Vec<Record336>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record337(r) => {
      RECORD_QUEUE_337.with(|tb: &RefCell<Vec<Record337>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record338(r) => {
      RECORD_QUEUE_338.with(|tb: &RefCell<Vec<Record338>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record339(r) => {
      RECORD_QUEUE_339.with(|tb: &RefCell<Vec<Record339>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record340(r) => {
      RECORD_QUEUE_340.with(|tb: &RefCell<Vec<Record340>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record341(r) => {
      RECORD_QUEUE_341.with(|tb: &RefCell<Vec<Record341>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record342(r) => {
      RECORD_QUEUE_342.with(|tb: &RefCell<Vec<Record342>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record343(r) => {
      RECORD_QUEUE_343.with(|tb: &RefCell<Vec<Record343>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record344(r) => {
      RECORD_QUEUE_344.with(|tb: &RefCell<Vec<Record344>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record345(r) => {
      RECORD_QUEUE_345.with(|tb: &RefCell<Vec<Record345>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record346(r) => {
      RECORD_QUEUE_346.with(|tb: &RefCell<Vec<Record346>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record347(r) => {
      RECORD_QUEUE_347.with(|tb: &RefCell<Vec<Record347>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record348(r) => {
      RECORD_QUEUE_348.with(|tb: &RefCell<Vec<Record348>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record349(r) => {
      RECORD_QUEUE_349.with(|tb: &RefCell<Vec<Record349>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record350(r) => {
      RECORD_QUEUE_350.with(|tb: &RefCell<Vec<Record350>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record351(r) => {
      RECORD_QUEUE_351.with(|tb: &RefCell<Vec<Record351>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record352(r) => {
      RECORD_QUEUE_352.with(|tb: &RefCell<Vec<Record352>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record353(r) => {
      RECORD_QUEUE_353.with(|tb: &RefCell<Vec<Record353>>| {
        tb.borrow_mut().push(r);
      });
    }
    Record::Record354(r) => {
      RECORD_QUEUE_354.with(|tb: &RefCell<Vec<Record354>>| {
        tb.borrow_mut().push(r);
      });
    }
  }
}

#[no_mangle]
pub extern "C" fn flush_records(db_url: *const c_char) -> bool {
  let db_url = match unsafe { std::ffi::CStr::from_ptr(db_url).to_str() } {
    Ok(s) => s.to_owned(),
    Err(_) => return false,
  };

  let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

  {
    let records_0 = RECORD_QUEUE_0.take();
    if !records_0.is_empty() {
      let mut tb_0 = TableBuilder0::new(&db_url, 1);
      for record in records_0.into_iter() {
        tb_0.push(record);
      }
      let _ = runtime.block_on(tb_0.flush());
    }
  }

  {
    let records_1 = RECORD_QUEUE_1.take();
    if !records_1.is_empty() {
      let mut tb_1 = TableBuilder1::new(&db_url, 1);
      for record in records_1.into_iter() {
        tb_1.push(record);
      }
      let _ = runtime.block_on(tb_1.flush());
    }
  }

  {
    let records_2 = RECORD_QUEUE_2.take();
    if !records_2.is_empty() {
      let mut tb_2 = TableBuilder2::new(&db_url, 1);
      for record in records_2.into_iter() {
        tb_2.push(record);
      }
      let _ = runtime.block_on(tb_2.flush());
    }
  }

  {
    let records_3 = RECORD_QUEUE_3.take();
    if !records_3.is_empty() {
      let mut tb_3 = TableBuilder3::new(&db_url, 1);
      for record in records_3.into_iter() {
        tb_3.push(record);
      }
      let _ = runtime.block_on(tb_3.flush());
    }
  }

  {
    let records_4 = RECORD_QUEUE_4.take();
    if !records_4.is_empty() {
      let mut tb_4 = TableBuilder4::new(&db_url, 1);
      for record in records_4.into_iter() {
        tb_4.push(record);
      }
      let _ = runtime.block_on(tb_4.flush());
    }
  }

  {
    let records_5 = RECORD_QUEUE_5.take();
    if !records_5.is_empty() {
      let mut tb_5 = TableBuilder5::new(&db_url, 1);
      for record in records_5.into_iter() {
        tb_5.push(record);
      }
      let _ = runtime.block_on(tb_5.flush());
    }
  }

  {
    let records_6 = RECORD_QUEUE_6.take();
    if !records_6.is_empty() {
      let mut tb_6 = TableBuilder6::new(&db_url, 1);
      for record in records_6.into_iter() {
        tb_6.push(record);
      }
      let _ = runtime.block_on(tb_6.flush());
    }
  }

  {
    let records_7 = RECORD_QUEUE_7.take();
    if !records_7.is_empty() {
      let mut tb_7 = TableBuilder7::new(&db_url, 1);
      for record in records_7.into_iter() {
        tb_7.push(record);
      }
      let _ = runtime.block_on(tb_7.flush());
    }
  }

  {
    let records_8 = RECORD_QUEUE_8.take();
    if !records_8.is_empty() {
      let mut tb_8 = TableBuilder8::new(&db_url, 1);
      for record in records_8.into_iter() {
        tb_8.push(record);
      }
      let _ = runtime.block_on(tb_8.flush());
    }
  }

  {
    let records_9 = RECORD_QUEUE_9.take();
    if !records_9.is_empty() {
      let mut tb_9 = TableBuilder9::new(&db_url, 1);
      for record in records_9.into_iter() {
        tb_9.push(record);
      }
      let _ = runtime.block_on(tb_9.flush());
    }
  }

  {
    let records_10 = RECORD_QUEUE_10.take();
    if !records_10.is_empty() {
      let mut tb_10 = TableBuilder10::new(&db_url, 1);
      for record in records_10.into_iter() {
        tb_10.push(record);
      }
      let _ = runtime.block_on(tb_10.flush());
    }
  }

  {
    let records_11 = RECORD_QUEUE_11.take();
    if !records_11.is_empty() {
      let mut tb_11 = TableBuilder11::new(&db_url, 1);
      for record in records_11.into_iter() {
        tb_11.push(record);
      }
      let _ = runtime.block_on(tb_11.flush());
    }
  }

  {
    let records_12 = RECORD_QUEUE_12.take();
    if !records_12.is_empty() {
      let mut tb_12 = TableBuilder12::new(&db_url, 1);
      for record in records_12.into_iter() {
        tb_12.push(record);
      }
      let _ = runtime.block_on(tb_12.flush());
    }
  }

  {
    let records_13 = RECORD_QUEUE_13.take();
    if !records_13.is_empty() {
      let mut tb_13 = TableBuilder13::new(&db_url, 1);
      for record in records_13.into_iter() {
        tb_13.push(record);
      }
      let _ = runtime.block_on(tb_13.flush());
    }
  }

  {
    let records_14 = RECORD_QUEUE_14.take();
    if !records_14.is_empty() {
      let mut tb_14 = TableBuilder14::new(&db_url, 1);
      for record in records_14.into_iter() {
        tb_14.push(record);
      }
      let _ = runtime.block_on(tb_14.flush());
    }
  }

  {
    let records_15 = RECORD_QUEUE_15.take();
    if !records_15.is_empty() {
      let mut tb_15 = TableBuilder15::new(&db_url, 1);
      for record in records_15.into_iter() {
        tb_15.push(record);
      }
      let _ = runtime.block_on(tb_15.flush());
    }
  }

  {
    let records_16 = RECORD_QUEUE_16.take();
    if !records_16.is_empty() {
      let mut tb_16 = TableBuilder16::new(&db_url, 1);
      for record in records_16.into_iter() {
        tb_16.push(record);
      }
      let _ = runtime.block_on(tb_16.flush());
    }
  }

  {
    let records_17 = RECORD_QUEUE_17.take();
    if !records_17.is_empty() {
      let mut tb_17 = TableBuilder17::new(&db_url, 1);
      for record in records_17.into_iter() {
        tb_17.push(record);
      }
      let _ = runtime.block_on(tb_17.flush());
    }
  }

  {
    let records_18 = RECORD_QUEUE_18.take();
    if !records_18.is_empty() {
      let mut tb_18 = TableBuilder18::new(&db_url, 1);
      for record in records_18.into_iter() {
        tb_18.push(record);
      }
      let _ = runtime.block_on(tb_18.flush());
    }
  }

  {
    let records_19 = RECORD_QUEUE_19.take();
    if !records_19.is_empty() {
      let mut tb_19 = TableBuilder19::new(&db_url, 1);
      for record in records_19.into_iter() {
        tb_19.push(record);
      }
      let _ = runtime.block_on(tb_19.flush());
    }
  }

  {
    let records_20 = RECORD_QUEUE_20.take();
    if !records_20.is_empty() {
      let mut tb_20 = TableBuilder20::new(&db_url, 1);
      for record in records_20.into_iter() {
        tb_20.push(record);
      }
      let _ = runtime.block_on(tb_20.flush());
    }
  }

  {
    let records_21 = RECORD_QUEUE_21.take();
    if !records_21.is_empty() {
      let mut tb_21 = TableBuilder21::new(&db_url, 1);
      for record in records_21.into_iter() {
        tb_21.push(record);
      }
      let _ = runtime.block_on(tb_21.flush());
    }
  }

  {
    let records_22 = RECORD_QUEUE_22.take();
    if !records_22.is_empty() {
      let mut tb_22 = TableBuilder22::new(&db_url, 1);
      for record in records_22.into_iter() {
        tb_22.push(record);
      }
      let _ = runtime.block_on(tb_22.flush());
    }
  }

  {
    let records_23 = RECORD_QUEUE_23.take();
    if !records_23.is_empty() {
      let mut tb_23 = TableBuilder23::new(&db_url, 1);
      for record in records_23.into_iter() {
        tb_23.push(record);
      }
      let _ = runtime.block_on(tb_23.flush());
    }
  }

  {
    let records_24 = RECORD_QUEUE_24.take();
    if !records_24.is_empty() {
      let mut tb_24 = TableBuilder24::new(&db_url, 1);
      for record in records_24.into_iter() {
        tb_24.push(record);
      }
      let _ = runtime.block_on(tb_24.flush());
    }
  }

  {
    let records_25 = RECORD_QUEUE_25.take();
    if !records_25.is_empty() {
      let mut tb_25 = TableBuilder25::new(&db_url, 1);
      for record in records_25.into_iter() {
        tb_25.push(record);
      }
      let _ = runtime.block_on(tb_25.flush());
    }
  }

  {
    let records_26 = RECORD_QUEUE_26.take();
    if !records_26.is_empty() {
      let mut tb_26 = TableBuilder26::new(&db_url, 1);
      for record in records_26.into_iter() {
        tb_26.push(record);
      }
      let _ = runtime.block_on(tb_26.flush());
    }
  }

  {
    let records_27 = RECORD_QUEUE_27.take();
    if !records_27.is_empty() {
      let mut tb_27 = TableBuilder27::new(&db_url, 1);
      for record in records_27.into_iter() {
        tb_27.push(record);
      }
      let _ = runtime.block_on(tb_27.flush());
    }
  }

  {
    let records_28 = RECORD_QUEUE_28.take();
    if !records_28.is_empty() {
      let mut tb_28 = TableBuilder28::new(&db_url, 1);
      for record in records_28.into_iter() {
        tb_28.push(record);
      }
      let _ = runtime.block_on(tb_28.flush());
    }
  }

  {
    let records_29 = RECORD_QUEUE_29.take();
    if !records_29.is_empty() {
      let mut tb_29 = TableBuilder29::new(&db_url, 1);
      for record in records_29.into_iter() {
        tb_29.push(record);
      }
      let _ = runtime.block_on(tb_29.flush());
    }
  }

  {
    let records_30 = RECORD_QUEUE_30.take();
    if !records_30.is_empty() {
      let mut tb_30 = TableBuilder30::new(&db_url, 1);
      for record in records_30.into_iter() {
        tb_30.push(record);
      }
      let _ = runtime.block_on(tb_30.flush());
    }
  }

  {
    let records_31 = RECORD_QUEUE_31.take();
    if !records_31.is_empty() {
      let mut tb_31 = TableBuilder31::new(&db_url, 1);
      for record in records_31.into_iter() {
        tb_31.push(record);
      }
      let _ = runtime.block_on(tb_31.flush());
    }
  }

  {
    let records_32 = RECORD_QUEUE_32.take();
    if !records_32.is_empty() {
      let mut tb_32 = TableBuilder32::new(&db_url, 1);
      for record in records_32.into_iter() {
        tb_32.push(record);
      }
      let _ = runtime.block_on(tb_32.flush());
    }
  }

  {
    let records_33 = RECORD_QUEUE_33.take();
    if !records_33.is_empty() {
      let mut tb_33 = TableBuilder33::new(&db_url, 1);
      for record in records_33.into_iter() {
        tb_33.push(record);
      }
      let _ = runtime.block_on(tb_33.flush());
    }
  }

  {
    let records_34 = RECORD_QUEUE_34.take();
    if !records_34.is_empty() {
      let mut tb_34 = TableBuilder34::new(&db_url, 1);
      for record in records_34.into_iter() {
        tb_34.push(record);
      }
      let _ = runtime.block_on(tb_34.flush());
    }
  }

  {
    let records_35 = RECORD_QUEUE_35.take();
    if !records_35.is_empty() {
      let mut tb_35 = TableBuilder35::new(&db_url, 1);
      for record in records_35.into_iter() {
        tb_35.push(record);
      }
      let _ = runtime.block_on(tb_35.flush());
    }
  }

  {
    let records_36 = RECORD_QUEUE_36.take();
    if !records_36.is_empty() {
      let mut tb_36 = TableBuilder36::new(&db_url, 1);
      for record in records_36.into_iter() {
        tb_36.push(record);
      }
      let _ = runtime.block_on(tb_36.flush());
    }
  }

  {
    let records_37 = RECORD_QUEUE_37.take();
    if !records_37.is_empty() {
      let mut tb_37 = TableBuilder37::new(&db_url, 1);
      for record in records_37.into_iter() {
        tb_37.push(record);
      }
      let _ = runtime.block_on(tb_37.flush());
    }
  }

  {
    let records_38 = RECORD_QUEUE_38.take();
    if !records_38.is_empty() {
      let mut tb_38 = TableBuilder38::new(&db_url, 1);
      for record in records_38.into_iter() {
        tb_38.push(record);
      }
      let _ = runtime.block_on(tb_38.flush());
    }
  }

  {
    let records_39 = RECORD_QUEUE_39.take();
    if !records_39.is_empty() {
      let mut tb_39 = TableBuilder39::new(&db_url, 1);
      for record in records_39.into_iter() {
        tb_39.push(record);
      }
      let _ = runtime.block_on(tb_39.flush());
    }
  }

  {
    let records_40 = RECORD_QUEUE_40.take();
    if !records_40.is_empty() {
      let mut tb_40 = TableBuilder40::new(&db_url, 1);
      for record in records_40.into_iter() {
        tb_40.push(record);
      }
      let _ = runtime.block_on(tb_40.flush());
    }
  }

  {
    let records_41 = RECORD_QUEUE_41.take();
    if !records_41.is_empty() {
      let mut tb_41 = TableBuilder41::new(&db_url, 1);
      for record in records_41.into_iter() {
        tb_41.push(record);
      }
      let _ = runtime.block_on(tb_41.flush());
    }
  }

  {
    let records_42 = RECORD_QUEUE_42.take();
    if !records_42.is_empty() {
      let mut tb_42 = TableBuilder42::new(&db_url, 1);
      for record in records_42.into_iter() {
        tb_42.push(record);
      }
      let _ = runtime.block_on(tb_42.flush());
    }
  }

  {
    let records_43 = RECORD_QUEUE_43.take();
    if !records_43.is_empty() {
      let mut tb_43 = TableBuilder43::new(&db_url, 1);
      for record in records_43.into_iter() {
        tb_43.push(record);
      }
      let _ = runtime.block_on(tb_43.flush());
    }
  }

  {
    let records_44 = RECORD_QUEUE_44.take();
    if !records_44.is_empty() {
      let mut tb_44 = TableBuilder44::new(&db_url, 1);
      for record in records_44.into_iter() {
        tb_44.push(record);
      }
      let _ = runtime.block_on(tb_44.flush());
    }
  }

  {
    let records_45 = RECORD_QUEUE_45.take();
    if !records_45.is_empty() {
      let mut tb_45 = TableBuilder45::new(&db_url, 1);
      for record in records_45.into_iter() {
        tb_45.push(record);
      }
      let _ = runtime.block_on(tb_45.flush());
    }
  }

  {
    let records_46 = RECORD_QUEUE_46.take();
    if !records_46.is_empty() {
      let mut tb_46 = TableBuilder46::new(&db_url, 1);
      for record in records_46.into_iter() {
        tb_46.push(record);
      }
      let _ = runtime.block_on(tb_46.flush());
    }
  }

  {
    let records_47 = RECORD_QUEUE_47.take();
    if !records_47.is_empty() {
      let mut tb_47 = TableBuilder47::new(&db_url, 1);
      for record in records_47.into_iter() {
        tb_47.push(record);
      }
      let _ = runtime.block_on(tb_47.flush());
    }
  }

  {
    let records_48 = RECORD_QUEUE_48.take();
    if !records_48.is_empty() {
      let mut tb_48 = TableBuilder48::new(&db_url, 1);
      for record in records_48.into_iter() {
        tb_48.push(record);
      }
      let _ = runtime.block_on(tb_48.flush());
    }
  }

  {
    let records_49 = RECORD_QUEUE_49.take();
    if !records_49.is_empty() {
      let mut tb_49 = TableBuilder49::new(&db_url, 1);
      for record in records_49.into_iter() {
        tb_49.push(record);
      }
      let _ = runtime.block_on(tb_49.flush());
    }
  }

  {
    let records_50 = RECORD_QUEUE_50.take();
    if !records_50.is_empty() {
      let mut tb_50 = TableBuilder50::new(&db_url, 1);
      for record in records_50.into_iter() {
        tb_50.push(record);
      }
      let _ = runtime.block_on(tb_50.flush());
    }
  }

  {
    let records_51 = RECORD_QUEUE_51.take();
    if !records_51.is_empty() {
      let mut tb_51 = TableBuilder51::new(&db_url, 1);
      for record in records_51.into_iter() {
        tb_51.push(record);
      }
      let _ = runtime.block_on(tb_51.flush());
    }
  }

  {
    let records_52 = RECORD_QUEUE_52.take();
    if !records_52.is_empty() {
      let mut tb_52 = TableBuilder52::new(&db_url, 1);
      for record in records_52.into_iter() {
        tb_52.push(record);
      }
      let _ = runtime.block_on(tb_52.flush());
    }
  }

  {
    let records_53 = RECORD_QUEUE_53.take();
    if !records_53.is_empty() {
      let mut tb_53 = TableBuilder53::new(&db_url, 1);
      for record in records_53.into_iter() {
        tb_53.push(record);
      }
      let _ = runtime.block_on(tb_53.flush());
    }
  }

  {
    let records_54 = RECORD_QUEUE_54.take();
    if !records_54.is_empty() {
      let mut tb_54 = TableBuilder54::new(&db_url, 1);
      for record in records_54.into_iter() {
        tb_54.push(record);
      }
      let _ = runtime.block_on(tb_54.flush());
    }
  }

  {
    let records_55 = RECORD_QUEUE_55.take();
    if !records_55.is_empty() {
      let mut tb_55 = TableBuilder55::new(&db_url, 1);
      for record in records_55.into_iter() {
        tb_55.push(record);
      }
      let _ = runtime.block_on(tb_55.flush());
    }
  }

  {
    let records_56 = RECORD_QUEUE_56.take();
    if !records_56.is_empty() {
      let mut tb_56 = TableBuilder56::new(&db_url, 1);
      for record in records_56.into_iter() {
        tb_56.push(record);
      }
      let _ = runtime.block_on(tb_56.flush());
    }
  }

  {
    let records_57 = RECORD_QUEUE_57.take();
    if !records_57.is_empty() {
      let mut tb_57 = TableBuilder57::new(&db_url, 1);
      for record in records_57.into_iter() {
        tb_57.push(record);
      }
      let _ = runtime.block_on(tb_57.flush());
    }
  }

  {
    let records_58 = RECORD_QUEUE_58.take();
    if !records_58.is_empty() {
      let mut tb_58 = TableBuilder58::new(&db_url, 1);
      for record in records_58.into_iter() {
        tb_58.push(record);
      }
      let _ = runtime.block_on(tb_58.flush());
    }
  }

  {
    let records_59 = RECORD_QUEUE_59.take();
    if !records_59.is_empty() {
      let mut tb_59 = TableBuilder59::new(&db_url, 1);
      for record in records_59.into_iter() {
        tb_59.push(record);
      }
      let _ = runtime.block_on(tb_59.flush());
    }
  }

  {
    let records_60 = RECORD_QUEUE_60.take();
    if !records_60.is_empty() {
      let mut tb_60 = TableBuilder60::new(&db_url, 1);
      for record in records_60.into_iter() {
        tb_60.push(record);
      }
      let _ = runtime.block_on(tb_60.flush());
    }
  }

  {
    let records_61 = RECORD_QUEUE_61.take();
    if !records_61.is_empty() {
      let mut tb_61 = TableBuilder61::new(&db_url, 1);
      for record in records_61.into_iter() {
        tb_61.push(record);
      }
      let _ = runtime.block_on(tb_61.flush());
    }
  }

  {
    let records_62 = RECORD_QUEUE_62.take();
    if !records_62.is_empty() {
      let mut tb_62 = TableBuilder62::new(&db_url, 1);
      for record in records_62.into_iter() {
        tb_62.push(record);
      }
      let _ = runtime.block_on(tb_62.flush());
    }
  }

  {
    let records_63 = RECORD_QUEUE_63.take();
    if !records_63.is_empty() {
      let mut tb_63 = TableBuilder63::new(&db_url, 1);
      for record in records_63.into_iter() {
        tb_63.push(record);
      }
      let _ = runtime.block_on(tb_63.flush());
    }
  }

  {
    let records_64 = RECORD_QUEUE_64.take();
    if !records_64.is_empty() {
      let mut tb_64 = TableBuilder64::new(&db_url, 1);
      for record in records_64.into_iter() {
        tb_64.push(record);
      }
      let _ = runtime.block_on(tb_64.flush());
    }
  }

  {
    let records_65 = RECORD_QUEUE_65.take();
    if !records_65.is_empty() {
      let mut tb_65 = TableBuilder65::new(&db_url, 1);
      for record in records_65.into_iter() {
        tb_65.push(record);
      }
      let _ = runtime.block_on(tb_65.flush());
    }
  }

  {
    let records_66 = RECORD_QUEUE_66.take();
    if !records_66.is_empty() {
      let mut tb_66 = TableBuilder66::new(&db_url, 1);
      for record in records_66.into_iter() {
        tb_66.push(record);
      }
      let _ = runtime.block_on(tb_66.flush());
    }
  }

  {
    let records_67 = RECORD_QUEUE_67.take();
    if !records_67.is_empty() {
      let mut tb_67 = TableBuilder67::new(&db_url, 1);
      for record in records_67.into_iter() {
        tb_67.push(record);
      }
      let _ = runtime.block_on(tb_67.flush());
    }
  }

  {
    let records_68 = RECORD_QUEUE_68.take();
    if !records_68.is_empty() {
      let mut tb_68 = TableBuilder68::new(&db_url, 1);
      for record in records_68.into_iter() {
        tb_68.push(record);
      }
      let _ = runtime.block_on(tb_68.flush());
    }
  }

  {
    let records_69 = RECORD_QUEUE_69.take();
    if !records_69.is_empty() {
      let mut tb_69 = TableBuilder69::new(&db_url, 1);
      for record in records_69.into_iter() {
        tb_69.push(record);
      }
      let _ = runtime.block_on(tb_69.flush());
    }
  }

  {
    let records_70 = RECORD_QUEUE_70.take();
    if !records_70.is_empty() {
      let mut tb_70 = TableBuilder70::new(&db_url, 1);
      for record in records_70.into_iter() {
        tb_70.push(record);
      }
      let _ = runtime.block_on(tb_70.flush());
    }
  }

  {
    let records_71 = RECORD_QUEUE_71.take();
    if !records_71.is_empty() {
      let mut tb_71 = TableBuilder71::new(&db_url, 1);
      for record in records_71.into_iter() {
        tb_71.push(record);
      }
      let _ = runtime.block_on(tb_71.flush());
    }
  }

  {
    let records_72 = RECORD_QUEUE_72.take();
    if !records_72.is_empty() {
      let mut tb_72 = TableBuilder72::new(&db_url, 1);
      for record in records_72.into_iter() {
        tb_72.push(record);
      }
      let _ = runtime.block_on(tb_72.flush());
    }
  }

  {
    let records_73 = RECORD_QUEUE_73.take();
    if !records_73.is_empty() {
      let mut tb_73 = TableBuilder73::new(&db_url, 1);
      for record in records_73.into_iter() {
        tb_73.push(record);
      }
      let _ = runtime.block_on(tb_73.flush());
    }
  }

  {
    let records_74 = RECORD_QUEUE_74.take();
    if !records_74.is_empty() {
      let mut tb_74 = TableBuilder74::new(&db_url, 1);
      for record in records_74.into_iter() {
        tb_74.push(record);
      }
      let _ = runtime.block_on(tb_74.flush());
    }
  }

  {
    let records_75 = RECORD_QUEUE_75.take();
    if !records_75.is_empty() {
      let mut tb_75 = TableBuilder75::new(&db_url, 1);
      for record in records_75.into_iter() {
        tb_75.push(record);
      }
      let _ = runtime.block_on(tb_75.flush());
    }
  }

  {
    let records_76 = RECORD_QUEUE_76.take();
    if !records_76.is_empty() {
      let mut tb_76 = TableBuilder76::new(&db_url, 1);
      for record in records_76.into_iter() {
        tb_76.push(record);
      }
      let _ = runtime.block_on(tb_76.flush());
    }
  }

  {
    let records_77 = RECORD_QUEUE_77.take();
    if !records_77.is_empty() {
      let mut tb_77 = TableBuilder77::new(&db_url, 1);
      for record in records_77.into_iter() {
        tb_77.push(record);
      }
      let _ = runtime.block_on(tb_77.flush());
    }
  }

  {
    let records_78 = RECORD_QUEUE_78.take();
    if !records_78.is_empty() {
      let mut tb_78 = TableBuilder78::new(&db_url, 1);
      for record in records_78.into_iter() {
        tb_78.push(record);
      }
      let _ = runtime.block_on(tb_78.flush());
    }
  }

  {
    let records_79 = RECORD_QUEUE_79.take();
    if !records_79.is_empty() {
      let mut tb_79 = TableBuilder79::new(&db_url, 1);
      for record in records_79.into_iter() {
        tb_79.push(record);
      }
      let _ = runtime.block_on(tb_79.flush());
    }
  }

  {
    let records_80 = RECORD_QUEUE_80.take();
    if !records_80.is_empty() {
      let mut tb_80 = TableBuilder80::new(&db_url, 1);
      for record in records_80.into_iter() {
        tb_80.push(record);
      }
      let _ = runtime.block_on(tb_80.flush());
    }
  }

  {
    let records_81 = RECORD_QUEUE_81.take();
    if !records_81.is_empty() {
      let mut tb_81 = TableBuilder81::new(&db_url, 1);
      for record in records_81.into_iter() {
        tb_81.push(record);
      }
      let _ = runtime.block_on(tb_81.flush());
    }
  }

  {
    let records_82 = RECORD_QUEUE_82.take();
    if !records_82.is_empty() {
      let mut tb_82 = TableBuilder82::new(&db_url, 1);
      for record in records_82.into_iter() {
        tb_82.push(record);
      }
      let _ = runtime.block_on(tb_82.flush());
    }
  }

  {
    let records_83 = RECORD_QUEUE_83.take();
    if !records_83.is_empty() {
      let mut tb_83 = TableBuilder83::new(&db_url, 1);
      for record in records_83.into_iter() {
        tb_83.push(record);
      }
      let _ = runtime.block_on(tb_83.flush());
    }
  }

  {
    let records_84 = RECORD_QUEUE_84.take();
    if !records_84.is_empty() {
      let mut tb_84 = TableBuilder84::new(&db_url, 1);
      for record in records_84.into_iter() {
        tb_84.push(record);
      }
      let _ = runtime.block_on(tb_84.flush());
    }
  }

  {
    let records_85 = RECORD_QUEUE_85.take();
    if !records_85.is_empty() {
      let mut tb_85 = TableBuilder85::new(&db_url, 1);
      for record in records_85.into_iter() {
        tb_85.push(record);
      }
      let _ = runtime.block_on(tb_85.flush());
    }
  }

  {
    let records_86 = RECORD_QUEUE_86.take();
    if !records_86.is_empty() {
      let mut tb_86 = TableBuilder86::new(&db_url, 1);
      for record in records_86.into_iter() {
        tb_86.push(record);
      }
      let _ = runtime.block_on(tb_86.flush());
    }
  }

  {
    let records_87 = RECORD_QUEUE_87.take();
    if !records_87.is_empty() {
      let mut tb_87 = TableBuilder87::new(&db_url, 1);
      for record in records_87.into_iter() {
        tb_87.push(record);
      }
      let _ = runtime.block_on(tb_87.flush());
    }
  }

  {
    let records_88 = RECORD_QUEUE_88.take();
    if !records_88.is_empty() {
      let mut tb_88 = TableBuilder88::new(&db_url, 1);
      for record in records_88.into_iter() {
        tb_88.push(record);
      }
      let _ = runtime.block_on(tb_88.flush());
    }
  }

  {
    let records_89 = RECORD_QUEUE_89.take();
    if !records_89.is_empty() {
      let mut tb_89 = TableBuilder89::new(&db_url, 1);
      for record in records_89.into_iter() {
        tb_89.push(record);
      }
      let _ = runtime.block_on(tb_89.flush());
    }
  }

  {
    let records_90 = RECORD_QUEUE_90.take();
    if !records_90.is_empty() {
      let mut tb_90 = TableBuilder90::new(&db_url, 1);
      for record in records_90.into_iter() {
        tb_90.push(record);
      }
      let _ = runtime.block_on(tb_90.flush());
    }
  }

  {
    let records_91 = RECORD_QUEUE_91.take();
    if !records_91.is_empty() {
      let mut tb_91 = TableBuilder91::new(&db_url, 1);
      for record in records_91.into_iter() {
        tb_91.push(record);
      }
      let _ = runtime.block_on(tb_91.flush());
    }
  }

  {
    let records_92 = RECORD_QUEUE_92.take();
    if !records_92.is_empty() {
      let mut tb_92 = TableBuilder92::new(&db_url, 1);
      for record in records_92.into_iter() {
        tb_92.push(record);
      }
      let _ = runtime.block_on(tb_92.flush());
    }
  }

  {
    let records_93 = RECORD_QUEUE_93.take();
    if !records_93.is_empty() {
      let mut tb_93 = TableBuilder93::new(&db_url, 1);
      for record in records_93.into_iter() {
        tb_93.push(record);
      }
      let _ = runtime.block_on(tb_93.flush());
    }
  }

  {
    let records_94 = RECORD_QUEUE_94.take();
    if !records_94.is_empty() {
      let mut tb_94 = TableBuilder94::new(&db_url, 1);
      for record in records_94.into_iter() {
        tb_94.push(record);
      }
      let _ = runtime.block_on(tb_94.flush());
    }
  }

  {
    let records_95 = RECORD_QUEUE_95.take();
    if !records_95.is_empty() {
      let mut tb_95 = TableBuilder95::new(&db_url, 1);
      for record in records_95.into_iter() {
        tb_95.push(record);
      }
      let _ = runtime.block_on(tb_95.flush());
    }
  }

  {
    let records_96 = RECORD_QUEUE_96.take();
    if !records_96.is_empty() {
      let mut tb_96 = TableBuilder96::new(&db_url, 1);
      for record in records_96.into_iter() {
        tb_96.push(record);
      }
      let _ = runtime.block_on(tb_96.flush());
    }
  }

  {
    let records_97 = RECORD_QUEUE_97.take();
    if !records_97.is_empty() {
      let mut tb_97 = TableBuilder97::new(&db_url, 1);
      for record in records_97.into_iter() {
        tb_97.push(record);
      }
      let _ = runtime.block_on(tb_97.flush());
    }
  }

  {
    let records_98 = RECORD_QUEUE_98.take();
    if !records_98.is_empty() {
      let mut tb_98 = TableBuilder98::new(&db_url, 1);
      for record in records_98.into_iter() {
        tb_98.push(record);
      }
      let _ = runtime.block_on(tb_98.flush());
    }
  }

  {
    let records_99 = RECORD_QUEUE_99.take();
    if !records_99.is_empty() {
      let mut tb_99 = TableBuilder99::new(&db_url, 1);
      for record in records_99.into_iter() {
        tb_99.push(record);
      }
      let _ = runtime.block_on(tb_99.flush());
    }
  }

  {
    let records_100 = RECORD_QUEUE_100.take();
    if !records_100.is_empty() {
      let mut tb_100 = TableBuilder100::new(&db_url, 1);
      for record in records_100.into_iter() {
        tb_100.push(record);
      }
      let _ = runtime.block_on(tb_100.flush());
    }
  }

  {
    let records_101 = RECORD_QUEUE_101.take();
    if !records_101.is_empty() {
      let mut tb_101 = TableBuilder101::new(&db_url, 1);
      for record in records_101.into_iter() {
        tb_101.push(record);
      }
      let _ = runtime.block_on(tb_101.flush());
    }
  }

  {
    let records_102 = RECORD_QUEUE_102.take();
    if !records_102.is_empty() {
      let mut tb_102 = TableBuilder102::new(&db_url, 1);
      for record in records_102.into_iter() {
        tb_102.push(record);
      }
      let _ = runtime.block_on(tb_102.flush());
    }
  }

  {
    let records_103 = RECORD_QUEUE_103.take();
    if !records_103.is_empty() {
      let mut tb_103 = TableBuilder103::new(&db_url, 1);
      for record in records_103.into_iter() {
        tb_103.push(record);
      }
      let _ = runtime.block_on(tb_103.flush());
    }
  }

  {
    let records_104 = RECORD_QUEUE_104.take();
    if !records_104.is_empty() {
      let mut tb_104 = TableBuilder104::new(&db_url, 1);
      for record in records_104.into_iter() {
        tb_104.push(record);
      }
      let _ = runtime.block_on(tb_104.flush());
    }
  }

  {
    let records_105 = RECORD_QUEUE_105.take();
    if !records_105.is_empty() {
      let mut tb_105 = TableBuilder105::new(&db_url, 1);
      for record in records_105.into_iter() {
        tb_105.push(record);
      }
      let _ = runtime.block_on(tb_105.flush());
    }
  }

  {
    let records_106 = RECORD_QUEUE_106.take();
    if !records_106.is_empty() {
      let mut tb_106 = TableBuilder106::new(&db_url, 1);
      for record in records_106.into_iter() {
        tb_106.push(record);
      }
      let _ = runtime.block_on(tb_106.flush());
    }
  }

  {
    let records_107 = RECORD_QUEUE_107.take();
    if !records_107.is_empty() {
      let mut tb_107 = TableBuilder107::new(&db_url, 1);
      for record in records_107.into_iter() {
        tb_107.push(record);
      }
      let _ = runtime.block_on(tb_107.flush());
    }
  }

  {
    let records_108 = RECORD_QUEUE_108.take();
    if !records_108.is_empty() {
      let mut tb_108 = TableBuilder108::new(&db_url, 1);
      for record in records_108.into_iter() {
        tb_108.push(record);
      }
      let _ = runtime.block_on(tb_108.flush());
    }
  }

  {
    let records_109 = RECORD_QUEUE_109.take();
    if !records_109.is_empty() {
      let mut tb_109 = TableBuilder109::new(&db_url, 1);
      for record in records_109.into_iter() {
        tb_109.push(record);
      }
      let _ = runtime.block_on(tb_109.flush());
    }
  }

  {
    let records_110 = RECORD_QUEUE_110.take();
    if !records_110.is_empty() {
      let mut tb_110 = TableBuilder110::new(&db_url, 1);
      for record in records_110.into_iter() {
        tb_110.push(record);
      }
      let _ = runtime.block_on(tb_110.flush());
    }
  }

  {
    let records_111 = RECORD_QUEUE_111.take();
    if !records_111.is_empty() {
      let mut tb_111 = TableBuilder111::new(&db_url, 1);
      for record in records_111.into_iter() {
        tb_111.push(record);
      }
      let _ = runtime.block_on(tb_111.flush());
    }
  }

  {
    let records_112 = RECORD_QUEUE_112.take();
    if !records_112.is_empty() {
      let mut tb_112 = TableBuilder112::new(&db_url, 1);
      for record in records_112.into_iter() {
        tb_112.push(record);
      }
      let _ = runtime.block_on(tb_112.flush());
    }
  }

  {
    let records_113 = RECORD_QUEUE_113.take();
    if !records_113.is_empty() {
      let mut tb_113 = TableBuilder113::new(&db_url, 1);
      for record in records_113.into_iter() {
        tb_113.push(record);
      }
      let _ = runtime.block_on(tb_113.flush());
    }
  }

  {
    let records_114 = RECORD_QUEUE_114.take();
    if !records_114.is_empty() {
      let mut tb_114 = TableBuilder114::new(&db_url, 1);
      for record in records_114.into_iter() {
        tb_114.push(record);
      }
      let _ = runtime.block_on(tb_114.flush());
    }
  }

  {
    let records_115 = RECORD_QUEUE_115.take();
    if !records_115.is_empty() {
      let mut tb_115 = TableBuilder115::new(&db_url, 1);
      for record in records_115.into_iter() {
        tb_115.push(record);
      }
      let _ = runtime.block_on(tb_115.flush());
    }
  }

  {
    let records_116 = RECORD_QUEUE_116.take();
    if !records_116.is_empty() {
      let mut tb_116 = TableBuilder116::new(&db_url, 1);
      for record in records_116.into_iter() {
        tb_116.push(record);
      }
      let _ = runtime.block_on(tb_116.flush());
    }
  }

  {
    let records_117 = RECORD_QUEUE_117.take();
    if !records_117.is_empty() {
      let mut tb_117 = TableBuilder117::new(&db_url, 1);
      for record in records_117.into_iter() {
        tb_117.push(record);
      }
      let _ = runtime.block_on(tb_117.flush());
    }
  }

  {
    let records_118 = RECORD_QUEUE_118.take();
    if !records_118.is_empty() {
      let mut tb_118 = TableBuilder118::new(&db_url, 1);
      for record in records_118.into_iter() {
        tb_118.push(record);
      }
      let _ = runtime.block_on(tb_118.flush());
    }
  }

  {
    let records_119 = RECORD_QUEUE_119.take();
    if !records_119.is_empty() {
      let mut tb_119 = TableBuilder119::new(&db_url, 1);
      for record in records_119.into_iter() {
        tb_119.push(record);
      }
      let _ = runtime.block_on(tb_119.flush());
    }
  }

  {
    let records_120 = RECORD_QUEUE_120.take();
    if !records_120.is_empty() {
      let mut tb_120 = TableBuilder120::new(&db_url, 1);
      for record in records_120.into_iter() {
        tb_120.push(record);
      }
      let _ = runtime.block_on(tb_120.flush());
    }
  }

  {
    let records_121 = RECORD_QUEUE_121.take();
    if !records_121.is_empty() {
      let mut tb_121 = TableBuilder121::new(&db_url, 1);
      for record in records_121.into_iter() {
        tb_121.push(record);
      }
      let _ = runtime.block_on(tb_121.flush());
    }
  }

  {
    let records_122 = RECORD_QUEUE_122.take();
    if !records_122.is_empty() {
      let mut tb_122 = TableBuilder122::new(&db_url, 1);
      for record in records_122.into_iter() {
        tb_122.push(record);
      }
      let _ = runtime.block_on(tb_122.flush());
    }
  }

  {
    let records_123 = RECORD_QUEUE_123.take();
    if !records_123.is_empty() {
      let mut tb_123 = TableBuilder123::new(&db_url, 1);
      for record in records_123.into_iter() {
        tb_123.push(record);
      }
      let _ = runtime.block_on(tb_123.flush());
    }
  }

  {
    let records_124 = RECORD_QUEUE_124.take();
    if !records_124.is_empty() {
      let mut tb_124 = TableBuilder124::new(&db_url, 1);
      for record in records_124.into_iter() {
        tb_124.push(record);
      }
      let _ = runtime.block_on(tb_124.flush());
    }
  }

  {
    let records_125 = RECORD_QUEUE_125.take();
    if !records_125.is_empty() {
      let mut tb_125 = TableBuilder125::new(&db_url, 1);
      for record in records_125.into_iter() {
        tb_125.push(record);
      }
      let _ = runtime.block_on(tb_125.flush());
    }
  }

  {
    let records_126 = RECORD_QUEUE_126.take();
    if !records_126.is_empty() {
      let mut tb_126 = TableBuilder126::new(&db_url, 1);
      for record in records_126.into_iter() {
        tb_126.push(record);
      }
      let _ = runtime.block_on(tb_126.flush());
    }
  }

  {
    let records_127 = RECORD_QUEUE_127.take();
    if !records_127.is_empty() {
      let mut tb_127 = TableBuilder127::new(&db_url, 1);
      for record in records_127.into_iter() {
        tb_127.push(record);
      }
      let _ = runtime.block_on(tb_127.flush());
    }
  }

  {
    let records_128 = RECORD_QUEUE_128.take();
    if !records_128.is_empty() {
      let mut tb_128 = TableBuilder128::new(&db_url, 1);
      for record in records_128.into_iter() {
        tb_128.push(record);
      }
      let _ = runtime.block_on(tb_128.flush());
    }
  }

  {
    let records_129 = RECORD_QUEUE_129.take();
    if !records_129.is_empty() {
      let mut tb_129 = TableBuilder129::new(&db_url, 1);
      for record in records_129.into_iter() {
        tb_129.push(record);
      }
      let _ = runtime.block_on(tb_129.flush());
    }
  }

  {
    let records_130 = RECORD_QUEUE_130.take();
    if !records_130.is_empty() {
      let mut tb_130 = TableBuilder130::new(&db_url, 1);
      for record in records_130.into_iter() {
        tb_130.push(record);
      }
      let _ = runtime.block_on(tb_130.flush());
    }
  }

  {
    let records_131 = RECORD_QUEUE_131.take();
    if !records_131.is_empty() {
      let mut tb_131 = TableBuilder131::new(&db_url, 1);
      for record in records_131.into_iter() {
        tb_131.push(record);
      }
      let _ = runtime.block_on(tb_131.flush());
    }
  }

  {
    let records_132 = RECORD_QUEUE_132.take();
    if !records_132.is_empty() {
      let mut tb_132 = TableBuilder132::new(&db_url, 1);
      for record in records_132.into_iter() {
        tb_132.push(record);
      }
      let _ = runtime.block_on(tb_132.flush());
    }
  }

  {
    let records_133 = RECORD_QUEUE_133.take();
    if !records_133.is_empty() {
      let mut tb_133 = TableBuilder133::new(&db_url, 1);
      for record in records_133.into_iter() {
        tb_133.push(record);
      }
      let _ = runtime.block_on(tb_133.flush());
    }
  }

  {
    let records_134 = RECORD_QUEUE_134.take();
    if !records_134.is_empty() {
      let mut tb_134 = TableBuilder134::new(&db_url, 1);
      for record in records_134.into_iter() {
        tb_134.push(record);
      }
      let _ = runtime.block_on(tb_134.flush());
    }
  }

  {
    let records_135 = RECORD_QUEUE_135.take();
    if !records_135.is_empty() {
      let mut tb_135 = TableBuilder135::new(&db_url, 1);
      for record in records_135.into_iter() {
        tb_135.push(record);
      }
      let _ = runtime.block_on(tb_135.flush());
    }
  }

  {
    let records_136 = RECORD_QUEUE_136.take();
    if !records_136.is_empty() {
      let mut tb_136 = TableBuilder136::new(&db_url, 1);
      for record in records_136.into_iter() {
        tb_136.push(record);
      }
      let _ = runtime.block_on(tb_136.flush());
    }
  }

  {
    let records_137 = RECORD_QUEUE_137.take();
    if !records_137.is_empty() {
      let mut tb_137 = TableBuilder137::new(&db_url, 1);
      for record in records_137.into_iter() {
        tb_137.push(record);
      }
      let _ = runtime.block_on(tb_137.flush());
    }
  }

  {
    let records_138 = RECORD_QUEUE_138.take();
    if !records_138.is_empty() {
      let mut tb_138 = TableBuilder138::new(&db_url, 1);
      for record in records_138.into_iter() {
        tb_138.push(record);
      }
      let _ = runtime.block_on(tb_138.flush());
    }
  }

  {
    let records_139 = RECORD_QUEUE_139.take();
    if !records_139.is_empty() {
      let mut tb_139 = TableBuilder139::new(&db_url, 1);
      for record in records_139.into_iter() {
        tb_139.push(record);
      }
      let _ = runtime.block_on(tb_139.flush());
    }
  }

  {
    let records_140 = RECORD_QUEUE_140.take();
    if !records_140.is_empty() {
      let mut tb_140 = TableBuilder140::new(&db_url, 1);
      for record in records_140.into_iter() {
        tb_140.push(record);
      }
      let _ = runtime.block_on(tb_140.flush());
    }
  }

  {
    let records_141 = RECORD_QUEUE_141.take();
    if !records_141.is_empty() {
      let mut tb_141 = TableBuilder141::new(&db_url, 1);
      for record in records_141.into_iter() {
        tb_141.push(record);
      }
      let _ = runtime.block_on(tb_141.flush());
    }
  }

  {
    let records_142 = RECORD_QUEUE_142.take();
    if !records_142.is_empty() {
      let mut tb_142 = TableBuilder142::new(&db_url, 1);
      for record in records_142.into_iter() {
        tb_142.push(record);
      }
      let _ = runtime.block_on(tb_142.flush());
    }
  }

  {
    let records_143 = RECORD_QUEUE_143.take();
    if !records_143.is_empty() {
      let mut tb_143 = TableBuilder143::new(&db_url, 1);
      for record in records_143.into_iter() {
        tb_143.push(record);
      }
      let _ = runtime.block_on(tb_143.flush());
    }
  }

  {
    let records_144 = RECORD_QUEUE_144.take();
    if !records_144.is_empty() {
      let mut tb_144 = TableBuilder144::new(&db_url, 1);
      for record in records_144.into_iter() {
        tb_144.push(record);
      }
      let _ = runtime.block_on(tb_144.flush());
    }
  }

  {
    let records_145 = RECORD_QUEUE_145.take();
    if !records_145.is_empty() {
      let mut tb_145 = TableBuilder145::new(&db_url, 1);
      for record in records_145.into_iter() {
        tb_145.push(record);
      }
      let _ = runtime.block_on(tb_145.flush());
    }
  }

  {
    let records_146 = RECORD_QUEUE_146.take();
    if !records_146.is_empty() {
      let mut tb_146 = TableBuilder146::new(&db_url, 1);
      for record in records_146.into_iter() {
        tb_146.push(record);
      }
      let _ = runtime.block_on(tb_146.flush());
    }
  }

  {
    let records_147 = RECORD_QUEUE_147.take();
    if !records_147.is_empty() {
      let mut tb_147 = TableBuilder147::new(&db_url, 1);
      for record in records_147.into_iter() {
        tb_147.push(record);
      }
      let _ = runtime.block_on(tb_147.flush());
    }
  }

  {
    let records_148 = RECORD_QUEUE_148.take();
    if !records_148.is_empty() {
      let mut tb_148 = TableBuilder148::new(&db_url, 1);
      for record in records_148.into_iter() {
        tb_148.push(record);
      }
      let _ = runtime.block_on(tb_148.flush());
    }
  }

  {
    let records_149 = RECORD_QUEUE_149.take();
    if !records_149.is_empty() {
      let mut tb_149 = TableBuilder149::new(&db_url, 1);
      for record in records_149.into_iter() {
        tb_149.push(record);
      }
      let _ = runtime.block_on(tb_149.flush());
    }
  }

  {
    let records_150 = RECORD_QUEUE_150.take();
    if !records_150.is_empty() {
      let mut tb_150 = TableBuilder150::new(&db_url, 1);
      for record in records_150.into_iter() {
        tb_150.push(record);
      }
      let _ = runtime.block_on(tb_150.flush());
    }
  }

  {
    let records_151 = RECORD_QUEUE_151.take();
    if !records_151.is_empty() {
      let mut tb_151 = TableBuilder151::new(&db_url, 1);
      for record in records_151.into_iter() {
        tb_151.push(record);
      }
      let _ = runtime.block_on(tb_151.flush());
    }
  }

  {
    let records_152 = RECORD_QUEUE_152.take();
    if !records_152.is_empty() {
      let mut tb_152 = TableBuilder152::new(&db_url, 1);
      for record in records_152.into_iter() {
        tb_152.push(record);
      }
      let _ = runtime.block_on(tb_152.flush());
    }
  }

  {
    let records_153 = RECORD_QUEUE_153.take();
    if !records_153.is_empty() {
      let mut tb_153 = TableBuilder153::new(&db_url, 1);
      for record in records_153.into_iter() {
        tb_153.push(record);
      }
      let _ = runtime.block_on(tb_153.flush());
    }
  }

  {
    let records_154 = RECORD_QUEUE_154.take();
    if !records_154.is_empty() {
      let mut tb_154 = TableBuilder154::new(&db_url, 1);
      for record in records_154.into_iter() {
        tb_154.push(record);
      }
      let _ = runtime.block_on(tb_154.flush());
    }
  }

  {
    let records_155 = RECORD_QUEUE_155.take();
    if !records_155.is_empty() {
      let mut tb_155 = TableBuilder155::new(&db_url, 1);
      for record in records_155.into_iter() {
        tb_155.push(record);
      }
      let _ = runtime.block_on(tb_155.flush());
    }
  }

  {
    let records_156 = RECORD_QUEUE_156.take();
    if !records_156.is_empty() {
      let mut tb_156 = TableBuilder156::new(&db_url, 1);
      for record in records_156.into_iter() {
        tb_156.push(record);
      }
      let _ = runtime.block_on(tb_156.flush());
    }
  }

  {
    let records_157 = RECORD_QUEUE_157.take();
    if !records_157.is_empty() {
      let mut tb_157 = TableBuilder157::new(&db_url, 1);
      for record in records_157.into_iter() {
        tb_157.push(record);
      }
      let _ = runtime.block_on(tb_157.flush());
    }
  }

  {
    let records_158 = RECORD_QUEUE_158.take();
    if !records_158.is_empty() {
      let mut tb_158 = TableBuilder158::new(&db_url, 1);
      for record in records_158.into_iter() {
        tb_158.push(record);
      }
      let _ = runtime.block_on(tb_158.flush());
    }
  }

  {
    let records_159 = RECORD_QUEUE_159.take();
    if !records_159.is_empty() {
      let mut tb_159 = TableBuilder159::new(&db_url, 1);
      for record in records_159.into_iter() {
        tb_159.push(record);
      }
      let _ = runtime.block_on(tb_159.flush());
    }
  }

  {
    let records_160 = RECORD_QUEUE_160.take();
    if !records_160.is_empty() {
      let mut tb_160 = TableBuilder160::new(&db_url, 1);
      for record in records_160.into_iter() {
        tb_160.push(record);
      }
      let _ = runtime.block_on(tb_160.flush());
    }
  }

  {
    let records_161 = RECORD_QUEUE_161.take();
    if !records_161.is_empty() {
      let mut tb_161 = TableBuilder161::new(&db_url, 1);
      for record in records_161.into_iter() {
        tb_161.push(record);
      }
      let _ = runtime.block_on(tb_161.flush());
    }
  }

  {
    let records_162 = RECORD_QUEUE_162.take();
    if !records_162.is_empty() {
      let mut tb_162 = TableBuilder162::new(&db_url, 1);
      for record in records_162.into_iter() {
        tb_162.push(record);
      }
      let _ = runtime.block_on(tb_162.flush());
    }
  }

  {
    let records_163 = RECORD_QUEUE_163.take();
    if !records_163.is_empty() {
      let mut tb_163 = TableBuilder163::new(&db_url, 1);
      for record in records_163.into_iter() {
        tb_163.push(record);
      }
      let _ = runtime.block_on(tb_163.flush());
    }
  }

  {
    let records_164 = RECORD_QUEUE_164.take();
    if !records_164.is_empty() {
      let mut tb_164 = TableBuilder164::new(&db_url, 1);
      for record in records_164.into_iter() {
        tb_164.push(record);
      }
      let _ = runtime.block_on(tb_164.flush());
    }
  }

  {
    let records_165 = RECORD_QUEUE_165.take();
    if !records_165.is_empty() {
      let mut tb_165 = TableBuilder165::new(&db_url, 1);
      for record in records_165.into_iter() {
        tb_165.push(record);
      }
      let _ = runtime.block_on(tb_165.flush());
    }
  }

  {
    let records_166 = RECORD_QUEUE_166.take();
    if !records_166.is_empty() {
      let mut tb_166 = TableBuilder166::new(&db_url, 1);
      for record in records_166.into_iter() {
        tb_166.push(record);
      }
      let _ = runtime.block_on(tb_166.flush());
    }
  }

  {
    let records_167 = RECORD_QUEUE_167.take();
    if !records_167.is_empty() {
      let mut tb_167 = TableBuilder167::new(&db_url, 1);
      for record in records_167.into_iter() {
        tb_167.push(record);
      }
      let _ = runtime.block_on(tb_167.flush());
    }
  }

  {
    let records_168 = RECORD_QUEUE_168.take();
    if !records_168.is_empty() {
      let mut tb_168 = TableBuilder168::new(&db_url, 1);
      for record in records_168.into_iter() {
        tb_168.push(record);
      }
      let _ = runtime.block_on(tb_168.flush());
    }
  }

  {
    let records_169 = RECORD_QUEUE_169.take();
    if !records_169.is_empty() {
      let mut tb_169 = TableBuilder169::new(&db_url, 1);
      for record in records_169.into_iter() {
        tb_169.push(record);
      }
      let _ = runtime.block_on(tb_169.flush());
    }
  }

  {
    let records_170 = RECORD_QUEUE_170.take();
    if !records_170.is_empty() {
      let mut tb_170 = TableBuilder170::new(&db_url, 1);
      for record in records_170.into_iter() {
        tb_170.push(record);
      }
      let _ = runtime.block_on(tb_170.flush());
    }
  }

  {
    let records_171 = RECORD_QUEUE_171.take();
    if !records_171.is_empty() {
      let mut tb_171 = TableBuilder171::new(&db_url, 1);
      for record in records_171.into_iter() {
        tb_171.push(record);
      }
      let _ = runtime.block_on(tb_171.flush());
    }
  }

  {
    let records_172 = RECORD_QUEUE_172.take();
    if !records_172.is_empty() {
      let mut tb_172 = TableBuilder172::new(&db_url, 1);
      for record in records_172.into_iter() {
        tb_172.push(record);
      }
      let _ = runtime.block_on(tb_172.flush());
    }
  }

  {
    let records_173 = RECORD_QUEUE_173.take();
    if !records_173.is_empty() {
      let mut tb_173 = TableBuilder173::new(&db_url, 1);
      for record in records_173.into_iter() {
        tb_173.push(record);
      }
      let _ = runtime.block_on(tb_173.flush());
    }
  }

  {
    let records_174 = RECORD_QUEUE_174.take();
    if !records_174.is_empty() {
      let mut tb_174 = TableBuilder174::new(&db_url, 1);
      for record in records_174.into_iter() {
        tb_174.push(record);
      }
      let _ = runtime.block_on(tb_174.flush());
    }
  }

  {
    let records_175 = RECORD_QUEUE_175.take();
    if !records_175.is_empty() {
      let mut tb_175 = TableBuilder175::new(&db_url, 1);
      for record in records_175.into_iter() {
        tb_175.push(record);
      }
      let _ = runtime.block_on(tb_175.flush());
    }
  }

  {
    let records_176 = RECORD_QUEUE_176.take();
    if !records_176.is_empty() {
      let mut tb_176 = TableBuilder176::new(&db_url, 1);
      for record in records_176.into_iter() {
        tb_176.push(record);
      }
      let _ = runtime.block_on(tb_176.flush());
    }
  }

  {
    let records_177 = RECORD_QUEUE_177.take();
    if !records_177.is_empty() {
      let mut tb_177 = TableBuilder177::new(&db_url, 1);
      for record in records_177.into_iter() {
        tb_177.push(record);
      }
      let _ = runtime.block_on(tb_177.flush());
    }
  }

  {
    let records_178 = RECORD_QUEUE_178.take();
    if !records_178.is_empty() {
      let mut tb_178 = TableBuilder178::new(&db_url, 1);
      for record in records_178.into_iter() {
        tb_178.push(record);
      }
      let _ = runtime.block_on(tb_178.flush());
    }
  }

  {
    let records_179 = RECORD_QUEUE_179.take();
    if !records_179.is_empty() {
      let mut tb_179 = TableBuilder179::new(&db_url, 1);
      for record in records_179.into_iter() {
        tb_179.push(record);
      }
      let _ = runtime.block_on(tb_179.flush());
    }
  }

  {
    let records_180 = RECORD_QUEUE_180.take();
    if !records_180.is_empty() {
      let mut tb_180 = TableBuilder180::new(&db_url, 1);
      for record in records_180.into_iter() {
        tb_180.push(record);
      }
      let _ = runtime.block_on(tb_180.flush());
    }
  }

  {
    let records_181 = RECORD_QUEUE_181.take();
    if !records_181.is_empty() {
      let mut tb_181 = TableBuilder181::new(&db_url, 1);
      for record in records_181.into_iter() {
        tb_181.push(record);
      }
      let _ = runtime.block_on(tb_181.flush());
    }
  }

  {
    let records_182 = RECORD_QUEUE_182.take();
    if !records_182.is_empty() {
      let mut tb_182 = TableBuilder182::new(&db_url, 1);
      for record in records_182.into_iter() {
        tb_182.push(record);
      }
      let _ = runtime.block_on(tb_182.flush());
    }
  }

  {
    let records_183 = RECORD_QUEUE_183.take();
    if !records_183.is_empty() {
      let mut tb_183 = TableBuilder183::new(&db_url, 1);
      for record in records_183.into_iter() {
        tb_183.push(record);
      }
      let _ = runtime.block_on(tb_183.flush());
    }
  }

  {
    let records_184 = RECORD_QUEUE_184.take();
    if !records_184.is_empty() {
      let mut tb_184 = TableBuilder184::new(&db_url, 1);
      for record in records_184.into_iter() {
        tb_184.push(record);
      }
      let _ = runtime.block_on(tb_184.flush());
    }
  }

  {
    let records_185 = RECORD_QUEUE_185.take();
    if !records_185.is_empty() {
      let mut tb_185 = TableBuilder185::new(&db_url, 1);
      for record in records_185.into_iter() {
        tb_185.push(record);
      }
      let _ = runtime.block_on(tb_185.flush());
    }
  }

  {
    let records_186 = RECORD_QUEUE_186.take();
    if !records_186.is_empty() {
      let mut tb_186 = TableBuilder186::new(&db_url, 1);
      for record in records_186.into_iter() {
        tb_186.push(record);
      }
      let _ = runtime.block_on(tb_186.flush());
    }
  }

  {
    let records_187 = RECORD_QUEUE_187.take();
    if !records_187.is_empty() {
      let mut tb_187 = TableBuilder187::new(&db_url, 1);
      for record in records_187.into_iter() {
        tb_187.push(record);
      }
      let _ = runtime.block_on(tb_187.flush());
    }
  }

  {
    let records_188 = RECORD_QUEUE_188.take();
    if !records_188.is_empty() {
      let mut tb_188 = TableBuilder188::new(&db_url, 1);
      for record in records_188.into_iter() {
        tb_188.push(record);
      }
      let _ = runtime.block_on(tb_188.flush());
    }
  }

  {
    let records_189 = RECORD_QUEUE_189.take();
    if !records_189.is_empty() {
      let mut tb_189 = TableBuilder189::new(&db_url, 1);
      for record in records_189.into_iter() {
        tb_189.push(record);
      }
      let _ = runtime.block_on(tb_189.flush());
    }
  }

  {
    let records_190 = RECORD_QUEUE_190.take();
    if !records_190.is_empty() {
      let mut tb_190 = TableBuilder190::new(&db_url, 1);
      for record in records_190.into_iter() {
        tb_190.push(record);
      }
      let _ = runtime.block_on(tb_190.flush());
    }
  }

  {
    let records_191 = RECORD_QUEUE_191.take();
    if !records_191.is_empty() {
      let mut tb_191 = TableBuilder191::new(&db_url, 1);
      for record in records_191.into_iter() {
        tb_191.push(record);
      }
      let _ = runtime.block_on(tb_191.flush());
    }
  }

  {
    let records_192 = RECORD_QUEUE_192.take();
    if !records_192.is_empty() {
      let mut tb_192 = TableBuilder192::new(&db_url, 1);
      for record in records_192.into_iter() {
        tb_192.push(record);
      }
      let _ = runtime.block_on(tb_192.flush());
    }
  }

  {
    let records_193 = RECORD_QUEUE_193.take();
    if !records_193.is_empty() {
      let mut tb_193 = TableBuilder193::new(&db_url, 1);
      for record in records_193.into_iter() {
        tb_193.push(record);
      }
      let _ = runtime.block_on(tb_193.flush());
    }
  }

  {
    let records_194 = RECORD_QUEUE_194.take();
    if !records_194.is_empty() {
      let mut tb_194 = TableBuilder194::new(&db_url, 1);
      for record in records_194.into_iter() {
        tb_194.push(record);
      }
      let _ = runtime.block_on(tb_194.flush());
    }
  }

  {
    let records_195 = RECORD_QUEUE_195.take();
    if !records_195.is_empty() {
      let mut tb_195 = TableBuilder195::new(&db_url, 1);
      for record in records_195.into_iter() {
        tb_195.push(record);
      }
      let _ = runtime.block_on(tb_195.flush());
    }
  }

  {
    let records_196 = RECORD_QUEUE_196.take();
    if !records_196.is_empty() {
      let mut tb_196 = TableBuilder196::new(&db_url, 1);
      for record in records_196.into_iter() {
        tb_196.push(record);
      }
      let _ = runtime.block_on(tb_196.flush());
    }
  }

  {
    let records_197 = RECORD_QUEUE_197.take();
    if !records_197.is_empty() {
      let mut tb_197 = TableBuilder197::new(&db_url, 1);
      for record in records_197.into_iter() {
        tb_197.push(record);
      }
      let _ = runtime.block_on(tb_197.flush());
    }
  }

  {
    let records_198 = RECORD_QUEUE_198.take();
    if !records_198.is_empty() {
      let mut tb_198 = TableBuilder198::new(&db_url, 1);
      for record in records_198.into_iter() {
        tb_198.push(record);
      }
      let _ = runtime.block_on(tb_198.flush());
    }
  }

  {
    let records_199 = RECORD_QUEUE_199.take();
    if !records_199.is_empty() {
      let mut tb_199 = TableBuilder199::new(&db_url, 1);
      for record in records_199.into_iter() {
        tb_199.push(record);
      }
      let _ = runtime.block_on(tb_199.flush());
    }
  }

  {
    let records_200 = RECORD_QUEUE_200.take();
    if !records_200.is_empty() {
      let mut tb_200 = TableBuilder200::new(&db_url, 1);
      for record in records_200.into_iter() {
        tb_200.push(record);
      }
      let _ = runtime.block_on(tb_200.flush());
    }
  }

  {
    let records_201 = RECORD_QUEUE_201.take();
    if !records_201.is_empty() {
      let mut tb_201 = TableBuilder201::new(&db_url, 1);
      for record in records_201.into_iter() {
        tb_201.push(record);
      }
      let _ = runtime.block_on(tb_201.flush());
    }
  }

  {
    let records_202 = RECORD_QUEUE_202.take();
    if !records_202.is_empty() {
      let mut tb_202 = TableBuilder202::new(&db_url, 1);
      for record in records_202.into_iter() {
        tb_202.push(record);
      }
      let _ = runtime.block_on(tb_202.flush());
    }
  }

  {
    let records_203 = RECORD_QUEUE_203.take();
    if !records_203.is_empty() {
      let mut tb_203 = TableBuilder203::new(&db_url, 1);
      for record in records_203.into_iter() {
        tb_203.push(record);
      }
      let _ = runtime.block_on(tb_203.flush());
    }
  }

  {
    let records_204 = RECORD_QUEUE_204.take();
    if !records_204.is_empty() {
      let mut tb_204 = TableBuilder204::new(&db_url, 1);
      for record in records_204.into_iter() {
        tb_204.push(record);
      }
      let _ = runtime.block_on(tb_204.flush());
    }
  }

  {
    let records_205 = RECORD_QUEUE_205.take();
    if !records_205.is_empty() {
      let mut tb_205 = TableBuilder205::new(&db_url, 1);
      for record in records_205.into_iter() {
        tb_205.push(record);
      }
      let _ = runtime.block_on(tb_205.flush());
    }
  }

  {
    let records_206 = RECORD_QUEUE_206.take();
    if !records_206.is_empty() {
      let mut tb_206 = TableBuilder206::new(&db_url, 1);
      for record in records_206.into_iter() {
        tb_206.push(record);
      }
      let _ = runtime.block_on(tb_206.flush());
    }
  }

  {
    let records_207 = RECORD_QUEUE_207.take();
    if !records_207.is_empty() {
      let mut tb_207 = TableBuilder207::new(&db_url, 1);
      for record in records_207.into_iter() {
        tb_207.push(record);
      }
      let _ = runtime.block_on(tb_207.flush());
    }
  }

  {
    let records_208 = RECORD_QUEUE_208.take();
    if !records_208.is_empty() {
      let mut tb_208 = TableBuilder208::new(&db_url, 1);
      for record in records_208.into_iter() {
        tb_208.push(record);
      }
      let _ = runtime.block_on(tb_208.flush());
    }
  }

  {
    let records_209 = RECORD_QUEUE_209.take();
    if !records_209.is_empty() {
      let mut tb_209 = TableBuilder209::new(&db_url, 1);
      for record in records_209.into_iter() {
        tb_209.push(record);
      }
      let _ = runtime.block_on(tb_209.flush());
    }
  }

  {
    let records_210 = RECORD_QUEUE_210.take();
    if !records_210.is_empty() {
      let mut tb_210 = TableBuilder210::new(&db_url, 1);
      for record in records_210.into_iter() {
        tb_210.push(record);
      }
      let _ = runtime.block_on(tb_210.flush());
    }
  }

  {
    let records_211 = RECORD_QUEUE_211.take();
    if !records_211.is_empty() {
      let mut tb_211 = TableBuilder211::new(&db_url, 1);
      for record in records_211.into_iter() {
        tb_211.push(record);
      }
      let _ = runtime.block_on(tb_211.flush());
    }
  }

  {
    let records_212 = RECORD_QUEUE_212.take();
    if !records_212.is_empty() {
      let mut tb_212 = TableBuilder212::new(&db_url, 1);
      for record in records_212.into_iter() {
        tb_212.push(record);
      }
      let _ = runtime.block_on(tb_212.flush());
    }
  }

  {
    let records_213 = RECORD_QUEUE_213.take();
    if !records_213.is_empty() {
      let mut tb_213 = TableBuilder213::new(&db_url, 1);
      for record in records_213.into_iter() {
        tb_213.push(record);
      }
      let _ = runtime.block_on(tb_213.flush());
    }
  }

  {
    let records_214 = RECORD_QUEUE_214.take();
    if !records_214.is_empty() {
      let mut tb_214 = TableBuilder214::new(&db_url, 1);
      for record in records_214.into_iter() {
        tb_214.push(record);
      }
      let _ = runtime.block_on(tb_214.flush());
    }
  }

  {
    let records_215 = RECORD_QUEUE_215.take();
    if !records_215.is_empty() {
      let mut tb_215 = TableBuilder215::new(&db_url, 1);
      for record in records_215.into_iter() {
        tb_215.push(record);
      }
      let _ = runtime.block_on(tb_215.flush());
    }
  }

  {
    let records_216 = RECORD_QUEUE_216.take();
    if !records_216.is_empty() {
      let mut tb_216 = TableBuilder216::new(&db_url, 1);
      for record in records_216.into_iter() {
        tb_216.push(record);
      }
      let _ = runtime.block_on(tb_216.flush());
    }
  }

  {
    let records_217 = RECORD_QUEUE_217.take();
    if !records_217.is_empty() {
      let mut tb_217 = TableBuilder217::new(&db_url, 1);
      for record in records_217.into_iter() {
        tb_217.push(record);
      }
      let _ = runtime.block_on(tb_217.flush());
    }
  }

  {
    let records_218 = RECORD_QUEUE_218.take();
    if !records_218.is_empty() {
      let mut tb_218 = TableBuilder218::new(&db_url, 1);
      for record in records_218.into_iter() {
        tb_218.push(record);
      }
      let _ = runtime.block_on(tb_218.flush());
    }
  }

  {
    let records_219 = RECORD_QUEUE_219.take();
    if !records_219.is_empty() {
      let mut tb_219 = TableBuilder219::new(&db_url, 1);
      for record in records_219.into_iter() {
        tb_219.push(record);
      }
      let _ = runtime.block_on(tb_219.flush());
    }
  }

  {
    let records_220 = RECORD_QUEUE_220.take();
    if !records_220.is_empty() {
      let mut tb_220 = TableBuilder220::new(&db_url, 1);
      for record in records_220.into_iter() {
        tb_220.push(record);
      }
      let _ = runtime.block_on(tb_220.flush());
    }
  }

  {
    let records_221 = RECORD_QUEUE_221.take();
    if !records_221.is_empty() {
      let mut tb_221 = TableBuilder221::new(&db_url, 1);
      for record in records_221.into_iter() {
        tb_221.push(record);
      }
      let _ = runtime.block_on(tb_221.flush());
    }
  }

  {
    let records_222 = RECORD_QUEUE_222.take();
    if !records_222.is_empty() {
      let mut tb_222 = TableBuilder222::new(&db_url, 1);
      for record in records_222.into_iter() {
        tb_222.push(record);
      }
      let _ = runtime.block_on(tb_222.flush());
    }
  }

  {
    let records_223 = RECORD_QUEUE_223.take();
    if !records_223.is_empty() {
      let mut tb_223 = TableBuilder223::new(&db_url, 1);
      for record in records_223.into_iter() {
        tb_223.push(record);
      }
      let _ = runtime.block_on(tb_223.flush());
    }
  }

  {
    let records_224 = RECORD_QUEUE_224.take();
    if !records_224.is_empty() {
      let mut tb_224 = TableBuilder224::new(&db_url, 1);
      for record in records_224.into_iter() {
        tb_224.push(record);
      }
      let _ = runtime.block_on(tb_224.flush());
    }
  }

  {
    let records_225 = RECORD_QUEUE_225.take();
    if !records_225.is_empty() {
      let mut tb_225 = TableBuilder225::new(&db_url, 1);
      for record in records_225.into_iter() {
        tb_225.push(record);
      }
      let _ = runtime.block_on(tb_225.flush());
    }
  }

  {
    let records_226 = RECORD_QUEUE_226.take();
    if !records_226.is_empty() {
      let mut tb_226 = TableBuilder226::new(&db_url, 1);
      for record in records_226.into_iter() {
        tb_226.push(record);
      }
      let _ = runtime.block_on(tb_226.flush());
    }
  }

  {
    let records_227 = RECORD_QUEUE_227.take();
    if !records_227.is_empty() {
      let mut tb_227 = TableBuilder227::new(&db_url, 1);
      for record in records_227.into_iter() {
        tb_227.push(record);
      }
      let _ = runtime.block_on(tb_227.flush());
    }
  }

  {
    let records_228 = RECORD_QUEUE_228.take();
    if !records_228.is_empty() {
      let mut tb_228 = TableBuilder228::new(&db_url, 1);
      for record in records_228.into_iter() {
        tb_228.push(record);
      }
      let _ = runtime.block_on(tb_228.flush());
    }
  }

  {
    let records_229 = RECORD_QUEUE_229.take();
    if !records_229.is_empty() {
      let mut tb_229 = TableBuilder229::new(&db_url, 1);
      for record in records_229.into_iter() {
        tb_229.push(record);
      }
      let _ = runtime.block_on(tb_229.flush());
    }
  }

  {
    let records_230 = RECORD_QUEUE_230.take();
    if !records_230.is_empty() {
      let mut tb_230 = TableBuilder230::new(&db_url, 1);
      for record in records_230.into_iter() {
        tb_230.push(record);
      }
      let _ = runtime.block_on(tb_230.flush());
    }
  }

  {
    let records_231 = RECORD_QUEUE_231.take();
    if !records_231.is_empty() {
      let mut tb_231 = TableBuilder231::new(&db_url, 1);
      for record in records_231.into_iter() {
        tb_231.push(record);
      }
      let _ = runtime.block_on(tb_231.flush());
    }
  }

  {
    let records_232 = RECORD_QUEUE_232.take();
    if !records_232.is_empty() {
      let mut tb_232 = TableBuilder232::new(&db_url, 1);
      for record in records_232.into_iter() {
        tb_232.push(record);
      }
      let _ = runtime.block_on(tb_232.flush());
    }
  }

  {
    let records_233 = RECORD_QUEUE_233.take();
    if !records_233.is_empty() {
      let mut tb_233 = TableBuilder233::new(&db_url, 1);
      for record in records_233.into_iter() {
        tb_233.push(record);
      }
      let _ = runtime.block_on(tb_233.flush());
    }
  }

  {
    let records_234 = RECORD_QUEUE_234.take();
    if !records_234.is_empty() {
      let mut tb_234 = TableBuilder234::new(&db_url, 1);
      for record in records_234.into_iter() {
        tb_234.push(record);
      }
      let _ = runtime.block_on(tb_234.flush());
    }
  }

  {
    let records_235 = RECORD_QUEUE_235.take();
    if !records_235.is_empty() {
      let mut tb_235 = TableBuilder235::new(&db_url, 1);
      for record in records_235.into_iter() {
        tb_235.push(record);
      }
      let _ = runtime.block_on(tb_235.flush());
    }
  }

  {
    let records_236 = RECORD_QUEUE_236.take();
    if !records_236.is_empty() {
      let mut tb_236 = TableBuilder236::new(&db_url, 1);
      for record in records_236.into_iter() {
        tb_236.push(record);
      }
      let _ = runtime.block_on(tb_236.flush());
    }
  }

  {
    let records_237 = RECORD_QUEUE_237.take();
    if !records_237.is_empty() {
      let mut tb_237 = TableBuilder237::new(&db_url, 1);
      for record in records_237.into_iter() {
        tb_237.push(record);
      }
      let _ = runtime.block_on(tb_237.flush());
    }
  }

  {
    let records_238 = RECORD_QUEUE_238.take();
    if !records_238.is_empty() {
      let mut tb_238 = TableBuilder238::new(&db_url, 1);
      for record in records_238.into_iter() {
        tb_238.push(record);
      }
      let _ = runtime.block_on(tb_238.flush());
    }
  }

  {
    let records_239 = RECORD_QUEUE_239.take();
    if !records_239.is_empty() {
      let mut tb_239 = TableBuilder239::new(&db_url, 1);
      for record in records_239.into_iter() {
        tb_239.push(record);
      }
      let _ = runtime.block_on(tb_239.flush());
    }
  }

  {
    let records_240 = RECORD_QUEUE_240.take();
    if !records_240.is_empty() {
      let mut tb_240 = TableBuilder240::new(&db_url, 1);
      for record in records_240.into_iter() {
        tb_240.push(record);
      }
      let _ = runtime.block_on(tb_240.flush());
    }
  }

  {
    let records_241 = RECORD_QUEUE_241.take();
    if !records_241.is_empty() {
      let mut tb_241 = TableBuilder241::new(&db_url, 1);
      for record in records_241.into_iter() {
        tb_241.push(record);
      }
      let _ = runtime.block_on(tb_241.flush());
    }
  }

  {
    let records_242 = RECORD_QUEUE_242.take();
    if !records_242.is_empty() {
      let mut tb_242 = TableBuilder242::new(&db_url, 1);
      for record in records_242.into_iter() {
        tb_242.push(record);
      }
      let _ = runtime.block_on(tb_242.flush());
    }
  }

  {
    let records_243 = RECORD_QUEUE_243.take();
    if !records_243.is_empty() {
      let mut tb_243 = TableBuilder243::new(&db_url, 1);
      for record in records_243.into_iter() {
        tb_243.push(record);
      }
      let _ = runtime.block_on(tb_243.flush());
    }
  }

  {
    let records_244 = RECORD_QUEUE_244.take();
    if !records_244.is_empty() {
      let mut tb_244 = TableBuilder244::new(&db_url, 1);
      for record in records_244.into_iter() {
        tb_244.push(record);
      }
      let _ = runtime.block_on(tb_244.flush());
    }
  }

  {
    let records_245 = RECORD_QUEUE_245.take();
    if !records_245.is_empty() {
      let mut tb_245 = TableBuilder245::new(&db_url, 1);
      for record in records_245.into_iter() {
        tb_245.push(record);
      }
      let _ = runtime.block_on(tb_245.flush());
    }
  }

  {
    let records_246 = RECORD_QUEUE_246.take();
    if !records_246.is_empty() {
      let mut tb_246 = TableBuilder246::new(&db_url, 1);
      for record in records_246.into_iter() {
        tb_246.push(record);
      }
      let _ = runtime.block_on(tb_246.flush());
    }
  }

  {
    let records_247 = RECORD_QUEUE_247.take();
    if !records_247.is_empty() {
      let mut tb_247 = TableBuilder247::new(&db_url, 1);
      for record in records_247.into_iter() {
        tb_247.push(record);
      }
      let _ = runtime.block_on(tb_247.flush());
    }
  }

  {
    let records_248 = RECORD_QUEUE_248.take();
    if !records_248.is_empty() {
      let mut tb_248 = TableBuilder248::new(&db_url, 1);
      for record in records_248.into_iter() {
        tb_248.push(record);
      }
      let _ = runtime.block_on(tb_248.flush());
    }
  }

  {
    let records_249 = RECORD_QUEUE_249.take();
    if !records_249.is_empty() {
      let mut tb_249 = TableBuilder249::new(&db_url, 1);
      for record in records_249.into_iter() {
        tb_249.push(record);
      }
      let _ = runtime.block_on(tb_249.flush());
    }
  }

  {
    let records_250 = RECORD_QUEUE_250.take();
    if !records_250.is_empty() {
      let mut tb_250 = TableBuilder250::new(&db_url, 1);
      for record in records_250.into_iter() {
        tb_250.push(record);
      }
      let _ = runtime.block_on(tb_250.flush());
    }
  }

  {
    let records_251 = RECORD_QUEUE_251.take();
    if !records_251.is_empty() {
      let mut tb_251 = TableBuilder251::new(&db_url, 1);
      for record in records_251.into_iter() {
        tb_251.push(record);
      }
      let _ = runtime.block_on(tb_251.flush());
    }
  }

  {
    let records_252 = RECORD_QUEUE_252.take();
    if !records_252.is_empty() {
      let mut tb_252 = TableBuilder252::new(&db_url, 1);
      for record in records_252.into_iter() {
        tb_252.push(record);
      }
      let _ = runtime.block_on(tb_252.flush());
    }
  }

  {
    let records_253 = RECORD_QUEUE_253.take();
    if !records_253.is_empty() {
      let mut tb_253 = TableBuilder253::new(&db_url, 1);
      for record in records_253.into_iter() {
        tb_253.push(record);
      }
      let _ = runtime.block_on(tb_253.flush());
    }
  }

  {
    let records_254 = RECORD_QUEUE_254.take();
    if !records_254.is_empty() {
      let mut tb_254 = TableBuilder254::new(&db_url, 1);
      for record in records_254.into_iter() {
        tb_254.push(record);
      }
      let _ = runtime.block_on(tb_254.flush());
    }
  }

  {
    let records_255 = RECORD_QUEUE_255.take();
    if !records_255.is_empty() {
      let mut tb_255 = TableBuilder255::new(&db_url, 1);
      for record in records_255.into_iter() {
        tb_255.push(record);
      }
      let _ = runtime.block_on(tb_255.flush());
    }
  }

  {
    let records_256 = RECORD_QUEUE_256.take();
    if !records_256.is_empty() {
      let mut tb_256 = TableBuilder256::new(&db_url, 1);
      for record in records_256.into_iter() {
        tb_256.push(record);
      }
      let _ = runtime.block_on(tb_256.flush());
    }
  }

  {
    let records_257 = RECORD_QUEUE_257.take();
    if !records_257.is_empty() {
      let mut tb_257 = TableBuilder257::new(&db_url, 1);
      for record in records_257.into_iter() {
        tb_257.push(record);
      }
      let _ = runtime.block_on(tb_257.flush());
    }
  }

  {
    let records_258 = RECORD_QUEUE_258.take();
    if !records_258.is_empty() {
      let mut tb_258 = TableBuilder258::new(&db_url, 1);
      for record in records_258.into_iter() {
        tb_258.push(record);
      }
      let _ = runtime.block_on(tb_258.flush());
    }
  }

  {
    let records_259 = RECORD_QUEUE_259.take();
    if !records_259.is_empty() {
      let mut tb_259 = TableBuilder259::new(&db_url, 1);
      for record in records_259.into_iter() {
        tb_259.push(record);
      }
      let _ = runtime.block_on(tb_259.flush());
    }
  }

  {
    let records_260 = RECORD_QUEUE_260.take();
    if !records_260.is_empty() {
      let mut tb_260 = TableBuilder260::new(&db_url, 1);
      for record in records_260.into_iter() {
        tb_260.push(record);
      }
      let _ = runtime.block_on(tb_260.flush());
    }
  }

  {
    let records_261 = RECORD_QUEUE_261.take();
    if !records_261.is_empty() {
      let mut tb_261 = TableBuilder261::new(&db_url, 1);
      for record in records_261.into_iter() {
        tb_261.push(record);
      }
      let _ = runtime.block_on(tb_261.flush());
    }
  }

  {
    let records_262 = RECORD_QUEUE_262.take();
    if !records_262.is_empty() {
      let mut tb_262 = TableBuilder262::new(&db_url, 1);
      for record in records_262.into_iter() {
        tb_262.push(record);
      }
      let _ = runtime.block_on(tb_262.flush());
    }
  }

  {
    let records_263 = RECORD_QUEUE_263.take();
    if !records_263.is_empty() {
      let mut tb_263 = TableBuilder263::new(&db_url, 1);
      for record in records_263.into_iter() {
        tb_263.push(record);
      }
      let _ = runtime.block_on(tb_263.flush());
    }
  }

  {
    let records_264 = RECORD_QUEUE_264.take();
    if !records_264.is_empty() {
      let mut tb_264 = TableBuilder264::new(&db_url, 1);
      for record in records_264.into_iter() {
        tb_264.push(record);
      }
      let _ = runtime.block_on(tb_264.flush());
    }
  }

  {
    let records_265 = RECORD_QUEUE_265.take();
    if !records_265.is_empty() {
      let mut tb_265 = TableBuilder265::new(&db_url, 1);
      for record in records_265.into_iter() {
        tb_265.push(record);
      }
      let _ = runtime.block_on(tb_265.flush());
    }
  }

  {
    let records_266 = RECORD_QUEUE_266.take();
    if !records_266.is_empty() {
      let mut tb_266 = TableBuilder266::new(&db_url, 1);
      for record in records_266.into_iter() {
        tb_266.push(record);
      }
      let _ = runtime.block_on(tb_266.flush());
    }
  }

  {
    let records_267 = RECORD_QUEUE_267.take();
    if !records_267.is_empty() {
      let mut tb_267 = TableBuilder267::new(&db_url, 1);
      for record in records_267.into_iter() {
        tb_267.push(record);
      }
      let _ = runtime.block_on(tb_267.flush());
    }
  }

  {
    let records_268 = RECORD_QUEUE_268.take();
    if !records_268.is_empty() {
      let mut tb_268 = TableBuilder268::new(&db_url, 1);
      for record in records_268.into_iter() {
        tb_268.push(record);
      }
      let _ = runtime.block_on(tb_268.flush());
    }
  }

  {
    let records_269 = RECORD_QUEUE_269.take();
    if !records_269.is_empty() {
      let mut tb_269 = TableBuilder269::new(&db_url, 1);
      for record in records_269.into_iter() {
        tb_269.push(record);
      }
      let _ = runtime.block_on(tb_269.flush());
    }
  }

  {
    let records_270 = RECORD_QUEUE_270.take();
    if !records_270.is_empty() {
      let mut tb_270 = TableBuilder270::new(&db_url, 1);
      for record in records_270.into_iter() {
        tb_270.push(record);
      }
      let _ = runtime.block_on(tb_270.flush());
    }
  }

  {
    let records_271 = RECORD_QUEUE_271.take();
    if !records_271.is_empty() {
      let mut tb_271 = TableBuilder271::new(&db_url, 1);
      for record in records_271.into_iter() {
        tb_271.push(record);
      }
      let _ = runtime.block_on(tb_271.flush());
    }
  }

  {
    let records_272 = RECORD_QUEUE_272.take();
    if !records_272.is_empty() {
      let mut tb_272 = TableBuilder272::new(&db_url, 1);
      for record in records_272.into_iter() {
        tb_272.push(record);
      }
      let _ = runtime.block_on(tb_272.flush());
    }
  }

  {
    let records_273 = RECORD_QUEUE_273.take();
    if !records_273.is_empty() {
      let mut tb_273 = TableBuilder273::new(&db_url, 1);
      for record in records_273.into_iter() {
        tb_273.push(record);
      }
      let _ = runtime.block_on(tb_273.flush());
    }
  }

  {
    let records_274 = RECORD_QUEUE_274.take();
    if !records_274.is_empty() {
      let mut tb_274 = TableBuilder274::new(&db_url, 1);
      for record in records_274.into_iter() {
        tb_274.push(record);
      }
      let _ = runtime.block_on(tb_274.flush());
    }
  }

  {
    let records_275 = RECORD_QUEUE_275.take();
    if !records_275.is_empty() {
      let mut tb_275 = TableBuilder275::new(&db_url, 1);
      for record in records_275.into_iter() {
        tb_275.push(record);
      }
      let _ = runtime.block_on(tb_275.flush());
    }
  }

  {
    let records_276 = RECORD_QUEUE_276.take();
    if !records_276.is_empty() {
      let mut tb_276 = TableBuilder276::new(&db_url, 1);
      for record in records_276.into_iter() {
        tb_276.push(record);
      }
      let _ = runtime.block_on(tb_276.flush());
    }
  }

  {
    let records_277 = RECORD_QUEUE_277.take();
    if !records_277.is_empty() {
      let mut tb_277 = TableBuilder277::new(&db_url, 1);
      for record in records_277.into_iter() {
        tb_277.push(record);
      }
      let _ = runtime.block_on(tb_277.flush());
    }
  }

  {
    let records_278 = RECORD_QUEUE_278.take();
    if !records_278.is_empty() {
      let mut tb_278 = TableBuilder278::new(&db_url, 1);
      for record in records_278.into_iter() {
        tb_278.push(record);
      }
      let _ = runtime.block_on(tb_278.flush());
    }
  }

  {
    let records_279 = RECORD_QUEUE_279.take();
    if !records_279.is_empty() {
      let mut tb_279 = TableBuilder279::new(&db_url, 1);
      for record in records_279.into_iter() {
        tb_279.push(record);
      }
      let _ = runtime.block_on(tb_279.flush());
    }
  }

  {
    let records_280 = RECORD_QUEUE_280.take();
    if !records_280.is_empty() {
      let mut tb_280 = TableBuilder280::new(&db_url, 1);
      for record in records_280.into_iter() {
        tb_280.push(record);
      }
      let _ = runtime.block_on(tb_280.flush());
    }
  }

  {
    let records_281 = RECORD_QUEUE_281.take();
    if !records_281.is_empty() {
      let mut tb_281 = TableBuilder281::new(&db_url, 1);
      for record in records_281.into_iter() {
        tb_281.push(record);
      }
      let _ = runtime.block_on(tb_281.flush());
    }
  }

  {
    let records_282 = RECORD_QUEUE_282.take();
    if !records_282.is_empty() {
      let mut tb_282 = TableBuilder282::new(&db_url, 1);
      for record in records_282.into_iter() {
        tb_282.push(record);
      }
      let _ = runtime.block_on(tb_282.flush());
    }
  }

  {
    let records_283 = RECORD_QUEUE_283.take();
    if !records_283.is_empty() {
      let mut tb_283 = TableBuilder283::new(&db_url, 1);
      for record in records_283.into_iter() {
        tb_283.push(record);
      }
      let _ = runtime.block_on(tb_283.flush());
    }
  }

  {
    let records_284 = RECORD_QUEUE_284.take();
    if !records_284.is_empty() {
      let mut tb_284 = TableBuilder284::new(&db_url, 1);
      for record in records_284.into_iter() {
        tb_284.push(record);
      }
      let _ = runtime.block_on(tb_284.flush());
    }
  }

  {
    let records_285 = RECORD_QUEUE_285.take();
    if !records_285.is_empty() {
      let mut tb_285 = TableBuilder285::new(&db_url, 1);
      for record in records_285.into_iter() {
        tb_285.push(record);
      }
      let _ = runtime.block_on(tb_285.flush());
    }
  }

  {
    let records_286 = RECORD_QUEUE_286.take();
    if !records_286.is_empty() {
      let mut tb_286 = TableBuilder286::new(&db_url, 1);
      for record in records_286.into_iter() {
        tb_286.push(record);
      }
      let _ = runtime.block_on(tb_286.flush());
    }
  }

  {
    let records_287 = RECORD_QUEUE_287.take();
    if !records_287.is_empty() {
      let mut tb_287 = TableBuilder287::new(&db_url, 1);
      for record in records_287.into_iter() {
        tb_287.push(record);
      }
      let _ = runtime.block_on(tb_287.flush());
    }
  }

  {
    let records_288 = RECORD_QUEUE_288.take();
    if !records_288.is_empty() {
      let mut tb_288 = TableBuilder288::new(&db_url, 1);
      for record in records_288.into_iter() {
        tb_288.push(record);
      }
      let _ = runtime.block_on(tb_288.flush());
    }
  }

  {
    let records_289 = RECORD_QUEUE_289.take();
    if !records_289.is_empty() {
      let mut tb_289 = TableBuilder289::new(&db_url, 1);
      for record in records_289.into_iter() {
        tb_289.push(record);
      }
      let _ = runtime.block_on(tb_289.flush());
    }
  }

  {
    let records_290 = RECORD_QUEUE_290.take();
    if !records_290.is_empty() {
      let mut tb_290 = TableBuilder290::new(&db_url, 1);
      for record in records_290.into_iter() {
        tb_290.push(record);
      }
      let _ = runtime.block_on(tb_290.flush());
    }
  }

  {
    let records_291 = RECORD_QUEUE_291.take();
    if !records_291.is_empty() {
      let mut tb_291 = TableBuilder291::new(&db_url, 1);
      for record in records_291.into_iter() {
        tb_291.push(record);
      }
      let _ = runtime.block_on(tb_291.flush());
    }
  }

  {
    let records_292 = RECORD_QUEUE_292.take();
    if !records_292.is_empty() {
      let mut tb_292 = TableBuilder292::new(&db_url, 1);
      for record in records_292.into_iter() {
        tb_292.push(record);
      }
      let _ = runtime.block_on(tb_292.flush());
    }
  }

  {
    let records_293 = RECORD_QUEUE_293.take();
    if !records_293.is_empty() {
      let mut tb_293 = TableBuilder293::new(&db_url, 1);
      for record in records_293.into_iter() {
        tb_293.push(record);
      }
      let _ = runtime.block_on(tb_293.flush());
    }
  }

  {
    let records_294 = RECORD_QUEUE_294.take();
    if !records_294.is_empty() {
      let mut tb_294 = TableBuilder294::new(&db_url, 1);
      for record in records_294.into_iter() {
        tb_294.push(record);
      }
      let _ = runtime.block_on(tb_294.flush());
    }
  }

  {
    let records_295 = RECORD_QUEUE_295.take();
    if !records_295.is_empty() {
      let mut tb_295 = TableBuilder295::new(&db_url, 1);
      for record in records_295.into_iter() {
        tb_295.push(record);
      }
      let _ = runtime.block_on(tb_295.flush());
    }
  }

  {
    let records_296 = RECORD_QUEUE_296.take();
    if !records_296.is_empty() {
      let mut tb_296 = TableBuilder296::new(&db_url, 1);
      for record in records_296.into_iter() {
        tb_296.push(record);
      }
      let _ = runtime.block_on(tb_296.flush());
    }
  }

  {
    let records_297 = RECORD_QUEUE_297.take();
    if !records_297.is_empty() {
      let mut tb_297 = TableBuilder297::new(&db_url, 1);
      for record in records_297.into_iter() {
        tb_297.push(record);
      }
      let _ = runtime.block_on(tb_297.flush());
    }
  }

  {
    let records_298 = RECORD_QUEUE_298.take();
    if !records_298.is_empty() {
      let mut tb_298 = TableBuilder298::new(&db_url, 1);
      for record in records_298.into_iter() {
        tb_298.push(record);
      }
      let _ = runtime.block_on(tb_298.flush());
    }
  }

  {
    let records_299 = RECORD_QUEUE_299.take();
    if !records_299.is_empty() {
      let mut tb_299 = TableBuilder299::new(&db_url, 1);
      for record in records_299.into_iter() {
        tb_299.push(record);
      }
      let _ = runtime.block_on(tb_299.flush());
    }
  }

  {
    let records_300 = RECORD_QUEUE_300.take();
    if !records_300.is_empty() {
      let mut tb_300 = TableBuilder300::new(&db_url, 1);
      for record in records_300.into_iter() {
        tb_300.push(record);
      }
      let _ = runtime.block_on(tb_300.flush());
    }
  }

  {
    let records_301 = RECORD_QUEUE_301.take();
    if !records_301.is_empty() {
      let mut tb_301 = TableBuilder301::new(&db_url, 1);
      for record in records_301.into_iter() {
        tb_301.push(record);
      }
      let _ = runtime.block_on(tb_301.flush());
    }
  }

  {
    let records_302 = RECORD_QUEUE_302.take();
    if !records_302.is_empty() {
      let mut tb_302 = TableBuilder302::new(&db_url, 1);
      for record in records_302.into_iter() {
        tb_302.push(record);
      }
      let _ = runtime.block_on(tb_302.flush());
    }
  }

  {
    let records_303 = RECORD_QUEUE_303.take();
    if !records_303.is_empty() {
      let mut tb_303 = TableBuilder303::new(&db_url, 1);
      for record in records_303.into_iter() {
        tb_303.push(record);
      }
      let _ = runtime.block_on(tb_303.flush());
    }
  }

  {
    let records_304 = RECORD_QUEUE_304.take();
    if !records_304.is_empty() {
      let mut tb_304 = TableBuilder304::new(&db_url, 1);
      for record in records_304.into_iter() {
        tb_304.push(record);
      }
      let _ = runtime.block_on(tb_304.flush());
    }
  }

  {
    let records_305 = RECORD_QUEUE_305.take();
    if !records_305.is_empty() {
      let mut tb_305 = TableBuilder305::new(&db_url, 1);
      for record in records_305.into_iter() {
        tb_305.push(record);
      }
      let _ = runtime.block_on(tb_305.flush());
    }
  }

  {
    let records_306 = RECORD_QUEUE_306.take();
    if !records_306.is_empty() {
      let mut tb_306 = TableBuilder306::new(&db_url, 1);
      for record in records_306.into_iter() {
        tb_306.push(record);
      }
      let _ = runtime.block_on(tb_306.flush());
    }
  }

  {
    let records_307 = RECORD_QUEUE_307.take();
    if !records_307.is_empty() {
      let mut tb_307 = TableBuilder307::new(&db_url, 1);
      for record in records_307.into_iter() {
        tb_307.push(record);
      }
      let _ = runtime.block_on(tb_307.flush());
    }
  }

  {
    let records_308 = RECORD_QUEUE_308.take();
    if !records_308.is_empty() {
      let mut tb_308 = TableBuilder308::new(&db_url, 1);
      for record in records_308.into_iter() {
        tb_308.push(record);
      }
      let _ = runtime.block_on(tb_308.flush());
    }
  }

  {
    let records_309 = RECORD_QUEUE_309.take();
    if !records_309.is_empty() {
      let mut tb_309 = TableBuilder309::new(&db_url, 1);
      for record in records_309.into_iter() {
        tb_309.push(record);
      }
      let _ = runtime.block_on(tb_309.flush());
    }
  }

  {
    let records_310 = RECORD_QUEUE_310.take();
    if !records_310.is_empty() {
      let mut tb_310 = TableBuilder310::new(&db_url, 1);
      for record in records_310.into_iter() {
        tb_310.push(record);
      }
      let _ = runtime.block_on(tb_310.flush());
    }
  }

  {
    let records_311 = RECORD_QUEUE_311.take();
    if !records_311.is_empty() {
      let mut tb_311 = TableBuilder311::new(&db_url, 1);
      for record in records_311.into_iter() {
        tb_311.push(record);
      }
      let _ = runtime.block_on(tb_311.flush());
    }
  }

  {
    let records_312 = RECORD_QUEUE_312.take();
    if !records_312.is_empty() {
      let mut tb_312 = TableBuilder312::new(&db_url, 1);
      for record in records_312.into_iter() {
        tb_312.push(record);
      }
      let _ = runtime.block_on(tb_312.flush());
    }
  }

  {
    let records_313 = RECORD_QUEUE_313.take();
    if !records_313.is_empty() {
      let mut tb_313 = TableBuilder313::new(&db_url, 1);
      for record in records_313.into_iter() {
        tb_313.push(record);
      }
      let _ = runtime.block_on(tb_313.flush());
    }
  }

  {
    let records_314 = RECORD_QUEUE_314.take();
    if !records_314.is_empty() {
      let mut tb_314 = TableBuilder314::new(&db_url, 1);
      for record in records_314.into_iter() {
        tb_314.push(record);
      }
      let _ = runtime.block_on(tb_314.flush());
    }
  }

  {
    let records_315 = RECORD_QUEUE_315.take();
    if !records_315.is_empty() {
      let mut tb_315 = TableBuilder315::new(&db_url, 1);
      for record in records_315.into_iter() {
        tb_315.push(record);
      }
      let _ = runtime.block_on(tb_315.flush());
    }
  }

  {
    let records_316 = RECORD_QUEUE_316.take();
    if !records_316.is_empty() {
      let mut tb_316 = TableBuilder316::new(&db_url, 1);
      for record in records_316.into_iter() {
        tb_316.push(record);
      }
      let _ = runtime.block_on(tb_316.flush());
    }
  }

  {
    let records_317 = RECORD_QUEUE_317.take();
    if !records_317.is_empty() {
      let mut tb_317 = TableBuilder317::new(&db_url, 1);
      for record in records_317.into_iter() {
        tb_317.push(record);
      }
      let _ = runtime.block_on(tb_317.flush());
    }
  }

  {
    let records_318 = RECORD_QUEUE_318.take();
    if !records_318.is_empty() {
      let mut tb_318 = TableBuilder318::new(&db_url, 1);
      for record in records_318.into_iter() {
        tb_318.push(record);
      }
      let _ = runtime.block_on(tb_318.flush());
    }
  }

  {
    let records_319 = RECORD_QUEUE_319.take();
    if !records_319.is_empty() {
      let mut tb_319 = TableBuilder319::new(&db_url, 1);
      for record in records_319.into_iter() {
        tb_319.push(record);
      }
      let _ = runtime.block_on(tb_319.flush());
    }
  }

  {
    let records_320 = RECORD_QUEUE_320.take();
    if !records_320.is_empty() {
      let mut tb_320 = TableBuilder320::new(&db_url, 1);
      for record in records_320.into_iter() {
        tb_320.push(record);
      }
      let _ = runtime.block_on(tb_320.flush());
    }
  }

  {
    let records_321 = RECORD_QUEUE_321.take();
    if !records_321.is_empty() {
      let mut tb_321 = TableBuilder321::new(&db_url, 1);
      for record in records_321.into_iter() {
        tb_321.push(record);
      }
      let _ = runtime.block_on(tb_321.flush());
    }
  }

  {
    let records_322 = RECORD_QUEUE_322.take();
    if !records_322.is_empty() {
      let mut tb_322 = TableBuilder322::new(&db_url, 1);
      for record in records_322.into_iter() {
        tb_322.push(record);
      }
      let _ = runtime.block_on(tb_322.flush());
    }
  }

  {
    let records_323 = RECORD_QUEUE_323.take();
    if !records_323.is_empty() {
      let mut tb_323 = TableBuilder323::new(&db_url, 1);
      for record in records_323.into_iter() {
        tb_323.push(record);
      }
      let _ = runtime.block_on(tb_323.flush());
    }
  }

  {
    let records_324 = RECORD_QUEUE_324.take();
    if !records_324.is_empty() {
      let mut tb_324 = TableBuilder324::new(&db_url, 1);
      for record in records_324.into_iter() {
        tb_324.push(record);
      }
      let _ = runtime.block_on(tb_324.flush());
    }
  }

  {
    let records_325 = RECORD_QUEUE_325.take();
    if !records_325.is_empty() {
      let mut tb_325 = TableBuilder325::new(&db_url, 1);
      for record in records_325.into_iter() {
        tb_325.push(record);
      }
      let _ = runtime.block_on(tb_325.flush());
    }
  }

  {
    let records_326 = RECORD_QUEUE_326.take();
    if !records_326.is_empty() {
      let mut tb_326 = TableBuilder326::new(&db_url, 1);
      for record in records_326.into_iter() {
        tb_326.push(record);
      }
      let _ = runtime.block_on(tb_326.flush());
    }
  }

  {
    let records_327 = RECORD_QUEUE_327.take();
    if !records_327.is_empty() {
      let mut tb_327 = TableBuilder327::new(&db_url, 1);
      for record in records_327.into_iter() {
        tb_327.push(record);
      }
      let _ = runtime.block_on(tb_327.flush());
    }
  }

  {
    let records_328 = RECORD_QUEUE_328.take();
    if !records_328.is_empty() {
      let mut tb_328 = TableBuilder328::new(&db_url, 1);
      for record in records_328.into_iter() {
        tb_328.push(record);
      }
      let _ = runtime.block_on(tb_328.flush());
    }
  }

  {
    let records_329 = RECORD_QUEUE_329.take();
    if !records_329.is_empty() {
      let mut tb_329 = TableBuilder329::new(&db_url, 1);
      for record in records_329.into_iter() {
        tb_329.push(record);
      }
      let _ = runtime.block_on(tb_329.flush());
    }
  }

  {
    let records_330 = RECORD_QUEUE_330.take();
    if !records_330.is_empty() {
      let mut tb_330 = TableBuilder330::new(&db_url, 1);
      for record in records_330.into_iter() {
        tb_330.push(record);
      }
      let _ = runtime.block_on(tb_330.flush());
    }
  }

  {
    let records_331 = RECORD_QUEUE_331.take();
    if !records_331.is_empty() {
      let mut tb_331 = TableBuilder331::new(&db_url, 1);
      for record in records_331.into_iter() {
        tb_331.push(record);
      }
      let _ = runtime.block_on(tb_331.flush());
    }
  }

  {
    let records_332 = RECORD_QUEUE_332.take();
    if !records_332.is_empty() {
      let mut tb_332 = TableBuilder332::new(&db_url, 1);
      for record in records_332.into_iter() {
        tb_332.push(record);
      }
      let _ = runtime.block_on(tb_332.flush());
    }
  }

  {
    let records_333 = RECORD_QUEUE_333.take();
    if !records_333.is_empty() {
      let mut tb_333 = TableBuilder333::new(&db_url, 1);
      for record in records_333.into_iter() {
        tb_333.push(record);
      }
      let _ = runtime.block_on(tb_333.flush());
    }
  }

  {
    let records_334 = RECORD_QUEUE_334.take();
    if !records_334.is_empty() {
      let mut tb_334 = TableBuilder334::new(&db_url, 1);
      for record in records_334.into_iter() {
        tb_334.push(record);
      }
      let _ = runtime.block_on(tb_334.flush());
    }
  }

  {
    let records_335 = RECORD_QUEUE_335.take();
    if !records_335.is_empty() {
      let mut tb_335 = TableBuilder335::new(&db_url, 1);
      for record in records_335.into_iter() {
        tb_335.push(record);
      }
      let _ = runtime.block_on(tb_335.flush());
    }
  }

  {
    let records_336 = RECORD_QUEUE_336.take();
    if !records_336.is_empty() {
      let mut tb_336 = TableBuilder336::new(&db_url, 1);
      for record in records_336.into_iter() {
        tb_336.push(record);
      }
      let _ = runtime.block_on(tb_336.flush());
    }
  }

  {
    let records_337 = RECORD_QUEUE_337.take();
    if !records_337.is_empty() {
      let mut tb_337 = TableBuilder337::new(&db_url, 1);
      for record in records_337.into_iter() {
        tb_337.push(record);
      }
      let _ = runtime.block_on(tb_337.flush());
    }
  }

  {
    let records_338 = RECORD_QUEUE_338.take();
    if !records_338.is_empty() {
      let mut tb_338 = TableBuilder338::new(&db_url, 1);
      for record in records_338.into_iter() {
        tb_338.push(record);
      }
      let _ = runtime.block_on(tb_338.flush());
    }
  }

  {
    let records_339 = RECORD_QUEUE_339.take();
    if !records_339.is_empty() {
      let mut tb_339 = TableBuilder339::new(&db_url, 1);
      for record in records_339.into_iter() {
        tb_339.push(record);
      }
      let _ = runtime.block_on(tb_339.flush());
    }
  }

  {
    let records_340 = RECORD_QUEUE_340.take();
    if !records_340.is_empty() {
      let mut tb_340 = TableBuilder340::new(&db_url, 1);
      for record in records_340.into_iter() {
        tb_340.push(record);
      }
      let _ = runtime.block_on(tb_340.flush());
    }
  }

  {
    let records_341 = RECORD_QUEUE_341.take();
    if !records_341.is_empty() {
      let mut tb_341 = TableBuilder341::new(&db_url, 1);
      for record in records_341.into_iter() {
        tb_341.push(record);
      }
      let _ = runtime.block_on(tb_341.flush());
    }
  }

  {
    let records_342 = RECORD_QUEUE_342.take();
    if !records_342.is_empty() {
      let mut tb_342 = TableBuilder342::new(&db_url, 1);
      for record in records_342.into_iter() {
        tb_342.push(record);
      }
      let _ = runtime.block_on(tb_342.flush());
    }
  }

  {
    let records_343 = RECORD_QUEUE_343.take();
    if !records_343.is_empty() {
      let mut tb_343 = TableBuilder343::new(&db_url, 1);
      for record in records_343.into_iter() {
        tb_343.push(record);
      }
      let _ = runtime.block_on(tb_343.flush());
    }
  }

  {
    let records_344 = RECORD_QUEUE_344.take();
    if !records_344.is_empty() {
      let mut tb_344 = TableBuilder344::new(&db_url, 1);
      for record in records_344.into_iter() {
        tb_344.push(record);
      }
      let _ = runtime.block_on(tb_344.flush());
    }
  }

  {
    let records_345 = RECORD_QUEUE_345.take();
    if !records_345.is_empty() {
      let mut tb_345 = TableBuilder345::new(&db_url, 1);
      for record in records_345.into_iter() {
        tb_345.push(record);
      }
      let _ = runtime.block_on(tb_345.flush());
    }
  }

  {
    let records_346 = RECORD_QUEUE_346.take();
    if !records_346.is_empty() {
      let mut tb_346 = TableBuilder346::new(&db_url, 1);
      for record in records_346.into_iter() {
        tb_346.push(record);
      }
      let _ = runtime.block_on(tb_346.flush());
    }
  }

  {
    let records_347 = RECORD_QUEUE_347.take();
    if !records_347.is_empty() {
      let mut tb_347 = TableBuilder347::new(&db_url, 1);
      for record in records_347.into_iter() {
        tb_347.push(record);
      }
      let _ = runtime.block_on(tb_347.flush());
    }
  }

  {
    let records_348 = RECORD_QUEUE_348.take();
    if !records_348.is_empty() {
      let mut tb_348 = TableBuilder348::new(&db_url, 1);
      for record in records_348.into_iter() {
        tb_348.push(record);
      }
      let _ = runtime.block_on(tb_348.flush());
    }
  }

  {
    let records_349 = RECORD_QUEUE_349.take();
    if !records_349.is_empty() {
      let mut tb_349 = TableBuilder349::new(&db_url, 1);
      for record in records_349.into_iter() {
        tb_349.push(record);
      }
      let _ = runtime.block_on(tb_349.flush());
    }
  }

  {
    let records_350 = RECORD_QUEUE_350.take();
    if !records_350.is_empty() {
      let mut tb_350 = TableBuilder350::new(&db_url, 1);
      for record in records_350.into_iter() {
        tb_350.push(record);
      }
      let _ = runtime.block_on(tb_350.flush());
    }
  }

  {
    let records_351 = RECORD_QUEUE_351.take();
    if !records_351.is_empty() {
      let mut tb_351 = TableBuilder351::new(&db_url, 1);
      for record in records_351.into_iter() {
        tb_351.push(record);
      }
      let _ = runtime.block_on(tb_351.flush());
    }
  }

  {
    let records_352 = RECORD_QUEUE_352.take();
    if !records_352.is_empty() {
      let mut tb_352 = TableBuilder352::new(&db_url, 1);
      for record in records_352.into_iter() {
        tb_352.push(record);
      }
      let _ = runtime.block_on(tb_352.flush());
    }
  }

  {
    let records_353 = RECORD_QUEUE_353.take();
    if !records_353.is_empty() {
      let mut tb_353 = TableBuilder353::new(&db_url, 1);
      for record in records_353.into_iter() {
        tb_353.push(record);
      }
      let _ = runtime.block_on(tb_353.flush());
    }
  }

  {
    let records_354 = RECORD_QUEUE_354.take();
    if !records_354.is_empty() {
      let mut tb_354 = TableBuilder354::new(&db_url, 1);
      for record in records_354.into_iter() {
        tb_354.push(record);
      }
      let _ = runtime.block_on(tb_354.flush());
    }
  }

  true
}

use crate::io::Record0;
use crate::io::TableBuilder0;
use crate::io::Record1;
use crate::io::TableBuilder1;
use crate::io::Record2;
use crate::io::TableBuilder2;
use crate::io::Record3;
use crate::io::TableBuilder3;
use crate::io::Record4;
use crate::io::TableBuilder4;
use crate::io::Record5;
use crate::io::TableBuilder5;
use crate::io::Record6;
use crate::io::TableBuilder6;
use crate::io::Record7;
use crate::io::TableBuilder7;
use crate::io::Record8;
use crate::io::TableBuilder8;
use crate::io::Record9;
use crate::io::TableBuilder9;
use crate::io::Record10;
use crate::io::TableBuilder10;
use crate::io::Record11;
use crate::io::TableBuilder11;
use crate::io::Record12;
use crate::io::TableBuilder12;
use crate::io::Record13;
use crate::io::TableBuilder13;
use crate::io::Record14;
use crate::io::TableBuilder14;
use crate::io::Record15;
use crate::io::TableBuilder15;
use crate::io::Record16;
use crate::io::TableBuilder16;
use crate::io::Record17;
use crate::io::TableBuilder17;
use crate::io::Record18;
use crate::io::TableBuilder18;
use crate::io::Record19;
use crate::io::TableBuilder19;
use crate::io::Record20;
use crate::io::TableBuilder20;
use crate::io::Record21;
use crate::io::TableBuilder21;
use crate::io::Record22;
use crate::io::TableBuilder22;
use crate::io::Record23;
use crate::io::TableBuilder23;
use crate::io::Record24;
use crate::io::TableBuilder24;
use crate::io::Record25;
use crate::io::TableBuilder25;
use crate::io::Record26;
use crate::io::TableBuilder26;
use crate::io::Record27;
use crate::io::TableBuilder27;
use crate::io::Record28;
use crate::io::TableBuilder28;
use crate::io::Record29;
use crate::io::TableBuilder29;
use crate::io::Record30;
use crate::io::TableBuilder30;
use crate::io::Record31;
use crate::io::TableBuilder31;
use crate::io::Record32;
use crate::io::TableBuilder32;
use crate::io::Record33;
use crate::io::TableBuilder33;
use crate::io::Record34;
use crate::io::TableBuilder34;
use crate::io::Record35;
use crate::io::TableBuilder35;
use crate::io::Record36;
use crate::io::TableBuilder36;
use crate::io::Record37;
use crate::io::TableBuilder37;
use crate::io::Record38;
use crate::io::TableBuilder38;
use crate::io::Record39;
use crate::io::TableBuilder39;
use crate::io::Record40;
use crate::io::TableBuilder40;
use crate::io::Record41;
use crate::io::TableBuilder41;
use crate::io::Record42;
use crate::io::TableBuilder42;
use crate::io::Record43;
use crate::io::TableBuilder43;
use crate::io::Record44;
use crate::io::TableBuilder44;
use crate::io::Record45;
use crate::io::TableBuilder45;
use crate::io::Record46;
use crate::io::TableBuilder46;
use crate::io::Record47;
use crate::io::TableBuilder47;
use crate::io::Record48;
use crate::io::TableBuilder48;
use crate::io::Record49;
use crate::io::TableBuilder49;
use crate::io::Record50;
use crate::io::TableBuilder50;
use crate::io::Record51;
use crate::io::TableBuilder51;
use crate::io::Record52;
use crate::io::TableBuilder52;
use crate::io::Record53;
use crate::io::TableBuilder53;
use crate::io::Record54;
use crate::io::TableBuilder54;
use crate::io::Record55;
use crate::io::TableBuilder55;
use crate::io::Record56;
use crate::io::TableBuilder56;
use crate::io::Record57;
use crate::io::TableBuilder57;
use crate::io::Record58;
use crate::io::TableBuilder58;
use crate::io::Record59;
use crate::io::TableBuilder59;
use crate::io::Record60;
use crate::io::TableBuilder60;
use crate::io::Record61;
use crate::io::TableBuilder61;
use crate::io::Record62;
use crate::io::TableBuilder62;
use crate::io::Record63;
use crate::io::TableBuilder63;
use crate::io::Record64;
use crate::io::TableBuilder64;
use crate::io::Record65;
use crate::io::TableBuilder65;
use crate::io::Record66;
use crate::io::TableBuilder66;
use crate::io::Record67;
use crate::io::TableBuilder67;
use crate::io::Record68;
use crate::io::TableBuilder68;
use crate::io::Record69;
use crate::io::TableBuilder69;
use crate::io::Record70;
use crate::io::TableBuilder70;
use crate::io::Record71;
use crate::io::TableBuilder71;
use crate::io::Record72;
use crate::io::TableBuilder72;
use crate::io::Record73;
use crate::io::TableBuilder73;
use crate::io::Record74;
use crate::io::TableBuilder74;
use crate::io::Record75;
use crate::io::TableBuilder75;
use crate::io::Record76;
use crate::io::TableBuilder76;
use crate::io::Record77;
use crate::io::TableBuilder77;
use crate::io::Record78;
use crate::io::TableBuilder78;
use crate::io::Record79;
use crate::io::TableBuilder79;
use crate::io::Record80;
use crate::io::TableBuilder80;
use crate::io::Record81;
use crate::io::TableBuilder81;
use crate::io::Record82;
use crate::io::TableBuilder82;
use crate::io::Record83;
use crate::io::TableBuilder83;
use crate::io::Record84;
use crate::io::TableBuilder84;
use crate::io::Record85;
use crate::io::TableBuilder85;
use crate::io::Record86;
use crate::io::TableBuilder86;
use crate::io::Record87;
use crate::io::TableBuilder87;
use crate::io::Record88;
use crate::io::TableBuilder88;
use crate::io::Record89;
use crate::io::TableBuilder89;
use crate::io::Record90;
use crate::io::TableBuilder90;
use crate::io::Record91;
use crate::io::TableBuilder91;
use crate::io::Record92;
use crate::io::TableBuilder92;
use crate::io::Record93;
use crate::io::TableBuilder93;
use crate::io::Record94;
use crate::io::TableBuilder94;
use crate::io::Record95;
use crate::io::TableBuilder95;
use crate::io::Record96;
use crate::io::TableBuilder96;
use crate::io::Record97;
use crate::io::TableBuilder97;
use crate::io::Record98;
use crate::io::TableBuilder98;
use crate::io::Record99;
use crate::io::TableBuilder99;
use crate::io::Record100;
use crate::io::TableBuilder100;
use crate::io::Record101;
use crate::io::TableBuilder101;
use crate::io::Record102;
use crate::io::TableBuilder102;
use crate::io::Record103;
use crate::io::TableBuilder103;
use crate::io::Record104;
use crate::io::TableBuilder104;
use crate::io::Record105;
use crate::io::TableBuilder105;
use crate::io::Record106;
use crate::io::TableBuilder106;
use crate::io::Record107;
use crate::io::TableBuilder107;
use crate::io::Record108;
use crate::io::TableBuilder108;
use crate::io::Record109;
use crate::io::TableBuilder109;
use crate::io::Record110;
use crate::io::TableBuilder110;
use crate::io::Record111;
use crate::io::TableBuilder111;
use crate::io::Record112;
use crate::io::TableBuilder112;
use crate::io::Record113;
use crate::io::TableBuilder113;
use crate::io::Record114;
use crate::io::TableBuilder114;
use crate::io::Record115;
use crate::io::TableBuilder115;
use crate::io::Record116;
use crate::io::TableBuilder116;
use crate::io::Record117;
use crate::io::TableBuilder117;
use crate::io::Record118;
use crate::io::TableBuilder118;
use crate::io::Record119;
use crate::io::TableBuilder119;
use crate::io::Record120;
use crate::io::TableBuilder120;
use crate::io::Record121;
use crate::io::TableBuilder121;
use crate::io::Record122;
use crate::io::TableBuilder122;
use crate::io::Record123;
use crate::io::TableBuilder123;
use crate::io::Record124;
use crate::io::TableBuilder124;
use crate::io::Record125;
use crate::io::TableBuilder125;
use crate::io::Record126;
use crate::io::TableBuilder126;
use crate::io::Record127;
use crate::io::TableBuilder127;
use crate::io::Record128;
use crate::io::TableBuilder128;
use crate::io::Record129;
use crate::io::TableBuilder129;
use crate::io::Record130;
use crate::io::TableBuilder130;
use crate::io::Record131;
use crate::io::TableBuilder131;
use crate::io::Record132;
use crate::io::TableBuilder132;
use crate::io::Record133;
use crate::io::TableBuilder133;
use crate::io::Record134;
use crate::io::TableBuilder134;
use crate::io::Record135;
use crate::io::TableBuilder135;
use crate::io::Record136;
use crate::io::TableBuilder136;
use crate::io::Record137;
use crate::io::TableBuilder137;
use crate::io::Record138;
use crate::io::TableBuilder138;
use crate::io::Record139;
use crate::io::TableBuilder139;
use crate::io::Record140;
use crate::io::TableBuilder140;
use crate::io::Record141;
use crate::io::TableBuilder141;
use crate::io::Record142;
use crate::io::TableBuilder142;
use crate::io::Record143;
use crate::io::TableBuilder143;
use crate::io::Record144;
use crate::io::TableBuilder144;
use crate::io::Record145;
use crate::io::TableBuilder145;
use crate::io::Record146;
use crate::io::TableBuilder146;
use crate::io::Record147;
use crate::io::TableBuilder147;
use crate::io::Record148;
use crate::io::TableBuilder148;
use crate::io::Record149;
use crate::io::TableBuilder149;
use crate::io::Record150;
use crate::io::TableBuilder150;
use crate::io::Record151;
use crate::io::TableBuilder151;
use crate::io::Record152;
use crate::io::TableBuilder152;
use crate::io::Record153;
use crate::io::TableBuilder153;
use crate::io::Record154;
use crate::io::TableBuilder154;
use crate::io::Record155;
use crate::io::TableBuilder155;
use crate::io::Record156;
use crate::io::TableBuilder156;
use crate::io::Record157;
use crate::io::TableBuilder157;
use crate::io::Record158;
use crate::io::TableBuilder158;
use crate::io::Record159;
use crate::io::TableBuilder159;
use crate::io::Record160;
use crate::io::TableBuilder160;
use crate::io::Record161;
use crate::io::TableBuilder161;
use crate::io::Record162;
use crate::io::TableBuilder162;
use crate::io::Record163;
use crate::io::TableBuilder163;
use crate::io::Record164;
use crate::io::TableBuilder164;
use crate::io::Record165;
use crate::io::TableBuilder165;
use crate::io::Record166;
use crate::io::TableBuilder166;
use crate::io::Record167;
use crate::io::TableBuilder167;
use crate::io::Record168;
use crate::io::TableBuilder168;
use crate::io::Record169;
use crate::io::TableBuilder169;
use crate::io::Record170;
use crate::io::TableBuilder170;
use crate::io::Record171;
use crate::io::TableBuilder171;
use crate::io::Record172;
use crate::io::TableBuilder172;
use crate::io::Record173;
use crate::io::TableBuilder173;
use crate::io::Record174;
use crate::io::TableBuilder174;
use crate::io::Record175;
use crate::io::TableBuilder175;
use crate::io::Record176;
use crate::io::TableBuilder176;
use crate::io::Record177;
use crate::io::TableBuilder177;
use crate::io::Record178;
use crate::io::TableBuilder178;
use crate::io::Record179;
use crate::io::TableBuilder179;
use crate::io::Record180;
use crate::io::TableBuilder180;
use crate::io::Record181;
use crate::io::TableBuilder181;
use crate::io::Record182;
use crate::io::TableBuilder182;
use crate::io::Record183;
use crate::io::TableBuilder183;
use crate::io::Record184;
use crate::io::TableBuilder184;
use crate::io::Record185;
use crate::io::TableBuilder185;
use crate::io::Record186;
use crate::io::TableBuilder186;
use crate::io::Record187;
use crate::io::TableBuilder187;
use crate::io::Record188;
use crate::io::TableBuilder188;
use crate::io::Record189;
use crate::io::TableBuilder189;
use crate::io::Record190;
use crate::io::TableBuilder190;
use crate::io::Record191;
use crate::io::TableBuilder191;
use crate::io::Record192;
use crate::io::TableBuilder192;
use crate::io::Record193;
use crate::io::TableBuilder193;
use crate::io::Record194;
use crate::io::TableBuilder194;
use crate::io::Record195;
use crate::io::TableBuilder195;
use crate::io::Record196;
use crate::io::TableBuilder196;
use crate::io::Record197;
use crate::io::TableBuilder197;
use crate::io::Record198;
use crate::io::TableBuilder198;
use crate::io::Record199;
use crate::io::TableBuilder199;
use crate::io::Record200;
use crate::io::TableBuilder200;
use crate::io::Record201;
use crate::io::TableBuilder201;
use crate::io::Record202;
use crate::io::TableBuilder202;
use crate::io::Record203;
use crate::io::TableBuilder203;
use crate::io::Record204;
use crate::io::TableBuilder204;
use crate::io::Record205;
use crate::io::TableBuilder205;
use crate::io::Record206;
use crate::io::TableBuilder206;
use crate::io::Record207;
use crate::io::TableBuilder207;
use crate::io::Record208;
use crate::io::TableBuilder208;
use crate::io::Record209;
use crate::io::TableBuilder209;
use crate::io::Record210;
use crate::io::TableBuilder210;
use crate::io::Record211;
use crate::io::TableBuilder211;
use crate::io::Record212;
use crate::io::TableBuilder212;
use crate::io::Record213;
use crate::io::TableBuilder213;
use crate::io::Record214;
use crate::io::TableBuilder214;
use crate::io::Record215;
use crate::io::TableBuilder215;
use crate::io::Record216;
use crate::io::TableBuilder216;
use crate::io::Record217;
use crate::io::TableBuilder217;
use crate::io::Record218;
use crate::io::TableBuilder218;
use crate::io::Record219;
use crate::io::TableBuilder219;
use crate::io::Record220;
use crate::io::TableBuilder220;
use crate::io::Record221;
use crate::io::TableBuilder221;
use crate::io::Record222;
use crate::io::TableBuilder222;
use crate::io::Record223;
use crate::io::TableBuilder223;
use crate::io::Record224;
use crate::io::TableBuilder224;
use crate::io::Record225;
use crate::io::TableBuilder225;
use crate::io::Record226;
use crate::io::TableBuilder226;
use crate::io::Record227;
use crate::io::TableBuilder227;
use crate::io::Record228;
use crate::io::TableBuilder228;
use crate::io::Record229;
use crate::io::TableBuilder229;
use crate::io::Record230;
use crate::io::TableBuilder230;
use crate::io::Record231;
use crate::io::TableBuilder231;
use crate::io::Record232;
use crate::io::TableBuilder232;
use crate::io::Record233;
use crate::io::TableBuilder233;
use crate::io::Record234;
use crate::io::TableBuilder234;
use crate::io::Record235;
use crate::io::TableBuilder235;
use crate::io::Record236;
use crate::io::TableBuilder236;
use crate::io::Record237;
use crate::io::TableBuilder237;
use crate::io::Record238;
use crate::io::TableBuilder238;
use crate::io::Record239;
use crate::io::TableBuilder239;
use crate::io::Record240;
use crate::io::TableBuilder240;
use crate::io::Record241;
use crate::io::TableBuilder241;
use crate::io::Record242;
use crate::io::TableBuilder242;
use crate::io::Record243;
use crate::io::TableBuilder243;
use crate::io::Record244;
use crate::io::TableBuilder244;
use crate::io::Record245;
use crate::io::TableBuilder245;
use crate::io::Record246;
use crate::io::TableBuilder246;
use crate::io::Record247;
use crate::io::TableBuilder247;
use crate::io::Record248;
use crate::io::TableBuilder248;
use crate::io::Record249;
use crate::io::TableBuilder249;
use crate::io::Record250;
use crate::io::TableBuilder250;
use crate::io::Record251;
use crate::io::TableBuilder251;
use crate::io::Record252;
use crate::io::TableBuilder252;
use crate::io::Record253;
use crate::io::TableBuilder253;
use crate::io::Record254;
use crate::io::TableBuilder254;
use crate::io::Record255;
use crate::io::TableBuilder255;
use crate::io::Record256;
use crate::io::TableBuilder256;
use crate::io::Record257;
use crate::io::TableBuilder257;
use crate::io::Record258;
use crate::io::TableBuilder258;
use crate::io::Record259;
use crate::io::TableBuilder259;
use crate::io::Record260;
use crate::io::TableBuilder260;
use crate::io::Record261;
use crate::io::TableBuilder261;
use crate::io::Record262;
use crate::io::TableBuilder262;
use crate::io::Record263;
use crate::io::TableBuilder263;
use crate::io::Record264;
use crate::io::TableBuilder264;
use crate::io::Record265;
use crate::io::TableBuilder265;
use crate::io::Record266;
use crate::io::TableBuilder266;
use crate::io::Record267;
use crate::io::TableBuilder267;
use crate::io::Record268;
use crate::io::TableBuilder268;
use crate::io::Record269;
use crate::io::TableBuilder269;
use crate::io::Record270;
use crate::io::TableBuilder270;
use crate::io::Record271;
use crate::io::TableBuilder271;
use crate::io::Record272;
use crate::io::TableBuilder272;
use crate::io::Record273;
use crate::io::TableBuilder273;
use crate::io::Record274;
use crate::io::TableBuilder274;
use crate::io::Record275;
use crate::io::TableBuilder275;
use crate::io::Record276;
use crate::io::TableBuilder276;
use crate::io::Record277;
use crate::io::TableBuilder277;
use crate::io::Record278;
use crate::io::TableBuilder278;
use crate::io::Record279;
use crate::io::TableBuilder279;
use crate::io::Record280;
use crate::io::TableBuilder280;
use crate::io::Record281;
use crate::io::TableBuilder281;
use crate::io::Record282;
use crate::io::TableBuilder282;
use crate::io::Record283;
use crate::io::TableBuilder283;
use crate::io::Record284;
use crate::io::TableBuilder284;
use crate::io::Record285;
use crate::io::TableBuilder285;
use crate::io::Record286;
use crate::io::TableBuilder286;
use crate::io::Record287;
use crate::io::TableBuilder287;
use crate::io::Record288;
use crate::io::TableBuilder288;
use crate::io::Record289;
use crate::io::TableBuilder289;
use crate::io::Record290;
use crate::io::TableBuilder290;
use crate::io::Record291;
use crate::io::TableBuilder291;
use crate::io::Record292;
use crate::io::TableBuilder292;
use crate::io::Record293;
use crate::io::TableBuilder293;
use crate::io::Record294;
use crate::io::TableBuilder294;
use crate::io::Record295;
use crate::io::TableBuilder295;
use crate::io::Record296;
use crate::io::TableBuilder296;
use crate::io::Record297;
use crate::io::TableBuilder297;
use crate::io::Record298;
use crate::io::TableBuilder298;
use crate::io::Record299;
use crate::io::TableBuilder299;
use crate::io::Record300;
use crate::io::TableBuilder300;
use crate::io::Record301;
use crate::io::TableBuilder301;
use crate::io::Record302;
use crate::io::TableBuilder302;
use crate::io::Record303;
use crate::io::TableBuilder303;
use crate::io::Record304;
use crate::io::TableBuilder304;
use crate::io::Record305;
use crate::io::TableBuilder305;
use crate::io::Record306;
use crate::io::TableBuilder306;
use crate::io::Record307;
use crate::io::TableBuilder307;
use crate::io::Record308;
use crate::io::TableBuilder308;
use crate::io::Record309;
use crate::io::TableBuilder309;
use crate::io::Record310;
use crate::io::TableBuilder310;
use crate::io::Record311;
use crate::io::TableBuilder311;
use crate::io::Record312;
use crate::io::TableBuilder312;
use crate::io::Record313;
use crate::io::TableBuilder313;
use crate::io::Record314;
use crate::io::TableBuilder314;
use crate::io::Record315;
use crate::io::TableBuilder315;
use crate::io::Record316;
use crate::io::TableBuilder316;
use crate::io::Record317;
use crate::io::TableBuilder317;
use crate::io::Record318;
use crate::io::TableBuilder318;
use crate::io::Record319;
use crate::io::TableBuilder319;
use crate::io::Record320;
use crate::io::TableBuilder320;
use crate::io::Record321;
use crate::io::TableBuilder321;
use crate::io::Record322;
use crate::io::TableBuilder322;
use crate::io::Record323;
use crate::io::TableBuilder323;
use crate::io::Record324;
use crate::io::TableBuilder324;
use crate::io::Record325;
use crate::io::TableBuilder325;
use crate::io::Record326;
use crate::io::TableBuilder326;
use crate::io::Record327;
use crate::io::TableBuilder327;
use crate::io::Record328;
use crate::io::TableBuilder328;
use crate::io::Record329;
use crate::io::TableBuilder329;
use crate::io::Record330;
use crate::io::TableBuilder330;
use crate::io::Record331;
use crate::io::TableBuilder331;
use crate::io::Record332;
use crate::io::TableBuilder332;
use crate::io::Record333;
use crate::io::TableBuilder333;
use crate::io::Record334;
use crate::io::TableBuilder334;
use crate::io::Record335;
use crate::io::TableBuilder335;
use crate::io::Record336;
use crate::io::TableBuilder336;
use crate::io::Record337;
use crate::io::TableBuilder337;
use crate::io::Record338;
use crate::io::TableBuilder338;
use crate::io::Record339;
use crate::io::TableBuilder339;
use crate::io::Record340;
use crate::io::TableBuilder340;
use crate::io::Record341;
use crate::io::TableBuilder341;
use crate::io::Record342;
use crate::io::TableBuilder342;
use crate::io::Record343;
use crate::io::TableBuilder343;
use crate::io::Record344;
use crate::io::TableBuilder344;
use crate::io::Record345;
use crate::io::TableBuilder345;
use crate::io::Record346;
use crate::io::TableBuilder346;
use crate::io::Record347;
use crate::io::TableBuilder347;
use crate::io::Record348;
use crate::io::TableBuilder348;
use crate::io::Record349;
use crate::io::TableBuilder349;
use crate::io::Record350;
use crate::io::TableBuilder350;
use crate::io::Record351;
use crate::io::TableBuilder351;
use crate::io::Record352;
use crate::io::TableBuilder352;
use crate::io::Record353;
use crate::io::TableBuilder353;
use crate::io::Record354;
use crate::io::TableBuilder354;
////   END ARBORETUM GENERATED CODE ////
