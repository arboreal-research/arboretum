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

pub static mut RECORD_SINK: Option<Box<dyn Fn(FfiMessage)>> = None;

// file
#[derive(Debug, Serialize, Deserialize)]
pub struct Record0 {
  pub c0: u64, // id
  pub c1: String, // filename
  pub c2: String, // content
}

#[no_mangle]
pub extern "C" fn arboretum_emit_file(c0: u64, c1: *const c_char, c2: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c1 = unsafe { CStr::from_ptr(c1) }.to_string_lossy().to_string();
  let c2 = unsafe { CStr::from_ptr(c2) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record0(Record0{c0, c1, c2, })));
}

// source_loc
#[derive(Debug, Serialize, Deserialize)]
pub struct Record1 {
  pub c0: u64, // id
  pub c1: u64, // file_id
  pub c2: u64, // line
  pub c3: u64, // col
  pub c4: u64, // expansion_loc
  pub c5: u64, // spelling_loc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_source_loc(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record1(Record1{c0, c1, c2, c3, c4, c5, })));
}

// source_range
#[derive(Debug, Serialize, Deserialize)]
pub struct Record2 {
  pub c0: u64, // id
  pub c1: u64, // begin
  pub c2: u64, // end
}

#[no_mangle]
pub extern "C" fn arboretum_emit_source_range(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record2(Record2{c0, c1, c2, })));
}

// enum
#[derive(Debug, Serialize, Deserialize)]
pub struct Record3 {
  pub c0: u64, // id
  pub c1: String, // name
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum(c0: u64, c1: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c1 = unsafe { CStr::from_ptr(c1) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record3(Record3{c0, c1, })));
}

// enum_value
#[derive(Debug, Serialize, Deserialize)]
pub struct Record4 {
  pub c0: u64, // id
  pub c1: u64, // enum_id
  pub c2: String, // name
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value(c0: u64, c1: u64, c2: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c2 = unsafe { CStr::from_ptr(c2) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record4(Record4{c0, c1, c2, })));
}

// QualType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record5 {
  pub c0: u64, // id
  pub c1: u64, // Type_id
  pub c2: bool, // is_const
  pub c3: bool, // is_volatile
  pub c4: bool, // is_restrict
}

#[no_mangle]
pub extern "C" fn arboretum_emit_QualType(c0: u64, c1: u64, c2: bool, c3: bool, c4: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record5(Record5{c0, c1, c2, c3, c4, })));
}

// CFG
#[derive(Debug, Serialize, Deserialize)]
pub struct Record6 {
  pub c0: u64, // id
  pub c1: u64, // entry_block_id
  pub c2: u64, // exit_block_id
  pub c3: bool, // is_linear
  pub c4: u64, // indirect_goto
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFG(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record6(Record6{c0, c1, c2, c3, c4, })));
}

// CFG_blocks
#[derive(Debug, Serialize, Deserialize)]
pub struct Record7 {
  pub c0: u64, // CFG_id
  pub c1: u64, // CFGBlock_id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFG_blocks(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record7(Record7{c0, c1, })));
}

// CFG_try_blocks
#[derive(Debug, Serialize, Deserialize)]
pub struct Record8 {
  pub c0: u64, // CFG_id
  pub c1: u64, // CFGBlock_id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFG_try_blocks(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record8(Record8{c0, c1, })));
}

// CFG_edges
#[derive(Debug, Serialize, Deserialize)]
pub struct Record9 {
  pub c0: u64, // CFGBlock_src
  pub c1: u64, // CFGBlock_dst
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFG_edges(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record9(Record9{c0, c1, })));
}

// CFGBlock
#[derive(Debug, Serialize, Deserialize)]
pub struct Record10 {
  pub c0: u64, // id
  pub c1: u64, // terminator_stmt
  pub c2: u64, // terminator_kind
  pub c3: u64, // terminator_cond
  pub c4: u64, // label_stmt
  pub c5: u64, // loop_target
  pub c6: bool, // has_no_return_element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGBlock(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record10(Record10{c0, c1, c2, c3, c4, c5, c6, })));
}

// CFGBlock_elements
#[derive(Debug, Serialize, Deserialize)]
pub struct Record11 {
  pub c0: u64, // CFGBlock_id
  pub c1: u64, // CFGElement_id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGBlock_elements(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record11(Record11{c0, c1, })));
}

// CFGElement
#[derive(Debug, Serialize, Deserialize)]
pub struct Record12 {
  pub c0: u64, // id
  pub c1: u64, // kind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGElement(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record12(Record12{c0, c1, })));
}

// CFGInitializer
#[derive(Debug, Serialize, Deserialize)]
pub struct Record13 {
  pub c0: u64, // id
  pub c1: u64, // getInitializer
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGInitializer(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record13(Record13{c0, c1, })));
}

// CFGScopeBegin
#[derive(Debug, Serialize, Deserialize)]
pub struct Record14 {
  pub c0: u64, // id
  pub c1: u64, // getTriggerStmt
  pub c2: u64, // getVarDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGScopeBegin(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record14(Record14{c0, c1, c2, })));
}

// CFGScopeEnd
#[derive(Debug, Serialize, Deserialize)]
pub struct Record15 {
  pub c0: u64, // id
  pub c1: u64, // getTriggerStmt
  pub c2: u64, // getVarDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGScopeEnd(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record15(Record15{c0, c1, c2, })));
}

// CFGNewAllocator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record16 {
  pub c0: u64, // id
  pub c1: u64, // getAllocatorExpr
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGNewAllocator(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record16(Record16{c0, c1, })));
}

// CFGLifetimeEnds
#[derive(Debug, Serialize, Deserialize)]
pub struct Record17 {
  pub c0: u64, // id
  pub c1: u64, // getTriggerStmt
  pub c2: u64, // getVarDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGLifetimeEnds(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record17(Record17{c0, c1, c2, })));
}

// CFGLoopExit
#[derive(Debug, Serialize, Deserialize)]
pub struct Record18 {
  pub c0: u64, // id
  pub c1: u64, // getLoopStmt
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGLoopExit(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record18(Record18{c0, c1, })));
}

// CFGStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record19 {
  pub c0: u64, // id
  pub c1: u64, // getStmt
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGStmt(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record19(Record19{c0, c1, })));
}

// CFGConstructor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record20 {
  pub c0: u64, // id
  pub c1: u64, // getStmt
  pub c2: u64, // getConstructionContext
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGConstructor(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record20(Record20{c0, c1, c2, })));
}

// CFGCXXRecordTypedCall
#[derive(Debug, Serialize, Deserialize)]
pub struct Record21 {
  pub c0: u64, // id
  pub c1: u64, // getStmt
  pub c2: u64, // getConstructionContext
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGCXXRecordTypedCall(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record21(Record21{c0, c1, c2, })));
}

// CFGAutomaticObjDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record22 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getTriggerStmt
  pub c3: u64, // getVarDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGAutomaticObjDtor(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record22(Record22{c0, c1, c2, c3, })));
}

// CFGDeleteDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record23 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getCXXRecordDecl
  pub c3: u64, // getDeleteExpr
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGDeleteDtor(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record23(Record23{c0, c1, c2, c3, })));
}

// CFGBaseDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record24 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getBaseSpecifier
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGBaseDtor(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record24(Record24{c0, c1, c2, })));
}

// CFGMemberDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record25 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getFieldDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGMemberDtor(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record25(Record25{c0, c1, c2, })));
}

// CFGTemporaryDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record26 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getBindTemporaryExpr
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGTemporaryDtor(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record26(Record26{c0, c1, c2, })));
}

// CFGCleanupFunction
#[derive(Debug, Serialize, Deserialize)]
pub struct Record27 {
  pub c0: u64, // id
  pub c1: u64, // getVarDecl
  pub c2: u64, // getFunctionDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CFGCleanupFunction(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record27(Record27{c0, c1, c2, })));
}

// Decl_usr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record28 {
  pub c0: u64, // id
  pub c1: String, // usr
}

#[no_mangle]
pub extern "C" fn arboretum_emit_Decl_usr(c0: u64, c1: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c1 = unsafe { CStr::from_ptr(c1) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record28(Record28{c0, c1, })));
}

// QualType_usr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record29 {
  pub c0: u64, // id
  pub c1: String, // usr
}

#[no_mangle]
pub extern "C" fn arboretum_emit_QualType_usr(c0: u64, c1: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c1 = unsafe { CStr::from_ptr(c1) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record29(Record29{c0, c1, })));
}

// FunctionDecl_cfg
#[derive(Debug, Serialize, Deserialize)]
pub struct Record30 {
  pub c0: u64, // id
  pub c1: u64, // cfg
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionDecl_cfg(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record30(Record30{c0, c1, })));
}

// RValueReferenceType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record31 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RValueReferenceType(c0: u64, c1: bool, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record31(Record31{c0, c1, c2, })));
}

// IncompleteArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record32 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_IncompleteArrayType(c0: u64, c1: bool, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record32(Record32{c0, c1, c2, })));
}

// DependentAddressSpaceType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record33 {
  pub c0: u64, // id
  pub c1: u64, // getAddrSpaceExpr
  pub c2: u64, // getPointeeType
  pub c3: u64, // getAttributeLoc
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentAddressSpaceType(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record33(Record33{c0, c1, c2, c3, c4, c5, })));
}

// DependentSizedExtVectorType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record34 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: u64, // getElementType
  pub c3: u64, // getAttributeLoc
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentSizedExtVectorType(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record34(Record34{c0, c1, c2, c3, c4, c5, })));
}

// DependentBitIntType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record35 {
  pub c0: u64, // id
  pub c1: bool, // isUnsigned
  pub c2: bool, // isSigned
  pub c3: u64, // getNumBitsExpr
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentBitIntType(c0: u64, c1: bool, c2: bool, c3: u64, c4: bool, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record35(Record35{c0, c1, c2, c3, c4, c5, })));
}

// SubstTemplateTypeParmType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record36 {
  pub c0: u64, // id
  pub c1: u64, // getReplacementType
  pub c2: u64, // getAssociatedDecl
  pub c3: u64, // getReplacedParameter
  pub c4: u32, // getIndex
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SubstTemplateTypeParmType(c0: u64, c1: u64, c2: u64, c3: u64, c4: u32, c5: bool, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record36(Record36{c0, c1, c2, c3, c4, c5, c6, })));
}

// VectorType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record37 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: u32, // getNumElements
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
  pub c5: u64, // getVectorKind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_VectorType(c0: u64, c1: u64, c2: u32, c3: bool, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record37(Record37{c0, c1, c2, c3, c4, c5, })));
}

// MacroQualifiedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record38 {
  pub c0: u64, // id
  pub c1: u64, // getUnderlyingType
  pub c2: u64, // getModifiedType
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MacroQualifiedType(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record38(Record38{c0, c1, c2, c3, c4, })));
}

// TypeOfType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record39 {
  pub c0: u64, // id
  pub c1: u64, // getUnmodifiedType
  pub c2: u64, // desugar
  pub c3: bool, // isSugared
  pub c4: u64, // getKind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypeOfType(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record39(Record39{c0, c1, c2, c3, c4, })));
}

// TagType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record40 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // isBeingDefined
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TagType(c0: u64, c1: u64, c2: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record40(Record40{c0, c1, c2, })));
}

// ConstantArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record41 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ConstantArrayType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record41(Record41{c0, c1, c2, c3, })));
}

// RecordType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record42 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // hasConstFields
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RecordType(c0: u64, c1: u64, c2: bool, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record42(Record42{c0, c1, c2, c3, c4, })));
}

// PipeType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record43 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
  pub c4: bool, // isReadOnly
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PipeType(c0: u64, c1: u64, c2: bool, c3: u64, c4: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record43(Record43{c0, c1, c2, c3, c4, })));
}

// ConstantMatrixType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record44 {
  pub c0: u64, // id
  pub c1: u32, // getNumRows
  pub c2: u32, // getNumColumns
  pub c3: u32, // getNumElementsFlattened
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ConstantMatrixType(c0: u64, c1: u32, c2: u32, c3: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record44(Record44{c0, c1, c2, c3, })));
}

// UsingType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record45 {
  pub c0: u64, // id
  pub c1: u64, // getFoundDecl
  pub c2: u64, // getUnderlyingType
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
  pub c5: bool, // typeMatchesDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UsingType(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64, c5: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record45(Record45{c0, c1, c2, c3, c4, c5, })));
}

// TypeWithKeyword
#[derive(Debug, Serialize, Deserialize)]
pub struct Record46 {
  pub c0: u64, // id
  pub c1: u64, // getKeyword
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypeWithKeyword(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record46(Record46{c0, c1, })));
}

// DeducedTemplateSpecializationType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record47 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DeducedTemplateSpecializationType(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record47(Record47{c0, })));
}

// DependentSizedMatrixType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record48 {
  pub c0: u64, // id
  pub c1: u64, // getRowExpr
  pub c2: u64, // getColumnExpr
  pub c3: u64, // getAttributeLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentSizedMatrixType(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record48(Record48{c0, c1, c2, c3, })));
}

// AttributedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record49 {
  pub c0: u64, // id
  pub c1: u64, // getAttrKind
  pub c2: u64, // getModifiedType
  pub c3: u64, // getEquivalentType
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
  pub c6: bool, // isQualifier
  pub c7: bool, // isMSTypeSpec
  pub c8: bool, // isWebAssemblyFuncrefSpec
  pub c9: bool, // isCallingConv
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AttributedType(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: u64, c6: bool, c7: bool, c8: bool, c9: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record49(Record49{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// DependentTemplateSpecializationType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record50 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentTemplateSpecializationType(c0: u64, c1: bool, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record50(Record50{c0, c1, c2, })));
}

// TemplateTypeParmType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record51 {
  pub c0: u64, // id
  pub c1: u32, // getDepth
  pub c2: u32, // getIndex
  pub c3: bool, // isParameterPack
  pub c4: u64, // getDecl
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TemplateTypeParmType(c0: u64, c1: u32, c2: u32, c3: bool, c4: u64, c5: bool, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record51(Record51{c0, c1, c2, c3, c4, c5, c6, })));
}

// BlockPointerType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record52 {
  pub c0: u64, // id
  pub c1: u64, // getPointeeType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BlockPointerType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record52(Record52{c0, c1, c2, c3, })));
}

// InjectedClassNameType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record53 {
  pub c0: u64, // id
  pub c1: u64, // getInjectedSpecializationType
  pub c2: u64, // getInjectedTST
  pub c3: u64, // getDecl
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_InjectedClassNameType(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record53(Record53{c0, c1, c2, c3, c4, c5, })));
}

// SubstTemplateTypeParmPackType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record54 {
  pub c0: u64, // id
  pub c1: u64, // getAssociatedDecl
  pub c2: u64, // getReplacedParameter
  pub c3: u32, // getIndex
  pub c4: bool, // getFinal
  pub c5: u32, // getNumArgs
  pub c6: bool, // isSugared
  pub c7: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SubstTemplateTypeParmPackType(c0: u64, c1: u64, c2: u64, c3: u32, c4: bool, c5: u32, c6: bool, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record54(Record54{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// BuiltinType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record55 {
  pub c0: u64, // id
  pub c1: u64, // getKind
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
  pub c4: bool, // isInteger
  pub c5: bool, // isSignedInteger
  pub c6: bool, // isUnsignedInteger
  pub c7: bool, // isFloatingPoint
  pub c8: bool, // isSVEBool
  pub c9: bool, // isSVECount
  pub c10: bool, // isPlaceholderType
  pub c11: bool, // isNonOverloadPlaceholderType
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BuiltinType(c0: u64, c1: u64, c2: bool, c3: u64, c4: bool, c5: bool, c6: bool, c7: bool, c8: bool, c9: bool, c10: bool, c11: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record55(Record55{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, })));
}

// DependentVectorType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record56 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: u64, // getElementType
  pub c3: u64, // getAttributeLoc
  pub c4: u64, // getVectorKind
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentVectorType(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record56(Record56{c0, c1, c2, c3, c4, c5, c6, })));
}

// ExtVectorType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record57 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ExtVectorType(c0: u64, c1: bool, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record57(Record57{c0, c1, c2, })));
}

// ParenType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record58 {
  pub c0: u64, // id
  pub c1: u64, // getInnerType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ParenType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record58(Record58{c0, c1, c2, c3, })));
}

// UnaryTransformType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record59 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
  pub c3: u64, // getUnderlyingType
  pub c4: u64, // getBaseType
  pub c5: u64, // getUTTKind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnaryTransformType(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record59(Record59{c0, c1, c2, c3, c4, c5, })));
}

// UnresolvedUsingType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record60 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnresolvedUsingType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record60(Record60{c0, c1, c2, c3, })));
}

// ComplexType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record61 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ComplexType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record61(Record61{c0, c1, c2, c3, })));
}

// PointerType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record62 {
  pub c0: u64, // id
  pub c1: u64, // getPointeeType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PointerType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record62(Record62{c0, c1, c2, c3, })));
}

// BTFTagAttributedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record63 {
  pub c0: u64, // id
  pub c1: u64, // getWrappedType
  pub c2: u64, // getAttr
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BTFTagAttributedType(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record63(Record63{c0, c1, c2, c3, c4, })));
}

// DependentNameType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record64 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentNameType(c0: u64, c1: bool, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record64(Record64{c0, c1, c2, })));
}

// Type
#[derive(Debug, Serialize, Deserialize)]
pub struct Record65 {
  pub c0: u64, // id
  pub c1: bool, // containsUnexpandedParameterPack
  pub c2: u64, // getLocallyUnqualifiedSingleStepDesugaredType
  pub c3: u64, // getAsPlaceholderType
  pub c4: u64, // getObjCARCImplicitLifetime
  pub c5: u64, // getDependence
  pub c6: bool, // containsErrors
  pub c7: bool, // hasSizedVLAType
  pub c8: bool, // hasUnnamedOrLocalType
  pub c9: bool, // canDecayToPointerType
  pub c10: bool, // hasPointerRepresentation
  pub c11: bool, // hasObjCPointerRepresentation
  pub c12: bool, // hasIntegerRepresentation
  pub c13: bool, // hasSignedIntegerRepresentation
  pub c14: bool, // hasUnsignedIntegerRepresentation
  pub c15: bool, // hasFloatingRepresentation
  pub c16: u64, // getAsStructureType
  pub c17: u64, // getAsUnionType
  pub c18: u64, // getAsComplexIntegerType
  pub c19: u64, // getAsObjCInterfaceType
  pub c20: u64, // getAsObjCInterfacePointerType
  pub c21: u64, // getAsObjCQualifiedIdType
  pub c22: u64, // getAsObjCQualifiedClassType
  pub c23: u64, // getAsObjCQualifiedInterfaceType
  pub c24: u64, // getAsCXXRecordDecl
  pub c25: u64, // getAsRecordDecl
  pub c26: u64, // getAsTagDecl
  pub c27: u64, // getPointeeCXXRecordDecl
  pub c28: u64, // getBaseElementTypeUnsafe
  pub c29: u64, // getArrayElementTypeNoTypeQual
  pub c30: u64, // getPointeeOrArrayElementType
  pub c31: u64, // getLinkage
  pub c32: u64, // getVisibility
  pub c33: bool, // acceptsObjCTypeParams
  pub c34: u64, // getCanonicalTypeInternal
}

#[no_mangle]
pub extern "C" fn arboretum_emit_Type(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64, c5: u64, c6: bool, c7: bool, c8: bool, c9: bool, c10: bool, c11: bool, c12: bool, c13: bool, c14: bool, c15: bool, c16: u64, c17: u64, c18: u64, c19: u64, c20: u64, c21: u64, c22: u64, c23: u64, c24: u64, c25: u64, c26: u64, c27: u64, c28: u64, c29: u64, c30: u64, c31: u64, c32: u64, c33: bool, c34: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record65(Record65{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, })));
}

// DependentUnaryTransformType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record66 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentUnaryTransformType(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record66(Record66{c0, })));
}

// AtomicType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record67 {
  pub c0: u64, // id
  pub c1: u64, // getValueType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AtomicType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record67(Record67{c0, c1, c2, c3, })));
}

// AutoType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record68 {
  pub c0: u64, // id
  pub c1: u64, // getTypeConstraintConcept
  pub c2: bool, // isConstrained
  pub c3: bool, // isDecltypeAuto
  pub c4: bool, // isGNUAutoType
  pub c5: u64, // getKeyword
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AutoType(c0: u64, c1: u64, c2: bool, c3: bool, c4: bool, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record68(Record68{c0, c1, c2, c3, c4, c5, })));
}

// TemplateSpecializationType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record69 {
  pub c0: u64, // id
  pub c1: bool, // isCurrentInstantiation
  pub c2: bool, // isTypeAlias
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TemplateSpecializationType(c0: u64, c1: bool, c2: bool, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record69(Record69{c0, c1, c2, c3, c4, })));
}

// ReferenceType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record70 {
  pub c0: u64, // id
  pub c1: bool, // isSpelledAsLValue
  pub c2: bool, // isInnerRef
  pub c3: u64, // getPointeeTypeAsWritten
  pub c4: u64, // getPointeeType
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ReferenceType(c0: u64, c1: bool, c2: bool, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record70(Record70{c0, c1, c2, c3, c4, })));
}

// DeducedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record71 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
  pub c3: u64, // getDeducedType
  pub c4: bool, // isDeduced
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DeducedType(c0: u64, c1: bool, c2: u64, c3: u64, c4: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record71(Record71{c0, c1, c2, c3, c4, })));
}

// PackExpansionType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record72 {
  pub c0: u64, // id
  pub c1: u64, // getPattern
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PackExpansionType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record72(Record72{c0, c1, c2, c3, })));
}

// DependentSizedArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record73 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: u64, // getBracketsRange
  pub c3: u64, // getLBracketLoc
  pub c4: u64, // getRBracketLoc
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentSizedArrayType(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record73(Record73{c0, c1, c2, c3, c4, c5, c6, })));
}

// DecltypeType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record74 {
  pub c0: u64, // id
  pub c1: u64, // getUnderlyingExpr
  pub c2: u64, // getUnderlyingType
  pub c3: u64, // desugar
  pub c4: bool, // isSugared
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DecltypeType(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record74(Record74{c0, c1, c2, c3, c4, })));
}

// LValueReferenceType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record75 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_LValueReferenceType(c0: u64, c1: bool, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record75(Record75{c0, c1, c2, })));
}

// DependentDecltypeType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record76 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentDecltypeType(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record76(Record76{c0, })));
}

// TypeOfExprType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record77 {
  pub c0: u64, // id
  pub c1: u64, // getUnderlyingExpr
  pub c2: u64, // getKind
  pub c3: u64, // desugar
  pub c4: bool, // isSugared
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypeOfExprType(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record77(Record77{c0, c1, c2, c3, c4, })));
}

// FunctionProtoType_getParamTypes
#[derive(Debug, Serialize, Deserialize)]
pub struct Record78 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionProtoType_getParamTypes(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record78(Record78{c0, c1, c2, })));
}

// FunctionProtoType_param_types
#[derive(Debug, Serialize, Deserialize)]
pub struct Record79 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionProtoType_param_types(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record79(Record79{c0, c1, c2, })));
}

// FunctionProtoType_exceptions
#[derive(Debug, Serialize, Deserialize)]
pub struct Record80 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionProtoType_exceptions(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record80(Record80{c0, c1, c2, })));
}

// FunctionProtoType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record81 {
  pub c0: u64, // id
  pub c1: u32, // getNumParams
  pub c2: u64, // getExceptionSpecType
  pub c3: bool, // hasExceptionSpec
  pub c4: bool, // hasDynamicExceptionSpec
  pub c5: bool, // hasNoexceptExceptionSpec
  pub c6: bool, // hasDependentExceptionSpec
  pub c7: bool, // hasInstantiationDependentExceptionSpec
  pub c8: u32, // getNumExceptions
  pub c9: u64, // getNoexceptExpr
  pub c10: u64, // getExceptionSpecDecl
  pub c11: u64, // getExceptionSpecTemplate
  pub c12: u64, // canThrow
  pub c13: bool, // isVariadic
  pub c14: u64, // getEllipsisLoc
  pub c15: bool, // isTemplateVariadic
  pub c16: bool, // hasTrailingReturn
  pub c17: u64, // getRefQualifier
  pub c18: bool, // hasExtParameterInfos
  pub c19: u32, // getAArch64SMEAttributes
  pub c20: bool, // isSugared
  pub c21: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionProtoType(c0: u64, c1: u32, c2: u64, c3: bool, c4: bool, c5: bool, c6: bool, c7: bool, c8: u32, c9: u64, c10: u64, c11: u64, c12: u64, c13: bool, c14: u64, c15: bool, c16: bool, c17: u64, c18: bool, c19: u32, c20: bool, c21: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record81(Record81{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, })));
}

// AdjustedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record82 {
  pub c0: u64, // id
  pub c1: u64, // getOriginalType
  pub c2: u64, // getAdjustedType
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AdjustedType(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record82(Record82{c0, c1, c2, c3, c4, })));
}

// ArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record83 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: u64, // getSizeModifier
  pub c3: u32, // getIndexTypeCVRQualifiers
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ArrayType(c0: u64, c1: u64, c2: u64, c3: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record83(Record83{c0, c1, c2, c3, })));
}

// VariableArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record84 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: u64, // getBracketsRange
  pub c3: u64, // getLBracketLoc
  pub c4: u64, // getRBracketLoc
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_VariableArrayType(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record84(Record84{c0, c1, c2, c3, c4, c5, c6, })));
}

// EnumType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record85 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_EnumType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record85(Record85{c0, c1, c2, c3, })));
}

// DependentTypeOfExprType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record86 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentTypeOfExprType(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record86(Record86{c0, })));
}

// DecayedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record87 {
  pub c0: u64, // id
  pub c1: u64, // getDecayedType
  pub c2: u64, // getPointeeType
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DecayedType(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record87(Record87{c0, c1, c2, })));
}

// MemberPointerType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record88 {
  pub c0: u64, // id
  pub c1: u64, // getPointeeType
  pub c2: bool, // isMemberFunctionPointer
  pub c3: bool, // isMemberDataPointer
  pub c4: u64, // getClass
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MemberPointerType(c0: u64, c1: u64, c2: bool, c3: bool, c4: u64, c5: bool, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record88(Record88{c0, c1, c2, c3, c4, c5, c6, })));
}

// BitIntType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record89 {
  pub c0: u64, // id
  pub c1: bool, // isUnsigned
  pub c2: bool, // isSigned
  pub c3: u32, // getNumBits
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BitIntType(c0: u64, c1: bool, c2: bool, c3: u32, c4: bool, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record89(Record89{c0, c1, c2, c3, c4, c5, })));
}

// TypedefType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record90 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
  pub c4: bool, // typeMatchesDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypedefType(c0: u64, c1: u64, c2: bool, c3: u64, c4: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record90(Record90{c0, c1, c2, c3, c4, })));
}

// FunctionType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record91 {
  pub c0: u64, // id
  pub c1: u64, // getReturnType
  pub c2: bool, // getHasRegParm
  pub c3: u32, // getRegParmType
  pub c4: bool, // getNoReturnAttr
  pub c5: bool, // getCmseNSCallAttr
  pub c6: u64, // getCallConv
  pub c7: bool, // isConst
  pub c8: bool, // isVolatile
  pub c9: bool, // isRestrict
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionType(c0: u64, c1: u64, c2: bool, c3: u32, c4: bool, c5: bool, c6: u64, c7: bool, c8: bool, c9: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record91(Record91{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// FunctionNoProtoType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record92 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionNoProtoType(c0: u64, c1: bool, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record92(Record92{c0, c1, c2, })));
}

// ElaboratedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record93 {
  pub c0: u64, // id
  pub c1: u64, // getNamedType
  pub c2: u64, // desugar
  pub c3: bool, // isSugared
  pub c4: u64, // getOwnedTagDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ElaboratedType(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record93(Record93{c0, c1, c2, c3, c4, })));
}

// MatrixType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record94 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MatrixType(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record94(Record94{c0, c1, c2, c3, })));
}

// ClassTemplatePartialSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record95 {
  pub c0: u64, // id
  pub c1: bool, // hasAssociatedConstraints
  pub c2: u64, // getInstantiatedFromMember
  pub c3: u64, // getInstantiatedFromMemberTemplate
  pub c4: u64, // getInjectedSpecializationType
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ClassTemplatePartialSpecializationDecl(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record95(Record95{c0, c1, c2, c3, c4, })));
}

// TemplateParamObjectDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record96 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TemplateParamObjectDecl(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record96(Record96{c0, c1, })));
}

// CXXRecordDecl_methods
#[derive(Debug, Serialize, Deserialize)]
pub struct Record97 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXRecordDecl_methods(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record97(Record97{c0, c1, c2, })));
}

// CXXRecordDecl_ctors
#[derive(Debug, Serialize, Deserialize)]
pub struct Record98 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXRecordDecl_ctors(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record98(Record98{c0, c1, c2, })));
}

// CXXRecordDecl_friends
#[derive(Debug, Serialize, Deserialize)]
pub struct Record99 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXRecordDecl_friends(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record99(Record99{c0, c1, c2, })));
}

// CXXRecordDecl_getLambdaExplicitTemplateParameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record100 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXRecordDecl_getLambdaExplicitTemplateParameters(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record100(Record100{c0, c1, c2, })));
}

// CXXRecordDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record101 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
  pub c2: u64, // getPreviousDecl
  pub c3: u64, // getMostRecentDecl
  pub c4: u64, // getDefinition
  pub c5: bool, // hasDefinition
  pub c6: bool, // isDynamicClass
  pub c7: bool, // mayBeDynamicClass
  pub c8: bool, // mayBeNonDynamicClass
  pub c9: bool, // isParsingBaseSpecifiers
  pub c10: u32, // getODRHash
  pub c11: u32, // getNumBases
  pub c12: u32, // getNumVBases
  pub c13: bool, // hasAnyDependentBases
  pub c14: bool, // hasFriends
  pub c15: bool, // hasSimpleCopyConstructor
  pub c16: bool, // hasSimpleMoveConstructor
  pub c17: bool, // hasSimpleCopyAssignment
  pub c18: bool, // hasSimpleMoveAssignment
  pub c19: bool, // hasSimpleDestructor
  pub c20: bool, // hasDefaultConstructor
  pub c21: bool, // needsImplicitDefaultConstructor
  pub c22: bool, // hasUserDeclaredConstructor
  pub c23: bool, // hasUserProvidedDefaultConstructor
  pub c24: bool, // hasUserDeclaredCopyConstructor
  pub c25: bool, // needsImplicitCopyConstructor
  pub c26: bool, // needsOverloadResolutionForCopyConstructor
  pub c27: bool, // implicitCopyConstructorHasConstParam
  pub c28: bool, // hasCopyConstructorWithConstParam
  pub c29: bool, // hasUserDeclaredMoveOperation
  pub c30: bool, // hasUserDeclaredMoveConstructor
  pub c31: bool, // hasMoveConstructor
  pub c32: bool, // needsImplicitMoveConstructor
  pub c33: bool, // needsOverloadResolutionForMoveConstructor
  pub c34: bool, // hasUserDeclaredCopyAssignment
  pub c35: bool, // needsImplicitCopyAssignment
  pub c36: bool, // needsOverloadResolutionForCopyAssignment
  pub c37: bool, // implicitCopyAssignmentHasConstParam
  pub c38: bool, // hasCopyAssignmentWithConstParam
  pub c39: bool, // hasUserDeclaredMoveAssignment
  pub c40: bool, // hasMoveAssignment
  pub c41: bool, // needsImplicitMoveAssignment
  pub c42: bool, // needsOverloadResolutionForMoveAssignment
  pub c43: bool, // hasUserDeclaredDestructor
  pub c44: bool, // needsImplicitDestructor
  pub c45: bool, // needsOverloadResolutionForDestructor
  pub c46: bool, // isLambda
  pub c47: bool, // isGenericLambda
  pub c48: bool, // lambdaIsDefaultConstructibleAndAssignable
  pub c49: u64, // getLambdaCallOperator
  pub c50: u64, // getDependentLambdaCallOperator
  pub c51: bool, // isCapturelessLambda
  pub c52: bool, // isAggregate
  pub c53: bool, // hasInClassInitializer
  pub c54: bool, // hasUninitializedReferenceMember
  pub c55: bool, // isPOD
  pub c56: bool, // isCLike
  pub c57: bool, // isEmpty
  pub c58: bool, // hasInitMethod
  pub c59: bool, // hasPrivateFields
  pub c60: bool, // hasProtectedFields
  pub c61: bool, // hasDirectFields
  pub c62: bool, // isPolymorphic
  pub c63: bool, // isAbstract
  pub c64: bool, // isStandardLayout
  pub c65: bool, // isCXX11StandardLayout
  pub c66: bool, // hasMutableFields
  pub c67: bool, // hasVariantMembers
  pub c68: bool, // hasTrivialDefaultConstructor
  pub c69: bool, // hasNonTrivialDefaultConstructor
  pub c70: bool, // hasConstexprNonCopyMoveConstructor
  pub c71: bool, // defaultedDefaultConstructorIsConstexpr
  pub c72: bool, // hasConstexprDefaultConstructor
  pub c73: bool, // hasTrivialCopyConstructor
  pub c74: bool, // hasTrivialCopyConstructorForCall
  pub c75: bool, // hasNonTrivialCopyConstructor
  pub c76: bool, // hasNonTrivialCopyConstructorForCall
  pub c77: bool, // hasTrivialMoveConstructor
  pub c78: bool, // hasTrivialMoveConstructorForCall
  pub c79: bool, // hasNonTrivialMoveConstructor
  pub c80: bool, // hasNonTrivialMoveConstructorForCall
  pub c81: bool, // hasTrivialCopyAssignment
  pub c82: bool, // hasNonTrivialCopyAssignment
  pub c83: bool, // hasTrivialMoveAssignment
  pub c84: bool, // hasNonTrivialMoveAssignment
  pub c85: bool, // defaultedDestructorIsConstexpr
  pub c86: bool, // hasConstexprDestructor
  pub c87: bool, // hasTrivialDestructor
  pub c88: bool, // hasTrivialDestructorForCall
  pub c89: bool, // hasNonTrivialDestructor
  pub c90: bool, // hasNonTrivialDestructorForCall
  pub c91: bool, // allowConstDefaultInit
  pub c92: bool, // hasIrrelevantDestructor
  pub c93: bool, // hasNonLiteralTypeFieldsOrBases
  pub c94: bool, // hasInheritedConstructor
  pub c95: bool, // hasInheritedAssignment
  pub c96: bool, // isTriviallyCopyable
  pub c97: bool, // isTriviallyCopyConstructible
  pub c98: bool, // isTrivial
  pub c99: bool, // isLiteral
  pub c100: bool, // isStructural
  pub c101: u64, // getInstantiatedFromMemberClass
  pub c102: u64, // getDescribedClassTemplate
  pub c103: u64, // getTemplateSpecializationKind
  pub c104: u64, // getTemplateInstantiationPattern
  pub c105: u64, // getDestructor
  pub c106: bool, // isAnyDestructorNoReturn
  pub c107: u64, // isLocalClass
  pub c108: bool, // mayBeAbstract
  pub c109: bool, // isEffectivelyFinal
  pub c110: u32, // getDeviceLambdaManglingNumber
  pub c111: u64, // getMSVtorDispMode
  pub c112: bool, // isDependentLambda
  pub c113: bool, // isNeverDependentLambda
  pub c114: u32, // getLambdaDependencyKind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXRecordDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: bool, c7: bool, c8: bool, c9: bool, c10: u32, c11: u32, c12: u32, c13: bool, c14: bool, c15: bool, c16: bool, c17: bool, c18: bool, c19: bool, c20: bool, c21: bool, c22: bool, c23: bool, c24: bool, c25: bool, c26: bool, c27: bool, c28: bool, c29: bool, c30: bool, c31: bool, c32: bool, c33: bool, c34: bool, c35: bool, c36: bool, c37: bool, c38: bool, c39: bool, c40: bool, c41: bool, c42: bool, c43: bool, c44: bool, c45: bool, c46: bool, c47: bool, c48: bool, c49: u64, c50: u64, c51: bool, c52: bool, c53: bool, c54: bool, c55: bool, c56: bool, c57: bool, c58: bool, c59: bool, c60: bool, c61: bool, c62: bool, c63: bool, c64: bool, c65: bool, c66: bool, c67: bool, c68: bool, c69: bool, c70: bool, c71: bool, c72: bool, c73: bool, c74: bool, c75: bool, c76: bool, c77: bool, c78: bool, c79: bool, c80: bool, c81: bool, c82: bool, c83: bool, c84: bool, c85: bool, c86: bool, c87: bool, c88: bool, c89: bool, c90: bool, c91: bool, c92: bool, c93: bool, c94: bool, c95: bool, c96: bool, c97: bool, c98: bool, c99: bool, c100: bool, c101: u64, c102: u64, c103: u64, c104: u64, c105: u64, c106: bool, c107: u64, c108: bool, c109: bool, c110: u32, c111: u64, c112: bool, c113: bool, c114: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record101(Record101{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41, c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53, c54, c55, c56, c57, c58, c59, c60, c61, c62, c63, c64, c65, c66, c67, c68, c69, c70, c71, c72, c73, c74, c75, c76, c77, c78, c79, c80, c81, c82, c83, c84, c85, c86, c87, c88, c89, c90, c91, c92, c93, c94, c95, c96, c97, c98, c99, c100, c101, c102, c103, c104, c105, c106, c107, c108, c109, c110, c111, c112, c113, c114, })));
}

// TagDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record102 {
  pub c0: u64, // id
  pub c1: u64, // getBraceRange
  pub c2: u64, // getInnerLocStart
  pub c3: u64, // getOuterLocStart
  pub c4: u64, // getSourceRange
  pub c5: u64, // getCanonicalDecl
  pub c6: bool, // isThisDeclarationADefinition
  pub c7: bool, // isCompleteDefinition
  pub c8: bool, // isCompleteDefinitionRequired
  pub c9: bool, // isBeingDefined
  pub c10: bool, // isEmbeddedInDeclarator
  pub c11: bool, // isFreeStanding
  pub c12: bool, // mayHaveOutOfDateDef
  pub c13: bool, // isDependentType
  pub c14: bool, // isThisDeclarationADemotedDefinition
  pub c15: u64, // getDefinition
  pub c16: String, // getKindName
  pub c17: u64, // getTagKind
  pub c18: bool, // isStruct
  pub c19: bool, // isInterface
  pub c20: bool, // isClass
  pub c21: bool, // isUnion
  pub c22: bool, // isEnum
  pub c23: bool, // hasNameForLinkage
  pub c24: u64, // getTypedefNameForAnonDecl
  pub c25: u32, // getNumTemplateParameterLists
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TagDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: bool, c7: bool, c8: bool, c9: bool, c10: bool, c11: bool, c12: bool, c13: bool, c14: bool, c15: u64, c16: *const c_char, c17: u64, c18: bool, c19: bool, c20: bool, c21: bool, c22: bool, c23: bool, c24: u64, c25: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c16 = unsafe { CStr::from_ptr(c16) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record102(Record102{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, })));
}

// HLSLBufferDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record103 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getLocStart
  pub c3: u64, // getLBraceLoc
  pub c4: u64, // getRBraceLoc
  pub c5: bool, // isCBuffer
}

#[no_mangle]
pub extern "C" fn arboretum_emit_HLSLBufferDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record103(Record103{c0, c1, c2, c3, c4, c5, })));
}

// UsingDirectiveDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record104 {
  pub c0: u64, // id
  pub c1: u64, // getNominatedNamespaceAsWritten
  pub c2: u64, // getNominatedNamespace
  pub c3: u64, // getUsingLoc
  pub c4: u64, // getNamespaceKeyLocation
  pub c5: u64, // getIdentLocation
  pub c6: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UsingDirectiveDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record104(Record104{c0, c1, c2, c3, c4, c5, c6, })));
}

// NamespaceAliasDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record105 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
  pub c2: u64, // getNamespace
  pub c3: u64, // getAliasLoc
  pub c4: u64, // getNamespaceLoc
  pub c5: u64, // getTargetNameLoc
  pub c6: u64, // getAliasedNamespace
  pub c7: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_NamespaceAliasDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record105(Record105{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// TypeDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record106 {
  pub c0: u64, // id
  pub c1: u64, // getTypeForDecl
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypeDecl(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record106(Record106{c0, c1, c2, c3, })));
}

// RedeclarableTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record107 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
  pub c2: bool, // isMemberSpecialization
  pub c3: u64, // getInstantiatedFromMemberTemplate
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RedeclarableTemplateDecl(c0: u64, c1: u64, c2: bool, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record107(Record107{c0, c1, c2, c3, })));
}

// ImplicitConceptSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record108 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ImplicitConceptSpecializationDecl(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record108(Record108{c0, })));
}

// ConstructorUsingShadowDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record109 {
  pub c0: u64, // id
  pub c1: u64, // getIntroducer
  pub c2: u64, // getParent
  pub c3: u64, // getNominatedBaseClassShadowDecl
  pub c4: u64, // getConstructedBaseClassShadowDecl
  pub c5: u64, // getNominatedBaseClass
  pub c6: u64, // getConstructedBaseClass
  pub c7: bool, // constructsVirtualBase
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ConstructorUsingShadowDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record109(Record109{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// BuiltinTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record110 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getBuiltinTemplateKind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BuiltinTemplateDecl(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record110(Record110{c0, c1, c2, })));
}

// UsingShadowDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record111 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
  pub c2: u64, // getTargetDecl
  pub c3: u64, // getIntroducer
  pub c4: u64, // getNextUsingShadowDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UsingShadowDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record111(Record111{c0, c1, c2, c3, c4, })));
}

// BindingDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record112 {
  pub c0: u64, // id
  pub c1: u64, // getBinding
  pub c2: u64, // getDecomposedDecl
  pub c3: u64, // getHoldingVar
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BindingDecl(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record112(Record112{c0, c1, c2, c3, })));
}

// UsingDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record113 {
  pub c0: u64, // id
  pub c1: u64, // getUsingLoc
  pub c2: bool, // isAccessDeclaration
  pub c3: bool, // hasTypename
  pub c4: u64, // getSourceRange
  pub c5: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UsingDecl(c0: u64, c1: u64, c2: bool, c3: bool, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record113(Record113{c0, c1, c2, c3, c4, c5, })));
}

// UnresolvedUsingTypenameDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record114 {
  pub c0: u64, // id
  pub c1: u64, // getUsingLoc
  pub c2: u64, // getTypenameLoc
  pub c3: bool, // isPackExpansion
  pub c4: u64, // getEllipsisLoc
  pub c5: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnresolvedUsingTypenameDecl(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record114(Record114{c0, c1, c2, c3, c4, c5, })));
}

// LabelDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record115 {
  pub c0: u64, // id
  pub c1: u64, // getStmt
  pub c2: bool, // isGnuLocal
  pub c3: u64, // getSourceRange
  pub c4: bool, // isMSAsmLabel
  pub c5: bool, // isResolvedMSAsmLabel
  pub c6: String, // getMSAsmLabel
}

#[no_mangle]
pub extern "C" fn arboretum_emit_LabelDecl(c0: u64, c1: u64, c2: bool, c3: u64, c4: bool, c5: bool, c6: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c6 = unsafe { CStr::from_ptr(c6) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record115(Record115{c0, c1, c2, c3, c4, c5, c6, })));
}

// BaseUsingDecl_shadows
#[derive(Debug, Serialize, Deserialize)]
pub struct Record116 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BaseUsingDecl_shadows(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record116(Record116{c0, c1, c2, })));
}

// BaseUsingDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record117 {
  pub c0: u64, // id
  pub c1: u32, // shadow_size
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BaseUsingDecl(c0: u64, c1: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record117(Record117{c0, c1, })));
}

// UsingPackDecl_expansions
#[derive(Debug, Serialize, Deserialize)]
pub struct Record118 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UsingPackDecl_expansions(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record118(Record118{c0, c1, c2, })));
}

// UsingPackDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record119 {
  pub c0: u64, // id
  pub c1: u64, // getInstantiatedFromUsingDecl
  pub c2: u64, // getSourceRange
  pub c3: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UsingPackDecl(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record119(Record119{c0, c1, c2, c3, })));
}

// CXXMethodDecl_overridden_methods
#[derive(Debug, Serialize, Deserialize)]
pub struct Record120 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXMethodDecl_overridden_methods(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record120(Record120{c0, c1, c2, })));
}

// CXXMethodDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record121 {
  pub c0: u64, // id
  pub c1: bool, // isStatic
  pub c2: bool, // isInstance
  pub c3: bool, // isExplicitObjectMemberFunction
  pub c4: bool, // isImplicitObjectMemberFunction
  pub c5: bool, // isConst
  pub c6: bool, // isVolatile
  pub c7: bool, // isVirtual
  pub c8: bool, // isCopyAssignmentOperator
  pub c9: bool, // isMoveAssignmentOperator
  pub c10: u64, // getCanonicalDecl
  pub c11: u64, // getMostRecentDecl
  pub c12: u32, // size_overridden_methods
  pub c13: u64, // getParent
  pub c14: u64, // getThisType
  pub c15: u64, // getFunctionObjectParameterReferenceType
  pub c16: u64, // getFunctionObjectParameterType
  pub c17: u32, // getNumExplicitParams
  pub c18: u64, // getRefQualifier
  pub c19: bool, // hasInlineBody
  pub c20: bool, // isLambdaStaticInvoker
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXMethodDecl(c0: u64, c1: bool, c2: bool, c3: bool, c4: bool, c5: bool, c6: bool, c7: bool, c8: bool, c9: bool, c10: u64, c11: u64, c12: u32, c13: u64, c14: u64, c15: u64, c16: u64, c17: u32, c18: u64, c19: bool, c20: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record121(Record121{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, })));
}

// TemplateTypeParmDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record122 {
  pub c0: u64, // id
  pub c1: bool, // wasDeclaredWithTypename
  pub c2: bool, // hasDefaultArgument
  pub c3: u64, // getDefaultArgumentLoc
  pub c4: bool, // defaultArgumentWasInherited
  pub c5: u32, // getDepth
  pub c6: u32, // getIndex
  pub c7: bool, // isParameterPack
  pub c8: bool, // isPackExpansion
  pub c9: bool, // isExpandedParameterPack
  pub c10: bool, // hasTypeConstraint
  pub c11: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TemplateTypeParmDecl(c0: u64, c1: bool, c2: bool, c3: u64, c4: bool, c5: u32, c6: u32, c7: bool, c8: bool, c9: bool, c10: bool, c11: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record122(Record122{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, })));
}

// UnresolvedUsingIfExistsDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record123 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnresolvedUsingIfExistsDecl(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record123(Record123{c0, })));
}

// VarDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record124 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getStorageClass
  pub c3: u64, // getTSCSpec
  pub c4: u64, // getTLSKind
  pub c5: bool, // hasLocalStorage
  pub c6: bool, // isStaticLocal
  pub c7: bool, // hasExternalStorage
  pub c8: bool, // hasGlobalStorage
  pub c9: u64, // getStorageDuration
  pub c10: u64, // getLanguageLinkage
  pub c11: bool, // isExternC
  pub c12: bool, // isInExternCContext
  pub c13: bool, // isInExternCXXContext
  pub c14: bool, // isLocalVarDecl
  pub c15: bool, // isLocalVarDeclOrParm
  pub c16: bool, // isFunctionOrMethodVarDecl
  pub c17: bool, // isStaticDataMember
  pub c18: u64, // getCanonicalDecl
  pub c19: u64, // isThisDeclarationADefinition
  pub c20: u64, // hasDefinition
  pub c21: u64, // getActingDefinition
  pub c22: u64, // getDefinition
  pub c23: bool, // isOutOfLine
  pub c24: bool, // isFileVarDecl
  pub c25: u64, // getAnyInitializer
  pub c26: bool, // hasInit
  pub c27: u64, // getInit
  pub c28: u64, // getInitializingDeclaration
  pub c29: bool, // hasConstantInitialization
  pub c30: u64, // getInitStyle
  pub c31: bool, // isDirectInit
  pub c32: bool, // isThisDeclarationADemotedDefinition
  pub c33: bool, // isExceptionVariable
  pub c34: bool, // isNRVOVariable
  pub c35: bool, // isCXXForRangeDecl
  pub c36: bool, // isObjCForDecl
  pub c37: bool, // isARCPseudoStrong
  pub c38: bool, // isInline
  pub c39: bool, // isInlineSpecified
  pub c40: bool, // isConstexpr
  pub c41: bool, // isInitCapture
  pub c42: bool, // isParameterPack
  pub c43: bool, // isPreviousDeclInSameBlockScope
  pub c44: bool, // isEscapingByref
  pub c45: bool, // isNonEscapingByref
  pub c46: bool, // hasDependentAlignment
  pub c47: u64, // getTemplateInstantiationPattern
  pub c48: u64, // getInstantiatedFromStaticDataMember
  pub c49: u64, // getTemplateSpecializationKind
  pub c50: u64, // getTemplateSpecializationKindForInstantiation
  pub c51: u64, // getPointOfInstantiation
  pub c52: u64, // getDescribedVarTemplate
  pub c53: bool, // isKnownToBeDefined
}

#[no_mangle]
pub extern "C" fn arboretum_emit_VarDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: bool, c7: bool, c8: bool, c9: u64, c10: u64, c11: bool, c12: bool, c13: bool, c14: bool, c15: bool, c16: bool, c17: bool, c18: u64, c19: u64, c20: u64, c21: u64, c22: u64, c23: bool, c24: bool, c25: u64, c26: bool, c27: u64, c28: u64, c29: bool, c30: u64, c31: bool, c32: bool, c33: bool, c34: bool, c35: bool, c36: bool, c37: bool, c38: bool, c39: bool, c40: bool, c41: bool, c42: bool, c43: bool, c44: bool, c45: bool, c46: bool, c47: u64, c48: u64, c49: u64, c50: u64, c51: u64, c52: u64, c53: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record124(Record124{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41, c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53, })));
}

// FunctionTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record125 {
  pub c0: u64, // id
  pub c1: u64, // getTemplatedDecl
  pub c2: bool, // isThisDeclarationADefinition
  pub c3: u64, // getCanonicalDecl
  pub c4: u64, // getPreviousDecl
  pub c5: u64, // getMostRecentDecl
  pub c6: u64, // getInstantiatedFromMemberTemplate
  pub c7: bool, // isAbbreviated
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionTemplateDecl(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64, c7: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record125(Record125{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// ClassTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record126 {
  pub c0: u64, // id
  pub c1: u64, // getTemplatedDecl
  pub c2: bool, // isThisDeclarationADefinition
  pub c3: u64, // getCanonicalDecl
  pub c4: u64, // getPreviousDecl
  pub c5: u64, // getMostRecentDecl
  pub c6: u64, // getInstantiatedFromMemberTemplate
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ClassTemplateDecl(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record126(Record126{c0, c1, c2, c3, c4, c5, c6, })));
}

// TypedefNameDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record127 {
  pub c0: u64, // id
  pub c1: bool, // isModed
  pub c2: u64, // getUnderlyingType
  pub c3: u64, // getCanonicalDecl
  pub c4: bool, // isTransparentTag
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypedefNameDecl(c0: u64, c1: bool, c2: u64, c3: u64, c4: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record127(Record127{c0, c1, c2, c3, c4, })));
}

// TypeAliasTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record128 {
  pub c0: u64, // id
  pub c1: u64, // getTemplatedDecl
  pub c2: u64, // getCanonicalDecl
  pub c3: u64, // getPreviousDecl
  pub c4: u64, // getInstantiatedFromMemberTemplate
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypeAliasTemplateDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record128(Record128{c0, c1, c2, c3, c4, })));
}

// RecordDecl_fields
#[derive(Debug, Serialize, Deserialize)]
pub struct Record129 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RecordDecl_fields(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record129(Record129{c0, c1, c2, })));
}

// RecordDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record130 {
  pub c0: u64, // id
  pub c1: u64, // getPreviousDecl
  pub c2: u64, // getMostRecentDecl
  pub c3: bool, // hasFlexibleArrayMember
  pub c4: bool, // isAnonymousStructOrUnion
  pub c5: bool, // hasObjectMember
  pub c6: bool, // hasVolatileMember
  pub c7: bool, // hasLoadedFieldsFromExternalStorage
  pub c8: bool, // isNonTrivialToPrimitiveDefaultInitialize
  pub c9: bool, // isNonTrivialToPrimitiveCopy
  pub c10: bool, // isNonTrivialToPrimitiveDestroy
  pub c11: bool, // hasNonTrivialToPrimitiveDefaultInitializeCUnion
  pub c12: bool, // hasNonTrivialToPrimitiveDestructCUnion
  pub c13: bool, // hasNonTrivialToPrimitiveCopyCUnion
  pub c14: bool, // canPassInRegisters
  pub c15: u64, // getArgPassingRestrictions
  pub c16: bool, // isParamDestroyedInCallee
  pub c17: bool, // isRandomized
  pub c18: bool, // isInjectedClassName
  pub c19: bool, // isLambda
  pub c20: bool, // isCapturedRecord
  pub c21: u64, // getDefinition
  pub c22: bool, // isOrContainsUnion
  pub c23: bool, // field_empty
  pub c24: u64, // findFirstNamedDataMember
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RecordDecl(c0: u64, c1: u64, c2: u64, c3: bool, c4: bool, c5: bool, c6: bool, c7: bool, c8: bool, c9: bool, c10: bool, c11: bool, c12: bool, c13: bool, c14: bool, c15: u64, c16: bool, c17: bool, c18: bool, c19: bool, c20: bool, c21: u64, c22: bool, c23: bool, c24: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record130(Record130{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, })));
}

// TemplateTemplateParmDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record131 {
  pub c0: u64, // id
  pub c1: bool, // isParameterPack
  pub c2: bool, // isPackExpansion
  pub c3: bool, // isExpandedParameterPack
  pub c4: bool, // hasDefaultArgument
  pub c5: u64, // getDefaultArgumentLoc
  pub c6: bool, // defaultArgumentWasInherited
  pub c7: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TemplateTemplateParmDecl(c0: u64, c1: bool, c2: bool, c3: bool, c4: bool, c5: u64, c6: bool, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record131(Record131{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// ExportDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record132 {
  pub c0: u64, // id
  pub c1: u64, // getExportLoc
  pub c2: u64, // getRBraceLoc
  pub c3: bool, // hasBraces
  pub c4: u64, // getEndLoc
  pub c5: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ExportDecl(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record132(Record132{c0, c1, c2, c3, c4, c5, })));
}

// UsingEnumDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record133 {
  pub c0: u64, // id
  pub c1: u64, // getUsingLoc
  pub c2: u64, // getEnumLoc
  pub c3: u64, // getEnumDecl
  pub c4: u64, // getSourceRange
  pub c5: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UsingEnumDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record133(Record133{c0, c1, c2, c3, c4, c5, })));
}

// PragmaDetectMismatchDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record134 {
  pub c0: u64, // id
  pub c1: String, // getName
  pub c2: String, // getValue
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PragmaDetectMismatchDecl(c0: u64, c1: *const c_char, c2: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c1 = unsafe { CStr::from_ptr(c1) }.to_string_lossy().to_string();
  let c2 = unsafe { CStr::from_ptr(c2) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record134(Record134{c0, c1, c2, })));
}

// VarTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record135 {
  pub c0: u64, // id
  pub c1: u64, // getTemplatedDecl
  pub c2: bool, // isThisDeclarationADefinition
  pub c3: u64, // getCanonicalDecl
  pub c4: u64, // getPreviousDecl
  pub c5: u64, // getMostRecentDecl
  pub c6: u64, // getInstantiatedFromMemberTemplate
}

#[no_mangle]
pub extern "C" fn arboretum_emit_VarTemplateDecl(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record135(Record135{c0, c1, c2, c3, c4, c5, c6, })));
}

// Decl_attrs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record136 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_Decl_attrs(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record136(Record136{c0, c1, c2, })));
}

// Decl_redecls
#[derive(Debug, Serialize, Deserialize)]
pub struct Record137 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_Decl_redecls(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record137(Record137{c0, c1, c2, })));
}

// Decl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record138 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getLocation
  pub c5: u64, // getNextDeclInContext
  pub c6: u64, // getNonClosureContext
  pub c7: u64, // getTranslationUnitDecl
  pub c8: bool, // isInAnonymousNamespace
  pub c9: bool, // isInStdNamespace
  pub c10: bool, // isFileContextDecl
  pub c11: u64, // getAccess
  pub c12: u64, // getAccessUnsafe
  pub c13: bool, // hasAttrs
  pub c14: bool, // isInvalidDecl
  pub c15: bool, // isImplicit
  pub c16: bool, // isReferenced
  pub c17: bool, // isThisDeclarationReferenced
  pub c18: bool, // isTopLevelDeclInObjCContainer
  pub c19: bool, // isModulePrivate
  pub c20: bool, // isInExportDeclContext
  pub c21: bool, // isInvisibleOutsideTheOwningModule
  pub c22: bool, // isInAnotherModuleUnit
  pub c23: bool, // isDiscardedInGlobalModuleFragment
  pub c24: bool, // shouldSkipCheckingODR
  pub c25: bool, // hasDefiningAttr
  pub c26: u64, // getDefiningAttr
  pub c27: bool, // isWeakImported
  pub c28: bool, // isFromASTFile
  pub c29: u32, // getGlobalID
  pub c30: u32, // getOwningModuleID
  pub c31: bool, // hasOwningModule
  pub c32: bool, // isUnconditionallyVisible
  pub c33: bool, // isReachable
  pub c34: u64, // getModuleOwnershipKind
  pub c35: u32, // getIdentifierNamespace
  pub c36: bool, // hasTagIdentifierNamespace
  pub c37: bool, // isOutOfLine
  pub c38: bool, // isTemplated
  pub c39: u32, // getTemplateDepth
  pub c40: bool, // isDefinedOutsideFunctionOrMethod
  pub c41: u64, // getCanonicalDecl
  pub c42: bool, // isCanonicalDecl
  pub c43: u64, // getPreviousDecl
  pub c44: bool, // isFirstDecl
  pub c45: u64, // getMostRecentDecl
  pub c46: u64, // getBody
  pub c47: bool, // hasBody
  pub c48: u64, // getBodyRBrace
  pub c49: bool, // isTemplateParameter
  pub c50: bool, // isTemplateParameterPack
  pub c51: bool, // isParameterPack
  pub c52: bool, // isTemplateDecl
  pub c53: bool, // isFunctionOrFunctionTemplate
  pub c54: u64, // getDescribedTemplate
  pub c55: u64, // getAsFunction
  pub c56: bool, // isLocalExternDecl
  pub c57: u64, // getFriendObjectKind
  pub c58: i64, // getID
  pub c59: bool, // isFunctionPointerType
}

#[no_mangle]
pub extern "C" fn arboretum_emit_Decl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: bool, c9: bool, c10: bool, c11: u64, c12: u64, c13: bool, c14: bool, c15: bool, c16: bool, c17: bool, c18: bool, c19: bool, c20: bool, c21: bool, c22: bool, c23: bool, c24: bool, c25: bool, c26: u64, c27: bool, c28: bool, c29: u32, c30: u32, c31: bool, c32: bool, c33: bool, c34: u64, c35: u32, c36: bool, c37: bool, c38: bool, c39: u32, c40: bool, c41: u64, c42: bool, c43: u64, c44: bool, c45: u64, c46: u64, c47: bool, c48: u64, c49: bool, c50: bool, c51: bool, c52: bool, c53: bool, c54: u64, c55: u64, c56: bool, c57: u64, c58: i64, c59: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record138(Record138{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41, c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53, c54, c55, c56, c57, c58, c59, })));
}

// EmptyDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record139 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_EmptyDecl(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record139(Record139{c0, })));
}

// MSGuidDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record140 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSGuidDecl(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record140(Record140{c0, })));
}

// CXXConstructorDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record141 {
  pub c0: u64, // id
  pub c1: bool, // isExplicit
  pub c2: u32, // getNumCtorInitializers
  pub c3: bool, // isDelegatingConstructor
  pub c4: bool, // isDefaultConstructor
  pub c5: bool, // isCopyConstructor
  pub c6: bool, // isMoveConstructor
  pub c7: bool, // isCopyOrMoveConstructor
  pub c8: bool, // isSpecializationCopyingObject
  pub c9: bool, // isInheritingConstructor
  pub c10: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXConstructorDecl(c0: u64, c1: bool, c2: u32, c3: bool, c4: bool, c5: bool, c6: bool, c7: bool, c8: bool, c9: bool, c10: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record141(Record141{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, })));
}

// UnnamedGlobalConstantDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record142 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnnamedGlobalConstantDecl(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record142(Record142{c0, })));
}

// FieldDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record143 {
  pub c0: u64, // id
  pub c1: u32, // getFieldIndex
  pub c2: bool, // isMutable
  pub c3: bool, // isBitField
  pub c4: bool, // isUnnamedBitfield
  pub c5: bool, // isAnonymousStructOrUnion
  pub c6: u64, // getBitWidth
  pub c7: bool, // isPotentiallyOverlapping
  pub c8: u64, // getInClassInitStyle
  pub c9: bool, // hasInClassInitializer
  pub c10: bool, // hasNonNullInClassInitializer
  pub c11: u64, // getInClassInitializer
  pub c12: bool, // hasCapturedVLAType
  pub c13: u64, // getCapturedVLAType
  pub c14: u64, // getParent
  pub c15: u64, // getSourceRange
  pub c16: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FieldDecl(c0: u64, c1: u32, c2: bool, c3: bool, c4: bool, c5: bool, c6: u64, c7: bool, c8: u64, c9: bool, c10: bool, c11: u64, c12: bool, c13: u64, c14: u64, c15: u64, c16: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record143(Record143{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, })));
}

// RequiresExprBodyDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record144 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RequiresExprBodyDecl(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record144(Record144{c0, })));
}

// TypedefDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record145 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypedefDecl(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record145(Record145{c0, c1, })));
}

// VarTemplateSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record146 {
  pub c0: u64, // id
  pub c1: u64, // getSpecializedTemplate
  pub c2: u64, // getSpecializationKind
  pub c3: bool, // isExplicitSpecialization
  pub c4: bool, // isClassScopeExplicitSpecialization
  pub c5: bool, // isExplicitInstantiationOrSpecialization
  pub c6: u64, // getPointOfInstantiation
  pub c7: u64, // getExternLoc
  pub c8: u64, // getTemplateKeywordLoc
  pub c9: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_VarTemplateSpecializationDecl(c0: u64, c1: u64, c2: u64, c3: bool, c4: bool, c5: bool, c6: u64, c7: u64, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record146(Record146{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// LifetimeExtendedTemporaryDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record147 {
  pub c0: u64, // id
  pub c1: u64, // getExtendingDecl
  pub c2: u64, // getStorageDuration
  pub c3: u64, // getTemporaryExpr
  pub c4: u32, // getManglingNumber
}

#[no_mangle]
pub extern "C" fn arboretum_emit_LifetimeExtendedTemporaryDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record147(Record147{c0, c1, c2, c3, c4, })));
}

// DecompositionDecl_bindings
#[derive(Debug, Serialize, Deserialize)]
pub struct Record148 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DecompositionDecl_bindings(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record148(Record148{c0, c1, c2, })));
}

// DecompositionDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record149 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DecompositionDecl(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record149(Record149{c0, })));
}

// PragmaCommentDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record150 {
  pub c0: u64, // id
  pub c1: u64, // getCommentKind
  pub c2: String, // getArg
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PragmaCommentDecl(c0: u64, c1: u64, c2: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c2 = unsafe { CStr::from_ptr(c2) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record150(Record150{c0, c1, c2, })));
}

// VarTemplatePartialSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record151 {
  pub c0: u64, // id
  pub c1: bool, // hasAssociatedConstraints
  pub c2: u64, // getInstantiatedFromMember
  pub c3: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_VarTemplatePartialSpecializationDecl(c0: u64, c1: bool, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record151(Record151{c0, c1, c2, c3, })));
}

// FunctionDecl_parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record152 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionDecl_parameters(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record152(Record152{c0, c1, c2, })));
}

// FunctionDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record153 {
  pub c0: u64, // id
  pub c1: u64, // getEllipsisLoc
  pub c2: u64, // getSourceRange
  pub c3: bool, // hasBody
  pub c4: bool, // hasTrivialBody
  pub c5: bool, // isDefined
  pub c6: u64, // getDefinition
  pub c7: u64, // getBody
  pub c8: bool, // isThisDeclarationADefinition
  pub c9: bool, // isThisDeclarationInstantiatedFromAFriendDefinition
  pub c10: bool, // doesThisDeclarationHaveABody
  pub c11: bool, // isVariadic
  pub c12: bool, // isVirtualAsWritten
  pub c13: bool, // isPureVirtual
  pub c14: bool, // isLateTemplateParsed
  pub c15: bool, // isTrivial
  pub c16: bool, // isTrivialForCall
  pub c17: bool, // isDefaulted
  pub c18: bool, // isExplicitlyDefaulted
  pub c19: u64, // getDefaultLoc
  pub c20: bool, // isUserProvided
  pub c21: bool, // isIneligibleOrNotSelected
  pub c22: bool, // hasImplicitReturnZero
  pub c23: bool, // hasPrototype
  pub c24: bool, // hasWrittenPrototype
  pub c25: bool, // hasInheritedPrototype
  pub c26: bool, // isConstexpr
  pub c27: u64, // getConstexprKind
  pub c28: bool, // isConstexprSpecified
  pub c29: bool, // isConsteval
  pub c30: bool, // BodyContainsImmediateEscalatingExpressions
  pub c31: bool, // isImmediateEscalating
  pub c32: bool, // isImmediateFunction
  pub c33: bool, // instantiationIsPending
  pub c34: bool, // usesSEHTry
  pub c35: bool, // isDeleted
  pub c36: bool, // isDeletedAsWritten
  pub c37: bool, // isMain
  pub c38: bool, // isMSVCRTEntryPoint
  pub c39: bool, // isReservedGlobalPlacementOperator
  pub c40: bool, // isInlineBuiltinDeclaration
  pub c41: bool, // isDestroyingOperatorDelete
  pub c42: u64, // getLanguageLinkage
  pub c43: bool, // isExternC
  pub c44: bool, // isInExternCContext
  pub c45: bool, // isInExternCXXContext
  pub c46: bool, // isGlobal
  pub c47: bool, // isNoReturn
  pub c48: bool, // hasSkippedBody
  pub c49: bool, // willHaveBody
  pub c50: bool, // isMultiVersion
  pub c51: bool, // FriendConstraintRefersToEnclosingTemplate
  pub c52: bool, // isMemberLikeConstrainedFriend
  pub c53: u64, // getMultiVersionKind
  pub c54: bool, // isCPUDispatchMultiVersion
  pub c55: bool, // isCPUSpecificMultiVersion
  pub c56: bool, // isTargetMultiVersion
  pub c57: bool, // isTargetClonesMultiVersion
  pub c58: u64, // getCanonicalDecl
  pub c59: bool, // param_empty
  pub c60: u64, // param_size
  pub c61: u32, // getNumParams
  pub c62: u32, // getMinRequiredArguments
  pub c63: u32, // getMinRequiredExplicitArguments
  pub c64: bool, // hasCXXExplicitFunctionObjectParameter
  pub c65: u32, // getNumNonObjectParams
  pub c66: bool, // hasOneParamOrDefaultArgs
  pub c67: u64, // getReturnType
  pub c68: u64, // getReturnTypeSourceRange
  pub c69: u64, // getParametersSourceRange
  pub c70: u64, // getDeclaredReturnType
  pub c71: u64, // getExceptionSpecType
  pub c72: u64, // getExceptionSpecSourceRange
  pub c73: u64, // getCallResultType
  pub c74: u64, // getStorageClass
  pub c75: bool, // isInlineSpecified
  pub c76: bool, // UsesFPIntrin
  pub c77: bool, // isInlined
  pub c78: bool, // isInlineDefinitionExternallyVisible
  pub c79: bool, // isMSExternInline
  pub c80: bool, // doesDeclarationForceExternallyVisibleDefinition
  pub c81: bool, // isStatic
  pub c82: bool, // isOverloadedOperator
  pub c83: u64, // getOverloadedOperator
  pub c84: u64, // getInstantiatedFromMemberFunction
  pub c85: u64, // getTemplatedKind
  pub c86: u64, // getInstantiatedFromDecl
  pub c87: u64, // getDescribedFunctionTemplate
  pub c88: bool, // isFunctionTemplateSpecialization
  pub c89: bool, // isImplicitlyInstantiable
  pub c90: bool, // isTemplateInstantiation
  pub c91: u64, // getPrimaryTemplate
  pub c92: u64, // getTemplateSpecializationKind
  pub c93: u64, // getTemplateSpecializationKindForInstantiation
  pub c94: u64, // getPointOfInstantiation
  pub c95: bool, // isOutOfLine
  pub c96: u32, // getMemoryFunctionKind
  pub c97: u32, // getODRHash
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionDecl(c0: u64, c1: u64, c2: u64, c3: bool, c4: bool, c5: bool, c6: u64, c7: u64, c8: bool, c9: bool, c10: bool, c11: bool, c12: bool, c13: bool, c14: bool, c15: bool, c16: bool, c17: bool, c18: bool, c19: u64, c20: bool, c21: bool, c22: bool, c23: bool, c24: bool, c25: bool, c26: bool, c27: u64, c28: bool, c29: bool, c30: bool, c31: bool, c32: bool, c33: bool, c34: bool, c35: bool, c36: bool, c37: bool, c38: bool, c39: bool, c40: bool, c41: bool, c42: u64, c43: bool, c44: bool, c45: bool, c46: bool, c47: bool, c48: bool, c49: bool, c50: bool, c51: bool, c52: bool, c53: u64, c54: bool, c55: bool, c56: bool, c57: bool, c58: u64, c59: bool, c60: u64, c61: u32, c62: u32, c63: u32, c64: bool, c65: u32, c66: bool, c67: u64, c68: u64, c69: u64, c70: u64, c71: u64, c72: u64, c73: u64, c74: u64, c75: bool, c76: bool, c77: bool, c78: bool, c79: bool, c80: bool, c81: bool, c82: bool, c83: u64, c84: u64, c85: u64, c86: u64, c87: u64, c88: bool, c89: bool, c90: bool, c91: u64, c92: u64, c93: u64, c94: u64, c95: bool, c96: u32, c97: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record153(Record153{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41, c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53, c54, c55, c56, c57, c58, c59, c60, c61, c62, c63, c64, c65, c66, c67, c68, c69, c70, c71, c72, c73, c74, c75, c76, c77, c78, c79, c80, c81, c82, c83, c84, c85, c86, c87, c88, c89, c90, c91, c92, c93, c94, c95, c96, c97, })));
}

// NonTypeTemplateParmDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record154 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: bool, // hasDefaultArgument
  pub c3: u64, // getDefaultArgument
  pub c4: u64, // getDefaultArgumentLoc
  pub c5: bool, // defaultArgumentWasInherited
  pub c6: bool, // isParameterPack
  pub c7: bool, // isPackExpansion
  pub c8: bool, // isExpandedParameterPack
  pub c9: u64, // getPlaceholderTypeConstraint
  pub c10: bool, // hasPlaceholderTypeConstraint
}

#[no_mangle]
pub extern "C" fn arboretum_emit_NonTypeTemplateParmDecl(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: bool, c6: bool, c7: bool, c8: bool, c9: u64, c10: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record154(Record154{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, })));
}

// MSPropertyDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record155 {
  pub c0: u64, // id
  pub c1: bool, // hasGetter
  pub c2: bool, // hasSetter
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSPropertyDecl(c0: u64, c1: bool, c2: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record155(Record155{c0, c1, c2, })));
}

// ImplicitParamDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record156 {
  pub c0: u64, // id
  pub c1: u64, // getParameterKind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ImplicitParamDecl(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record156(Record156{c0, c1, })));
}

// NamedDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record157 {
  pub c0: u64, // id
  pub c1: String, // getNameAsString
  pub c2: String, // getQualifiedNameAsString
  pub c3: bool, // hasLinkage
  pub c4: bool, // isCXXClassMember
  pub c5: bool, // isCXXInstanceMember
  pub c6: u64, // getLinkageInternal
  pub c7: u64, // getFormalLinkage
  pub c8: bool, // hasExternalFormalLinkage
  pub c9: bool, // isExternallyVisible
  pub c10: bool, // isExternallyDeclarable
  pub c11: bool, // isLinkageValid
  pub c12: bool, // hasLinkageBeenComputed
  pub c13: u64, // getUnderlyingDecl
  pub c14: u64, // getMostRecentDecl
  pub c15: u64, // getObjCFStringFormattingFamily
}

#[no_mangle]
pub extern "C" fn arboretum_emit_NamedDecl(c0: u64, c1: *const c_char, c2: *const c_char, c3: bool, c4: bool, c5: bool, c6: u64, c7: u64, c8: bool, c9: bool, c10: bool, c11: bool, c12: bool, c13: u64, c14: u64, c15: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c1 = unsafe { CStr::from_ptr(c1) }.to_string_lossy().to_string();
  let c2 = unsafe { CStr::from_ptr(c2) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record157(Record157{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, })));
}

// CXXDestructorDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record158 {
  pub c0: u64, // id
  pub c1: u64, // getOperatorDelete
  pub c2: u64, // getOperatorDeleteThisArg
  pub c3: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXDestructorDecl(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record158(Record158{c0, c1, c2, c3, })));
}

// ValueDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record159 {
  pub c0: u64, // id
  pub c1: u64, // getType
  pub c2: bool, // isWeak
  pub c3: bool, // isInitCapture
  pub c4: u64, // getPotentiallyDecomposedVarDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ValueDecl(c0: u64, c1: u64, c2: bool, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record159(Record159{c0, c1, c2, c3, c4, })));
}

// CapturedDecl_parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record160 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CapturedDecl_parameters(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record160(Record160{c0, c1, c2, })));
}

// CapturedDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record161 {
  pub c0: u64, // id
  pub c1: u64, // getBody
  pub c2: bool, // isNothrow
  pub c3: u32, // getNumParams
  pub c4: u64, // getContextParam
  pub c5: u32, // getContextParamPosition
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CapturedDecl(c0: u64, c1: u64, c2: bool, c3: u32, c4: u64, c5: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record161(Record161{c0, c1, c2, c3, c4, c5, })));
}

// FriendTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record162 {
  pub c0: u64, // id
  pub c1: u64, // getFriendDecl
  pub c2: u64, // getFriendLoc
  pub c3: u32, // getNumTemplateParameters
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FriendTemplateDecl(c0: u64, c1: u64, c2: u64, c3: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record162(Record162{c0, c1, c2, c3, })));
}

// IndirectFieldDecl_chain
#[derive(Debug, Serialize, Deserialize)]
pub struct Record163 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_IndirectFieldDecl_chain(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record163(Record163{c0, c1, c2, })));
}

// IndirectFieldDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record164 {
  pub c0: u64, // id
  pub c1: u32, // getChainingSize
  pub c2: u64, // getAnonField
  pub c3: u64, // getVarDecl
  pub c4: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_IndirectFieldDecl(c0: u64, c1: u32, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record164(Record164{c0, c1, c2, c3, c4, })));
}

// EnumConstantDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record165 {
  pub c0: u64, // id
  pub c1: u64, // getInitExpr
  pub c2: u64, // getSourceRange
  pub c3: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_EnumConstantDecl(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record165(Record165{c0, c1, c2, c3, })));
}

// CXXConversionDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record166 {
  pub c0: u64, // id
  pub c1: bool, // isExplicit
  pub c2: u64, // getConversionType
  pub c3: bool, // isLambdaToBlockPointerConversion
  pub c4: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXConversionDecl(c0: u64, c1: bool, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record166(Record166{c0, c1, c2, c3, c4, })));
}

// EnumDecl_enumerators
#[derive(Debug, Serialize, Deserialize)]
pub struct Record167 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_EnumDecl_enumerators(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record167(Record167{c0, c1, c2, })));
}

// EnumDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record168 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
  pub c2: u64, // getPreviousDecl
  pub c3: u64, // getMostRecentDecl
  pub c4: u64, // getDefinition
  pub c5: u64, // getSourceRange
  pub c6: u64, // getPromotionType
  pub c7: u64, // getIntegerType
  pub c8: u64, // getIntegerTypeRange
  pub c9: u32, // getNumPositiveBits
  pub c10: u32, // getNumNegativeBits
  pub c11: bool, // isScoped
  pub c12: bool, // isScopedUsingClassTag
  pub c13: bool, // isFixed
  pub c14: bool, // isComplete
  pub c15: bool, // isClosed
  pub c16: bool, // isClosedFlag
  pub c17: bool, // isClosedNonFlag
  pub c18: u64, // getTemplateInstantiationPattern
  pub c19: u64, // getInstantiatedFromMemberEnum
  pub c20: u64, // getTemplateSpecializationKind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_EnumDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u32, c10: u32, c11: bool, c12: bool, c13: bool, c14: bool, c15: bool, c16: bool, c17: bool, c18: u64, c19: u64, c20: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record168(Record168{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, })));
}

// UnresolvedUsingValueDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record169 {
  pub c0: u64, // id
  pub c1: u64, // getUsingLoc
  pub c2: bool, // isAccessDeclaration
  pub c3: bool, // isPackExpansion
  pub c4: u64, // getEllipsisLoc
  pub c5: u64, // getSourceRange
  pub c6: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnresolvedUsingValueDecl(c0: u64, c1: u64, c2: bool, c3: bool, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record169(Record169{c0, c1, c2, c3, c4, c5, c6, })));
}

// BlockDecl_parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record170 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BlockDecl_parameters(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record170(Record170{c0, c1, c2, })));
}

// BlockDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record171 {
  pub c0: u64, // id
  pub c1: u64, // getCaretLocation
  pub c2: bool, // isVariadic
  pub c3: u64, // getCompoundBody
  pub c4: u64, // getBody
  pub c5: bool, // param_empty
  pub c6: u64, // param_size
  pub c7: u32, // getNumParams
  pub c8: bool, // hasCaptures
  pub c9: u32, // getNumCaptures
  pub c10: bool, // capturesCXXThis
  pub c11: bool, // blockMissingReturnType
  pub c12: bool, // isConversionFromLambda
  pub c13: bool, // doesNotEscape
  pub c14: bool, // canAvoidCopyToHeap
  pub c15: u32, // getBlockManglingNumber
  pub c16: u64, // getBlockManglingContextDecl
  pub c17: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BlockDecl(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: bool, c6: u64, c7: u32, c8: bool, c9: u32, c10: bool, c11: bool, c12: bool, c13: bool, c14: bool, c15: u32, c16: u64, c17: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record171(Record171{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, })));
}

// DeclaratorDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record172 {
  pub c0: u64, // id
  pub c1: u64, // getInnerLocStart
  pub c2: u64, // getOuterLocStart
  pub c3: u64, // getSourceRange
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getTrailingRequiresClause
  pub c6: u32, // getNumTemplateParameterLists
  pub c7: u64, // getTypeSpecStartLoc
  pub c8: u64, // getTypeSpecEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DeclaratorDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u32, c7: u64, c8: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record172(Record172{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// TypeAliasDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record173 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getDescribedAliasTemplate
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypeAliasDecl(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record173(Record173{c0, c1, c2, })));
}

// CXXDeductionGuideDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record174 {
  pub c0: u64, // id
  pub c1: bool, // isExplicit
  pub c2: u64, // getDeducedTemplate
  pub c3: u64, // getCorrespondingConstructor
  pub c4: u64, // getDeductionCandidateKind
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXDeductionGuideDecl(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record174(Record174{c0, c1, c2, c3, c4, })));
}

// ImportDecl_getIdentifierLocs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record175 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ImportDecl_getIdentifierLocs(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record175(Record175{c0, c1, c2, })));
}

// ImportDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record176 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ImportDecl(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record176(Record176{c0, c1, })));
}

// AccessSpecDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record177 {
  pub c0: u64, // id
  pub c1: u64, // getAccessSpecifierLoc
  pub c2: u64, // getColonLoc
  pub c3: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AccessSpecDecl(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record177(Record177{c0, c1, c2, c3, })));
}

// ParmVarDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record178 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: bool, // isObjCMethodParameter
  pub c3: bool, // isDestroyedInCallee
  pub c4: u32, // getFunctionScopeDepth
  pub c5: u32, // getFunctionScopeIndex
  pub c6: u64, // getObjCDeclQualifier
  pub c7: bool, // isKNRPromoted
  pub c8: bool, // isExplicitObjectParameter
  pub c9: u64, // getExplicitObjectParamThisLoc
  pub c10: u64, // getDefaultArg
  pub c11: u64, // getDefaultArgRange
  pub c12: u64, // getUninstantiatedDefaultArg
  pub c13: bool, // hasDefaultArg
  pub c14: bool, // hasUnparsedDefaultArg
  pub c15: bool, // hasUninstantiatedDefaultArg
  pub c16: bool, // hasInheritedDefaultArg
  pub c17: u64, // getOriginalType
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ParmVarDecl(c0: u64, c1: u64, c2: bool, c3: bool, c4: u32, c5: u32, c6: u64, c7: bool, c8: bool, c9: u64, c10: u64, c11: u64, c12: u64, c13: bool, c14: bool, c15: bool, c16: bool, c17: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record178(Record178{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, })));
}

// FriendDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record179 {
  pub c0: u64, // id
  pub c1: u32, // getFriendTypeNumTemplateParameterLists
  pub c2: u64, // getFriendDecl
  pub c3: u64, // getFriendLoc
  pub c4: u64, // getSourceRange
  pub c5: bool, // isUnsupportedFriend
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FriendDecl(c0: u64, c1: u32, c2: u64, c3: u64, c4: u64, c5: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record179(Record179{c0, c1, c2, c3, c4, c5, })));
}

// FileScopeAsmDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record180 {
  pub c0: u64, // id
  pub c1: u64, // getAsmLoc
  pub c2: u64, // getRParenLoc
  pub c3: u64, // getSourceRange
  pub c4: u64, // getAsmString
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FileScopeAsmDecl(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record180(Record180{c0, c1, c2, c3, c4, })));
}

// StaticAssertDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record181 {
  pub c0: u64, // id
  pub c1: u64, // getAssertExpr
  pub c2: u64, // getMessage
  pub c3: bool, // isFailed
  pub c4: u64, // getRParenLoc
  pub c5: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_StaticAssertDecl(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record181(Record181{c0, c1, c2, c3, c4, c5, })));
}

// TranslationUnitDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record182 {
  pub c0: u64, // id
  pub c1: u64, // getAnonymousNamespace
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TranslationUnitDecl(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record182(Record182{c0, c1, })));
}

// LinkageSpecDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record183 {
  pub c0: u64, // id
  pub c1: u64, // getLanguage
  pub c2: bool, // hasBraces
  pub c3: u64, // getExternLoc
  pub c4: u64, // getRBraceLoc
  pub c5: u64, // getEndLoc
  pub c6: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_LinkageSpecDecl(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record183(Record183{c0, c1, c2, c3, c4, c5, c6, })));
}

// ExternCContextDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record184 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ExternCContextDecl(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record184(Record184{c0, })));
}

// TopLevelStmtDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record185 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getStmt
  pub c3: bool, // isSemiMissing
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TopLevelStmtDecl(c0: u64, c1: u64, c2: u64, c3: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record185(Record185{c0, c1, c2, c3, })));
}

// TemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record186 {
  pub c0: u64, // id
  pub c1: bool, // hasAssociatedConstraints
  pub c2: u64, // getTemplatedDecl
  pub c3: bool, // isTypeAlias
  pub c4: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TemplateDecl(c0: u64, c1: bool, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record186(Record186{c0, c1, c2, c3, c4, })));
}

// ClassTemplateSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record187 {
  pub c0: u64, // id
  pub c1: u64, // getSpecializedTemplate
  pub c2: u64, // getSpecializationKind
  pub c3: bool, // isExplicitSpecialization
  pub c4: bool, // isClassScopeExplicitSpecialization
  pub c5: bool, // isExplicitInstantiationOrSpecialization
  pub c6: u64, // getPointOfInstantiation
  pub c7: u64, // getExternLoc
  pub c8: u64, // getTemplateKeywordLoc
  pub c9: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ClassTemplateSpecializationDecl(c0: u64, c1: u64, c2: u64, c3: bool, c4: bool, c5: bool, c6: u64, c7: u64, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record187(Record187{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// ConceptDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record188 {
  pub c0: u64, // id
  pub c1: u64, // getConstraintExpr
  pub c2: u64, // getSourceRange
  pub c3: bool, // isTypeConcept
  pub c4: u64, // getCanonicalDecl
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ConceptDecl(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record188(Record188{c0, c1, c2, c3, c4, })));
}

// NamespaceDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record189 {
  pub c0: u64, // id
  pub c1: bool, // isAnonymousNamespace
  pub c2: bool, // isInline
  pub c3: bool, // isNested
  pub c4: u64, // getOriginalNamespace
  pub c5: bool, // isOriginalNamespace
  pub c6: u64, // getAnonymousNamespace
  pub c7: u64, // getCanonicalDecl
  pub c8: u64, // getSourceRange
  pub c9: u64, // getBeginLoc
  pub c10: u64, // getRBraceLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_NamespaceDecl(c0: u64, c1: bool, c2: bool, c3: bool, c4: u64, c5: bool, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record189(Record189{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, })));
}

// AsmStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record190 {
  pub c0: u64, // id
  pub c1: u64, // getAsmLoc
  pub c2: bool, // isSimple
  pub c3: bool, // isVolatile
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
  pub c6: u32, // getNumOutputs
  pub c7: u32, // getNumPlusOperands
  pub c8: u32, // getNumInputs
  pub c9: u32, // getNumClobbers
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AsmStmt(c0: u64, c1: u64, c2: bool, c3: bool, c4: u64, c5: u64, c6: u32, c7: u32, c8: u32, c9: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record190(Record190{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// FullExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record191 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FullExpr(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record191(Record191{c0, c1, })));
}

// CXXTemporaryObjectExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record192 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXTemporaryObjectExpr(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record192(Record192{c0, c1, c2, })));
}

// CXXDefaultInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record193 {
  pub c0: u64, // id
  pub c1: bool, // hasRewrittenInit
  pub c2: u64, // getField
  pub c3: u64, // getExpr
  pub c4: u64, // getRewrittenExpr
  pub c5: u64, // getUsedLocation
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXDefaultInitExpr(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record193(Record193{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// CXXConstructExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record194 {
  pub c0: u64, // id
  pub c1: u64, // getConstructor
  pub c2: u64, // getLocation
  pub c3: bool, // isElidable
  pub c4: bool, // hadMultipleCandidates
  pub c5: bool, // isListInitialization
  pub c6: bool, // isStdInitListInitialization
  pub c7: bool, // requiresZeroInitialization
  pub c8: u64, // getConstructionKind
  pub c9: u32, // getNumArgs
  pub c10: bool, // isImmediateEscalating
  pub c11: u64, // getBeginLoc
  pub c12: u64, // getEndLoc
  pub c13: u64, // getParenOrBraceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXConstructExpr(c0: u64, c1: u64, c2: u64, c3: bool, c4: bool, c5: bool, c6: bool, c7: bool, c8: u64, c9: u32, c10: bool, c11: u64, c12: u64, c13: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record194(Record194{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, })));
}

// Expr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record195 {
  pub c0: u64, // id
  pub c1: u64, // getType
  pub c2: u64, // getDependence
  pub c3: bool, // isValueDependent
  pub c4: bool, // isTypeDependent
  pub c5: bool, // isInstantiationDependent
  pub c6: bool, // containsUnexpandedParameterPack
  pub c7: bool, // containsErrors
  pub c8: bool, // isLValue
  pub c9: bool, // isPRValue
  pub c10: bool, // isXValue
  pub c11: bool, // isGLValue
  pub c12: u64, // getValueKind
  pub c13: u64, // getObjectKind
  pub c14: bool, // isOrdinaryOrBitFieldObject
  pub c15: bool, // refersToBitField
  pub c16: u64, // getSourceBitField
  pub c17: u64, // getReferencedDeclOfCallee
  pub c18: u64, // getObjCProperty
  pub c19: bool, // isObjCSelfExpr
  pub c20: bool, // refersToVectorElement
  pub c21: bool, // refersToMatrixElement
  pub c22: bool, // refersToGlobalRegisterVar
  pub c23: u64, // IgnoreImpCasts
  pub c24: u64, // IgnoreCasts
  pub c25: u64, // IgnoreImplicit
  pub c26: u64, // IgnoreImplicitAsWritten
  pub c27: u64, // IgnoreParens
  pub c28: u64, // IgnoreParenImpCasts
  pub c29: u64, // IgnoreParenCasts
  pub c30: u64, // IgnoreConversionOperatorSingleStep
  pub c31: u64, // IgnoreParenLValueCasts
  pub c32: u64, // IgnoreParenBaseCasts
  pub c33: bool, // isDefaultArgument
  pub c34: bool, // isImplicitCXXThis
  pub c35: u64, // skipRValueSubobjectAdjustments
}

#[no_mangle]
pub extern "C" fn arboretum_emit_Expr(c0: u64, c1: u64, c2: u64, c3: bool, c4: bool, c5: bool, c6: bool, c7: bool, c8: bool, c9: bool, c10: bool, c11: bool, c12: u64, c13: u64, c14: bool, c15: bool, c16: u64, c17: u64, c18: u64, c19: bool, c20: bool, c21: bool, c22: bool, c23: u64, c24: u64, c25: u64, c26: u64, c27: u64, c28: u64, c29: u64, c30: u64, c31: u64, c32: u64, c33: bool, c34: bool, c35: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record195(Record195{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, })));
}

// WhileStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record196 {
  pub c0: u64, // id
  pub c1: bool, // hasVarStorage
  pub c2: u64, // getCond
  pub c3: u64, // getBody
  pub c4: u64, // getConditionVariable
  pub c5: u64, // getConditionVariableDeclStmt
  pub c6: u64, // getWhileLoc
  pub c7: u64, // getLParenLoc
  pub c8: u64, // getRParenLoc
  pub c9: u64, // getBeginLoc
  pub c10: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_WhileStmt(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record196(Record196{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, })));
}

// ValueStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record197 {
  pub c0: u64, // id
  pub c1: u64, // getExprStmt
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ValueStmt(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record197(Record197{c0, c1, })));
}

// DoStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record198 {
  pub c0: u64, // id
  pub c1: u64, // getCond
  pub c2: u64, // getBody
  pub c3: u64, // getDoLoc
  pub c4: u64, // getWhileLoc
  pub c5: u64, // getRParenLoc
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DoStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record198(Record198{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// DependentScopeDeclRefExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record199 {
  pub c0: u64, // id
  pub c1: u64, // getLocation
  pub c2: u64, // getTemplateKeywordLoc
  pub c3: u64, // getLAngleLoc
  pub c4: u64, // getRAngleLoc
  pub c5: bool, // hasTemplateKeyword
  pub c6: bool, // hasExplicitTemplateArgs
  pub c7: u32, // getNumTemplateArgs
  pub c8: u64, // getBeginLoc
  pub c9: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentScopeDeclRefExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: bool, c7: u32, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record199(Record199{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// IndirectGotoStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record200 {
  pub c0: u64, // id
  pub c1: u64, // getGotoLoc
  pub c2: u64, // getStarLoc
  pub c3: u64, // getTarget
  pub c4: u64, // getConstantTarget
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_IndirectGotoStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record200(Record200{c0, c1, c2, c3, c4, c5, c6, })));
}

// ContinueStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record201 {
  pub c0: u64, // id
  pub c1: u64, // getContinueLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ContinueStmt(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record201(Record201{c0, c1, c2, c3, })));
}

// ConceptSpecializationExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record202 {
  pub c0: u64, // id
  pub c1: u64, // getNamedConcept
  pub c2: bool, // hasExplicitTemplateArgs
  pub c3: u64, // getConceptNameLoc
  pub c4: u64, // getTemplateKWLoc
  pub c5: u64, // getFoundDecl
  pub c6: u64, // getSpecializationDecl
  pub c7: u64, // getBeginLoc
  pub c8: u64, // getEndLoc
  pub c9: u64, // getExprLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ConceptSpecializationExpr(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record202(Record202{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// ReturnStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record203 {
  pub c0: u64, // id
  pub c1: u64, // getRetValue
  pub c2: u64, // getNRVOCandidate
  pub c3: u64, // getReturnLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ReturnStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record203(Record203{c0, c1, c2, c3, c4, c5, })));
}

// GCCAsmStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record204 {
  pub c0: u64, // id
  pub c1: u64, // getRParenLoc
  pub c2: u64, // getAsmString
  pub c3: bool, // isAsmGoto
  pub c4: u32, // getNumLabels
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_GCCAsmStmt(c0: u64, c1: u64, c2: u64, c3: bool, c4: u32, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record204(Record204{c0, c1, c2, c3, c4, c5, c6, })));
}

// MSAsmStmt_getAllConstraints
#[derive(Debug, Serialize, Deserialize)]
pub struct Record205 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: String, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSAsmStmt_getAllConstraints(c0: u64, c1: u64, c2: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c2 = unsafe { CStr::from_ptr(c2) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record205(Record205{c0, c1, c2, })));
}

// MSAsmStmt_getClobbers
#[derive(Debug, Serialize, Deserialize)]
pub struct Record206 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: String, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSAsmStmt_getClobbers(c0: u64, c1: u64, c2: *const c_char) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c2 = unsafe { CStr::from_ptr(c2) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record206(Record206{c0, c1, c2, })));
}

// MSAsmStmt_getAllExprs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record207 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSAsmStmt_getAllExprs(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record207(Record207{c0, c1, c2, })));
}

// MSAsmStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record208 {
  pub c0: u64, // id
  pub c1: u64, // getLBraceLoc
  pub c2: u64, // getEndLoc
  pub c3: bool, // hasBraces
  pub c4: String, // getAsmString
  pub c5: u64, // getBeginLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSAsmStmt(c0: u64, c1: u64, c2: u64, c3: bool, c4: *const c_char, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c4 = unsafe { CStr::from_ptr(c4) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record208(Record208{c0, c1, c2, c3, c4, c5, })));
}

// SEHExceptStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record209 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getExceptLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getFilterExpr
  pub c5: u64, // getBlock
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SEHExceptStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record209(Record209{c0, c1, c2, c3, c4, c5, })));
}

// SEHTryStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record210 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getTryLoc
  pub c3: u64, // getEndLoc
  pub c4: bool, // getIsCXXTry
  pub c5: u64, // getTryBlock
  pub c6: u64, // getHandler
  pub c7: u64, // getExceptHandler
  pub c8: u64, // getFinallyHandler
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SEHTryStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: u64, c6: u64, c7: u64, c8: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record210(Record210{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// InitListExpr_inits
#[derive(Debug, Serialize, Deserialize)]
pub struct Record211 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_InitListExpr_inits(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record211(Record211{c0, c1, c2, })));
}

// InitListExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record212 {
  pub c0: u64, // id
  pub c1: u32, // getNumInits
  pub c2: u64, // getArrayFiller
  pub c3: bool, // hasArrayFiller
  pub c4: bool, // hasDesignatedInit
  pub c5: u64, // getInitializedFieldInUnion
  pub c6: bool, // isExplicit
  pub c7: bool, // isStringLiteralInit
  pub c8: bool, // isTransparent
  pub c9: u64, // getLBraceLoc
  pub c10: u64, // getRBraceLoc
  pub c11: bool, // isSemanticForm
  pub c12: u64, // getSemanticForm
  pub c13: bool, // isSyntacticForm
  pub c14: u64, // getSyntacticForm
  pub c15: bool, // hadArrayRangeDesignator
  pub c16: u64, // getBeginLoc
  pub c17: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_InitListExpr(c0: u64, c1: u32, c2: u64, c3: bool, c4: bool, c5: u64, c6: bool, c7: bool, c8: bool, c9: u64, c10: u64, c11: bool, c12: u64, c13: bool, c14: u64, c15: bool, c16: u64, c17: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record212(Record212{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, })));
}

// SEHLeaveStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record213 {
  pub c0: u64, // id
  pub c1: u64, // getLeaveLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SEHLeaveStmt(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record213(Record213{c0, c1, c2, c3, })));
}

// CapturedStmt_capture_inits
#[derive(Debug, Serialize, Deserialize)]
pub struct Record214 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CapturedStmt_capture_inits(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record214(Record214{c0, c1, c2, })));
}

// CapturedStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record215 {
  pub c0: u64, // id
  pub c1: u64, // getCapturedStmt
  pub c2: u64, // getCapturedDecl
  pub c3: u64, // getCapturedRegionKind
  pub c4: u64, // getCapturedRecordDecl
  pub c5: u32, // capture_size
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
  pub c8: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CapturedStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u32, c6: u64, c7: u64, c8: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record215(Record215{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// CXXCatchStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record216 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getCatchLoc
  pub c4: u64, // getExceptionDecl
  pub c5: u64, // getCaughtType
  pub c6: u64, // getHandlerBlock
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXCatchStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record216(Record216{c0, c1, c2, c3, c4, c5, c6, })));
}

// CXXForRangeStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record217 {
  pub c0: u64, // id
  pub c1: u64, // getInit
  pub c2: u64, // getLoopVariable
  pub c3: u64, // getRangeInit
  pub c4: u64, // getRangeStmt
  pub c5: u64, // getBeginStmt
  pub c6: u64, // getEndStmt
  pub c7: u64, // getCond
  pub c8: u64, // getInc
  pub c9: u64, // getLoopVarStmt
  pub c10: u64, // getBody
  pub c11: u64, // getForLoc
  pub c12: u64, // getCoawaitLoc
  pub c13: u64, // getColonLoc
  pub c14: u64, // getRParenLoc
  pub c15: u64, // getBeginLoc
  pub c16: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXForRangeStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64, c11: u64, c12: u64, c13: u64, c14: u64, c15: u64, c16: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record217(Record217{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, })));
}

// CXXNewExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record218 {
  pub c0: u64, // id
  pub c1: u64, // getAllocatedType
  pub c2: u64, // getOperatorNew
  pub c3: u64, // getOperatorDelete
  pub c4: bool, // isArray
  pub c5: u32, // getNumPlacementArgs
  pub c6: bool, // isParenTypeId
  pub c7: u64, // getTypeIdParens
  pub c8: bool, // isGlobalNew
  pub c9: bool, // hasInitializer
  pub c10: u64, // getInitializationStyle
  pub c11: u64, // getInitializer
  pub c12: u64, // getConstructExpr
  pub c13: bool, // passAlignment
  pub c14: bool, // doesUsualArrayDeleteWantSize
  pub c15: u64, // getBeginLoc
  pub c16: u64, // getEndLoc
  pub c17: u64, // getDirectInitRange
  pub c18: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXNewExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: u32, c6: bool, c7: u64, c8: bool, c9: bool, c10: u64, c11: u64, c12: u64, c13: bool, c14: bool, c15: u64, c16: u64, c17: u64, c18: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record218(Record218{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, })));
}

// CoroutineBodyStmt_getParamMoves
#[derive(Debug, Serialize, Deserialize)]
pub struct Record219 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CoroutineBodyStmt_getParamMoves(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record219(Record219{c0, c1, c2, })));
}

// CoroutineBodyStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record220 {
  pub c0: u64, // id
  pub c1: bool, // hasDependentPromiseType
  pub c2: u64, // getBody
  pub c3: u64, // getPromiseDeclStmt
  pub c4: u64, // getPromiseDecl
  pub c5: u64, // getInitSuspendStmt
  pub c6: u64, // getFinalSuspendStmt
  pub c7: u64, // getExceptionHandler
  pub c8: u64, // getFallthroughHandler
  pub c9: u64, // getAllocate
  pub c10: u64, // getDeallocate
  pub c11: u64, // getResultDecl
  pub c12: u64, // getReturnValueInit
  pub c13: u64, // getReturnValue
  pub c14: u64, // getReturnStmt
  pub c15: u64, // getReturnStmtOnAllocFailure
  pub c16: u64, // getBeginLoc
  pub c17: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CoroutineBodyStmt(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64, c11: u64, c12: u64, c13: u64, c14: u64, c15: u64, c16: u64, c17: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record220(Record220{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, })));
}

// ParenListExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record221 {
  pub c0: u64, // id
  pub c1: u32, // getNumExprs
  pub c2: u64, // getLParenLoc
  pub c3: u64, // getRParenLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ParenListExpr(c0: u64, c1: u32, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record221(Record221{c0, c1, c2, c3, c4, c5, })));
}

// CXXScalarValueInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record222 {
  pub c0: u64, // id
  pub c1: u64, // getRParenLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXScalarValueInitExpr(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record222(Record222{c0, c1, c2, c3, })));
}

// MSPropertyRefExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record223 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: bool, // isImplicitAccess
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u64, // getBaseExpr
  pub c6: u64, // getPropertyDecl
  pub c7: bool, // isArrow
  pub c8: u64, // getMemberLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSPropertyRefExpr(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64, c7: bool, c8: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record223(Record223{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// SEHFinallyStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record224 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getFinallyLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getBlock
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SEHFinallyStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record224(Record224{c0, c1, c2, c3, c4, })));
}

// DeclRefExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record225 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: u64, // getLocation
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: bool, // hasQualifier
  pub c6: u64, // getFoundDecl
  pub c7: bool, // hasTemplateKWAndArgsInfo
  pub c8: u64, // getTemplateKeywordLoc
  pub c9: u64, // getLAngleLoc
  pub c10: u64, // getRAngleLoc
  pub c11: bool, // hasTemplateKeyword
  pub c12: bool, // hasExplicitTemplateArgs
  pub c13: u32, // getNumTemplateArgs
  pub c14: bool, // hadMultipleCandidates
  pub c15: u64, // isNonOdrUse
  pub c16: bool, // refersToEnclosingVariableOrCapture
  pub c17: bool, // isImmediateEscalating
  pub c18: bool, // isCapturedByCopyInLambdaWithExplicitObjectParameter
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DeclRefExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: u64, c7: bool, c8: u64, c9: u64, c10: u64, c11: bool, c12: bool, c13: u32, c14: bool, c15: u64, c16: bool, c17: bool, c18: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record225(Record225{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, })));
}

// CStyleCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record226 {
  pub c0: u64, // id
  pub c1: u64, // getLParenLoc
  pub c2: u64, // getRParenLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CStyleCastExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record226(Record226{c0, c1, c2, c3, c4, })));
}

// CXXNullPtrLiteralExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record227 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getLocation
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXNullPtrLiteralExpr(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record227(Record227{c0, c1, c2, c3, })));
}

// ForStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record228 {
  pub c0: u64, // id
  pub c1: u64, // getConditionVariable
  pub c2: u64, // getConditionVariableDeclStmt
  pub c3: u64, // getInit
  pub c4: u64, // getCond
  pub c5: u64, // getInc
  pub c6: u64, // getBody
  pub c7: u64, // getForLoc
  pub c8: u64, // getLParenLoc
  pub c9: u64, // getRParenLoc
  pub c10: u64, // getBeginLoc
  pub c11: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ForStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64, c11: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record228(Record228{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, })));
}

// AsTypeExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record229 {
  pub c0: u64, // id
  pub c1: u64, // getSrcExpr
  pub c2: u64, // getBuiltinLoc
  pub c3: u64, // getRParenLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AsTypeExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record229(Record229{c0, c1, c2, c3, c4, c5, })));
}

// MatrixSubscriptExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record230 {
  pub c0: u64, // id
  pub c1: bool, // isIncomplete
  pub c2: u64, // getBase
  pub c3: u64, // getRowIdx
  pub c4: u64, // getColumnIdx
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
  pub c7: u64, // getExprLoc
  pub c8: u64, // getRBracketLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MatrixSubscriptExpr(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record230(Record230{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// BuiltinBitCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record231 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BuiltinBitCastExpr(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record231(Record231{c0, c1, c2, })));
}

// CXXNamedCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record232 {
  pub c0: u64, // id
  pub c1: u64, // getOperatorLoc
  pub c2: u64, // getRParenLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u64, // getAngleBrackets
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXNamedCastExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record232(Record232{c0, c1, c2, c3, c4, c5, })));
}

// MemberExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record233 {
  pub c0: u64, // id
  pub c1: u64, // getBase
  pub c2: u64, // getMemberDecl
  pub c3: bool, // hasQualifier
  pub c4: u64, // getTemplateKeywordLoc
  pub c5: u64, // getLAngleLoc
  pub c6: u64, // getRAngleLoc
  pub c7: bool, // hasTemplateKeyword
  pub c8: bool, // hasExplicitTemplateArgs
  pub c9: u32, // getNumTemplateArgs
  pub c10: u64, // getOperatorLoc
  pub c11: bool, // isArrow
  pub c12: u64, // getMemberLoc
  pub c13: u64, // getBeginLoc
  pub c14: u64, // getEndLoc
  pub c15: u64, // getExprLoc
  pub c16: bool, // isImplicitAccess
  pub c17: bool, // hadMultipleCandidates
  pub c18: u64, // isNonOdrUse
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MemberExpr(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64, c5: u64, c6: u64, c7: bool, c8: bool, c9: u32, c10: u64, c11: bool, c12: u64, c13: u64, c14: u64, c15: u64, c16: bool, c17: bool, c18: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record233(Record233{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, })));
}

// CXXNoexceptExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record234 {
  pub c0: u64, // id
  pub c1: u64, // getOperand
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getSourceRange
  pub c5: bool, // getValue
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXNoexceptExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record234(Record234{c0, c1, c2, c3, c4, c5, })));
}

// BlockExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record235 {
  pub c0: u64, // id
  pub c1: u64, // getBlockDecl
  pub c2: u64, // getCaretLocation
  pub c3: u64, // getBody
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
  pub c6: u64, // getFunctionType
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BlockExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record235(Record235{c0, c1, c2, c3, c4, c5, c6, })));
}

// BreakStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record236 {
  pub c0: u64, // id
  pub c1: u64, // getBreakLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BreakStmt(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record236(Record236{c0, c1, c2, c3, })));
}

// CXXTryStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record237 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getTryLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getTryBlock
  pub c5: u32, // getNumHandlers
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXTryStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record237(Record237{c0, c1, c2, c3, c4, c5, })));
}

// Stmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record238 {
  pub c0: u64, // id
  pub c1: u64, // stripLabelLikeStatements
}

#[no_mangle]
pub extern "C" fn arboretum_emit_Stmt(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record238(Record238{c0, c1, })));
}

// BinaryOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record239 {
  pub c0: u64, // id
  pub c1: u64, // getExprLoc
  pub c2: u64, // getOperatorLoc
  pub c3: u64, // getOpcode
  pub c4: u64, // getLHS
  pub c5: u64, // getRHS
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
  pub c8: String, // getOpcodeStr
  pub c9: bool, // isPtrMemOp
  pub c10: bool, // isMultiplicativeOp
  pub c11: bool, // isAdditiveOp
  pub c12: bool, // isShiftOp
  pub c13: bool, // isBitwiseOp
  pub c14: bool, // isRelationalOp
  pub c15: bool, // isEqualityOp
  pub c16: bool, // isComparisonOp
  pub c17: bool, // isCommaOp
  pub c18: bool, // isLogicalOp
  pub c19: bool, // isAssignmentOp
  pub c20: bool, // isCompoundAssignmentOp
  pub c21: bool, // isShiftAssignOp
  pub c22: bool, // hasStoredFPFeatures
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BinaryOperator(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: *const c_char, c9: bool, c10: bool, c11: bool, c12: bool, c13: bool, c14: bool, c15: bool, c16: bool, c17: bool, c18: bool, c19: bool, c20: bool, c21: bool, c22: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c8 = unsafe { CStr::from_ptr(c8) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record239(Record239{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, })));
}

// OpaqueValueExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record240 {
  pub c0: u64, // id
  pub c1: u64, // getLocation
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getExprLoc
  pub c5: u64, // getSourceExpr
  pub c6: bool, // isUnique
}

#[no_mangle]
pub extern "C" fn arboretum_emit_OpaqueValueExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record240(Record240{c0, c1, c2, c3, c4, c5, c6, })));
}

// UnresolvedMemberExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record241 {
  pub c0: u64, // id
  pub c1: bool, // isImplicitAccess
  pub c2: u64, // getBaseType
  pub c3: bool, // hasUnresolvedUsing
  pub c4: bool, // isArrow
  pub c5: u64, // getOperatorLoc
  pub c6: u64, // getNamingClass
  pub c7: u64, // getMemberLoc
  pub c8: u64, // getExprLoc
  pub c9: u64, // getBeginLoc
  pub c10: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnresolvedMemberExpr(c0: u64, c1: bool, c2: u64, c3: bool, c4: bool, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record241(Record241{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, })));
}

// StmtExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record242 {
  pub c0: u64, // id
  pub c1: u64, // getSubStmt
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getLParenLoc
  pub c5: u64, // getRParenLoc
  pub c6: u32, // getTemplateDepth
}

#[no_mangle]
pub extern "C" fn arboretum_emit_StmtExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record242(Record242{c0, c1, c2, c3, c4, c5, c6, })));
}

// FunctionParmPackExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record243 {
  pub c0: u64, // id
  pub c1: u64, // getParameterPack
  pub c2: u64, // getParameterPackLocation
  pub c3: u32, // getNumExpansions
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionParmPackExpr(c0: u64, c1: u64, c2: u64, c3: u32, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record243(Record243{c0, c1, c2, c3, c4, c5, })));
}

// ImplicitCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record244 {
  pub c0: u64, // id
  pub c1: bool, // isPartOfExplicitCast
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ImplicitCastExpr(c0: u64, c1: bool, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record244(Record244{c0, c1, c2, c3, })));
}

// UserDefinedLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record245 {
  pub c0: u64, // id
  pub c1: u64, // getLiteralOperatorKind
  pub c2: u64, // getCookedLiteral
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u64, // getUDSuffixLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UserDefinedLiteral(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record245(Record245{c0, c1, c2, c3, c4, c5, })));
}

// CXXDynamicCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record246 {
  pub c0: u64, // id
  pub c1: bool, // isAlwaysNull
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXDynamicCastExpr(c0: u64, c1: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record246(Record246{c0, c1, })));
}

// ArrayTypeTraitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record247 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getTrait
  pub c4: u64, // getQueriedType
  pub c5: u64, // getDimensionExpression
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ArrayTypeTraitExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record247(Record247{c0, c1, c2, c3, c4, c5, })));
}

// CompoundStmt_body
#[derive(Debug, Serialize, Deserialize)]
pub struct Record248 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CompoundStmt_body(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record248(Record248{c0, c1, c2, })));
}

// CompoundStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record249 {
  pub c0: u64, // id
  pub c1: bool, // body_empty
  pub c2: u32, // size
  pub c3: bool, // hasStoredFPFeatures
  pub c4: u64, // body_front
  pub c5: u64, // body_back
  pub c6: u64, // getStmtExprResult
  pub c7: u64, // getBeginLoc
  pub c8: u64, // getEndLoc
  pub c9: u64, // getLBracLoc
  pub c10: u64, // getRBracLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CompoundStmt(c0: u64, c1: bool, c2: u32, c3: bool, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record249(Record249{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, })));
}

// CXXPseudoDestructorExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record250 {
  pub c0: u64, // id
  pub c1: u64, // getBase
  pub c2: bool, // hasQualifier
  pub c3: bool, // isArrow
  pub c4: u64, // getOperatorLoc
  pub c5: u64, // getColonColonLoc
  pub c6: u64, // getTildeLoc
  pub c7: u64, // getDestroyedType
  pub c8: u64, // getDestroyedTypeLoc
  pub c9: u64, // getBeginLoc
  pub c10: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXPseudoDestructorExpr(c0: u64, c1: u64, c2: bool, c3: bool, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record250(Record250{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, })));
}

// StringLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record251 {
  pub c0: u64, // id
  pub c1: String, // getString
  pub c2: String, // getBytes
  pub c3: u32, // getByteLength
  pub c4: u32, // getLength
  pub c5: u32, // getCharByteWidth
  pub c6: u64, // getKind
  pub c7: bool, // isOrdinary
  pub c8: bool, // isWide
  pub c9: bool, // isUTF8
  pub c10: bool, // isUTF16
  pub c11: bool, // isUTF32
  pub c12: bool, // isUnevaluated
  pub c13: bool, // isPascal
  pub c14: bool, // containsNonAscii
  pub c15: bool, // containsNonAsciiOrNull
  pub c16: u32, // getNumConcatenated
  pub c17: u64, // getBeginLoc
  pub c18: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_StringLiteral(c0: u64, c1: *const c_char, c2: *const c_char, c3: u32, c4: u32, c5: u32, c6: u64, c7: bool, c8: bool, c9: bool, c10: bool, c11: bool, c12: bool, c13: bool, c14: bool, c15: bool, c16: u32, c17: u64, c18: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c1 = unsafe { CStr::from_ptr(c1) }.to_string_lossy().to_string();
  let c2 = unsafe { CStr::from_ptr(c2) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record251(Record251{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, })));
}

// CXXDefaultArgExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record252 {
  pub c0: u64, // id
  pub c1: u64, // getParam
  pub c2: bool, // hasRewrittenInit
  pub c3: u64, // getExpr
  pub c4: u64, // getRewrittenExpr
  pub c5: u64, // getAdjustedRewrittenExpr
  pub c6: u64, // getUsedLocation
  pub c7: u64, // getBeginLoc
  pub c8: u64, // getEndLoc
  pub c9: u64, // getExprLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXDefaultArgExpr(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record252(Record252{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// CXXThisExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record253 {
  pub c0: u64, // id
  pub c1: u64, // getLocation
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: bool, // isImplicit
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXThisExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record253(Record253{c0, c1, c2, c3, c4, })));
}

// CXXUuidofExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record254 {
  pub c0: u64, // id
  pub c1: bool, // isTypeOperand
  pub c2: u64, // getExprOperand
  pub c3: u64, // getGuidDecl
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
  pub c6: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXUuidofExpr(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record254(Record254{c0, c1, c2, c3, c4, c5, c6, })));
}

// ShuffleVectorExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record255 {
  pub c0: u64, // id
  pub c1: u64, // getBuiltinLoc
  pub c2: u64, // getRParenLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u32, // getNumSubExprs
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ShuffleVectorExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record255(Record255{c0, c1, c2, c3, c4, c5, })));
}

// CXXStdInitializerListExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record256 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXStdInitializerListExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record256(Record256{c0, c1, c2, c3, c4, })));
}

// DeclStmt_decls
#[derive(Debug, Serialize, Deserialize)]
pub struct Record257 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DeclStmt_decls(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record257(Record257{c0, c1, c2, })));
}

// DeclStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record258 {
  pub c0: u64, // id
  pub c1: bool, // isSingleDecl
  pub c2: u64, // getEndLoc
  pub c3: u64, // getBeginLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DeclStmt(c0: u64, c1: bool, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record258(Record258{c0, c1, c2, c3, })));
}

// CoyieldExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record259 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CoyieldExpr(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record259(Record259{c0, })));
}

// AtomicExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record260 {
  pub c0: u64, // id
  pub c1: u64, // getPtr
  pub c2: u64, // getOrder
  pub c3: u64, // getScope
  pub c4: u64, // getVal1
  pub c5: u64, // getOrderFail
  pub c6: u64, // getVal2
  pub c7: u64, // getWeak
  pub c8: u64, // getValueType
  pub c9: u64, // getOp
  pub c10: String, // getOpAsString
  pub c11: u32, // getNumSubExprs
  pub c12: bool, // isVolatile
  pub c13: bool, // isCmpXChg
  pub c14: bool, // isOpenCL
  pub c15: u64, // getBuiltinLoc
  pub c16: u64, // getRParenLoc
  pub c17: u64, // getBeginLoc
  pub c18: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AtomicExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: *const c_char, c11: u32, c12: bool, c13: bool, c14: bool, c15: u64, c16: u64, c17: u64, c18: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c10 = unsafe { CStr::from_ptr(c10) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record260(Record260{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, })));
}

// ImplicitValueInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record261 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ImplicitValueInitExpr(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record261(Record261{c0, c1, c2, })));
}

// NoInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record262 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_NoInitExpr(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record262(Record262{c0, c1, c2, })));
}

// CXXThrowExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record263 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getThrowLoc
  pub c3: bool, // isThrownVariableInScope
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXThrowExpr(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record263(Record263{c0, c1, c2, c3, c4, c5, })));
}

// AbstractConditionalOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record264 {
  pub c0: u64, // id
  pub c1: u64, // getCond
  pub c2: u64, // getTrueExpr
  pub c3: u64, // getFalseExpr
  pub c4: u64, // getQuestionLoc
  pub c5: u64, // getColonLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AbstractConditionalOperator(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record264(Record264{c0, c1, c2, c3, c4, c5, })));
}

// RecoveryExpr_subExpressions
#[derive(Debug, Serialize, Deserialize)]
pub struct Record265 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RecoveryExpr_subExpressions(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record265(Record265{c0, c1, c2, })));
}

// RecoveryExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record266 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RecoveryExpr(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record266(Record266{c0, c1, c2, })));
}

// BinaryConditionalOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record267 {
  pub c0: u64, // id
  pub c1: u64, // getCommon
  pub c2: u64, // getOpaqueValue
  pub c3: u64, // getCond
  pub c4: u64, // getTrueExpr
  pub c5: u64, // getFalseExpr
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_BinaryConditionalOperator(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record267(Record267{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// ExtVectorElementExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record268 {
  pub c0: u64, // id
  pub c1: u64, // getBase
  pub c2: u64, // getAccessorLoc
  pub c3: u32, // getNumElements
  pub c4: bool, // containsDuplicateElements
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
  pub c7: bool, // isArrow
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ExtVectorElementExpr(c0: u64, c1: u64, c2: u64, c3: u32, c4: bool, c5: u64, c6: u64, c7: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record268(Record268{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// CXXTypeidExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record269 {
  pub c0: u64, // id
  pub c1: bool, // isPotentiallyEvaluated
  pub c2: bool, // isTypeOperand
  pub c3: u64, // getExprOperand
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
  pub c6: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXTypeidExpr(c0: u64, c1: bool, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record269(Record269{c0, c1, c2, c3, c4, c5, c6, })));
}

// GenericSelectionExpr_getAssocExprs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record270 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_GenericSelectionExpr_getAssocExprs(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record270(Record270{c0, c1, c2, })));
}

// GenericSelectionExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record271 {
  pub c0: u64, // id
  pub c1: u32, // getNumAssocs
  pub c2: u32, // getResultIndex
  pub c3: bool, // isResultDependent
  pub c4: bool, // isExprPredicate
  pub c5: bool, // isTypePredicate
  pub c6: u64, // getControllingExpr
  pub c7: u64, // getResultExpr
  pub c8: u64, // getGenericLoc
  pub c9: u64, // getDefaultLoc
  pub c10: u64, // getRParenLoc
  pub c11: u64, // getBeginLoc
  pub c12: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_GenericSelectionExpr(c0: u64, c1: u32, c2: u32, c3: bool, c4: bool, c5: bool, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64, c11: u64, c12: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record271(Record271{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, })));
}

// ExpressionTraitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record272 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getTrait
  pub c4: u64, // getQueriedExpression
  pub c5: bool, // getValue
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ExpressionTraitExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record272(Record272{c0, c1, c2, c3, c4, c5, })));
}

// CXXMemberCallExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record273 {
  pub c0: u64, // id
  pub c1: u64, // getImplicitObjectArgument
  pub c2: u64, // getObjectType
  pub c3: u64, // getMethodDecl
  pub c4: u64, // getRecordDecl
  pub c5: u64, // getExprLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXMemberCallExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record273(Record273{c0, c1, c2, c3, c4, c5, })));
}

// ArraySubscriptExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record274 {
  pub c0: u64, // id
  pub c1: u64, // getLHS
  pub c2: u64, // getRHS
  pub c3: u64, // getBase
  pub c4: u64, // getIdx
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
  pub c7: u64, // getRBracketLoc
  pub c8: u64, // getExprLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ArraySubscriptExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record274(Record274{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// RequiresExpr_getLocalParameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record275 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RequiresExpr_getLocalParameters(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record275(Record275{c0, c1, c2, })));
}

// RequiresExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record276 {
  pub c0: u64, // id
  pub c1: u64, // getBody
  pub c2: u64, // getRequiresKWLoc
  pub c3: u64, // getLParenLoc
  pub c4: u64, // getRParenLoc
  pub c5: u64, // getRBraceLoc
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_RequiresExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record276(Record276{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// ImaginaryLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record277 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ImaginaryLiteral(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record277(Record277{c0, c1, c2, c3, })));
}

// CastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record278 {
  pub c0: u64, // id
  pub c1: u64, // getCastKind
  pub c2: u64, // getSubExpr
  pub c3: u64, // getSubExprAsWritten
  pub c4: u64, // getConversionFunction
  pub c5: bool, // path_empty
  pub c6: u32, // path_size
  pub c7: bool, // hasStoredFPFeatures
  pub c8: bool, // changesVolatileQualification
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CastExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: u32, c7: bool, c8: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record278(Record278{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// PackExpansionExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record279 {
  pub c0: u64, // id
  pub c1: u64, // getPattern
  pub c2: u64, // getEllipsisLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PackExpansionExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record279(Record279{c0, c1, c2, c3, c4, })));
}

// CXXStaticCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record280 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXStaticCastExpr(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record280(Record280{c0, })));
}

// OffsetOfExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record281 {
  pub c0: u64, // id
  pub c1: u64, // getOperatorLoc
  pub c2: u64, // getRParenLoc
  pub c3: u32, // getNumComponents
  pub c4: u32, // getNumExpressions
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_OffsetOfExpr(c0: u64, c1: u64, c2: u64, c3: u32, c4: u32, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record281(Record281{c0, c1, c2, c3, c4, c5, c6, })));
}

// UnaryExprOrTypeTraitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record282 {
  pub c0: u64, // id
  pub c1: u64, // getKind
  pub c2: bool, // isArgumentType
  pub c3: u64, // getTypeOfArgument
  pub c4: u64, // getOperatorLoc
  pub c5: u64, // getRParenLoc
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnaryExprOrTypeTraitExpr(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record282(Record282{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// LabelStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record283 {
  pub c0: u64, // id
  pub c1: u64, // getIdentLoc
  pub c2: u64, // getDecl
  pub c3: u64, // getSubStmt
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
  pub c6: bool, // isSideEntry
}

#[no_mangle]
pub extern "C" fn arboretum_emit_LabelStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record283(Record283{c0, c1, c2, c3, c4, c5, c6, })));
}

// CXXBoolLiteralExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record284 {
  pub c0: u64, // id
  pub c1: bool, // getValue
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getLocation
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXBoolLiteralExpr(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record284(Record284{c0, c1, c2, c3, c4, })));
}

// CharacterLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record285 {
  pub c0: u64, // id
  pub c1: u64, // getLocation
  pub c2: u64, // getKind
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u32, // getValue
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CharacterLiteral(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record285(Record285{c0, c1, c2, c3, c4, c5, })));
}

// AttributedStmt_getAttrs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record286 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AttributedStmt_getAttrs(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record286(Record286{c0, c1, c2, })));
}

// AttributedStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record287 {
  pub c0: u64, // id
  pub c1: u64, // getAttrLoc
  pub c2: u64, // getSubStmt
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AttributedStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record287(Record287{c0, c1, c2, c3, c4, })));
}

// SwitchStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record288 {
  pub c0: u64, // id
  pub c1: bool, // hasInitStorage
  pub c2: bool, // hasVarStorage
  pub c3: u64, // getCond
  pub c4: u64, // getBody
  pub c5: u64, // getInit
  pub c6: u64, // getConditionVariable
  pub c7: u64, // getConditionVariableDeclStmt
  pub c8: u64, // getSwitchCaseList
  pub c9: u64, // getSwitchLoc
  pub c10: u64, // getLParenLoc
  pub c11: u64, // getRParenLoc
  pub c12: bool, // isAllEnumCasesCovered
  pub c13: u64, // getBeginLoc
  pub c14: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SwitchStmt(c0: u64, c1: bool, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64, c11: u64, c12: bool, c13: u64, c14: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record288(Record288{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, })));
}

// ConstantExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record289 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getResultAPValueKind
  pub c4: u64, // getResultStorageKind
  pub c5: bool, // isImmediateInvocation
  pub c6: bool, // hasAPValueResult
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ConstantExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record289(Record289{c0, c1, c2, c3, c4, c5, c6, })));
}

// ConvertVectorExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record290 {
  pub c0: u64, // id
  pub c1: u64, // getSrcExpr
  pub c2: u64, // getBuiltinLoc
  pub c3: u64, // getRParenLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ConvertVectorExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record290(Record290{c0, c1, c2, c3, c4, c5, })));
}

// IntegerLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record291 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getLocation
}

#[no_mangle]
pub extern "C" fn arboretum_emit_IntegerLiteral(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record291(Record291{c0, c1, c2, c3, })));
}

// CXXReinterpretCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record292 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXReinterpretCastExpr(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record292(Record292{c0, })));
}

// MSPropertySubscriptExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record293 {
  pub c0: u64, // id
  pub c1: u64, // getBase
  pub c2: u64, // getIdx
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u64, // getRBracketLoc
  pub c6: u64, // getExprLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSPropertySubscriptExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record293(Record293{c0, c1, c2, c3, c4, c5, c6, })));
}

// NullStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record294 {
  pub c0: u64, // id
  pub c1: u64, // getSemiLoc
  pub c2: bool, // hasLeadingEmptyMacro
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_NullStmt(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record294(Record294{c0, c1, c2, c3, c4, })));
}

// CXXAddrspaceCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record295 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXAddrspaceCastExpr(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record295(Record295{c0, })));
}

// CXXFunctionalCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record296 {
  pub c0: u64, // id
  pub c1: u64, // getLParenLoc
  pub c2: u64, // getRParenLoc
  pub c3: bool, // isListInitialization
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXFunctionalCastExpr(c0: u64, c1: u64, c2: u64, c3: bool, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record296(Record296{c0, c1, c2, c3, c4, c5, })));
}

// CXXOperatorCallExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record297 {
  pub c0: u64, // id
  pub c1: u64, // getOperator
  pub c2: bool, // isAssignmentOp
  pub c3: bool, // isComparisonOp
  pub c4: bool, // isInfixBinaryOp
  pub c5: u64, // getOperatorLoc
  pub c6: u64, // getExprLoc
  pub c7: u64, // getBeginLoc
  pub c8: u64, // getEndLoc
  pub c9: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXOperatorCallExpr(c0: u64, c1: u64, c2: bool, c3: bool, c4: bool, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record297(Record297{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// CXXBindTemporaryExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record298 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXBindTemporaryExpr(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record298(Record298{c0, c1, c2, c3, })));
}

// CXXRewrittenBinaryOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record299 {
  pub c0: u64, // id
  pub c1: u64, // getSemanticForm
  pub c2: bool, // isReversed
  pub c3: u64, // getOperator
  pub c4: u64, // getOpcode
  pub c5: String, // getOpcodeStr
  pub c6: bool, // isComparisonOp
  pub c7: bool, // isAssignmentOp
  pub c8: u64, // getLHS
  pub c9: u64, // getRHS
  pub c10: u64, // getOperatorLoc
  pub c11: u64, // getExprLoc
  pub c12: u64, // getBeginLoc
  pub c13: u64, // getEndLoc
  pub c14: u64, // getSourceRange
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXRewrittenBinaryOperator(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: *const c_char, c6: bool, c7: bool, c8: u64, c9: u64, c10: u64, c11: u64, c12: u64, c13: u64, c14: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c5 = unsafe { CStr::from_ptr(c5) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record299(Record299{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, })));
}

// OverloadExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record300 {
  pub c0: u64, // id
  pub c1: u64, // getNamingClass
  pub c2: u32, // getNumDecls
  pub c3: u64, // getNameLoc
  pub c4: u64, // getTemplateKeywordLoc
  pub c5: u64, // getLAngleLoc
  pub c6: u64, // getRAngleLoc
  pub c7: bool, // hasTemplateKeyword
  pub c8: bool, // hasExplicitTemplateArgs
  pub c9: u32, // getNumTemplateArgs
}

#[no_mangle]
pub extern "C" fn arboretum_emit_OverloadExpr(c0: u64, c1: u64, c2: u32, c3: u64, c4: u64, c5: u64, c6: u64, c7: bool, c8: bool, c9: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record300(Record300{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// DesignatedInitUpdateExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record301 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getBase
  pub c4: u64, // getUpdater
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DesignatedInitUpdateExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record301(Record301{c0, c1, c2, c3, c4, })));
}

// ConditionalOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record302 {
  pub c0: u64, // id
  pub c1: u64, // getCond
  pub c2: u64, // getTrueExpr
  pub c3: u64, // getFalseExpr
  pub c4: u64, // getLHS
  pub c5: u64, // getRHS
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ConditionalOperator(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record302(Record302{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// TypeTraitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record303 {
  pub c0: u64, // id
  pub c1: u64, // getTrait
  pub c2: u32, // getNumArgs
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypeTraitExpr(c0: u64, c1: u64, c2: u32, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record303(Record303{c0, c1, c2, c3, c4, })));
}

// CXXInheritedCtorInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record304 {
  pub c0: u64, // id
  pub c1: u64, // getConstructor
  pub c2: bool, // constructsVBase
  pub c3: u64, // getConstructionKind
  pub c4: bool, // inheritedFromVBase
  pub c5: u64, // getLocation
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXInheritedCtorInitExpr(c0: u64, c1: u64, c2: bool, c3: u64, c4: bool, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record304(Record304{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// DefaultStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record305 {
  pub c0: u64, // id
  pub c1: u64, // getSubStmt
  pub c2: u64, // getDefaultLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DefaultStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record305(Record305{c0, c1, c2, c3, c4, })));
}

// CompoundAssignOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record306 {
  pub c0: u64, // id
  pub c1: u64, // getComputationLHSType
  pub c2: u64, // getComputationResultType
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CompoundAssignOperator(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record306(Record306{c0, c1, c2, })));
}

// ParenExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record307 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getLParen
  pub c5: u64, // getRParen
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ParenExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record307(Record307{c0, c1, c2, c3, c4, c5, })));
}

// PseudoObjectExpr_semantics
#[derive(Debug, Serialize, Deserialize)]
pub struct Record308 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PseudoObjectExpr_semantics(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record308(Record308{c0, c1, c2, })));
}

// PseudoObjectExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record309 {
  pub c0: u64, // id
  pub c1: u64, // getSyntacticForm
  pub c2: u32, // getResultExprIndex
  pub c3: u64, // getResultExpr
  pub c4: u32, // getNumSemanticExprs
  pub c5: u64, // getExprLoc
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PseudoObjectExpr(c0: u64, c1: u64, c2: u32, c3: u64, c4: u32, c5: u64, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record309(Record309{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// FixedPointLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record310 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getLocation
  pub c4: u32, // getScale
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FixedPointLiteral(c0: u64, c1: u64, c2: u64, c3: u64, c4: u32) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record310(Record310{c0, c1, c2, c3, c4, })));
}

// SwitchCase
#[derive(Debug, Serialize, Deserialize)]
pub struct Record311 {
  pub c0: u64, // id
  pub c1: u64, // getNextSwitchCase
  pub c2: u64, // getKeywordLoc
  pub c3: u64, // getColonLoc
  pub c4: u64, // getSubStmt
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SwitchCase(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record311(Record311{c0, c1, c2, c3, c4, c5, c6, })));
}

// GotoStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record312 {
  pub c0: u64, // id
  pub c1: u64, // getLabel
  pub c2: u64, // getGotoLoc
  pub c3: u64, // getLabelLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_GotoStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record312(Record312{c0, c1, c2, c3, c4, c5, })));
}

// CompoundLiteralExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record313 {
  pub c0: u64, // id
  pub c1: u64, // getInitializer
  pub c2: bool, // isFileScope
  pub c3: u64, // getLParenLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CompoundLiteralExpr(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record313(Record313{c0, c1, c2, c3, c4, c5, })));
}

// VAArgExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record314 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: bool, // isMicrosoftABI
  pub c3: u64, // getBuiltinLoc
  pub c4: u64, // getRParenLoc
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_VAArgExpr(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record314(Record314{c0, c1, c2, c3, c4, c5, c6, })));
}

// AddrLabelExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record315 {
  pub c0: u64, // id
  pub c1: u64, // getAmpAmpLoc
  pub c2: u64, // getLabelLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u64, // getLabel
}

#[no_mangle]
pub extern "C" fn arboretum_emit_AddrLabelExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record315(Record315{c0, c1, c2, c3, c4, c5, })));
}

// GNUNullExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record316 {
  pub c0: u64, // id
  pub c1: u64, // getTokenLocation
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_GNUNullExpr(c0: u64, c1: u64, c2: u64, c3: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record316(Record316{c0, c1, c2, c3, })));
}

// UnresolvedLookupExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record317 {
  pub c0: u64, // id
  pub c1: bool, // requiresADL
  pub c2: bool, // isOverloaded
  pub c3: u64, // getNamingClass
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnresolvedLookupExpr(c0: u64, c1: bool, c2: bool, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record317(Record317{c0, c1, c2, c3, c4, c5, })));
}

// ArrayInitIndexExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record318 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ArrayInitIndexExpr(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record318(Record318{c0, c1, c2, })));
}

// CXXUnresolvedConstructExpr_arguments
#[derive(Debug, Serialize, Deserialize)]
pub struct Record319 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXUnresolvedConstructExpr_arguments(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record319(Record319{c0, c1, c2, })));
}

// CXXUnresolvedConstructExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record320 {
  pub c0: u64, // id
  pub c1: u64, // getTypeAsWritten
  pub c2: u64, // getLParenLoc
  pub c3: u64, // getRParenLoc
  pub c4: bool, // isListInitialization
  pub c5: u32, // getNumArgs
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXUnresolvedConstructExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: u32, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record320(Record320{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// ExplicitCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record321 {
  pub c0: u64, // id
  pub c1: u64, // getTypeAsWritten
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ExplicitCastExpr(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record321(Record321{c0, c1, })));
}

// ChooseExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record322 {
  pub c0: u64, // id
  pub c1: bool, // isConditionTrue
  pub c2: bool, // isConditionDependent
  pub c3: u64, // getChosenSubExpr
  pub c4: u64, // getCond
  pub c5: u64, // getLHS
  pub c6: u64, // getRHS
  pub c7: u64, // getBuiltinLoc
  pub c8: u64, // getRParenLoc
  pub c9: u64, // getBeginLoc
  pub c10: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ChooseExpr(c0: u64, c1: bool, c2: bool, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record322(Record322{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, })));
}

// CXXDependentScopeMemberExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record323 {
  pub c0: u64, // id
  pub c1: bool, // isImplicitAccess
  pub c2: u64, // getBaseType
  pub c3: bool, // isArrow
  pub c4: u64, // getOperatorLoc
  pub c5: u64, // getFirstQualifierFoundInScope
  pub c6: u64, // getMemberLoc
  pub c7: u64, // getTemplateKeywordLoc
  pub c8: u64, // getLAngleLoc
  pub c9: u64, // getRAngleLoc
  pub c10: bool, // hasTemplateKeyword
  pub c11: bool, // hasExplicitTemplateArgs
  pub c12: u32, // getNumTemplateArgs
  pub c13: u64, // getBeginLoc
  pub c14: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXDependentScopeMemberExpr(c0: u64, c1: bool, c2: u64, c3: bool, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: bool, c11: bool, c12: u32, c13: u64, c14: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record323(Record323{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, })));
}

// CaseStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record324 {
  pub c0: u64, // id
  pub c1: bool, // caseStmtIsGNURange
  pub c2: u64, // getCaseLoc
  pub c3: u64, // getEllipsisLoc
  pub c4: u64, // getLHS
  pub c5: u64, // getRHS
  pub c6: u64, // getSubStmt
  pub c7: u64, // getBeginLoc
  pub c8: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CaseStmt(c0: u64, c1: bool, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record324(Record324{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// DesignatedInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record325 {
  pub c0: u64, // id
  pub c1: u32, // size
  pub c2: u64, // getEqualOrColonLoc
  pub c3: bool, // isDirectInit
  pub c4: bool, // usesGNUSyntax
  pub c5: u64, // getInit
  pub c6: u32, // getNumSubExprs
  pub c7: u64, // getDesignatorsSourceRange
  pub c8: u64, // getBeginLoc
  pub c9: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DesignatedInitExpr(c0: u64, c1: u32, c2: u64, c3: bool, c4: bool, c5: u64, c6: u32, c7: u64, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record325(Record325{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// TypoExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record326 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_TypoExpr(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record326(Record326{c0, c1, c2, })));
}

// SizeOfPackExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record327 {
  pub c0: u64, // id
  pub c1: u64, // getOperatorLoc
  pub c2: u64, // getPackLoc
  pub c3: u64, // getRParenLoc
  pub c4: u64, // getPack
  pub c5: bool, // isPartiallySubstituted
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SizeOfPackExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: bool, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record327(Record327{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// PredefinedExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record328 {
  pub c0: u64, // id
  pub c1: u64, // getIdentKind
  pub c2: bool, // isTransparent
  pub c3: u64, // getLocation
  pub c4: u64, // getFunctionName
  pub c5: String, // getIdentKindName
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_PredefinedExpr(c0: u64, c1: u64, c2: bool, c3: u64, c4: u64, c5: *const c_char, c6: u64, c7: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c5 = unsafe { CStr::from_ptr(c5) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record328(Record328{c0, c1, c2, c3, c4, c5, c6, c7, })));
}

// SubstNonTypeTemplateParmExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record329 {
  pub c0: u64, // id
  pub c1: u64, // getNameLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getReplacement
  pub c5: u64, // getAssociatedDecl
  pub c6: u32, // getIndex
  pub c7: u64, // getParameter
  pub c8: bool, // isReferenceParameter
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SubstNonTypeTemplateParmExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u32, c7: u64, c8: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record329(Record329{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// SubstNonTypeTemplateParmPackExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record330 {
  pub c0: u64, // id
  pub c1: u64, // getAssociatedDecl
  pub c2: u32, // getIndex
  pub c3: u64, // getParameterPack
  pub c4: u64, // getParameterPackLocation
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SubstNonTypeTemplateParmPackExpr(c0: u64, c1: u64, c2: u32, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record330(Record330{c0, c1, c2, c3, c4, c5, c6, })));
}

// CXXParenListInitExpr_getInitExprs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record331 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXParenListInitExpr_getInitExprs(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record331(Record331{c0, c1, c2, })));
}

// CXXParenListInitExpr_getUserSpecifiedInitExprs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record332 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXParenListInitExpr_getUserSpecifiedInitExprs(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record332(Record332{c0, c1, c2, })));
}

// CXXParenListInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record333 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getInitLoc
  pub c4: u64, // getSourceRange
  pub c5: u64, // getArrayFiller
  pub c6: u64, // getInitializedFieldInUnion
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXParenListInitExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record333(Record333{c0, c1, c2, c3, c4, c5, c6, })));
}

// CUDAKernelCallExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record334 {
  pub c0: u64, // id
  pub c1: u64, // getConfig
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CUDAKernelCallExpr(c0: u64, c1: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record334(Record334{c0, c1, })));
}

// CoroutineSuspendExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record335 {
  pub c0: u64, // id
  pub c1: u64, // getCommonExpr
  pub c2: u64, // getOpaqueValue
  pub c3: u64, // getReadyExpr
  pub c4: u64, // getSuspendExpr
  pub c5: u64, // getResumeExpr
  pub c6: u64, // getOperand
  pub c7: u64, // getKeywordLoc
  pub c8: u64, // getBeginLoc
  pub c9: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CoroutineSuspendExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record335(Record335{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// UnaryOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record336 {
  pub c0: u64, // id
  pub c1: u64, // getOpcode
  pub c2: u64, // getSubExpr
  pub c3: u64, // getOperatorLoc
  pub c4: bool, // canOverflow
  pub c5: bool, // isPrefix
  pub c6: bool, // isPostfix
  pub c7: bool, // isIncrementOp
  pub c8: bool, // isDecrementOp
  pub c9: bool, // isIncrementDecrementOp
  pub c10: bool, // isArithmeticOp
  pub c11: u64, // getBeginLoc
  pub c12: u64, // getEndLoc
  pub c13: u64, // getExprLoc
  pub c14: bool, // hasStoredFPFeatures
}

#[no_mangle]
pub extern "C" fn arboretum_emit_UnaryOperator(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: bool, c6: bool, c7: bool, c8: bool, c9: bool, c10: bool, c11: u64, c12: u64, c13: u64, c14: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record336(Record336{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, })));
}

// DependentCoawaitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record337 {
  pub c0: u64, // id
  pub c1: u64, // getOperand
  pub c2: u64, // getOperatorCoawaitLookup
  pub c3: u64, // getKeywordLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_DependentCoawaitExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record337(Record337{c0, c1, c2, c3, c4, c5, })));
}

// LambdaExpr_capture_inits
#[derive(Debug, Serialize, Deserialize)]
pub struct Record338 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_LambdaExpr_capture_inits(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record338(Record338{c0, c1, c2, })));
}

// LambdaExpr_getExplicitTemplateParameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record339 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

#[no_mangle]
pub extern "C" fn arboretum_emit_LambdaExpr_getExplicitTemplateParameters(c0: u64, c1: u64, c2: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record339(Record339{c0, c1, c2, })));
}

// LambdaExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record340 {
  pub c0: u64, // id
  pub c1: u64, // getCaptureDefault
  pub c2: u64, // getCaptureDefaultLoc
  pub c3: u32, // capture_size
  pub c4: u64, // getIntroducerRange
  pub c5: u64, // getLambdaClass
  pub c6: u64, // getCallOperator
  pub c7: u64, // getDependentCallOperator
  pub c8: u64, // getTrailingRequiresClause
  pub c9: bool, // isGenericLambda
  pub c10: u64, // getBody
  pub c11: u64, // getCompoundStmtBody
  pub c12: bool, // isMutable
  pub c13: bool, // hasExplicitParameters
  pub c14: bool, // hasExplicitResultType
  pub c15: u64, // getBeginLoc
  pub c16: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_LambdaExpr(c0: u64, c1: u64, c2: u64, c3: u32, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: bool, c10: u64, c11: u64, c12: bool, c13: bool, c14: bool, c15: u64, c16: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record340(Record340{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, })));
}

// ExprWithCleanups
#[derive(Debug, Serialize, Deserialize)]
pub struct Record341 {
  pub c0: u64, // id
  pub c1: u32, // getNumObjects
  pub c2: bool, // cleanupsHaveSideEffects
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ExprWithCleanups(c0: u64, c1: u32, c2: bool, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record341(Record341{c0, c1, c2, c3, c4, })));
}

// SYCLUniqueStableNameExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record342 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getLocation
  pub c4: u64, // getLParenLocation
  pub c5: u64, // getRParenLocation
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SYCLUniqueStableNameExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record342(Record342{c0, c1, c2, c3, c4, c5, })));
}

// CXXDeleteExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record343 {
  pub c0: u64, // id
  pub c1: bool, // isGlobalDelete
  pub c2: bool, // isArrayForm
  pub c3: bool, // isArrayFormAsWritten
  pub c4: bool, // doesUsualArrayDeleteWantSize
  pub c5: u64, // getOperatorDelete
  pub c6: u64, // getArgument
  pub c7: u64, // getDestroyedType
  pub c8: u64, // getBeginLoc
  pub c9: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXDeleteExpr(c0: u64, c1: bool, c2: bool, c3: bool, c4: bool, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record343(Record343{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, })));
}

// CallExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record344 {
  pub c0: u64, // id
  pub c1: u64, // getCallee
  pub c2: u64, // getADLCallKind
  pub c3: bool, // usesADL
  pub c4: bool, // hasStoredFPFeatures
  pub c5: u64, // getCalleeDecl
  pub c6: u64, // getDirectCallee
  pub c7: u32, // getNumArgs
  pub c8: u32, // getBuiltinCallee
  pub c9: u64, // getRParenLoc
  pub c10: u64, // getBeginLoc
  pub c11: u64, // getEndLoc
  pub c12: bool, // isCallToStdMove
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CallExpr(c0: u64, c1: u64, c2: u64, c3: bool, c4: bool, c5: u64, c6: u64, c7: u32, c8: u32, c9: u64, c10: u64, c11: u64, c12: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record344(Record344{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, })));
}

// MaterializeTemporaryExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record345 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getStorageDuration
  pub c3: u64, // getLifetimeExtendedTemporaryDecl
  pub c4: u64, // getExtendingDecl
  pub c5: u32, // getManglingNumber
  pub c6: bool, // isBoundToLvalueReference
  pub c7: u64, // getBeginLoc
  pub c8: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MaterializeTemporaryExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64, c5: u32, c6: bool, c7: u64, c8: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record345(Record345{c0, c1, c2, c3, c4, c5, c6, c7, c8, })));
}

// SourceLocExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record346 {
  pub c0: u64, // id
  pub c1: String, // getBuiltinStr
  pub c2: u64, // getIdentKind
  pub c3: bool, // isIntType
  pub c4: u64, // getLocation
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_SourceLocExpr(c0: u64, c1: *const c_char, c2: u64, c3: bool, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  let c1 = unsafe { CStr::from_ptr(c1) }.to_string_lossy().to_string();
  sink(FfiMessage::Record(Record::Record346(Record346{c0, c1, c2, c3, c4, c5, c6, })));
}

// CoawaitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record347 {
  pub c0: u64, // id
  pub c1: bool, // isImplicit
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CoawaitExpr(c0: u64, c1: bool) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record347(Record347{c0, c1, })));
}

// CXXConstCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record348 {
  pub c0: u64, // id
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXConstCastExpr(c0: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record348(Record348{c0, })));
}

// CoreturnStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record349 {
  pub c0: u64, // id
  pub c1: u64, // getKeywordLoc
  pub c2: u64, // getOperand
  pub c3: u64, // getPromiseCall
  pub c4: bool, // isImplicit
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CoreturnStmt(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record349(Record349{c0, c1, c2, c3, c4, c5, c6, })));
}

// ArrayInitLoopExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record350 {
  pub c0: u64, // id
  pub c1: u64, // getCommonExpr
  pub c2: u64, // getSubExpr
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_ArrayInitLoopExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record350(Record350{c0, c1, c2, c3, c4, })));
}

// MSDependentExistsStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record351 {
  pub c0: u64, // id
  pub c1: u64, // getKeywordLoc
  pub c2: bool, // isIfExists
  pub c3: bool, // isIfNotExists
  pub c4: u64, // getSubStmt
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_MSDependentExistsStmt(c0: u64, c1: u64, c2: bool, c3: bool, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record351(Record351{c0, c1, c2, c3, c4, c5, c6, })));
}

// IfStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record352 {
  pub c0: u64, // id
  pub c1: bool, // hasInitStorage
  pub c2: bool, // hasVarStorage
  pub c3: bool, // hasElseStorage
  pub c4: u64, // getCond
  pub c5: u64, // getThen
  pub c6: u64, // getElse
  pub c7: u64, // getConditionVariable
  pub c8: u64, // getConditionVariableDeclStmt
  pub c9: u64, // getInit
  pub c10: u64, // getIfLoc
  pub c11: u64, // getElseLoc
  pub c12: bool, // isConsteval
  pub c13: bool, // isNonNegatedConsteval
  pub c14: bool, // isNegatedConsteval
  pub c15: bool, // isConstexpr
  pub c16: u64, // getStatementKind
  pub c17: bool, // isObjCAvailabilityCheck
  pub c18: u64, // getBeginLoc
  pub c19: u64, // getEndLoc
  pub c20: u64, // getLParenLoc
  pub c21: u64, // getRParenLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_IfStmt(c0: u64, c1: bool, c2: bool, c3: bool, c4: u64, c5: u64, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64, c11: u64, c12: bool, c13: bool, c14: bool, c15: bool, c16: u64, c17: bool, c18: u64, c19: u64, c20: u64, c21: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record352(Record352{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, })));
}

// CXXFoldExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record353 {
  pub c0: u64, // id
  pub c1: u64, // getCallee
  pub c2: u64, // getLHS
  pub c3: u64, // getRHS
  pub c4: bool, // isRightFold
  pub c5: bool, // isLeftFold
  pub c6: u64, // getPattern
  pub c7: u64, // getInit
  pub c8: u64, // getLParenLoc
  pub c9: u64, // getRParenLoc
  pub c10: u64, // getEllipsisLoc
  pub c11: u64, // getOperator
  pub c12: u64, // getBeginLoc
  pub c13: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_CXXFoldExpr(c0: u64, c1: u64, c2: u64, c3: u64, c4: bool, c5: bool, c6: u64, c7: u64, c8: u64, c9: u64, c10: u64, c11: u64, c12: u64, c13: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record353(Record353{c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, })));
}

// FloatingLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record354 {
  pub c0: u64, // id
  pub c1: u64, // getRawSemantics
  pub c2: bool, // isExact
  pub c3: f64, // getValueAsApproximateDouble
  pub c4: u64, // getLocation
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

#[no_mangle]
pub extern "C" fn arboretum_emit_FloatingLiteral(c0: u64, c1: u64, c2: bool, c3: f64, c4: u64, c5: u64, c6: u64) {
  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();
  sink(FfiMessage::Record(Record::Record354(Record354{c0, c1, c2, c3, c4, c5, c6, })));
}

////   END ARBORETUM GENERATED CODE ////
