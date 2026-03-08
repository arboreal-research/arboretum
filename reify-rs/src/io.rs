use crate::ffi::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ffi::c_char;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::AsyncArrowWriter;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;

struct ParquetWriter {
    row_count: usize,
    partition_size: usize,
    partition_counter: usize,
    output_path: PathBuf,
    schema: Arc<Schema>,
}

impl ParquetWriter {
    pub fn new(output_path: impl AsRef<Path>, partition_size: usize, schema: Arc<Schema>) -> Self {
        Self {
            row_count: 0,
            partition_size,
            partition_counter: 0,
            output_path: output_path.as_ref().to_path_buf(),
            schema,
        }
    }

    pub fn should_flush(&self) -> bool {
        self.row_count >= self.partition_size
    }

    pub fn add_row(&mut self) -> bool {
        self.row_count += 1;
        self.should_flush()
    }

    pub async fn flush(&mut self, batch: RecordBatch) -> Result<()> {
        if self.row_count == 0 {
            return Ok(());
        }

        let path = self
            .output_path
            .with_extension(format!("{}.parquet", self.partition_counter));
        let file = File::create(path).await?;
        let mut writer = AsyncArrowWriter::try_new(file, self.schema.clone(), None)?;

        writer.write(&batch).await?;
        writer.close().await?;

        self.row_count = 0;
        self.partition_counter += 1;

        Ok(())
    }

    pub async fn cancel(&mut self) -> Result<()> {
        for partition in 0..self.partition_counter {
            let path = self
                .output_path
                .with_extension(format!("{}.parquet", partition));
            let _ = tokio::fs::remove_file(path).await;
        }
        Ok(())
    }
}

//// BEGIN ARBORETUM GENERATED CODE ////
use deadpool_postgres::tokio_postgres::types;

pub struct TableBuilders {
  db_url: String,
  t0: TableBuilder0,
  t1: TableBuilder1,
  t2: TableBuilder2,
  t3: TableBuilder3,
  t4: TableBuilder4,
  t5: TableBuilder5,
  t6: TableBuilder6,
  t7: TableBuilder7,
  t8: TableBuilder8,
  t9: TableBuilder9,
  t10: TableBuilder10,
  t11: TableBuilder11,
  t12: TableBuilder12,
  t13: TableBuilder13,
  t14: TableBuilder14,
  t15: TableBuilder15,
  t16: TableBuilder16,
  t17: TableBuilder17,
  t18: TableBuilder18,
  t19: TableBuilder19,
  t20: TableBuilder20,
  t21: TableBuilder21,
  t22: TableBuilder22,
  t23: TableBuilder23,
  t24: TableBuilder24,
  t25: TableBuilder25,
  t26: TableBuilder26,
  t27: TableBuilder27,
  t28: TableBuilder28,
  t29: TableBuilder29,
  t30: TableBuilder30,
  t31: TableBuilder31,
  t32: TableBuilder32,
  t33: TableBuilder33,
  t34: TableBuilder34,
  t35: TableBuilder35,
  t36: TableBuilder36,
  t37: TableBuilder37,
  t38: TableBuilder38,
  t39: TableBuilder39,
  t40: TableBuilder40,
  t41: TableBuilder41,
  t42: TableBuilder42,
  t43: TableBuilder43,
  t44: TableBuilder44,
  t45: TableBuilder45,
  t46: TableBuilder46,
  t47: TableBuilder47,
  t48: TableBuilder48,
  t49: TableBuilder49,
  t50: TableBuilder50,
  t51: TableBuilder51,
  t52: TableBuilder52,
  t53: TableBuilder53,
  t54: TableBuilder54,
  t55: TableBuilder55,
  t56: TableBuilder56,
  t57: TableBuilder57,
  t58: TableBuilder58,
  t59: TableBuilder59,
  t60: TableBuilder60,
  t61: TableBuilder61,
  t62: TableBuilder62,
  t63: TableBuilder63,
  t64: TableBuilder64,
  t65: TableBuilder65,
  t66: TableBuilder66,
  t67: TableBuilder67,
  t68: TableBuilder68,
  t69: TableBuilder69,
  t70: TableBuilder70,
  t71: TableBuilder71,
  t72: TableBuilder72,
  t73: TableBuilder73,
  t74: TableBuilder74,
  t75: TableBuilder75,
  t76: TableBuilder76,
  t77: TableBuilder77,
  t78: TableBuilder78,
  t79: TableBuilder79,
  t80: TableBuilder80,
  t81: TableBuilder81,
  t82: TableBuilder82,
  t83: TableBuilder83,
  t84: TableBuilder84,
  t85: TableBuilder85,
  t86: TableBuilder86,
  t87: TableBuilder87,
  t88: TableBuilder88,
  t89: TableBuilder89,
  t90: TableBuilder90,
  t91: TableBuilder91,
  t92: TableBuilder92,
  t93: TableBuilder93,
  t94: TableBuilder94,
  t95: TableBuilder95,
  t96: TableBuilder96,
  t97: TableBuilder97,
  t98: TableBuilder98,
  t99: TableBuilder99,
  t100: TableBuilder100,
  t101: TableBuilder101,
  t102: TableBuilder102,
  t103: TableBuilder103,
  t104: TableBuilder104,
  t105: TableBuilder105,
  t106: TableBuilder106,
  t107: TableBuilder107,
  t108: TableBuilder108,
  t109: TableBuilder109,
  t110: TableBuilder110,
  t111: TableBuilder111,
  t112: TableBuilder112,
  t113: TableBuilder113,
  t114: TableBuilder114,
  t115: TableBuilder115,
  t116: TableBuilder116,
  t117: TableBuilder117,
  t118: TableBuilder118,
  t119: TableBuilder119,
  t120: TableBuilder120,
  t121: TableBuilder121,
  t122: TableBuilder122,
  t123: TableBuilder123,
  t124: TableBuilder124,
  t125: TableBuilder125,
  t126: TableBuilder126,
  t127: TableBuilder127,
  t128: TableBuilder128,
  t129: TableBuilder129,
  t130: TableBuilder130,
  t131: TableBuilder131,
  t132: TableBuilder132,
  t133: TableBuilder133,
  t134: TableBuilder134,
  t135: TableBuilder135,
  t136: TableBuilder136,
  t137: TableBuilder137,
  t138: TableBuilder138,
  t139: TableBuilder139,
  t140: TableBuilder140,
  t141: TableBuilder141,
  t142: TableBuilder142,
  t143: TableBuilder143,
  t144: TableBuilder144,
  t145: TableBuilder145,
  t146: TableBuilder146,
  t147: TableBuilder147,
  t148: TableBuilder148,
  t149: TableBuilder149,
  t150: TableBuilder150,
  t151: TableBuilder151,
  t152: TableBuilder152,
  t153: TableBuilder153,
  t154: TableBuilder154,
  t155: TableBuilder155,
  t156: TableBuilder156,
  t157: TableBuilder157,
  t158: TableBuilder158,
  t159: TableBuilder159,
  t160: TableBuilder160,
  t161: TableBuilder161,
  t162: TableBuilder162,
  t163: TableBuilder163,
  t164: TableBuilder164,
  t165: TableBuilder165,
  t166: TableBuilder166,
  t167: TableBuilder167,
  t168: TableBuilder168,
  t169: TableBuilder169,
  t170: TableBuilder170,
  t171: TableBuilder171,
  t172: TableBuilder172,
  t173: TableBuilder173,
  t174: TableBuilder174,
  t175: TableBuilder175,
  t176: TableBuilder176,
  t177: TableBuilder177,
  t178: TableBuilder178,
  t179: TableBuilder179,
  t180: TableBuilder180,
  t181: TableBuilder181,
  t182: TableBuilder182,
  t183: TableBuilder183,
  t184: TableBuilder184,
  t185: TableBuilder185,
  t186: TableBuilder186,
  t187: TableBuilder187,
  t188: TableBuilder188,
  t189: TableBuilder189,
  t190: TableBuilder190,
  t191: TableBuilder191,
  t192: TableBuilder192,
  t193: TableBuilder193,
  t194: TableBuilder194,
  t195: TableBuilder195,
  t196: TableBuilder196,
  t197: TableBuilder197,
  t198: TableBuilder198,
  t199: TableBuilder199,
  t200: TableBuilder200,
  t201: TableBuilder201,
  t202: TableBuilder202,
  t203: TableBuilder203,
  t204: TableBuilder204,
  t205: TableBuilder205,
  t206: TableBuilder206,
  t207: TableBuilder207,
  t208: TableBuilder208,
  t209: TableBuilder209,
  t210: TableBuilder210,
  t211: TableBuilder211,
  t212: TableBuilder212,
  t213: TableBuilder213,
  t214: TableBuilder214,
  t215: TableBuilder215,
  t216: TableBuilder216,
  t217: TableBuilder217,
  t218: TableBuilder218,
  t219: TableBuilder219,
  t220: TableBuilder220,
  t221: TableBuilder221,
  t222: TableBuilder222,
  t223: TableBuilder223,
  t224: TableBuilder224,
  t225: TableBuilder225,
  t226: TableBuilder226,
  t227: TableBuilder227,
  t228: TableBuilder228,
  t229: TableBuilder229,
  t230: TableBuilder230,
  t231: TableBuilder231,
  t232: TableBuilder232,
  t233: TableBuilder233,
  t234: TableBuilder234,
  t235: TableBuilder235,
  t236: TableBuilder236,
  t237: TableBuilder237,
  t238: TableBuilder238,
  t239: TableBuilder239,
  t240: TableBuilder240,
  t241: TableBuilder241,
  t242: TableBuilder242,
  t243: TableBuilder243,
  t244: TableBuilder244,
  t245: TableBuilder245,
  t246: TableBuilder246,
  t247: TableBuilder247,
  t248: TableBuilder248,
  t249: TableBuilder249,
  t250: TableBuilder250,
  t251: TableBuilder251,
  t252: TableBuilder252,
  t253: TableBuilder253,
  t254: TableBuilder254,
  t255: TableBuilder255,
  t256: TableBuilder256,
  t257: TableBuilder257,
  t258: TableBuilder258,
  t259: TableBuilder259,
  t260: TableBuilder260,
  t261: TableBuilder261,
  t262: TableBuilder262,
  t263: TableBuilder263,
  t264: TableBuilder264,
  t265: TableBuilder265,
  t266: TableBuilder266,
  t267: TableBuilder267,
  t268: TableBuilder268,
  t269: TableBuilder269,
  t270: TableBuilder270,
  t271: TableBuilder271,
  t272: TableBuilder272,
  t273: TableBuilder273,
  t274: TableBuilder274,
  t275: TableBuilder275,
  t276: TableBuilder276,
  t277: TableBuilder277,
  t278: TableBuilder278,
  t279: TableBuilder279,
  t280: TableBuilder280,
  t281: TableBuilder281,
  t282: TableBuilder282,
  t283: TableBuilder283,
  t284: TableBuilder284,
  t285: TableBuilder285,
  t286: TableBuilder286,
  t287: TableBuilder287,
  t288: TableBuilder288,
  t289: TableBuilder289,
  t290: TableBuilder290,
  t291: TableBuilder291,
  t292: TableBuilder292,
  t293: TableBuilder293,
  t294: TableBuilder294,
  t295: TableBuilder295,
  t296: TableBuilder296,
  t297: TableBuilder297,
  t298: TableBuilder298,
  t299: TableBuilder299,
  t300: TableBuilder300,
  t301: TableBuilder301,
  t302: TableBuilder302,
  t303: TableBuilder303,
  t304: TableBuilder304,
  t305: TableBuilder305,
  t306: TableBuilder306,
  t307: TableBuilder307,
  t308: TableBuilder308,
  t309: TableBuilder309,
  t310: TableBuilder310,
  t311: TableBuilder311,
  t312: TableBuilder312,
  t313: TableBuilder313,
  t314: TableBuilder314,
  t315: TableBuilder315,
  t316: TableBuilder316,
  t317: TableBuilder317,
  t318: TableBuilder318,
  t319: TableBuilder319,
  t320: TableBuilder320,
  t321: TableBuilder321,
  t322: TableBuilder322,
  t323: TableBuilder323,
  t324: TableBuilder324,
  t325: TableBuilder325,
  t326: TableBuilder326,
  t327: TableBuilder327,
  t328: TableBuilder328,
  t329: TableBuilder329,
  t330: TableBuilder330,
  t331: TableBuilder331,
  t332: TableBuilder332,
  t333: TableBuilder333,
  t334: TableBuilder334,
  t335: TableBuilder335,
  t336: TableBuilder336,
  t337: TableBuilder337,
  t338: TableBuilder338,
  t339: TableBuilder339,
  t340: TableBuilder340,
  t341: TableBuilder341,
  t342: TableBuilder342,
  t343: TableBuilder343,
  t344: TableBuilder344,
  t345: TableBuilder345,
  t346: TableBuilder346,
  t347: TableBuilder347,
  t348: TableBuilder348,
  t349: TableBuilder349,
  t350: TableBuilder350,
  t351: TableBuilder351,
  t352: TableBuilder352,
  t353: TableBuilder353,
  t354: TableBuilder354,
}

impl TableBuilders {
  pub fn new(db_url: impl AsRef<str>) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      t0: TableBuilder0::new(db_url.as_ref(), 1),
      t1: TableBuilder1::new(db_url.as_ref(), 1),
      t2: TableBuilder2::new(db_url.as_ref(), 1),
      t3: TableBuilder3::new(db_url.as_ref(), 1),
      t4: TableBuilder4::new(db_url.as_ref(), 1),
      t5: TableBuilder5::new(db_url.as_ref(), 1),
      t6: TableBuilder6::new(db_url.as_ref(), 1),
      t7: TableBuilder7::new(db_url.as_ref(), 1),
      t8: TableBuilder8::new(db_url.as_ref(), 1),
      t9: TableBuilder9::new(db_url.as_ref(), 1),
      t10: TableBuilder10::new(db_url.as_ref(), 1),
      t11: TableBuilder11::new(db_url.as_ref(), 1),
      t12: TableBuilder12::new(db_url.as_ref(), 1),
      t13: TableBuilder13::new(db_url.as_ref(), 1),
      t14: TableBuilder14::new(db_url.as_ref(), 1),
      t15: TableBuilder15::new(db_url.as_ref(), 1),
      t16: TableBuilder16::new(db_url.as_ref(), 1),
      t17: TableBuilder17::new(db_url.as_ref(), 1),
      t18: TableBuilder18::new(db_url.as_ref(), 1),
      t19: TableBuilder19::new(db_url.as_ref(), 1),
      t20: TableBuilder20::new(db_url.as_ref(), 1),
      t21: TableBuilder21::new(db_url.as_ref(), 1),
      t22: TableBuilder22::new(db_url.as_ref(), 1),
      t23: TableBuilder23::new(db_url.as_ref(), 1),
      t24: TableBuilder24::new(db_url.as_ref(), 1),
      t25: TableBuilder25::new(db_url.as_ref(), 1),
      t26: TableBuilder26::new(db_url.as_ref(), 1),
      t27: TableBuilder27::new(db_url.as_ref(), 1),
      t28: TableBuilder28::new(db_url.as_ref(), 1),
      t29: TableBuilder29::new(db_url.as_ref(), 1),
      t30: TableBuilder30::new(db_url.as_ref(), 1),
      t31: TableBuilder31::new(db_url.as_ref(), 1),
      t32: TableBuilder32::new(db_url.as_ref(), 1),
      t33: TableBuilder33::new(db_url.as_ref(), 1),
      t34: TableBuilder34::new(db_url.as_ref(), 1),
      t35: TableBuilder35::new(db_url.as_ref(), 1),
      t36: TableBuilder36::new(db_url.as_ref(), 1),
      t37: TableBuilder37::new(db_url.as_ref(), 1),
      t38: TableBuilder38::new(db_url.as_ref(), 1),
      t39: TableBuilder39::new(db_url.as_ref(), 1),
      t40: TableBuilder40::new(db_url.as_ref(), 1),
      t41: TableBuilder41::new(db_url.as_ref(), 1),
      t42: TableBuilder42::new(db_url.as_ref(), 1),
      t43: TableBuilder43::new(db_url.as_ref(), 1),
      t44: TableBuilder44::new(db_url.as_ref(), 1),
      t45: TableBuilder45::new(db_url.as_ref(), 1),
      t46: TableBuilder46::new(db_url.as_ref(), 1),
      t47: TableBuilder47::new(db_url.as_ref(), 1),
      t48: TableBuilder48::new(db_url.as_ref(), 1),
      t49: TableBuilder49::new(db_url.as_ref(), 1),
      t50: TableBuilder50::new(db_url.as_ref(), 1),
      t51: TableBuilder51::new(db_url.as_ref(), 1),
      t52: TableBuilder52::new(db_url.as_ref(), 1),
      t53: TableBuilder53::new(db_url.as_ref(), 1),
      t54: TableBuilder54::new(db_url.as_ref(), 1),
      t55: TableBuilder55::new(db_url.as_ref(), 1),
      t56: TableBuilder56::new(db_url.as_ref(), 1),
      t57: TableBuilder57::new(db_url.as_ref(), 1),
      t58: TableBuilder58::new(db_url.as_ref(), 1),
      t59: TableBuilder59::new(db_url.as_ref(), 1),
      t60: TableBuilder60::new(db_url.as_ref(), 1),
      t61: TableBuilder61::new(db_url.as_ref(), 1),
      t62: TableBuilder62::new(db_url.as_ref(), 1),
      t63: TableBuilder63::new(db_url.as_ref(), 1),
      t64: TableBuilder64::new(db_url.as_ref(), 1),
      t65: TableBuilder65::new(db_url.as_ref(), 1),
      t66: TableBuilder66::new(db_url.as_ref(), 1),
      t67: TableBuilder67::new(db_url.as_ref(), 1),
      t68: TableBuilder68::new(db_url.as_ref(), 1),
      t69: TableBuilder69::new(db_url.as_ref(), 1),
      t70: TableBuilder70::new(db_url.as_ref(), 1),
      t71: TableBuilder71::new(db_url.as_ref(), 1),
      t72: TableBuilder72::new(db_url.as_ref(), 1),
      t73: TableBuilder73::new(db_url.as_ref(), 1),
      t74: TableBuilder74::new(db_url.as_ref(), 1),
      t75: TableBuilder75::new(db_url.as_ref(), 1),
      t76: TableBuilder76::new(db_url.as_ref(), 1),
      t77: TableBuilder77::new(db_url.as_ref(), 1),
      t78: TableBuilder78::new(db_url.as_ref(), 1),
      t79: TableBuilder79::new(db_url.as_ref(), 1),
      t80: TableBuilder80::new(db_url.as_ref(), 1),
      t81: TableBuilder81::new(db_url.as_ref(), 1),
      t82: TableBuilder82::new(db_url.as_ref(), 1),
      t83: TableBuilder83::new(db_url.as_ref(), 1),
      t84: TableBuilder84::new(db_url.as_ref(), 1),
      t85: TableBuilder85::new(db_url.as_ref(), 1),
      t86: TableBuilder86::new(db_url.as_ref(), 1),
      t87: TableBuilder87::new(db_url.as_ref(), 1),
      t88: TableBuilder88::new(db_url.as_ref(), 1),
      t89: TableBuilder89::new(db_url.as_ref(), 1),
      t90: TableBuilder90::new(db_url.as_ref(), 1),
      t91: TableBuilder91::new(db_url.as_ref(), 1),
      t92: TableBuilder92::new(db_url.as_ref(), 1),
      t93: TableBuilder93::new(db_url.as_ref(), 1),
      t94: TableBuilder94::new(db_url.as_ref(), 1),
      t95: TableBuilder95::new(db_url.as_ref(), 1),
      t96: TableBuilder96::new(db_url.as_ref(), 1),
      t97: TableBuilder97::new(db_url.as_ref(), 1),
      t98: TableBuilder98::new(db_url.as_ref(), 1),
      t99: TableBuilder99::new(db_url.as_ref(), 1),
      t100: TableBuilder100::new(db_url.as_ref(), 1),
      t101: TableBuilder101::new(db_url.as_ref(), 1),
      t102: TableBuilder102::new(db_url.as_ref(), 1),
      t103: TableBuilder103::new(db_url.as_ref(), 1),
      t104: TableBuilder104::new(db_url.as_ref(), 1),
      t105: TableBuilder105::new(db_url.as_ref(), 1),
      t106: TableBuilder106::new(db_url.as_ref(), 1),
      t107: TableBuilder107::new(db_url.as_ref(), 1),
      t108: TableBuilder108::new(db_url.as_ref(), 1),
      t109: TableBuilder109::new(db_url.as_ref(), 1),
      t110: TableBuilder110::new(db_url.as_ref(), 1),
      t111: TableBuilder111::new(db_url.as_ref(), 1),
      t112: TableBuilder112::new(db_url.as_ref(), 1),
      t113: TableBuilder113::new(db_url.as_ref(), 1),
      t114: TableBuilder114::new(db_url.as_ref(), 1),
      t115: TableBuilder115::new(db_url.as_ref(), 1),
      t116: TableBuilder116::new(db_url.as_ref(), 1),
      t117: TableBuilder117::new(db_url.as_ref(), 1),
      t118: TableBuilder118::new(db_url.as_ref(), 1),
      t119: TableBuilder119::new(db_url.as_ref(), 1),
      t120: TableBuilder120::new(db_url.as_ref(), 1),
      t121: TableBuilder121::new(db_url.as_ref(), 1),
      t122: TableBuilder122::new(db_url.as_ref(), 1),
      t123: TableBuilder123::new(db_url.as_ref(), 1),
      t124: TableBuilder124::new(db_url.as_ref(), 1),
      t125: TableBuilder125::new(db_url.as_ref(), 1),
      t126: TableBuilder126::new(db_url.as_ref(), 1),
      t127: TableBuilder127::new(db_url.as_ref(), 1),
      t128: TableBuilder128::new(db_url.as_ref(), 1),
      t129: TableBuilder129::new(db_url.as_ref(), 1),
      t130: TableBuilder130::new(db_url.as_ref(), 1),
      t131: TableBuilder131::new(db_url.as_ref(), 1),
      t132: TableBuilder132::new(db_url.as_ref(), 1),
      t133: TableBuilder133::new(db_url.as_ref(), 1),
      t134: TableBuilder134::new(db_url.as_ref(), 1),
      t135: TableBuilder135::new(db_url.as_ref(), 1),
      t136: TableBuilder136::new(db_url.as_ref(), 1),
      t137: TableBuilder137::new(db_url.as_ref(), 1),
      t138: TableBuilder138::new(db_url.as_ref(), 1),
      t139: TableBuilder139::new(db_url.as_ref(), 1),
      t140: TableBuilder140::new(db_url.as_ref(), 1),
      t141: TableBuilder141::new(db_url.as_ref(), 1),
      t142: TableBuilder142::new(db_url.as_ref(), 1),
      t143: TableBuilder143::new(db_url.as_ref(), 1),
      t144: TableBuilder144::new(db_url.as_ref(), 1),
      t145: TableBuilder145::new(db_url.as_ref(), 1),
      t146: TableBuilder146::new(db_url.as_ref(), 1),
      t147: TableBuilder147::new(db_url.as_ref(), 1),
      t148: TableBuilder148::new(db_url.as_ref(), 1),
      t149: TableBuilder149::new(db_url.as_ref(), 1),
      t150: TableBuilder150::new(db_url.as_ref(), 1),
      t151: TableBuilder151::new(db_url.as_ref(), 1),
      t152: TableBuilder152::new(db_url.as_ref(), 1),
      t153: TableBuilder153::new(db_url.as_ref(), 1),
      t154: TableBuilder154::new(db_url.as_ref(), 1),
      t155: TableBuilder155::new(db_url.as_ref(), 1),
      t156: TableBuilder156::new(db_url.as_ref(), 1),
      t157: TableBuilder157::new(db_url.as_ref(), 1),
      t158: TableBuilder158::new(db_url.as_ref(), 1),
      t159: TableBuilder159::new(db_url.as_ref(), 1),
      t160: TableBuilder160::new(db_url.as_ref(), 1),
      t161: TableBuilder161::new(db_url.as_ref(), 1),
      t162: TableBuilder162::new(db_url.as_ref(), 1),
      t163: TableBuilder163::new(db_url.as_ref(), 1),
      t164: TableBuilder164::new(db_url.as_ref(), 1),
      t165: TableBuilder165::new(db_url.as_ref(), 1),
      t166: TableBuilder166::new(db_url.as_ref(), 1),
      t167: TableBuilder167::new(db_url.as_ref(), 1),
      t168: TableBuilder168::new(db_url.as_ref(), 1),
      t169: TableBuilder169::new(db_url.as_ref(), 1),
      t170: TableBuilder170::new(db_url.as_ref(), 1),
      t171: TableBuilder171::new(db_url.as_ref(), 1),
      t172: TableBuilder172::new(db_url.as_ref(), 1),
      t173: TableBuilder173::new(db_url.as_ref(), 1),
      t174: TableBuilder174::new(db_url.as_ref(), 1),
      t175: TableBuilder175::new(db_url.as_ref(), 1),
      t176: TableBuilder176::new(db_url.as_ref(), 1),
      t177: TableBuilder177::new(db_url.as_ref(), 1),
      t178: TableBuilder178::new(db_url.as_ref(), 1),
      t179: TableBuilder179::new(db_url.as_ref(), 1),
      t180: TableBuilder180::new(db_url.as_ref(), 1),
      t181: TableBuilder181::new(db_url.as_ref(), 1),
      t182: TableBuilder182::new(db_url.as_ref(), 1),
      t183: TableBuilder183::new(db_url.as_ref(), 1),
      t184: TableBuilder184::new(db_url.as_ref(), 1),
      t185: TableBuilder185::new(db_url.as_ref(), 1),
      t186: TableBuilder186::new(db_url.as_ref(), 1),
      t187: TableBuilder187::new(db_url.as_ref(), 1),
      t188: TableBuilder188::new(db_url.as_ref(), 1),
      t189: TableBuilder189::new(db_url.as_ref(), 1),
      t190: TableBuilder190::new(db_url.as_ref(), 1),
      t191: TableBuilder191::new(db_url.as_ref(), 1),
      t192: TableBuilder192::new(db_url.as_ref(), 1),
      t193: TableBuilder193::new(db_url.as_ref(), 1),
      t194: TableBuilder194::new(db_url.as_ref(), 1),
      t195: TableBuilder195::new(db_url.as_ref(), 1),
      t196: TableBuilder196::new(db_url.as_ref(), 1),
      t197: TableBuilder197::new(db_url.as_ref(), 1),
      t198: TableBuilder198::new(db_url.as_ref(), 1),
      t199: TableBuilder199::new(db_url.as_ref(), 1),
      t200: TableBuilder200::new(db_url.as_ref(), 1),
      t201: TableBuilder201::new(db_url.as_ref(), 1),
      t202: TableBuilder202::new(db_url.as_ref(), 1),
      t203: TableBuilder203::new(db_url.as_ref(), 1),
      t204: TableBuilder204::new(db_url.as_ref(), 1),
      t205: TableBuilder205::new(db_url.as_ref(), 1),
      t206: TableBuilder206::new(db_url.as_ref(), 1),
      t207: TableBuilder207::new(db_url.as_ref(), 1),
      t208: TableBuilder208::new(db_url.as_ref(), 1),
      t209: TableBuilder209::new(db_url.as_ref(), 1),
      t210: TableBuilder210::new(db_url.as_ref(), 1),
      t211: TableBuilder211::new(db_url.as_ref(), 1),
      t212: TableBuilder212::new(db_url.as_ref(), 1),
      t213: TableBuilder213::new(db_url.as_ref(), 1),
      t214: TableBuilder214::new(db_url.as_ref(), 1),
      t215: TableBuilder215::new(db_url.as_ref(), 1),
      t216: TableBuilder216::new(db_url.as_ref(), 1),
      t217: TableBuilder217::new(db_url.as_ref(), 1),
      t218: TableBuilder218::new(db_url.as_ref(), 1),
      t219: TableBuilder219::new(db_url.as_ref(), 1),
      t220: TableBuilder220::new(db_url.as_ref(), 1),
      t221: TableBuilder221::new(db_url.as_ref(), 1),
      t222: TableBuilder222::new(db_url.as_ref(), 1),
      t223: TableBuilder223::new(db_url.as_ref(), 1),
      t224: TableBuilder224::new(db_url.as_ref(), 1),
      t225: TableBuilder225::new(db_url.as_ref(), 1),
      t226: TableBuilder226::new(db_url.as_ref(), 1),
      t227: TableBuilder227::new(db_url.as_ref(), 1),
      t228: TableBuilder228::new(db_url.as_ref(), 1),
      t229: TableBuilder229::new(db_url.as_ref(), 1),
      t230: TableBuilder230::new(db_url.as_ref(), 1),
      t231: TableBuilder231::new(db_url.as_ref(), 1),
      t232: TableBuilder232::new(db_url.as_ref(), 1),
      t233: TableBuilder233::new(db_url.as_ref(), 1),
      t234: TableBuilder234::new(db_url.as_ref(), 1),
      t235: TableBuilder235::new(db_url.as_ref(), 1),
      t236: TableBuilder236::new(db_url.as_ref(), 1),
      t237: TableBuilder237::new(db_url.as_ref(), 1),
      t238: TableBuilder238::new(db_url.as_ref(), 1),
      t239: TableBuilder239::new(db_url.as_ref(), 1),
      t240: TableBuilder240::new(db_url.as_ref(), 1),
      t241: TableBuilder241::new(db_url.as_ref(), 1),
      t242: TableBuilder242::new(db_url.as_ref(), 1),
      t243: TableBuilder243::new(db_url.as_ref(), 1),
      t244: TableBuilder244::new(db_url.as_ref(), 1),
      t245: TableBuilder245::new(db_url.as_ref(), 1),
      t246: TableBuilder246::new(db_url.as_ref(), 1),
      t247: TableBuilder247::new(db_url.as_ref(), 1),
      t248: TableBuilder248::new(db_url.as_ref(), 1),
      t249: TableBuilder249::new(db_url.as_ref(), 1),
      t250: TableBuilder250::new(db_url.as_ref(), 1),
      t251: TableBuilder251::new(db_url.as_ref(), 1),
      t252: TableBuilder252::new(db_url.as_ref(), 1),
      t253: TableBuilder253::new(db_url.as_ref(), 1),
      t254: TableBuilder254::new(db_url.as_ref(), 1),
      t255: TableBuilder255::new(db_url.as_ref(), 1),
      t256: TableBuilder256::new(db_url.as_ref(), 1),
      t257: TableBuilder257::new(db_url.as_ref(), 1),
      t258: TableBuilder258::new(db_url.as_ref(), 1),
      t259: TableBuilder259::new(db_url.as_ref(), 1),
      t260: TableBuilder260::new(db_url.as_ref(), 1),
      t261: TableBuilder261::new(db_url.as_ref(), 1),
      t262: TableBuilder262::new(db_url.as_ref(), 1),
      t263: TableBuilder263::new(db_url.as_ref(), 1),
      t264: TableBuilder264::new(db_url.as_ref(), 1),
      t265: TableBuilder265::new(db_url.as_ref(), 1),
      t266: TableBuilder266::new(db_url.as_ref(), 1),
      t267: TableBuilder267::new(db_url.as_ref(), 1),
      t268: TableBuilder268::new(db_url.as_ref(), 1),
      t269: TableBuilder269::new(db_url.as_ref(), 1),
      t270: TableBuilder270::new(db_url.as_ref(), 1),
      t271: TableBuilder271::new(db_url.as_ref(), 1),
      t272: TableBuilder272::new(db_url.as_ref(), 1),
      t273: TableBuilder273::new(db_url.as_ref(), 1),
      t274: TableBuilder274::new(db_url.as_ref(), 1),
      t275: TableBuilder275::new(db_url.as_ref(), 1),
      t276: TableBuilder276::new(db_url.as_ref(), 1),
      t277: TableBuilder277::new(db_url.as_ref(), 1),
      t278: TableBuilder278::new(db_url.as_ref(), 1),
      t279: TableBuilder279::new(db_url.as_ref(), 1),
      t280: TableBuilder280::new(db_url.as_ref(), 1),
      t281: TableBuilder281::new(db_url.as_ref(), 1),
      t282: TableBuilder282::new(db_url.as_ref(), 1),
      t283: TableBuilder283::new(db_url.as_ref(), 1),
      t284: TableBuilder284::new(db_url.as_ref(), 1),
      t285: TableBuilder285::new(db_url.as_ref(), 1),
      t286: TableBuilder286::new(db_url.as_ref(), 1),
      t287: TableBuilder287::new(db_url.as_ref(), 1),
      t288: TableBuilder288::new(db_url.as_ref(), 1),
      t289: TableBuilder289::new(db_url.as_ref(), 1),
      t290: TableBuilder290::new(db_url.as_ref(), 1),
      t291: TableBuilder291::new(db_url.as_ref(), 1),
      t292: TableBuilder292::new(db_url.as_ref(), 1),
      t293: TableBuilder293::new(db_url.as_ref(), 1),
      t294: TableBuilder294::new(db_url.as_ref(), 1),
      t295: TableBuilder295::new(db_url.as_ref(), 1),
      t296: TableBuilder296::new(db_url.as_ref(), 1),
      t297: TableBuilder297::new(db_url.as_ref(), 1),
      t298: TableBuilder298::new(db_url.as_ref(), 1),
      t299: TableBuilder299::new(db_url.as_ref(), 1),
      t300: TableBuilder300::new(db_url.as_ref(), 1),
      t301: TableBuilder301::new(db_url.as_ref(), 1),
      t302: TableBuilder302::new(db_url.as_ref(), 1),
      t303: TableBuilder303::new(db_url.as_ref(), 1),
      t304: TableBuilder304::new(db_url.as_ref(), 1),
      t305: TableBuilder305::new(db_url.as_ref(), 1),
      t306: TableBuilder306::new(db_url.as_ref(), 1),
      t307: TableBuilder307::new(db_url.as_ref(), 1),
      t308: TableBuilder308::new(db_url.as_ref(), 1),
      t309: TableBuilder309::new(db_url.as_ref(), 1),
      t310: TableBuilder310::new(db_url.as_ref(), 1),
      t311: TableBuilder311::new(db_url.as_ref(), 1),
      t312: TableBuilder312::new(db_url.as_ref(), 1),
      t313: TableBuilder313::new(db_url.as_ref(), 1),
      t314: TableBuilder314::new(db_url.as_ref(), 1),
      t315: TableBuilder315::new(db_url.as_ref(), 1),
      t316: TableBuilder316::new(db_url.as_ref(), 1),
      t317: TableBuilder317::new(db_url.as_ref(), 1),
      t318: TableBuilder318::new(db_url.as_ref(), 1),
      t319: TableBuilder319::new(db_url.as_ref(), 1),
      t320: TableBuilder320::new(db_url.as_ref(), 1),
      t321: TableBuilder321::new(db_url.as_ref(), 1),
      t322: TableBuilder322::new(db_url.as_ref(), 1),
      t323: TableBuilder323::new(db_url.as_ref(), 1),
      t324: TableBuilder324::new(db_url.as_ref(), 1),
      t325: TableBuilder325::new(db_url.as_ref(), 1),
      t326: TableBuilder326::new(db_url.as_ref(), 1),
      t327: TableBuilder327::new(db_url.as_ref(), 1),
      t328: TableBuilder328::new(db_url.as_ref(), 1),
      t329: TableBuilder329::new(db_url.as_ref(), 1),
      t330: TableBuilder330::new(db_url.as_ref(), 1),
      t331: TableBuilder331::new(db_url.as_ref(), 1),
      t332: TableBuilder332::new(db_url.as_ref(), 1),
      t333: TableBuilder333::new(db_url.as_ref(), 1),
      t334: TableBuilder334::new(db_url.as_ref(), 1),
      t335: TableBuilder335::new(db_url.as_ref(), 1),
      t336: TableBuilder336::new(db_url.as_ref(), 1),
      t337: TableBuilder337::new(db_url.as_ref(), 1),
      t338: TableBuilder338::new(db_url.as_ref(), 1),
      t339: TableBuilder339::new(db_url.as_ref(), 1),
      t340: TableBuilder340::new(db_url.as_ref(), 1),
      t341: TableBuilder341::new(db_url.as_ref(), 1),
      t342: TableBuilder342::new(db_url.as_ref(), 1),
      t343: TableBuilder343::new(db_url.as_ref(), 1),
      t344: TableBuilder344::new(db_url.as_ref(), 1),
      t345: TableBuilder345::new(db_url.as_ref(), 1),
      t346: TableBuilder346::new(db_url.as_ref(), 1),
      t347: TableBuilder347::new(db_url.as_ref(), 1),
      t348: TableBuilder348::new(db_url.as_ref(), 1),
      t349: TableBuilder349::new(db_url.as_ref(), 1),
      t350: TableBuilder350::new(db_url.as_ref(), 1),
      t351: TableBuilder351::new(db_url.as_ref(), 1),
      t352: TableBuilder352::new(db_url.as_ref(), 1),
      t353: TableBuilder353::new(db_url.as_ref(), 1),
      t354: TableBuilder354::new(db_url.as_ref(), 1),
    }
  }
  pub fn push(&mut self, record: Record) {
    match record {
      Record::Record0(record) => self.t0.push(record),
      Record::Record1(record) => self.t1.push(record),
      Record::Record2(record) => self.t2.push(record),
      Record::Record3(record) => self.t3.push(record),
      Record::Record4(record) => self.t4.push(record),
      Record::Record5(record) => self.t5.push(record),
      Record::Record6(record) => self.t6.push(record),
      Record::Record7(record) => self.t7.push(record),
      Record::Record8(record) => self.t8.push(record),
      Record::Record9(record) => self.t9.push(record),
      Record::Record10(record) => self.t10.push(record),
      Record::Record11(record) => self.t11.push(record),
      Record::Record12(record) => self.t12.push(record),
      Record::Record13(record) => self.t13.push(record),
      Record::Record14(record) => self.t14.push(record),
      Record::Record15(record) => self.t15.push(record),
      Record::Record16(record) => self.t16.push(record),
      Record::Record17(record) => self.t17.push(record),
      Record::Record18(record) => self.t18.push(record),
      Record::Record19(record) => self.t19.push(record),
      Record::Record20(record) => self.t20.push(record),
      Record::Record21(record) => self.t21.push(record),
      Record::Record22(record) => self.t22.push(record),
      Record::Record23(record) => self.t23.push(record),
      Record::Record24(record) => self.t24.push(record),
      Record::Record25(record) => self.t25.push(record),
      Record::Record26(record) => self.t26.push(record),
      Record::Record27(record) => self.t27.push(record),
      Record::Record28(record) => self.t28.push(record),
      Record::Record29(record) => self.t29.push(record),
      Record::Record30(record) => self.t30.push(record),
      Record::Record31(record) => self.t31.push(record),
      Record::Record32(record) => self.t32.push(record),
      Record::Record33(record) => self.t33.push(record),
      Record::Record34(record) => self.t34.push(record),
      Record::Record35(record) => self.t35.push(record),
      Record::Record36(record) => self.t36.push(record),
      Record::Record37(record) => self.t37.push(record),
      Record::Record38(record) => self.t38.push(record),
      Record::Record39(record) => self.t39.push(record),
      Record::Record40(record) => self.t40.push(record),
      Record::Record41(record) => self.t41.push(record),
      Record::Record42(record) => self.t42.push(record),
      Record::Record43(record) => self.t43.push(record),
      Record::Record44(record) => self.t44.push(record),
      Record::Record45(record) => self.t45.push(record),
      Record::Record46(record) => self.t46.push(record),
      Record::Record47(record) => self.t47.push(record),
      Record::Record48(record) => self.t48.push(record),
      Record::Record49(record) => self.t49.push(record),
      Record::Record50(record) => self.t50.push(record),
      Record::Record51(record) => self.t51.push(record),
      Record::Record52(record) => self.t52.push(record),
      Record::Record53(record) => self.t53.push(record),
      Record::Record54(record) => self.t54.push(record),
      Record::Record55(record) => self.t55.push(record),
      Record::Record56(record) => self.t56.push(record),
      Record::Record57(record) => self.t57.push(record),
      Record::Record58(record) => self.t58.push(record),
      Record::Record59(record) => self.t59.push(record),
      Record::Record60(record) => self.t60.push(record),
      Record::Record61(record) => self.t61.push(record),
      Record::Record62(record) => self.t62.push(record),
      Record::Record63(record) => self.t63.push(record),
      Record::Record64(record) => self.t64.push(record),
      Record::Record65(record) => self.t65.push(record),
      Record::Record66(record) => self.t66.push(record),
      Record::Record67(record) => self.t67.push(record),
      Record::Record68(record) => self.t68.push(record),
      Record::Record69(record) => self.t69.push(record),
      Record::Record70(record) => self.t70.push(record),
      Record::Record71(record) => self.t71.push(record),
      Record::Record72(record) => self.t72.push(record),
      Record::Record73(record) => self.t73.push(record),
      Record::Record74(record) => self.t74.push(record),
      Record::Record75(record) => self.t75.push(record),
      Record::Record76(record) => self.t76.push(record),
      Record::Record77(record) => self.t77.push(record),
      Record::Record78(record) => self.t78.push(record),
      Record::Record79(record) => self.t79.push(record),
      Record::Record80(record) => self.t80.push(record),
      Record::Record81(record) => self.t81.push(record),
      Record::Record82(record) => self.t82.push(record),
      Record::Record83(record) => self.t83.push(record),
      Record::Record84(record) => self.t84.push(record),
      Record::Record85(record) => self.t85.push(record),
      Record::Record86(record) => self.t86.push(record),
      Record::Record87(record) => self.t87.push(record),
      Record::Record88(record) => self.t88.push(record),
      Record::Record89(record) => self.t89.push(record),
      Record::Record90(record) => self.t90.push(record),
      Record::Record91(record) => self.t91.push(record),
      Record::Record92(record) => self.t92.push(record),
      Record::Record93(record) => self.t93.push(record),
      Record::Record94(record) => self.t94.push(record),
      Record::Record95(record) => self.t95.push(record),
      Record::Record96(record) => self.t96.push(record),
      Record::Record97(record) => self.t97.push(record),
      Record::Record98(record) => self.t98.push(record),
      Record::Record99(record) => self.t99.push(record),
      Record::Record100(record) => self.t100.push(record),
      Record::Record101(record) => self.t101.push(record),
      Record::Record102(record) => self.t102.push(record),
      Record::Record103(record) => self.t103.push(record),
      Record::Record104(record) => self.t104.push(record),
      Record::Record105(record) => self.t105.push(record),
      Record::Record106(record) => self.t106.push(record),
      Record::Record107(record) => self.t107.push(record),
      Record::Record108(record) => self.t108.push(record),
      Record::Record109(record) => self.t109.push(record),
      Record::Record110(record) => self.t110.push(record),
      Record::Record111(record) => self.t111.push(record),
      Record::Record112(record) => self.t112.push(record),
      Record::Record113(record) => self.t113.push(record),
      Record::Record114(record) => self.t114.push(record),
      Record::Record115(record) => self.t115.push(record),
      Record::Record116(record) => self.t116.push(record),
      Record::Record117(record) => self.t117.push(record),
      Record::Record118(record) => self.t118.push(record),
      Record::Record119(record) => self.t119.push(record),
      Record::Record120(record) => self.t120.push(record),
      Record::Record121(record) => self.t121.push(record),
      Record::Record122(record) => self.t122.push(record),
      Record::Record123(record) => self.t123.push(record),
      Record::Record124(record) => self.t124.push(record),
      Record::Record125(record) => self.t125.push(record),
      Record::Record126(record) => self.t126.push(record),
      Record::Record127(record) => self.t127.push(record),
      Record::Record128(record) => self.t128.push(record),
      Record::Record129(record) => self.t129.push(record),
      Record::Record130(record) => self.t130.push(record),
      Record::Record131(record) => self.t131.push(record),
      Record::Record132(record) => self.t132.push(record),
      Record::Record133(record) => self.t133.push(record),
      Record::Record134(record) => self.t134.push(record),
      Record::Record135(record) => self.t135.push(record),
      Record::Record136(record) => self.t136.push(record),
      Record::Record137(record) => self.t137.push(record),
      Record::Record138(record) => self.t138.push(record),
      Record::Record139(record) => self.t139.push(record),
      Record::Record140(record) => self.t140.push(record),
      Record::Record141(record) => self.t141.push(record),
      Record::Record142(record) => self.t142.push(record),
      Record::Record143(record) => self.t143.push(record),
      Record::Record144(record) => self.t144.push(record),
      Record::Record145(record) => self.t145.push(record),
      Record::Record146(record) => self.t146.push(record),
      Record::Record147(record) => self.t147.push(record),
      Record::Record148(record) => self.t148.push(record),
      Record::Record149(record) => self.t149.push(record),
      Record::Record150(record) => self.t150.push(record),
      Record::Record151(record) => self.t151.push(record),
      Record::Record152(record) => self.t152.push(record),
      Record::Record153(record) => self.t153.push(record),
      Record::Record154(record) => self.t154.push(record),
      Record::Record155(record) => self.t155.push(record),
      Record::Record156(record) => self.t156.push(record),
      Record::Record157(record) => self.t157.push(record),
      Record::Record158(record) => self.t158.push(record),
      Record::Record159(record) => self.t159.push(record),
      Record::Record160(record) => self.t160.push(record),
      Record::Record161(record) => self.t161.push(record),
      Record::Record162(record) => self.t162.push(record),
      Record::Record163(record) => self.t163.push(record),
      Record::Record164(record) => self.t164.push(record),
      Record::Record165(record) => self.t165.push(record),
      Record::Record166(record) => self.t166.push(record),
      Record::Record167(record) => self.t167.push(record),
      Record::Record168(record) => self.t168.push(record),
      Record::Record169(record) => self.t169.push(record),
      Record::Record170(record) => self.t170.push(record),
      Record::Record171(record) => self.t171.push(record),
      Record::Record172(record) => self.t172.push(record),
      Record::Record173(record) => self.t173.push(record),
      Record::Record174(record) => self.t174.push(record),
      Record::Record175(record) => self.t175.push(record),
      Record::Record176(record) => self.t176.push(record),
      Record::Record177(record) => self.t177.push(record),
      Record::Record178(record) => self.t178.push(record),
      Record::Record179(record) => self.t179.push(record),
      Record::Record180(record) => self.t180.push(record),
      Record::Record181(record) => self.t181.push(record),
      Record::Record182(record) => self.t182.push(record),
      Record::Record183(record) => self.t183.push(record),
      Record::Record184(record) => self.t184.push(record),
      Record::Record185(record) => self.t185.push(record),
      Record::Record186(record) => self.t186.push(record),
      Record::Record187(record) => self.t187.push(record),
      Record::Record188(record) => self.t188.push(record),
      Record::Record189(record) => self.t189.push(record),
      Record::Record190(record) => self.t190.push(record),
      Record::Record191(record) => self.t191.push(record),
      Record::Record192(record) => self.t192.push(record),
      Record::Record193(record) => self.t193.push(record),
      Record::Record194(record) => self.t194.push(record),
      Record::Record195(record) => self.t195.push(record),
      Record::Record196(record) => self.t196.push(record),
      Record::Record197(record) => self.t197.push(record),
      Record::Record198(record) => self.t198.push(record),
      Record::Record199(record) => self.t199.push(record),
      Record::Record200(record) => self.t200.push(record),
      Record::Record201(record) => self.t201.push(record),
      Record::Record202(record) => self.t202.push(record),
      Record::Record203(record) => self.t203.push(record),
      Record::Record204(record) => self.t204.push(record),
      Record::Record205(record) => self.t205.push(record),
      Record::Record206(record) => self.t206.push(record),
      Record::Record207(record) => self.t207.push(record),
      Record::Record208(record) => self.t208.push(record),
      Record::Record209(record) => self.t209.push(record),
      Record::Record210(record) => self.t210.push(record),
      Record::Record211(record) => self.t211.push(record),
      Record::Record212(record) => self.t212.push(record),
      Record::Record213(record) => self.t213.push(record),
      Record::Record214(record) => self.t214.push(record),
      Record::Record215(record) => self.t215.push(record),
      Record::Record216(record) => self.t216.push(record),
      Record::Record217(record) => self.t217.push(record),
      Record::Record218(record) => self.t218.push(record),
      Record::Record219(record) => self.t219.push(record),
      Record::Record220(record) => self.t220.push(record),
      Record::Record221(record) => self.t221.push(record),
      Record::Record222(record) => self.t222.push(record),
      Record::Record223(record) => self.t223.push(record),
      Record::Record224(record) => self.t224.push(record),
      Record::Record225(record) => self.t225.push(record),
      Record::Record226(record) => self.t226.push(record),
      Record::Record227(record) => self.t227.push(record),
      Record::Record228(record) => self.t228.push(record),
      Record::Record229(record) => self.t229.push(record),
      Record::Record230(record) => self.t230.push(record),
      Record::Record231(record) => self.t231.push(record),
      Record::Record232(record) => self.t232.push(record),
      Record::Record233(record) => self.t233.push(record),
      Record::Record234(record) => self.t234.push(record),
      Record::Record235(record) => self.t235.push(record),
      Record::Record236(record) => self.t236.push(record),
      Record::Record237(record) => self.t237.push(record),
      Record::Record238(record) => self.t238.push(record),
      Record::Record239(record) => self.t239.push(record),
      Record::Record240(record) => self.t240.push(record),
      Record::Record241(record) => self.t241.push(record),
      Record::Record242(record) => self.t242.push(record),
      Record::Record243(record) => self.t243.push(record),
      Record::Record244(record) => self.t244.push(record),
      Record::Record245(record) => self.t245.push(record),
      Record::Record246(record) => self.t246.push(record),
      Record::Record247(record) => self.t247.push(record),
      Record::Record248(record) => self.t248.push(record),
      Record::Record249(record) => self.t249.push(record),
      Record::Record250(record) => self.t250.push(record),
      Record::Record251(record) => self.t251.push(record),
      Record::Record252(record) => self.t252.push(record),
      Record::Record253(record) => self.t253.push(record),
      Record::Record254(record) => self.t254.push(record),
      Record::Record255(record) => self.t255.push(record),
      Record::Record256(record) => self.t256.push(record),
      Record::Record257(record) => self.t257.push(record),
      Record::Record258(record) => self.t258.push(record),
      Record::Record259(record) => self.t259.push(record),
      Record::Record260(record) => self.t260.push(record),
      Record::Record261(record) => self.t261.push(record),
      Record::Record262(record) => self.t262.push(record),
      Record::Record263(record) => self.t263.push(record),
      Record::Record264(record) => self.t264.push(record),
      Record::Record265(record) => self.t265.push(record),
      Record::Record266(record) => self.t266.push(record),
      Record::Record267(record) => self.t267.push(record),
      Record::Record268(record) => self.t268.push(record),
      Record::Record269(record) => self.t269.push(record),
      Record::Record270(record) => self.t270.push(record),
      Record::Record271(record) => self.t271.push(record),
      Record::Record272(record) => self.t272.push(record),
      Record::Record273(record) => self.t273.push(record),
      Record::Record274(record) => self.t274.push(record),
      Record::Record275(record) => self.t275.push(record),
      Record::Record276(record) => self.t276.push(record),
      Record::Record277(record) => self.t277.push(record),
      Record::Record278(record) => self.t278.push(record),
      Record::Record279(record) => self.t279.push(record),
      Record::Record280(record) => self.t280.push(record),
      Record::Record281(record) => self.t281.push(record),
      Record::Record282(record) => self.t282.push(record),
      Record::Record283(record) => self.t283.push(record),
      Record::Record284(record) => self.t284.push(record),
      Record::Record285(record) => self.t285.push(record),
      Record::Record286(record) => self.t286.push(record),
      Record::Record287(record) => self.t287.push(record),
      Record::Record288(record) => self.t288.push(record),
      Record::Record289(record) => self.t289.push(record),
      Record::Record290(record) => self.t290.push(record),
      Record::Record291(record) => self.t291.push(record),
      Record::Record292(record) => self.t292.push(record),
      Record::Record293(record) => self.t293.push(record),
      Record::Record294(record) => self.t294.push(record),
      Record::Record295(record) => self.t295.push(record),
      Record::Record296(record) => self.t296.push(record),
      Record::Record297(record) => self.t297.push(record),
      Record::Record298(record) => self.t298.push(record),
      Record::Record299(record) => self.t299.push(record),
      Record::Record300(record) => self.t300.push(record),
      Record::Record301(record) => self.t301.push(record),
      Record::Record302(record) => self.t302.push(record),
      Record::Record303(record) => self.t303.push(record),
      Record::Record304(record) => self.t304.push(record),
      Record::Record305(record) => self.t305.push(record),
      Record::Record306(record) => self.t306.push(record),
      Record::Record307(record) => self.t307.push(record),
      Record::Record308(record) => self.t308.push(record),
      Record::Record309(record) => self.t309.push(record),
      Record::Record310(record) => self.t310.push(record),
      Record::Record311(record) => self.t311.push(record),
      Record::Record312(record) => self.t312.push(record),
      Record::Record313(record) => self.t313.push(record),
      Record::Record314(record) => self.t314.push(record),
      Record::Record315(record) => self.t315.push(record),
      Record::Record316(record) => self.t316.push(record),
      Record::Record317(record) => self.t317.push(record),
      Record::Record318(record) => self.t318.push(record),
      Record::Record319(record) => self.t319.push(record),
      Record::Record320(record) => self.t320.push(record),
      Record::Record321(record) => self.t321.push(record),
      Record::Record322(record) => self.t322.push(record),
      Record::Record323(record) => self.t323.push(record),
      Record::Record324(record) => self.t324.push(record),
      Record::Record325(record) => self.t325.push(record),
      Record::Record326(record) => self.t326.push(record),
      Record::Record327(record) => self.t327.push(record),
      Record::Record328(record) => self.t328.push(record),
      Record::Record329(record) => self.t329.push(record),
      Record::Record330(record) => self.t330.push(record),
      Record::Record331(record) => self.t331.push(record),
      Record::Record332(record) => self.t332.push(record),
      Record::Record333(record) => self.t333.push(record),
      Record::Record334(record) => self.t334.push(record),
      Record::Record335(record) => self.t335.push(record),
      Record::Record336(record) => self.t336.push(record),
      Record::Record337(record) => self.t337.push(record),
      Record::Record338(record) => self.t338.push(record),
      Record::Record339(record) => self.t339.push(record),
      Record::Record340(record) => self.t340.push(record),
      Record::Record341(record) => self.t341.push(record),
      Record::Record342(record) => self.t342.push(record),
      Record::Record343(record) => self.t343.push(record),
      Record::Record344(record) => self.t344.push(record),
      Record::Record345(record) => self.t345.push(record),
      Record::Record346(record) => self.t346.push(record),
      Record::Record347(record) => self.t347.push(record),
      Record::Record348(record) => self.t348.push(record),
      Record::Record349(record) => self.t349.push(record),
      Record::Record350(record) => self.t350.push(record),
      Record::Record351(record) => self.t351.push(record),
      Record::Record352(record) => self.t352.push(record),
      Record::Record353(record) => self.t353.push(record),
      Record::Record354(record) => self.t354.push(record),
    }
  }

  pub async fn flush(&self) -> Result<()> {
    self.t0.flush().await?;
    self.t1.flush().await?;
    self.t2.flush().await?;
    self.t3.flush().await?;
    self.t4.flush().await?;
    self.t5.flush().await?;
    self.t6.flush().await?;
    self.t7.flush().await?;
    self.t8.flush().await?;
    self.t9.flush().await?;
    self.t10.flush().await?;
    self.t11.flush().await?;
    self.t12.flush().await?;
    self.t13.flush().await?;
    self.t14.flush().await?;
    self.t15.flush().await?;
    self.t16.flush().await?;
    self.t17.flush().await?;
    self.t18.flush().await?;
    self.t19.flush().await?;
    self.t20.flush().await?;
    self.t21.flush().await?;
    self.t22.flush().await?;
    self.t23.flush().await?;
    self.t24.flush().await?;
    self.t25.flush().await?;
    self.t26.flush().await?;
    self.t27.flush().await?;
    self.t28.flush().await?;
    self.t29.flush().await?;
    self.t30.flush().await?;
    self.t31.flush().await?;
    self.t32.flush().await?;
    self.t33.flush().await?;
    self.t34.flush().await?;
    self.t35.flush().await?;
    self.t36.flush().await?;
    self.t37.flush().await?;
    self.t38.flush().await?;
    self.t39.flush().await?;
    self.t40.flush().await?;
    self.t41.flush().await?;
    self.t42.flush().await?;
    self.t43.flush().await?;
    self.t44.flush().await?;
    self.t45.flush().await?;
    self.t46.flush().await?;
    self.t47.flush().await?;
    self.t48.flush().await?;
    self.t49.flush().await?;
    self.t50.flush().await?;
    self.t51.flush().await?;
    self.t52.flush().await?;
    self.t53.flush().await?;
    self.t54.flush().await?;
    self.t55.flush().await?;
    self.t56.flush().await?;
    self.t57.flush().await?;
    self.t58.flush().await?;
    self.t59.flush().await?;
    self.t60.flush().await?;
    self.t61.flush().await?;
    self.t62.flush().await?;
    self.t63.flush().await?;
    self.t64.flush().await?;
    self.t65.flush().await?;
    self.t66.flush().await?;
    self.t67.flush().await?;
    self.t68.flush().await?;
    self.t69.flush().await?;
    self.t70.flush().await?;
    self.t71.flush().await?;
    self.t72.flush().await?;
    self.t73.flush().await?;
    self.t74.flush().await?;
    self.t75.flush().await?;
    self.t76.flush().await?;
    self.t77.flush().await?;
    self.t78.flush().await?;
    self.t79.flush().await?;
    self.t80.flush().await?;
    self.t81.flush().await?;
    self.t82.flush().await?;
    self.t83.flush().await?;
    self.t84.flush().await?;
    self.t85.flush().await?;
    self.t86.flush().await?;
    self.t87.flush().await?;
    self.t88.flush().await?;
    self.t89.flush().await?;
    self.t90.flush().await?;
    self.t91.flush().await?;
    self.t92.flush().await?;
    self.t93.flush().await?;
    self.t94.flush().await?;
    self.t95.flush().await?;
    self.t96.flush().await?;
    self.t97.flush().await?;
    self.t98.flush().await?;
    self.t99.flush().await?;
    self.t100.flush().await?;
    self.t101.flush().await?;
    self.t102.flush().await?;
    self.t103.flush().await?;
    self.t104.flush().await?;
    self.t105.flush().await?;
    self.t106.flush().await?;
    self.t107.flush().await?;
    self.t108.flush().await?;
    self.t109.flush().await?;
    self.t110.flush().await?;
    self.t111.flush().await?;
    self.t112.flush().await?;
    self.t113.flush().await?;
    self.t114.flush().await?;
    self.t115.flush().await?;
    self.t116.flush().await?;
    self.t117.flush().await?;
    self.t118.flush().await?;
    self.t119.flush().await?;
    self.t120.flush().await?;
    self.t121.flush().await?;
    self.t122.flush().await?;
    self.t123.flush().await?;
    self.t124.flush().await?;
    self.t125.flush().await?;
    self.t126.flush().await?;
    self.t127.flush().await?;
    self.t128.flush().await?;
    self.t129.flush().await?;
    self.t130.flush().await?;
    self.t131.flush().await?;
    self.t132.flush().await?;
    self.t133.flush().await?;
    self.t134.flush().await?;
    self.t135.flush().await?;
    self.t136.flush().await?;
    self.t137.flush().await?;
    self.t138.flush().await?;
    self.t139.flush().await?;
    self.t140.flush().await?;
    self.t141.flush().await?;
    self.t142.flush().await?;
    self.t143.flush().await?;
    self.t144.flush().await?;
    self.t145.flush().await?;
    self.t146.flush().await?;
    self.t147.flush().await?;
    self.t148.flush().await?;
    self.t149.flush().await?;
    self.t150.flush().await?;
    self.t151.flush().await?;
    self.t152.flush().await?;
    self.t153.flush().await?;
    self.t154.flush().await?;
    self.t155.flush().await?;
    self.t156.flush().await?;
    self.t157.flush().await?;
    self.t158.flush().await?;
    self.t159.flush().await?;
    self.t160.flush().await?;
    self.t161.flush().await?;
    self.t162.flush().await?;
    self.t163.flush().await?;
    self.t164.flush().await?;
    self.t165.flush().await?;
    self.t166.flush().await?;
    self.t167.flush().await?;
    self.t168.flush().await?;
    self.t169.flush().await?;
    self.t170.flush().await?;
    self.t171.flush().await?;
    self.t172.flush().await?;
    self.t173.flush().await?;
    self.t174.flush().await?;
    self.t175.flush().await?;
    self.t176.flush().await?;
    self.t177.flush().await?;
    self.t178.flush().await?;
    self.t179.flush().await?;
    self.t180.flush().await?;
    self.t181.flush().await?;
    self.t182.flush().await?;
    self.t183.flush().await?;
    self.t184.flush().await?;
    self.t185.flush().await?;
    self.t186.flush().await?;
    self.t187.flush().await?;
    self.t188.flush().await?;
    self.t189.flush().await?;
    self.t190.flush().await?;
    self.t191.flush().await?;
    self.t192.flush().await?;
    self.t193.flush().await?;
    self.t194.flush().await?;
    self.t195.flush().await?;
    self.t196.flush().await?;
    self.t197.flush().await?;
    self.t198.flush().await?;
    self.t199.flush().await?;
    self.t200.flush().await?;
    self.t201.flush().await?;
    self.t202.flush().await?;
    self.t203.flush().await?;
    self.t204.flush().await?;
    self.t205.flush().await?;
    self.t206.flush().await?;
    self.t207.flush().await?;
    self.t208.flush().await?;
    self.t209.flush().await?;
    self.t210.flush().await?;
    self.t211.flush().await?;
    self.t212.flush().await?;
    self.t213.flush().await?;
    self.t214.flush().await?;
    self.t215.flush().await?;
    self.t216.flush().await?;
    self.t217.flush().await?;
    self.t218.flush().await?;
    self.t219.flush().await?;
    self.t220.flush().await?;
    self.t221.flush().await?;
    self.t222.flush().await?;
    self.t223.flush().await?;
    self.t224.flush().await?;
    self.t225.flush().await?;
    self.t226.flush().await?;
    self.t227.flush().await?;
    self.t228.flush().await?;
    self.t229.flush().await?;
    self.t230.flush().await?;
    self.t231.flush().await?;
    self.t232.flush().await?;
    self.t233.flush().await?;
    self.t234.flush().await?;
    self.t235.flush().await?;
    self.t236.flush().await?;
    self.t237.flush().await?;
    self.t238.flush().await?;
    self.t239.flush().await?;
    self.t240.flush().await?;
    self.t241.flush().await?;
    self.t242.flush().await?;
    self.t243.flush().await?;
    self.t244.flush().await?;
    self.t245.flush().await?;
    self.t246.flush().await?;
    self.t247.flush().await?;
    self.t248.flush().await?;
    self.t249.flush().await?;
    self.t250.flush().await?;
    self.t251.flush().await?;
    self.t252.flush().await?;
    self.t253.flush().await?;
    self.t254.flush().await?;
    self.t255.flush().await?;
    self.t256.flush().await?;
    self.t257.flush().await?;
    self.t258.flush().await?;
    self.t259.flush().await?;
    self.t260.flush().await?;
    self.t261.flush().await?;
    self.t262.flush().await?;
    self.t263.flush().await?;
    self.t264.flush().await?;
    self.t265.flush().await?;
    self.t266.flush().await?;
    self.t267.flush().await?;
    self.t268.flush().await?;
    self.t269.flush().await?;
    self.t270.flush().await?;
    self.t271.flush().await?;
    self.t272.flush().await?;
    self.t273.flush().await?;
    self.t274.flush().await?;
    self.t275.flush().await?;
    self.t276.flush().await?;
    self.t277.flush().await?;
    self.t278.flush().await?;
    self.t279.flush().await?;
    self.t280.flush().await?;
    self.t281.flush().await?;
    self.t282.flush().await?;
    self.t283.flush().await?;
    self.t284.flush().await?;
    self.t285.flush().await?;
    self.t286.flush().await?;
    self.t287.flush().await?;
    self.t288.flush().await?;
    self.t289.flush().await?;
    self.t290.flush().await?;
    self.t291.flush().await?;
    self.t292.flush().await?;
    self.t293.flush().await?;
    self.t294.flush().await?;
    self.t295.flush().await?;
    self.t296.flush().await?;
    self.t297.flush().await?;
    self.t298.flush().await?;
    self.t299.flush().await?;
    self.t300.flush().await?;
    self.t301.flush().await?;
    self.t302.flush().await?;
    self.t303.flush().await?;
    self.t304.flush().await?;
    self.t305.flush().await?;
    self.t306.flush().await?;
    self.t307.flush().await?;
    self.t308.flush().await?;
    self.t309.flush().await?;
    self.t310.flush().await?;
    self.t311.flush().await?;
    self.t312.flush().await?;
    self.t313.flush().await?;
    self.t314.flush().await?;
    self.t315.flush().await?;
    self.t316.flush().await?;
    self.t317.flush().await?;
    self.t318.flush().await?;
    self.t319.flush().await?;
    self.t320.flush().await?;
    self.t321.flush().await?;
    self.t322.flush().await?;
    self.t323.flush().await?;
    self.t324.flush().await?;
    self.t325.flush().await?;
    self.t326.flush().await?;
    self.t327.flush().await?;
    self.t328.flush().await?;
    self.t329.flush().await?;
    self.t330.flush().await?;
    self.t331.flush().await?;
    self.t332.flush().await?;
    self.t333.flush().await?;
    self.t334.flush().await?;
    self.t335.flush().await?;
    self.t336.flush().await?;
    self.t337.flush().await?;
    self.t338.flush().await?;
    self.t339.flush().await?;
    self.t340.flush().await?;
    self.t341.flush().await?;
    self.t342.flush().await?;
    self.t343.flush().await?;
    self.t344.flush().await?;
    self.t345.flush().await?;
    self.t346.flush().await?;
    self.t347.flush().await?;
    self.t348.flush().await?;
    self.t349.flush().await?;
    self.t350.flush().await?;
    self.t351.flush().await?;
    self.t352.flush().await?;
    self.t353.flush().await?;
    self.t354.flush().await?;
    Ok(())
  }
  pub async fn cancel(&self) -> Result<()> {
    self.t0.cancel().await?;
    self.t1.cancel().await?;
    self.t2.cancel().await?;
    self.t3.cancel().await?;
    self.t4.cancel().await?;
    self.t5.cancel().await?;
    self.t6.cancel().await?;
    self.t7.cancel().await?;
    self.t8.cancel().await?;
    self.t9.cancel().await?;
    self.t10.cancel().await?;
    self.t11.cancel().await?;
    self.t12.cancel().await?;
    self.t13.cancel().await?;
    self.t14.cancel().await?;
    self.t15.cancel().await?;
    self.t16.cancel().await?;
    self.t17.cancel().await?;
    self.t18.cancel().await?;
    self.t19.cancel().await?;
    self.t20.cancel().await?;
    self.t21.cancel().await?;
    self.t22.cancel().await?;
    self.t23.cancel().await?;
    self.t24.cancel().await?;
    self.t25.cancel().await?;
    self.t26.cancel().await?;
    self.t27.cancel().await?;
    self.t28.cancel().await?;
    self.t29.cancel().await?;
    self.t30.cancel().await?;
    self.t31.cancel().await?;
    self.t32.cancel().await?;
    self.t33.cancel().await?;
    self.t34.cancel().await?;
    self.t35.cancel().await?;
    self.t36.cancel().await?;
    self.t37.cancel().await?;
    self.t38.cancel().await?;
    self.t39.cancel().await?;
    self.t40.cancel().await?;
    self.t41.cancel().await?;
    self.t42.cancel().await?;
    self.t43.cancel().await?;
    self.t44.cancel().await?;
    self.t45.cancel().await?;
    self.t46.cancel().await?;
    self.t47.cancel().await?;
    self.t48.cancel().await?;
    self.t49.cancel().await?;
    self.t50.cancel().await?;
    self.t51.cancel().await?;
    self.t52.cancel().await?;
    self.t53.cancel().await?;
    self.t54.cancel().await?;
    self.t55.cancel().await?;
    self.t56.cancel().await?;
    self.t57.cancel().await?;
    self.t58.cancel().await?;
    self.t59.cancel().await?;
    self.t60.cancel().await?;
    self.t61.cancel().await?;
    self.t62.cancel().await?;
    self.t63.cancel().await?;
    self.t64.cancel().await?;
    self.t65.cancel().await?;
    self.t66.cancel().await?;
    self.t67.cancel().await?;
    self.t68.cancel().await?;
    self.t69.cancel().await?;
    self.t70.cancel().await?;
    self.t71.cancel().await?;
    self.t72.cancel().await?;
    self.t73.cancel().await?;
    self.t74.cancel().await?;
    self.t75.cancel().await?;
    self.t76.cancel().await?;
    self.t77.cancel().await?;
    self.t78.cancel().await?;
    self.t79.cancel().await?;
    self.t80.cancel().await?;
    self.t81.cancel().await?;
    self.t82.cancel().await?;
    self.t83.cancel().await?;
    self.t84.cancel().await?;
    self.t85.cancel().await?;
    self.t86.cancel().await?;
    self.t87.cancel().await?;
    self.t88.cancel().await?;
    self.t89.cancel().await?;
    self.t90.cancel().await?;
    self.t91.cancel().await?;
    self.t92.cancel().await?;
    self.t93.cancel().await?;
    self.t94.cancel().await?;
    self.t95.cancel().await?;
    self.t96.cancel().await?;
    self.t97.cancel().await?;
    self.t98.cancel().await?;
    self.t99.cancel().await?;
    self.t100.cancel().await?;
    self.t101.cancel().await?;
    self.t102.cancel().await?;
    self.t103.cancel().await?;
    self.t104.cancel().await?;
    self.t105.cancel().await?;
    self.t106.cancel().await?;
    self.t107.cancel().await?;
    self.t108.cancel().await?;
    self.t109.cancel().await?;
    self.t110.cancel().await?;
    self.t111.cancel().await?;
    self.t112.cancel().await?;
    self.t113.cancel().await?;
    self.t114.cancel().await?;
    self.t115.cancel().await?;
    self.t116.cancel().await?;
    self.t117.cancel().await?;
    self.t118.cancel().await?;
    self.t119.cancel().await?;
    self.t120.cancel().await?;
    self.t121.cancel().await?;
    self.t122.cancel().await?;
    self.t123.cancel().await?;
    self.t124.cancel().await?;
    self.t125.cancel().await?;
    self.t126.cancel().await?;
    self.t127.cancel().await?;
    self.t128.cancel().await?;
    self.t129.cancel().await?;
    self.t130.cancel().await?;
    self.t131.cancel().await?;
    self.t132.cancel().await?;
    self.t133.cancel().await?;
    self.t134.cancel().await?;
    self.t135.cancel().await?;
    self.t136.cancel().await?;
    self.t137.cancel().await?;
    self.t138.cancel().await?;
    self.t139.cancel().await?;
    self.t140.cancel().await?;
    self.t141.cancel().await?;
    self.t142.cancel().await?;
    self.t143.cancel().await?;
    self.t144.cancel().await?;
    self.t145.cancel().await?;
    self.t146.cancel().await?;
    self.t147.cancel().await?;
    self.t148.cancel().await?;
    self.t149.cancel().await?;
    self.t150.cancel().await?;
    self.t151.cancel().await?;
    self.t152.cancel().await?;
    self.t153.cancel().await?;
    self.t154.cancel().await?;
    self.t155.cancel().await?;
    self.t156.cancel().await?;
    self.t157.cancel().await?;
    self.t158.cancel().await?;
    self.t159.cancel().await?;
    self.t160.cancel().await?;
    self.t161.cancel().await?;
    self.t162.cancel().await?;
    self.t163.cancel().await?;
    self.t164.cancel().await?;
    self.t165.cancel().await?;
    self.t166.cancel().await?;
    self.t167.cancel().await?;
    self.t168.cancel().await?;
    self.t169.cancel().await?;
    self.t170.cancel().await?;
    self.t171.cancel().await?;
    self.t172.cancel().await?;
    self.t173.cancel().await?;
    self.t174.cancel().await?;
    self.t175.cancel().await?;
    self.t176.cancel().await?;
    self.t177.cancel().await?;
    self.t178.cancel().await?;
    self.t179.cancel().await?;
    self.t180.cancel().await?;
    self.t181.cancel().await?;
    self.t182.cancel().await?;
    self.t183.cancel().await?;
    self.t184.cancel().await?;
    self.t185.cancel().await?;
    self.t186.cancel().await?;
    self.t187.cancel().await?;
    self.t188.cancel().await?;
    self.t189.cancel().await?;
    self.t190.cancel().await?;
    self.t191.cancel().await?;
    self.t192.cancel().await?;
    self.t193.cancel().await?;
    self.t194.cancel().await?;
    self.t195.cancel().await?;
    self.t196.cancel().await?;
    self.t197.cancel().await?;
    self.t198.cancel().await?;
    self.t199.cancel().await?;
    self.t200.cancel().await?;
    self.t201.cancel().await?;
    self.t202.cancel().await?;
    self.t203.cancel().await?;
    self.t204.cancel().await?;
    self.t205.cancel().await?;
    self.t206.cancel().await?;
    self.t207.cancel().await?;
    self.t208.cancel().await?;
    self.t209.cancel().await?;
    self.t210.cancel().await?;
    self.t211.cancel().await?;
    self.t212.cancel().await?;
    self.t213.cancel().await?;
    self.t214.cancel().await?;
    self.t215.cancel().await?;
    self.t216.cancel().await?;
    self.t217.cancel().await?;
    self.t218.cancel().await?;
    self.t219.cancel().await?;
    self.t220.cancel().await?;
    self.t221.cancel().await?;
    self.t222.cancel().await?;
    self.t223.cancel().await?;
    self.t224.cancel().await?;
    self.t225.cancel().await?;
    self.t226.cancel().await?;
    self.t227.cancel().await?;
    self.t228.cancel().await?;
    self.t229.cancel().await?;
    self.t230.cancel().await?;
    self.t231.cancel().await?;
    self.t232.cancel().await?;
    self.t233.cancel().await?;
    self.t234.cancel().await?;
    self.t235.cancel().await?;
    self.t236.cancel().await?;
    self.t237.cancel().await?;
    self.t238.cancel().await?;
    self.t239.cancel().await?;
    self.t240.cancel().await?;
    self.t241.cancel().await?;
    self.t242.cancel().await?;
    self.t243.cancel().await?;
    self.t244.cancel().await?;
    self.t245.cancel().await?;
    self.t246.cancel().await?;
    self.t247.cancel().await?;
    self.t248.cancel().await?;
    self.t249.cancel().await?;
    self.t250.cancel().await?;
    self.t251.cancel().await?;
    self.t252.cancel().await?;
    self.t253.cancel().await?;
    self.t254.cancel().await?;
    self.t255.cancel().await?;
    self.t256.cancel().await?;
    self.t257.cancel().await?;
    self.t258.cancel().await?;
    self.t259.cancel().await?;
    self.t260.cancel().await?;
    self.t261.cancel().await?;
    self.t262.cancel().await?;
    self.t263.cancel().await?;
    self.t264.cancel().await?;
    self.t265.cancel().await?;
    self.t266.cancel().await?;
    self.t267.cancel().await?;
    self.t268.cancel().await?;
    self.t269.cancel().await?;
    self.t270.cancel().await?;
    self.t271.cancel().await?;
    self.t272.cancel().await?;
    self.t273.cancel().await?;
    self.t274.cancel().await?;
    self.t275.cancel().await?;
    self.t276.cancel().await?;
    self.t277.cancel().await?;
    self.t278.cancel().await?;
    self.t279.cancel().await?;
    self.t280.cancel().await?;
    self.t281.cancel().await?;
    self.t282.cancel().await?;
    self.t283.cancel().await?;
    self.t284.cancel().await?;
    self.t285.cancel().await?;
    self.t286.cancel().await?;
    self.t287.cancel().await?;
    self.t288.cancel().await?;
    self.t289.cancel().await?;
    self.t290.cancel().await?;
    self.t291.cancel().await?;
    self.t292.cancel().await?;
    self.t293.cancel().await?;
    self.t294.cancel().await?;
    self.t295.cancel().await?;
    self.t296.cancel().await?;
    self.t297.cancel().await?;
    self.t298.cancel().await?;
    self.t299.cancel().await?;
    self.t300.cancel().await?;
    self.t301.cancel().await?;
    self.t302.cancel().await?;
    self.t303.cancel().await?;
    self.t304.cancel().await?;
    self.t305.cancel().await?;
    self.t306.cancel().await?;
    self.t307.cancel().await?;
    self.t308.cancel().await?;
    self.t309.cancel().await?;
    self.t310.cancel().await?;
    self.t311.cancel().await?;
    self.t312.cancel().await?;
    self.t313.cancel().await?;
    self.t314.cancel().await?;
    self.t315.cancel().await?;
    self.t316.cancel().await?;
    self.t317.cancel().await?;
    self.t318.cancel().await?;
    self.t319.cancel().await?;
    self.t320.cancel().await?;
    self.t321.cancel().await?;
    self.t322.cancel().await?;
    self.t323.cancel().await?;
    self.t324.cancel().await?;
    self.t325.cancel().await?;
    self.t326.cancel().await?;
    self.t327.cancel().await?;
    self.t328.cancel().await?;
    self.t329.cancel().await?;
    self.t330.cancel().await?;
    self.t331.cancel().await?;
    self.t332.cancel().await?;
    self.t333.cancel().await?;
    self.t334.cancel().await?;
    self.t335.cancel().await?;
    self.t336.cancel().await?;
    self.t337.cancel().await?;
    self.t338.cancel().await?;
    self.t339.cancel().await?;
    self.t340.cancel().await?;
    self.t341.cancel().await?;
    self.t342.cancel().await?;
    self.t343.cancel().await?;
    self.t344.cancel().await?;
    self.t345.cancel().await?;
    self.t346.cancel().await?;
    self.t347.cancel().await?;
    self.t348.cancel().await?;
    self.t349.cancel().await?;
    self.t350.cancel().await?;
    self.t351.cancel().await?;
    self.t352.cancel().await?;
    self.t353.cancel().await?;
    self.t354.cancel().await?;
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }
}

// file
#[derive(Debug, Serialize, Deserialize)]
pub struct Record0 {
  pub c0: u64, // id
  pub c1: String, // filename
  pub c2: String, // content
}

pub struct TableBuilder0 {
  db_url: String,
  records: Vec<Record0>,
}

impl TableBuilder0 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record0) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
      ];
      conn.execute(
        "INSERT INTO file (id, filename, content) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder1 {
  db_url: String,
  records: Vec<Record1>,
}

impl TableBuilder1 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record1) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO source_loc (id, file_id, line, col, expansion_loc, spelling_loc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// source_range
#[derive(Debug, Serialize, Deserialize)]
pub struct Record2 {
  pub c0: u64, // id
  pub c1: u64, // begin
  pub c2: u64, // end
}

pub struct TableBuilder2 {
  db_url: String,
  records: Vec<Record2>,
}

impl TableBuilder2 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record2) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO source_range (id, begin, end) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// enum
#[derive(Debug, Serialize, Deserialize)]
pub struct Record3 {
  pub c0: u64, // id
  pub c1: String, // name
}

pub struct TableBuilder3 {
  db_url: String,
  records: Vec<Record3>,
}

impl TableBuilder3 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record3) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &record.c1,
      ];
      conn.execute(
        "INSERT INTO enum (id, name) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// enum_value
#[derive(Debug, Serialize, Deserialize)]
pub struct Record4 {
  pub c0: u64, // id
  pub c1: u64, // enum_id
  pub c2: String, // name
}

pub struct TableBuilder4 {
  db_url: String,
  records: Vec<Record4>,
}

impl TableBuilder4 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record4) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
      ];
      conn.execute(
        "INSERT INTO enum_value (id, enum_id, name) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder5 {
  db_url: String,
  records: Vec<Record5>,
}

impl TableBuilder5 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record5) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO QualType (id, Type_id, is_const, is_volatile, is_restrict) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder6 {
  db_url: String,
  records: Vec<Record6>,
}

impl TableBuilder6 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record6) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO CFG (id, entry_block_id, exit_block_id, is_linear, indirect_goto) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFG_blocks
#[derive(Debug, Serialize, Deserialize)]
pub struct Record7 {
  pub c0: u64, // CFG_id
  pub c1: u64, // CFGBlock_id
}

pub struct TableBuilder7 {
  db_url: String,
  records: Vec<Record7>,
}

impl TableBuilder7 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record7) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFG_blocks (CFG_id, CFGBlock_id) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFG_try_blocks
#[derive(Debug, Serialize, Deserialize)]
pub struct Record8 {
  pub c0: u64, // CFG_id
  pub c1: u64, // CFGBlock_id
}

pub struct TableBuilder8 {
  db_url: String,
  records: Vec<Record8>,
}

impl TableBuilder8 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record8) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFG_try_blocks (CFG_id, CFGBlock_id) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFG_edges
#[derive(Debug, Serialize, Deserialize)]
pub struct Record9 {
  pub c0: u64, // CFGBlock_src
  pub c1: u64, // CFGBlock_dst
}

pub struct TableBuilder9 {
  db_url: String,
  records: Vec<Record9>,
}

impl TableBuilder9 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record9) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFG_edges (CFGBlock_src, CFGBlock_dst) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder10 {
  db_url: String,
  records: Vec<Record10>,
}

impl TableBuilder10 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record10) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
      ];
      conn.execute(
        "INSERT INTO CFGBlock (id, terminator_stmt, terminator_kind, terminator_cond, label_stmt, loop_target, has_no_return_element) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGBlock_elements
#[derive(Debug, Serialize, Deserialize)]
pub struct Record11 {
  pub c0: u64, // CFGBlock_id
  pub c1: u64, // CFGElement_id
}

pub struct TableBuilder11 {
  db_url: String,
  records: Vec<Record11>,
}

impl TableBuilder11 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record11) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGBlock_elements (CFGBlock_id, CFGElement_id) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGElement
#[derive(Debug, Serialize, Deserialize)]
pub struct Record12 {
  pub c0: u64, // id
  pub c1: u64, // kind
}

pub struct TableBuilder12 {
  db_url: String,
  records: Vec<Record12>,
}

impl TableBuilder12 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record12) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGElement (id, kind) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGInitializer
#[derive(Debug, Serialize, Deserialize)]
pub struct Record13 {
  pub c0: u64, // id
  pub c1: u64, // getInitializer
}

pub struct TableBuilder13 {
  db_url: String,
  records: Vec<Record13>,
}

impl TableBuilder13 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record13) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGInitializer (id, getInitializer) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGScopeBegin
#[derive(Debug, Serialize, Deserialize)]
pub struct Record14 {
  pub c0: u64, // id
  pub c1: u64, // getTriggerStmt
  pub c2: u64, // getVarDecl
}

pub struct TableBuilder14 {
  db_url: String,
  records: Vec<Record14>,
}

impl TableBuilder14 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record14) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGScopeBegin (id, getTriggerStmt, getVarDecl) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGScopeEnd
#[derive(Debug, Serialize, Deserialize)]
pub struct Record15 {
  pub c0: u64, // id
  pub c1: u64, // getTriggerStmt
  pub c2: u64, // getVarDecl
}

pub struct TableBuilder15 {
  db_url: String,
  records: Vec<Record15>,
}

impl TableBuilder15 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record15) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGScopeEnd (id, getTriggerStmt, getVarDecl) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGNewAllocator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record16 {
  pub c0: u64, // id
  pub c1: u64, // getAllocatorExpr
}

pub struct TableBuilder16 {
  db_url: String,
  records: Vec<Record16>,
}

impl TableBuilder16 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record16) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGNewAllocator (id, getAllocatorExpr) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGLifetimeEnds
#[derive(Debug, Serialize, Deserialize)]
pub struct Record17 {
  pub c0: u64, // id
  pub c1: u64, // getTriggerStmt
  pub c2: u64, // getVarDecl
}

pub struct TableBuilder17 {
  db_url: String,
  records: Vec<Record17>,
}

impl TableBuilder17 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record17) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGLifetimeEnds (id, getTriggerStmt, getVarDecl) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGLoopExit
#[derive(Debug, Serialize, Deserialize)]
pub struct Record18 {
  pub c0: u64, // id
  pub c1: u64, // getLoopStmt
}

pub struct TableBuilder18 {
  db_url: String,
  records: Vec<Record18>,
}

impl TableBuilder18 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record18) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGLoopExit (id, getLoopStmt) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record19 {
  pub c0: u64, // id
  pub c1: u64, // getStmt
}

pub struct TableBuilder19 {
  db_url: String,
  records: Vec<Record19>,
}

impl TableBuilder19 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record19) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGStmt (id, getStmt) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGConstructor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record20 {
  pub c0: u64, // id
  pub c1: u64, // getStmt
  pub c2: u64, // getConstructionContext
}

pub struct TableBuilder20 {
  db_url: String,
  records: Vec<Record20>,
}

impl TableBuilder20 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record20) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGConstructor (id, getStmt, getConstructionContext) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGCXXRecordTypedCall
#[derive(Debug, Serialize, Deserialize)]
pub struct Record21 {
  pub c0: u64, // id
  pub c1: u64, // getStmt
  pub c2: u64, // getConstructionContext
}

pub struct TableBuilder21 {
  db_url: String,
  records: Vec<Record21>,
}

impl TableBuilder21 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record21) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGCXXRecordTypedCall (id, getStmt, getConstructionContext) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGAutomaticObjDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record22 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getTriggerStmt
  pub c3: u64, // getVarDecl
}

pub struct TableBuilder22 {
  db_url: String,
  records: Vec<Record22>,
}

impl TableBuilder22 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record22) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGAutomaticObjDtor (id, getDestructorDecl, getTriggerStmt, getVarDecl) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGDeleteDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record23 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getCXXRecordDecl
  pub c3: u64, // getDeleteExpr
}

pub struct TableBuilder23 {
  db_url: String,
  records: Vec<Record23>,
}

impl TableBuilder23 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record23) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGDeleteDtor (id, getDestructorDecl, getCXXRecordDecl, getDeleteExpr) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGBaseDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record24 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getBaseSpecifier
}

pub struct TableBuilder24 {
  db_url: String,
  records: Vec<Record24>,
}

impl TableBuilder24 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record24) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGBaseDtor (id, getDestructorDecl, getBaseSpecifier) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGMemberDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record25 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getFieldDecl
}

pub struct TableBuilder25 {
  db_url: String,
  records: Vec<Record25>,
}

impl TableBuilder25 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record25) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGMemberDtor (id, getDestructorDecl, getFieldDecl) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGTemporaryDtor
#[derive(Debug, Serialize, Deserialize)]
pub struct Record26 {
  pub c0: u64, // id
  pub c1: u64, // getDestructorDecl
  pub c2: u64, // getBindTemporaryExpr
}

pub struct TableBuilder26 {
  db_url: String,
  records: Vec<Record26>,
}

impl TableBuilder26 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record26) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGTemporaryDtor (id, getDestructorDecl, getBindTemporaryExpr) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CFGCleanupFunction
#[derive(Debug, Serialize, Deserialize)]
pub struct Record27 {
  pub c0: u64, // id
  pub c1: u64, // getVarDecl
  pub c2: u64, // getFunctionDecl
}

pub struct TableBuilder27 {
  db_url: String,
  records: Vec<Record27>,
}

impl TableBuilder27 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record27) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CFGCleanupFunction (id, getVarDecl, getFunctionDecl) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// Decl_usr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record28 {
  pub c0: u64, // id
  pub c1: String, // usr
}

pub struct TableBuilder28 {
  db_url: String,
  records: Vec<Record28>,
}

impl TableBuilder28 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record28) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &record.c1,
      ];
      conn.execute(
        "INSERT INTO Decl_usr (id, usr) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// QualType_usr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record29 {
  pub c0: u64, // id
  pub c1: String, // usr
}

pub struct TableBuilder29 {
  db_url: String,
  records: Vec<Record29>,
}

impl TableBuilder29 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record29) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &record.c1,
      ];
      conn.execute(
        "INSERT INTO QualType_usr (id, usr) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionDecl_cfg
#[derive(Debug, Serialize, Deserialize)]
pub struct Record30 {
  pub c0: u64, // id
  pub c1: u64, // cfg
}

pub struct TableBuilder30 {
  db_url: String,
  records: Vec<Record30>,
}

impl TableBuilder30 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record30) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO FunctionDecl_cfg (id, cfg) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// LValueReferenceType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record31 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

pub struct TableBuilder31 {
  db_url: String,
  records: Vec<Record31>,
}

impl TableBuilder31 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record31) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO LValueReferenceType (id, isSugared, desugar) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RValueReferenceType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record32 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

pub struct TableBuilder32 {
  db_url: String,
  records: Vec<Record32>,
}

impl TableBuilder32 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record32) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO RValueReferenceType (id, isSugared, desugar) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ConstantArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record33 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder33 {
  db_url: String,
  records: Vec<Record33>,
}

impl TableBuilder33 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record33) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO ConstantArrayType (id, getSizeExpr, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// VariableArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record34 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: u64, // getBracketsRange
  pub c3: u64, // getLBracketLoc
  pub c4: u64, // getRBracketLoc
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

pub struct TableBuilder34 {
  db_url: String,
  records: Vec<Record34>,
}

impl TableBuilder34 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record34) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO VariableArrayType (id, getSizeExpr, getBracketsRange, getLBracketLoc, getRBracketLoc, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record35 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: u64, // getSizeModifier
  pub c3: u32, // getIndexTypeCVRQualifiers
}

pub struct TableBuilder35 {
  db_url: String,
  records: Vec<Record35>,
}

impl TableBuilder35 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record35) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
      ];
      conn.execute(
        "INSERT INTO ArrayType (id, getElementType, getSizeModifier, getIndexTypeCVRQualifiers) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentAddressSpaceType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record36 {
  pub c0: u64, // id
  pub c1: u64, // getAddrSpaceExpr
  pub c2: u64, // getPointeeType
  pub c3: u64, // getAttributeLoc
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

pub struct TableBuilder36 {
  db_url: String,
  records: Vec<Record36>,
}

impl TableBuilder36 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record36) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentAddressSpaceType (id, getAddrSpaceExpr, getPointeeType, getAttributeLoc, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentSizedExtVectorType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record37 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: u64, // getElementType
  pub c3: u64, // getAttributeLoc
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

pub struct TableBuilder37 {
  db_url: String,
  records: Vec<Record37>,
}

impl TableBuilder37 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record37) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentSizedExtVectorType (id, getSizeExpr, getElementType, getAttributeLoc, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypeWithKeyword
#[derive(Debug, Serialize, Deserialize)]
pub struct Record38 {
  pub c0: u64, // id
  pub c1: u64, // getKeyword
}

pub struct TableBuilder38 {
  db_url: String,
  records: Vec<Record38>,
}

impl TableBuilder38 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record38) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO TypeWithKeyword (id, getKeyword) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UsingType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record39 {
  pub c0: u64, // id
  pub c1: u64, // getFoundDecl
  pub c2: u64, // getUnderlyingType
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
  pub c5: bool, // typeMatchesDecl
}

pub struct TableBuilder39 {
  db_url: String,
  records: Vec<Record39>,
}

impl TableBuilder39 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record39) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO UsingType (id, getFoundDecl, getUnderlyingType, isSugared, desugar, typeMatchesDecl) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypedefType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record40 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
  pub c4: bool, // typeMatchesDecl
}

pub struct TableBuilder40 {
  db_url: String,
  records: Vec<Record40>,
}

impl TableBuilder40 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record40) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO TypedefType (id, getDecl, isSugared, desugar, typeMatchesDecl) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ConstantMatrixType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record41 {
  pub c0: u64, // id
  pub c1: u32, // getNumRows
  pub c2: u32, // getNumColumns
  pub c3: u32, // getNumElementsFlattened
}

pub struct TableBuilder41 {
  db_url: String,
  records: Vec<Record41>,
}

impl TableBuilder41 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record41) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
      ];
      conn.execute(
        "INSERT INTO ConstantMatrixType (id, getNumRows, getNumColumns, getNumElementsFlattened) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AutoType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record42 {
  pub c0: u64, // id
  pub c1: u64, // getTypeConstraintConcept
  pub c2: bool, // isConstrained
  pub c3: bool, // isDecltypeAuto
  pub c4: bool, // isGNUAutoType
  pub c5: u64, // getKeyword
}

pub struct TableBuilder42 {
  db_url: String,
  records: Vec<Record42>,
}

impl TableBuilder42 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record42) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO AutoType (id, getTypeConstraintConcept, isConstrained, isDecltypeAuto, isGNUAutoType, getKeyword) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// InjectedClassNameType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record43 {
  pub c0: u64, // id
  pub c1: u64, // getInjectedSpecializationType
  pub c2: u64, // getInjectedTST
  pub c3: u64, // getDecl
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

pub struct TableBuilder43 {
  db_url: String,
  records: Vec<Record43>,
}

impl TableBuilder43 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record43) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO InjectedClassNameType (id, getInjectedSpecializationType, getInjectedTST, getDecl, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// Type
#[derive(Debug, Serialize, Deserialize)]
pub struct Record44 {
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

pub struct TableBuilder44 {
  db_url: String,
  records: Vec<Record44>,
}

impl TableBuilder44 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record44) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 35] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &(record.c16 as i64),
        &(record.c17 as i64),
        &(record.c18 as i64),
        &(record.c19 as i64),
        &(record.c20 as i64),
        &(record.c21 as i64),
        &(record.c22 as i64),
        &(record.c23 as i64),
        &(record.c24 as i64),
        &(record.c25 as i64),
        &(record.c26 as i64),
        &(record.c27 as i64),
        &(record.c28 as i64),
        &(record.c29 as i64),
        &(record.c30 as i64),
        &(record.c31 as i64),
        &(record.c32 as i64),
        &record.c33,
        &(record.c34 as i64),
      ];
      conn.execute(
        "INSERT INTO Type (id, containsUnexpandedParameterPack, getLocallyUnqualifiedSingleStepDesugaredType, getAsPlaceholderType, getObjCARCImplicitLifetime, getDependence, containsErrors, hasSizedVLAType, hasUnnamedOrLocalType, canDecayToPointerType, hasPointerRepresentation, hasObjCPointerRepresentation, hasIntegerRepresentation, hasSignedIntegerRepresentation, hasUnsignedIntegerRepresentation, hasFloatingRepresentation, getAsStructureType, getAsUnionType, getAsComplexIntegerType, getAsObjCInterfaceType, getAsObjCInterfacePointerType, getAsObjCQualifiedIdType, getAsObjCQualifiedClassType, getAsObjCQualifiedInterfaceType, getAsCXXRecordDecl, getAsRecordDecl, getAsTagDecl, getPointeeCXXRecordDecl, getBaseElementTypeUnsafe, getArrayElementTypeNoTypeQual, getPointeeOrArrayElementType, getLinkage, getVisibility, acceptsObjCTypeParams, getCanonicalTypeInternal) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentNameType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record45 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

pub struct TableBuilder45 {
  db_url: String,
  records: Vec<Record45>,
}

impl TableBuilder45 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record45) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentNameType (id, isSugared, desugar) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BitIntType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record46 {
  pub c0: u64, // id
  pub c1: bool, // isUnsigned
  pub c2: bool, // isSigned
  pub c3: u32, // getNumBits
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

pub struct TableBuilder46 {
  db_url: String,
  records: Vec<Record46>,
}

impl TableBuilder46 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record46) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO BitIntType (id, isUnsigned, isSigned, getNumBits, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DecayedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record47 {
  pub c0: u64, // id
  pub c1: u64, // getDecayedType
  pub c2: u64, // getPointeeType
}

pub struct TableBuilder47 {
  db_url: String,
  records: Vec<Record47>,
}

impl TableBuilder47 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record47) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO DecayedType (id, getDecayedType, getPointeeType) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MemberPointerType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record48 {
  pub c0: u64, // id
  pub c1: u64, // getPointeeType
  pub c2: bool, // isMemberFunctionPointer
  pub c3: bool, // isMemberDataPointer
  pub c4: u64, // getClass
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

pub struct TableBuilder48 {
  db_url: String,
  records: Vec<Record48>,
}

impl TableBuilder48 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record48) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO MemberPointerType (id, getPointeeType, isMemberFunctionPointer, isMemberDataPointer, getClass, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BuiltinType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record49 {
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

pub struct TableBuilder49 {
  db_url: String,
  records: Vec<Record49>,
}

impl TableBuilder49 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record49) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 12] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
      ];
      conn.execute(
        "INSERT INTO BuiltinType (id, getKind, isSugared, desugar, isInteger, isSignedInteger, isUnsignedInteger, isFloatingPoint, isSVEBool, isSVECount, isPlaceholderType, isNonOverloadPlaceholderType) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentSizedArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record50 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: u64, // getBracketsRange
  pub c3: u64, // getLBracketLoc
  pub c4: u64, // getRBracketLoc
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

pub struct TableBuilder50 {
  db_url: String,
  records: Vec<Record50>,
}

impl TableBuilder50 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record50) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentSizedArrayType (id, getSizeExpr, getBracketsRange, getLBracketLoc, getRBracketLoc, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ComplexType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record51 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder51 {
  db_url: String,
  records: Vec<Record51>,
}

impl TableBuilder51 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record51) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO ComplexType (id, getElementType, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnresolvedUsingType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record52 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder52 {
  db_url: String,
  records: Vec<Record52>,
}

impl TableBuilder52 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record52) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO UnresolvedUsingType (id, getDecl, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TemplateTypeParmType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record53 {
  pub c0: u64, // id
  pub c1: u32, // getDepth
  pub c2: u32, // getIndex
  pub c3: bool, // isParameterPack
  pub c4: u64, // getDecl
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

pub struct TableBuilder53 {
  db_url: String,
  records: Vec<Record53>,
}

impl TableBuilder53 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record53) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO TemplateTypeParmType (id, getDepth, getIndex, isParameterPack, getDecl, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BlockPointerType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record54 {
  pub c0: u64, // id
  pub c1: u64, // getPointeeType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder54 {
  db_url: String,
  records: Vec<Record54>,
}

impl TableBuilder54 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record54) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO BlockPointerType (id, getPointeeType, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AtomicType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record55 {
  pub c0: u64, // id
  pub c1: u64, // getValueType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder55 {
  db_url: String,
  records: Vec<Record55>,
}

impl TableBuilder55 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record55) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO AtomicType (id, getValueType, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentSizedMatrixType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record56 {
  pub c0: u64, // id
  pub c1: u64, // getRowExpr
  pub c2: u64, // getColumnExpr
  pub c3: u64, // getAttributeLoc
}

pub struct TableBuilder56 {
  db_url: String,
  records: Vec<Record56>,
}

impl TableBuilder56 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record56) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentSizedMatrixType (id, getRowExpr, getColumnExpr, getAttributeLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DeducedTemplateSpecializationType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record57 {
  pub c0: u64, // id
}

pub struct TableBuilder57 {
  db_url: String,
  records: Vec<Record57>,
}

impl TableBuilder57 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record57) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO DeducedTemplateSpecializationType (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record58 {
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

pub struct TableBuilder58 {
  db_url: String,
  records: Vec<Record58>,
}

impl TableBuilder58 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record58) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &record.c7,
        &record.c8,
        &record.c9,
      ];
      conn.execute(
        "INSERT INTO FunctionType (id, getReturnType, getHasRegParm, getRegParmType, getNoReturnAttr, getCmseNSCallAttr, getCallConv, isConst, isVolatile, isRestrict) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ParenType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record59 {
  pub c0: u64, // id
  pub c1: u64, // getInnerType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder59 {
  db_url: String,
  records: Vec<Record59>,
}

impl TableBuilder59 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record59) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO ParenType (id, getInnerType, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BTFTagAttributedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record60 {
  pub c0: u64, // id
  pub c1: u64, // getWrappedType
  pub c2: u64, // getAttr
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

pub struct TableBuilder60 {
  db_url: String,
  records: Vec<Record60>,
}

impl TableBuilder60 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record60) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO BTFTagAttributedType (id, getWrappedType, getAttr, isSugared, desugar) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RecordType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record61 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // hasConstFields
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

pub struct TableBuilder61 {
  db_url: String,
  records: Vec<Record61>,
}

impl TableBuilder61 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record61) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO RecordType (id, getDecl, hasConstFields, isSugared, desugar) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TagType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record62 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // isBeingDefined
}

pub struct TableBuilder62 {
  db_url: String,
  records: Vec<Record62>,
}

impl TableBuilder62 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record62) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
      ];
      conn.execute(
        "INSERT INTO TagType (id, getDecl, isBeingDefined) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentBitIntType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record63 {
  pub c0: u64, // id
  pub c1: bool, // isUnsigned
  pub c2: bool, // isSigned
  pub c3: u64, // getNumBitsExpr
  pub c4: bool, // isSugared
  pub c5: u64, // desugar
}

pub struct TableBuilder63 {
  db_url: String,
  records: Vec<Record63>,
}

impl TableBuilder63 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record63) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentBitIntType (id, isUnsigned, isSigned, getNumBitsExpr, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PointerType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record64 {
  pub c0: u64, // id
  pub c1: u64, // getPointeeType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder64 {
  db_url: String,
  records: Vec<Record64>,
}

impl TableBuilder64 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record64) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO PointerType (id, getPointeeType, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AttributedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record65 {
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

pub struct TableBuilder65 {
  db_url: String,
  records: Vec<Record65>,
}

impl TableBuilder65 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record65) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
      ];
      conn.execute(
        "INSERT INTO AttributedType (id, getAttrKind, getModifiedType, getEquivalentType, isSugared, desugar, isQualifier, isMSTypeSpec, isWebAssemblyFuncrefSpec, isCallingConv) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentTemplateSpecializationType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record66 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

pub struct TableBuilder66 {
  db_url: String,
  records: Vec<Record66>,
}

impl TableBuilder66 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record66) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentTemplateSpecializationType (id, isSugared, desugar) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnaryTransformType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record67 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
  pub c3: u64, // getUnderlyingType
  pub c4: u64, // getBaseType
  pub c5: u64, // getUTTKind
}

pub struct TableBuilder67 {
  db_url: String,
  records: Vec<Record67>,
}

impl TableBuilder67 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record67) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO UnaryTransformType (id, isSugared, desugar, getUnderlyingType, getBaseType, getUTTKind) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SubstTemplateTypeParmPackType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record68 {
  pub c0: u64, // id
  pub c1: u64, // getAssociatedDecl
  pub c2: u64, // getReplacedParameter
  pub c3: u32, // getIndex
  pub c4: bool, // getFinal
  pub c5: u32, // getNumArgs
  pub c6: bool, // isSugared
  pub c7: u64, // desugar
}

pub struct TableBuilder68 {
  db_url: String,
  records: Vec<Record68>,
}

impl TableBuilder68 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record68) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &record.c5,
        &record.c6,
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO SubstTemplateTypeParmPackType (id, getAssociatedDecl, getReplacedParameter, getIndex, getFinal, getNumArgs, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder69 {
  db_url: String,
  records: Vec<Record69>,
}

impl TableBuilder69 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record69) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO TemplateSpecializationType (id, isCurrentInstantiation, isTypeAlias, isSugared, desugar) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder70 {
  db_url: String,
  records: Vec<Record70>,
}

impl TableBuilder70 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record70) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO ReferenceType (id, isSpelledAsLValue, isInnerRef, getPointeeTypeAsWritten, getPointeeType) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder71 {
  db_url: String,
  records: Vec<Record71>,
}

impl TableBuilder71 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record71) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO DeducedType (id, isSugared, desugar, getDeducedType, isDeduced) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PackExpansionType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record72 {
  pub c0: u64, // id
  pub c1: u64, // getPattern
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder72 {
  db_url: String,
  records: Vec<Record72>,
}

impl TableBuilder72 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record72) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO PackExpansionType (id, getPattern, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionProtoType_getParamTypes
#[derive(Debug, Serialize, Deserialize)]
pub struct Record73 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder73 {
  db_url: String,
  records: Vec<Record73>,
}

impl TableBuilder73 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record73) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO FunctionProtoType_getParamTypes (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionProtoType_param_types
#[derive(Debug, Serialize, Deserialize)]
pub struct Record74 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder74 {
  db_url: String,
  records: Vec<Record74>,
}

impl TableBuilder74 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record74) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO FunctionProtoType_param_types (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionProtoType_exceptions
#[derive(Debug, Serialize, Deserialize)]
pub struct Record75 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder75 {
  db_url: String,
  records: Vec<Record75>,
}

impl TableBuilder75 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record75) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO FunctionProtoType_exceptions (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionProtoType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record76 {
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

pub struct TableBuilder76 {
  db_url: String,
  records: Vec<Record76>,
}

impl TableBuilder76 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record76) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 22] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &(record.c12 as i64),
        &record.c13,
        &(record.c14 as i64),
        &record.c15,
        &record.c16,
        &(record.c17 as i64),
        &record.c18,
        &record.c19,
        &record.c20,
        &(record.c21 as i64),
      ];
      conn.execute(
        "INSERT INTO FunctionProtoType (id, getNumParams, getExceptionSpecType, hasExceptionSpec, hasDynamicExceptionSpec, hasNoexceptExceptionSpec, hasDependentExceptionSpec, hasInstantiationDependentExceptionSpec, getNumExceptions, getNoexceptExpr, getExceptionSpecDecl, getExceptionSpecTemplate, canThrow, isVariadic, getEllipsisLoc, isTemplateVariadic, hasTrailingReturn, getRefQualifier, hasExtParameterInfos, getAArch64SMEAttributes, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AdjustedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record77 {
  pub c0: u64, // id
  pub c1: u64, // getOriginalType
  pub c2: u64, // getAdjustedType
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

pub struct TableBuilder77 {
  db_url: String,
  records: Vec<Record77>,
}

impl TableBuilder77 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record77) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO AdjustedType (id, getOriginalType, getAdjustedType, isSugared, desugar) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentUnaryTransformType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record78 {
  pub c0: u64, // id
}

pub struct TableBuilder78 {
  db_url: String,
  records: Vec<Record78>,
}

impl TableBuilder78 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record78) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentUnaryTransformType (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// IncompleteArrayType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record79 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

pub struct TableBuilder79 {
  db_url: String,
  records: Vec<Record79>,
}

impl TableBuilder79 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record79) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO IncompleteArrayType (id, isSugared, desugar) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DecltypeType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record80 {
  pub c0: u64, // id
  pub c1: u64, // getUnderlyingExpr
  pub c2: u64, // getUnderlyingType
  pub c3: u64, // desugar
  pub c4: bool, // isSugared
}

pub struct TableBuilder80 {
  db_url: String,
  records: Vec<Record80>,
}

impl TableBuilder80 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record80) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO DecltypeType (id, getUnderlyingExpr, getUnderlyingType, desugar, isSugared) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentVectorType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record81 {
  pub c0: u64, // id
  pub c1: u64, // getSizeExpr
  pub c2: u64, // getElementType
  pub c3: u64, // getAttributeLoc
  pub c4: u64, // getVectorKind
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

pub struct TableBuilder81 {
  db_url: String,
  records: Vec<Record81>,
}

impl TableBuilder81 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record81) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentVectorType (id, getSizeExpr, getElementType, getAttributeLoc, getVectorKind, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ExtVectorType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record82 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

pub struct TableBuilder82 {
  db_url: String,
  records: Vec<Record82>,
}

impl TableBuilder82 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record82) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO ExtVectorType (id, isSugared, desugar) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentDecltypeType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record83 {
  pub c0: u64, // id
}

pub struct TableBuilder83 {
  db_url: String,
  records: Vec<Record83>,
}

impl TableBuilder83 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record83) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentDecltypeType (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypeOfType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record84 {
  pub c0: u64, // id
  pub c1: u64, // getUnmodifiedType
  pub c2: u64, // desugar
  pub c3: bool, // isSugared
  pub c4: u64, // getKind
}

pub struct TableBuilder84 {
  db_url: String,
  records: Vec<Record84>,
}

impl TableBuilder84 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record84) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO TypeOfType (id, getUnmodifiedType, desugar, isSugared, getKind) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PipeType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record85 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
  pub c4: bool, // isReadOnly
}

pub struct TableBuilder85 {
  db_url: String,
  records: Vec<Record85>,
}

impl TableBuilder85 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record85) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO PipeType (id, getElementType, isSugared, desugar, isReadOnly) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MacroQualifiedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record86 {
  pub c0: u64, // id
  pub c1: u64, // getUnderlyingType
  pub c2: u64, // getModifiedType
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
}

pub struct TableBuilder86 {
  db_url: String,
  records: Vec<Record86>,
}

impl TableBuilder86 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record86) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO MacroQualifiedType (id, getUnderlyingType, getModifiedType, isSugared, desugar) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypeOfExprType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record87 {
  pub c0: u64, // id
  pub c1: u64, // getUnderlyingExpr
  pub c2: u64, // getKind
  pub c3: u64, // desugar
  pub c4: bool, // isSugared
}

pub struct TableBuilder87 {
  db_url: String,
  records: Vec<Record87>,
}

impl TableBuilder87 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record87) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO TypeOfExprType (id, getUnderlyingExpr, getKind, desugar, isSugared) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// EnumType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record88 {
  pub c0: u64, // id
  pub c1: u64, // getDecl
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder88 {
  db_url: String,
  records: Vec<Record88>,
}

impl TableBuilder88 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record88) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO EnumType (id, getDecl, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentTypeOfExprType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record89 {
  pub c0: u64, // id
}

pub struct TableBuilder89 {
  db_url: String,
  records: Vec<Record89>,
}

impl TableBuilder89 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record89) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentTypeOfExprType (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionNoProtoType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record90 {
  pub c0: u64, // id
  pub c1: bool, // isSugared
  pub c2: u64, // desugar
}

pub struct TableBuilder90 {
  db_url: String,
  records: Vec<Record90>,
}

impl TableBuilder90 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record90) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO FunctionNoProtoType (id, isSugared, desugar) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ElaboratedType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record91 {
  pub c0: u64, // id
  pub c1: u64, // getNamedType
  pub c2: u64, // desugar
  pub c3: bool, // isSugared
  pub c4: u64, // getOwnedTagDecl
}

pub struct TableBuilder91 {
  db_url: String,
  records: Vec<Record91>,
}

impl TableBuilder91 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record91) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO ElaboratedType (id, getNamedType, desugar, isSugared, getOwnedTagDecl) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MatrixType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record92 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: bool, // isSugared
  pub c3: u64, // desugar
}

pub struct TableBuilder92 {
  db_url: String,
  records: Vec<Record92>,
}

impl TableBuilder92 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record92) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO MatrixType (id, getElementType, isSugared, desugar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SubstTemplateTypeParmType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record93 {
  pub c0: u64, // id
  pub c1: u64, // getReplacementType
  pub c2: u64, // getAssociatedDecl
  pub c3: u64, // getReplacedParameter
  pub c4: u32, // getIndex
  pub c5: bool, // isSugared
  pub c6: u64, // desugar
}

pub struct TableBuilder93 {
  db_url: String,
  records: Vec<Record93>,
}

impl TableBuilder93 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record93) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO SubstTemplateTypeParmType (id, getReplacementType, getAssociatedDecl, getReplacedParameter, getIndex, isSugared, desugar) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// VectorType
#[derive(Debug, Serialize, Deserialize)]
pub struct Record94 {
  pub c0: u64, // id
  pub c1: u64, // getElementType
  pub c2: u32, // getNumElements
  pub c3: bool, // isSugared
  pub c4: u64, // desugar
  pub c5: u64, // getVectorKind
}

pub struct TableBuilder94 {
  db_url: String,
  records: Vec<Record94>,
}

impl TableBuilder94 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record94) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO VectorType (id, getElementType, getNumElements, isSugared, desugar, getVectorKind) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder95 {
  db_url: String,
  records: Vec<Record95>,
}

impl TableBuilder95 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record95) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO ClassTemplatePartialSpecializationDecl (id, hasAssociatedConstraints, getInstantiatedFromMember, getInstantiatedFromMemberTemplate, getInjectedSpecializationType) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TemplateParamObjectDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record96 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
}

pub struct TableBuilder96 {
  db_url: String,
  records: Vec<Record96>,
}

impl TableBuilder96 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record96) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO TemplateParamObjectDecl (id, getCanonicalDecl) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RecordDecl_fields
#[derive(Debug, Serialize, Deserialize)]
pub struct Record97 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder97 {
  db_url: String,
  records: Vec<Record97>,
}

impl TableBuilder97 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record97) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO RecordDecl_fields (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RecordDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record98 {
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

pub struct TableBuilder98 {
  db_url: String,
  records: Vec<Record98>,
}

impl TableBuilder98 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record98) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 25] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &(record.c15 as i64),
        &record.c16,
        &record.c17,
        &record.c18,
        &record.c19,
        &record.c20,
        &(record.c21 as i64),
        &record.c22,
        &record.c23,
        &(record.c24 as i64),
      ];
      conn.execute(
        "INSERT INTO RecordDecl (id, getPreviousDecl, getMostRecentDecl, hasFlexibleArrayMember, isAnonymousStructOrUnion, hasObjectMember, hasVolatileMember, hasLoadedFieldsFromExternalStorage, isNonTrivialToPrimitiveDefaultInitialize, isNonTrivialToPrimitiveCopy, isNonTrivialToPrimitiveDestroy, hasNonTrivialToPrimitiveDefaultInitializeCUnion, hasNonTrivialToPrimitiveDestructCUnion, hasNonTrivialToPrimitiveCopyCUnion, canPassInRegisters, getArgPassingRestrictions, isParamDestroyedInCallee, isRandomized, isInjectedClassName, isLambda, isCapturedRecord, getDefinition, isOrContainsUnion, field_empty, findFirstNamedDataMember) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TagDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record99 {
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

pub struct TableBuilder99 {
  db_url: String,
  records: Vec<Record99>,
}

impl TableBuilder99 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record99) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 26] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &(record.c15 as i64),
        &record.c16,
        &(record.c17 as i64),
        &record.c18,
        &record.c19,
        &record.c20,
        &record.c21,
        &record.c22,
        &record.c23,
        &(record.c24 as i64),
        &record.c25,
      ];
      conn.execute(
        "INSERT INTO TagDecl (id, getBraceRange, getInnerLocStart, getOuterLocStart, getSourceRange, getCanonicalDecl, isThisDeclarationADefinition, isCompleteDefinition, isCompleteDefinitionRequired, isBeingDefined, isEmbeddedInDeclarator, isFreeStanding, mayHaveOutOfDateDef, isDependentType, isThisDeclarationADemotedDefinition, getDefinition, getKindName, getTagKind, isStruct, isInterface, isClass, isUnion, isEnum, hasNameForLinkage, getTypedefNameForAnonDecl, getNumTemplateParameterLists) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnresolvedUsingTypenameDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record100 {
  pub c0: u64, // id
  pub c1: u64, // getUsingLoc
  pub c2: u64, // getTypenameLoc
  pub c3: bool, // isPackExpansion
  pub c4: u64, // getEllipsisLoc
  pub c5: u64, // getCanonicalDecl
}

pub struct TableBuilder100 {
  db_url: String,
  records: Vec<Record100>,
}

impl TableBuilder100 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record100) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO UnresolvedUsingTypenameDecl (id, getUsingLoc, getTypenameLoc, isPackExpansion, getEllipsisLoc, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UsingDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record101 {
  pub c0: u64, // id
  pub c1: u64, // getUsingLoc
  pub c2: bool, // isAccessDeclaration
  pub c3: bool, // hasTypename
  pub c4: u64, // getSourceRange
  pub c5: u64, // getCanonicalDecl
}

pub struct TableBuilder101 {
  db_url: String,
  records: Vec<Record101>,
}

impl TableBuilder101 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record101) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO UsingDecl (id, getUsingLoc, isAccessDeclaration, hasTypename, getSourceRange, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypeDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record102 {
  pub c0: u64, // id
  pub c1: u64, // getTypeForDecl
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getSourceRange
}

pub struct TableBuilder102 {
  db_url: String,
  records: Vec<Record102>,
}

impl TableBuilder102 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record102) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO TypeDecl (id, getTypeForDecl, getBeginLoc, getSourceRange) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UsingDirectiveDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record103 {
  pub c0: u64, // id
  pub c1: u64, // getNominatedNamespaceAsWritten
  pub c2: u64, // getNominatedNamespace
  pub c3: u64, // getUsingLoc
  pub c4: u64, // getNamespaceKeyLocation
  pub c5: u64, // getIdentLocation
  pub c6: u64, // getSourceRange
}

pub struct TableBuilder103 {
  db_url: String,
  records: Vec<Record103>,
}

impl TableBuilder103 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record103) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO UsingDirectiveDecl (id, getNominatedNamespaceAsWritten, getNominatedNamespace, getUsingLoc, getNamespaceKeyLocation, getIdentLocation, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// NamespaceAliasDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record104 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
  pub c2: u64, // getNamespace
  pub c3: u64, // getAliasLoc
  pub c4: u64, // getNamespaceLoc
  pub c5: u64, // getTargetNameLoc
  pub c6: u64, // getAliasedNamespace
  pub c7: u64, // getSourceRange
}

pub struct TableBuilder104 {
  db_url: String,
  records: Vec<Record104>,
}

impl TableBuilder104 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record104) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO NamespaceAliasDecl (id, getCanonicalDecl, getNamespace, getAliasLoc, getNamespaceLoc, getTargetNameLoc, getAliasedNamespace, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ConstructorUsingShadowDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record105 {
  pub c0: u64, // id
  pub c1: u64, // getIntroducer
  pub c2: u64, // getParent
  pub c3: u64, // getNominatedBaseClassShadowDecl
  pub c4: u64, // getConstructedBaseClassShadowDecl
  pub c5: u64, // getNominatedBaseClass
  pub c6: u64, // getConstructedBaseClass
  pub c7: bool, // constructsVirtualBase
}

pub struct TableBuilder105 {
  db_url: String,
  records: Vec<Record105>,
}

impl TableBuilder105 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record105) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &record.c7,
      ];
      conn.execute(
        "INSERT INTO ConstructorUsingShadowDecl (id, getIntroducer, getParent, getNominatedBaseClassShadowDecl, getConstructedBaseClassShadowDecl, getNominatedBaseClass, getConstructedBaseClass, constructsVirtualBase) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BuiltinTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record106 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getBuiltinTemplateKind
}

pub struct TableBuilder106 {
  db_url: String,
  records: Vec<Record106>,
}

impl TableBuilder106 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record106) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO BuiltinTemplateDecl (id, getSourceRange, getBuiltinTemplateKind) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PragmaCommentDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record107 {
  pub c0: u64, // id
  pub c1: u64, // getCommentKind
  pub c2: String, // getArg
}

pub struct TableBuilder107 {
  db_url: String,
  records: Vec<Record107>,
}

impl TableBuilder107 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record107) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
      ];
      conn.execute(
        "INSERT INTO PragmaCommentDecl (id, getCommentKind, getArg) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UsingShadowDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record108 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
  pub c2: u64, // getTargetDecl
  pub c3: u64, // getIntroducer
  pub c4: u64, // getNextUsingShadowDecl
}

pub struct TableBuilder108 {
  db_url: String,
  records: Vec<Record108>,
}

impl TableBuilder108 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record108) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO UsingShadowDecl (id, getCanonicalDecl, getTargetDecl, getIntroducer, getNextUsingShadowDecl) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// EmptyDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record109 {
  pub c0: u64, // id
}

pub struct TableBuilder109 {
  db_url: String,
  records: Vec<Record109>,
}

impl TableBuilder109 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record109) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO EmptyDecl (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BindingDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record110 {
  pub c0: u64, // id
  pub c1: u64, // getBinding
  pub c2: u64, // getDecomposedDecl
  pub c3: u64, // getHoldingVar
}

pub struct TableBuilder110 {
  db_url: String,
  records: Vec<Record110>,
}

impl TableBuilder110 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record110) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO BindingDecl (id, getBinding, getDecomposedDecl, getHoldingVar) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BaseUsingDecl_shadows
#[derive(Debug, Serialize, Deserialize)]
pub struct Record111 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder111 {
  db_url: String,
  records: Vec<Record111>,
}

impl TableBuilder111 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record111) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO BaseUsingDecl_shadows (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BaseUsingDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record112 {
  pub c0: u64, // id
  pub c1: u32, // shadow_size
}

pub struct TableBuilder112 {
  db_url: String,
  records: Vec<Record112>,
}

impl TableBuilder112 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record112) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &record.c1,
      ];
      conn.execute(
        "INSERT INTO BaseUsingDecl (id, shadow_size) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UsingPackDecl_expansions
#[derive(Debug, Serialize, Deserialize)]
pub struct Record113 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder113 {
  db_url: String,
  records: Vec<Record113>,
}

impl TableBuilder113 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record113) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO UsingPackDecl_expansions (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UsingPackDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record114 {
  pub c0: u64, // id
  pub c1: u64, // getInstantiatedFromUsingDecl
  pub c2: u64, // getSourceRange
  pub c3: u64, // getCanonicalDecl
}

pub struct TableBuilder114 {
  db_url: String,
  records: Vec<Record114>,
}

impl TableBuilder114 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record114) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO UsingPackDecl (id, getInstantiatedFromUsingDecl, getSourceRange, getCanonicalDecl) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXMethodDecl_overridden_methods
#[derive(Debug, Serialize, Deserialize)]
pub struct Record115 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder115 {
  db_url: String,
  records: Vec<Record115>,
}

impl TableBuilder115 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record115) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXMethodDecl_overridden_methods (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXMethodDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record116 {
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

pub struct TableBuilder116 {
  db_url: String,
  records: Vec<Record116>,
}

impl TableBuilder116 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record116) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 21] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &(record.c10 as i64),
        &(record.c11 as i64),
        &record.c12,
        &(record.c13 as i64),
        &(record.c14 as i64),
        &(record.c15 as i64),
        &(record.c16 as i64),
        &record.c17,
        &(record.c18 as i64),
        &record.c19,
        &record.c20,
      ];
      conn.execute(
        "INSERT INTO CXXMethodDecl (id, isStatic, isInstance, isExplicitObjectMemberFunction, isImplicitObjectMemberFunction, isConst, isVolatile, isVirtual, isCopyAssignmentOperator, isMoveAssignmentOperator, getCanonicalDecl, getMostRecentDecl, size_overridden_methods, getParent, getThisType, getFunctionObjectParameterReferenceType, getFunctionObjectParameterType, getNumExplicitParams, getRefQualifier, hasInlineBody, isLambdaStaticInvoker) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record117 {
  pub c0: u64, // id
  pub c1: u64, // getTemplatedDecl
  pub c2: bool, // isThisDeclarationADefinition
  pub c3: u64, // getCanonicalDecl
  pub c4: u64, // getPreviousDecl
  pub c5: u64, // getMostRecentDecl
  pub c6: u64, // getInstantiatedFromMemberTemplate
  pub c7: bool, // isAbbreviated
}

pub struct TableBuilder117 {
  db_url: String,
  records: Vec<Record117>,
}

impl TableBuilder117 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record117) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &record.c7,
      ];
      conn.execute(
        "INSERT INTO FunctionTemplateDecl (id, getTemplatedDecl, isThisDeclarationADefinition, getCanonicalDecl, getPreviousDecl, getMostRecentDecl, getInstantiatedFromMemberTemplate, isAbbreviated) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ClassTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record118 {
  pub c0: u64, // id
  pub c1: u64, // getTemplatedDecl
  pub c2: bool, // isThisDeclarationADefinition
  pub c3: u64, // getCanonicalDecl
  pub c4: u64, // getPreviousDecl
  pub c5: u64, // getMostRecentDecl
  pub c6: u64, // getInstantiatedFromMemberTemplate
}

pub struct TableBuilder118 {
  db_url: String,
  records: Vec<Record118>,
}

impl TableBuilder118 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record118) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO ClassTemplateDecl (id, getTemplatedDecl, isThisDeclarationADefinition, getCanonicalDecl, getPreviousDecl, getMostRecentDecl, getInstantiatedFromMemberTemplate) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PragmaDetectMismatchDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record119 {
  pub c0: u64, // id
  pub c1: String, // getName
  pub c2: String, // getValue
}

pub struct TableBuilder119 {
  db_url: String,
  records: Vec<Record119>,
}

impl TableBuilder119 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record119) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
      ];
      conn.execute(
        "INSERT INTO PragmaDetectMismatchDecl (id, getName, getValue) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypeAliasTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record120 {
  pub c0: u64, // id
  pub c1: u64, // getTemplatedDecl
  pub c2: u64, // getCanonicalDecl
  pub c3: u64, // getPreviousDecl
  pub c4: u64, // getInstantiatedFromMemberTemplate
}

pub struct TableBuilder120 {
  db_url: String,
  records: Vec<Record120>,
}

impl TableBuilder120 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record120) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO TypeAliasTemplateDecl (id, getTemplatedDecl, getCanonicalDecl, getPreviousDecl, getInstantiatedFromMemberTemplate) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypedefNameDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record121 {
  pub c0: u64, // id
  pub c1: bool, // isModed
  pub c2: u64, // getUnderlyingType
  pub c3: u64, // getCanonicalDecl
  pub c4: bool, // isTransparentTag
}

pub struct TableBuilder121 {
  db_url: String,
  records: Vec<Record121>,
}

impl TableBuilder121 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record121) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO TypedefNameDecl (id, isModed, getUnderlyingType, getCanonicalDecl, isTransparentTag) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TemplateTemplateParmDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record122 {
  pub c0: u64, // id
  pub c1: bool, // isParameterPack
  pub c2: bool, // isPackExpansion
  pub c3: bool, // isExpandedParameterPack
  pub c4: bool, // hasDefaultArgument
  pub c5: u64, // getDefaultArgumentLoc
  pub c6: bool, // defaultArgumentWasInherited
  pub c7: u64, // getSourceRange
}

pub struct TableBuilder122 {
  db_url: String,
  records: Vec<Record122>,
}

impl TableBuilder122 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record122) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &record.c6,
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO TemplateTemplateParmDecl (id, isParameterPack, isPackExpansion, isExpandedParameterPack, hasDefaultArgument, getDefaultArgumentLoc, defaultArgumentWasInherited, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UsingEnumDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record123 {
  pub c0: u64, // id
  pub c1: u64, // getUsingLoc
  pub c2: u64, // getEnumLoc
  pub c3: u64, // getEnumDecl
  pub c4: u64, // getSourceRange
  pub c5: u64, // getCanonicalDecl
}

pub struct TableBuilder123 {
  db_url: String,
  records: Vec<Record123>,
}

impl TableBuilder123 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record123) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO UsingEnumDecl (id, getUsingLoc, getEnumLoc, getEnumDecl, getSourceRange, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// NamespaceDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record124 {
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

pub struct TableBuilder124 {
  db_url: String,
  records: Vec<Record124>,
}

impl TableBuilder124 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record124) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 11] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
      ];
      conn.execute(
        "INSERT INTO NamespaceDecl (id, isAnonymousNamespace, isInline, isNested, getOriginalNamespace, isOriginalNamespace, getAnonymousNamespace, getCanonicalDecl, getSourceRange, getBeginLoc, getRBraceLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// VarTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record125 {
  pub c0: u64, // id
  pub c1: u64, // getTemplatedDecl
  pub c2: bool, // isThisDeclarationADefinition
  pub c3: u64, // getCanonicalDecl
  pub c4: u64, // getPreviousDecl
  pub c5: u64, // getMostRecentDecl
  pub c6: u64, // getInstantiatedFromMemberTemplate
}

pub struct TableBuilder125 {
  db_url: String,
  records: Vec<Record125>,
}

impl TableBuilder125 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record125) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO VarTemplateDecl (id, getTemplatedDecl, isThisDeclarationADefinition, getCanonicalDecl, getPreviousDecl, getMostRecentDecl, getInstantiatedFromMemberTemplate) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ConceptDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record126 {
  pub c0: u64, // id
  pub c1: u64, // getConstraintExpr
  pub c2: u64, // getSourceRange
  pub c3: bool, // isTypeConcept
  pub c4: u64, // getCanonicalDecl
}

pub struct TableBuilder126 {
  db_url: String,
  records: Vec<Record126>,
}

impl TableBuilder126 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record126) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO ConceptDecl (id, getConstraintExpr, getSourceRange, isTypeConcept, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ClassTemplateSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record127 {
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

pub struct TableBuilder127 {
  db_url: String,
  records: Vec<Record127>,
}

impl TableBuilder127 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record127) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO ClassTemplateSpecializationDecl (id, getSpecializedTemplate, getSpecializationKind, isExplicitSpecialization, isClassScopeExplicitSpecialization, isExplicitInstantiationOrSpecialization, getPointOfInstantiation, getExternLoc, getTemplateKeywordLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record128 {
  pub c0: u64, // id
  pub c1: bool, // hasAssociatedConstraints
  pub c2: u64, // getTemplatedDecl
  pub c3: bool, // isTypeAlias
  pub c4: u64, // getSourceRange
}

pub struct TableBuilder128 {
  db_url: String,
  records: Vec<Record128>,
}

impl TableBuilder128 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record128) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO TemplateDecl (id, hasAssociatedConstraints, getTemplatedDecl, isTypeAlias, getSourceRange) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RequiresExprBodyDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record129 {
  pub c0: u64, // id
}

pub struct TableBuilder129 {
  db_url: String,
  records: Vec<Record129>,
}

impl TableBuilder129 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record129) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO RequiresExprBodyDecl (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypedefDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record130 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
}

pub struct TableBuilder130 {
  db_url: String,
  records: Vec<Record130>,
}

impl TableBuilder130 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record130) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO TypedefDecl (id, getSourceRange) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSGuidDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record131 {
  pub c0: u64, // id
}

pub struct TableBuilder131 {
  db_url: String,
  records: Vec<Record131>,
}

impl TableBuilder131 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record131) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO MSGuidDecl (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnresolvedUsingValueDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record132 {
  pub c0: u64, // id
  pub c1: u64, // getUsingLoc
  pub c2: bool, // isAccessDeclaration
  pub c3: bool, // isPackExpansion
  pub c4: u64, // getEllipsisLoc
  pub c5: u64, // getSourceRange
  pub c6: u64, // getCanonicalDecl
}

pub struct TableBuilder132 {
  db_url: String,
  records: Vec<Record132>,
}

impl TableBuilder132 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record132) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO UnresolvedUsingValueDecl (id, getUsingLoc, isAccessDeclaration, isPackExpansion, getEllipsisLoc, getSourceRange, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnnamedGlobalConstantDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record133 {
  pub c0: u64, // id
}

pub struct TableBuilder133 {
  db_url: String,
  records: Vec<Record133>,
}

impl TableBuilder133 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record133) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO UnnamedGlobalConstantDecl (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXConstructorDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record134 {
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

pub struct TableBuilder134 {
  db_url: String,
  records: Vec<Record134>,
}

impl TableBuilder134 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record134) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 11] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &(record.c10 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXConstructorDecl (id, isExplicit, getNumCtorInitializers, isDelegatingConstructor, isDefaultConstructor, isCopyConstructor, isMoveConstructor, isCopyOrMoveConstructor, isSpecializationCopyingObject, isInheritingConstructor, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionDecl_parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record135 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder135 {
  db_url: String,
  records: Vec<Record135>,
}

impl TableBuilder135 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record135) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO FunctionDecl_parameters (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record136 {
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

pub struct TableBuilder136 {
  db_url: String,
  records: Vec<Record136>,
}

impl TableBuilder136 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record136) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 98] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &record.c16,
        &record.c17,
        &record.c18,
        &(record.c19 as i64),
        &record.c20,
        &record.c21,
        &record.c22,
        &record.c23,
        &record.c24,
        &record.c25,
        &record.c26,
        &(record.c27 as i64),
        &record.c28,
        &record.c29,
        &record.c30,
        &record.c31,
        &record.c32,
        &record.c33,
        &record.c34,
        &record.c35,
        &record.c36,
        &record.c37,
        &record.c38,
        &record.c39,
        &record.c40,
        &record.c41,
        &(record.c42 as i64),
        &record.c43,
        &record.c44,
        &record.c45,
        &record.c46,
        &record.c47,
        &record.c48,
        &record.c49,
        &record.c50,
        &record.c51,
        &record.c52,
        &(record.c53 as i64),
        &record.c54,
        &record.c55,
        &record.c56,
        &record.c57,
        &(record.c58 as i64),
        &record.c59,
        &(record.c60 as i64),
        &record.c61,
        &record.c62,
        &record.c63,
        &record.c64,
        &record.c65,
        &record.c66,
        &(record.c67 as i64),
        &(record.c68 as i64),
        &(record.c69 as i64),
        &(record.c70 as i64),
        &(record.c71 as i64),
        &(record.c72 as i64),
        &(record.c73 as i64),
        &(record.c74 as i64),
        &record.c75,
        &record.c76,
        &record.c77,
        &record.c78,
        &record.c79,
        &record.c80,
        &record.c81,
        &record.c82,
        &(record.c83 as i64),
        &(record.c84 as i64),
        &(record.c85 as i64),
        &(record.c86 as i64),
        &(record.c87 as i64),
        &record.c88,
        &record.c89,
        &record.c90,
        &(record.c91 as i64),
        &(record.c92 as i64),
        &(record.c93 as i64),
        &(record.c94 as i64),
        &record.c95,
        &record.c96,
        &record.c97,
      ];
      conn.execute(
        "INSERT INTO FunctionDecl (id, getEllipsisLoc, getSourceRange, hasBody, hasTrivialBody, isDefined, getDefinition, getBody, isThisDeclarationADefinition, isThisDeclarationInstantiatedFromAFriendDefinition, doesThisDeclarationHaveABody, isVariadic, isVirtualAsWritten, isPureVirtual, isLateTemplateParsed, isTrivial, isTrivialForCall, isDefaulted, isExplicitlyDefaulted, getDefaultLoc, isUserProvided, isIneligibleOrNotSelected, hasImplicitReturnZero, hasPrototype, hasWrittenPrototype, hasInheritedPrototype, isConstexpr, getConstexprKind, isConstexprSpecified, isConsteval, BodyContainsImmediateEscalatingExpressions, isImmediateEscalating, isImmediateFunction, instantiationIsPending, usesSEHTry, isDeleted, isDeletedAsWritten, isMain, isMSVCRTEntryPoint, isReservedGlobalPlacementOperator, isInlineBuiltinDeclaration, isDestroyingOperatorDelete, getLanguageLinkage, isExternC, isInExternCContext, isInExternCXXContext, isGlobal, isNoReturn, hasSkippedBody, willHaveBody, isMultiVersion, FriendConstraintRefersToEnclosingTemplate, isMemberLikeConstrainedFriend, getMultiVersionKind, isCPUDispatchMultiVersion, isCPUSpecificMultiVersion, isTargetMultiVersion, isTargetClonesMultiVersion, getCanonicalDecl, param_empty, param_size, getNumParams, getMinRequiredArguments, getMinRequiredExplicitArguments, hasCXXExplicitFunctionObjectParameter, getNumNonObjectParams, hasOneParamOrDefaultArgs, getReturnType, getReturnTypeSourceRange, getParametersSourceRange, getDeclaredReturnType, getExceptionSpecType, getExceptionSpecSourceRange, getCallResultType, getStorageClass, isInlineSpecified, UsesFPIntrin, isInlined, isInlineDefinitionExternallyVisible, isMSExternInline, doesDeclarationForceExternallyVisibleDefinition, isStatic, isOverloadedOperator, getOverloadedOperator, getInstantiatedFromMemberFunction, getTemplatedKind, getInstantiatedFromDecl, getDescribedFunctionTemplate, isFunctionTemplateSpecialization, isImplicitlyInstantiable, isTemplateInstantiation, getPrimaryTemplate, getTemplateSpecializationKind, getTemplateSpecializationKindForInstantiation, getPointOfInstantiation, isOutOfLine, getMemoryFunctionKind, getODRHash) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, $55, $56, $57, $58, $59, $60, $61, $62, $63, $64, $65, $66, $67, $68, $69, $70, $71, $72, $73, $74, $75, $76, $77, $78, $79, $80, $81, $82, $83, $84, $85, $86, $87, $88, $89, $90, $91, $92, $93, $94, $95, $96, $97, $98)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ExportDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record137 {
  pub c0: u64, // id
  pub c1: u64, // getExportLoc
  pub c2: u64, // getRBraceLoc
  pub c3: bool, // hasBraces
  pub c4: u64, // getEndLoc
  pub c5: u64, // getSourceRange
}

pub struct TableBuilder137 {
  db_url: String,
  records: Vec<Record137>,
}

impl TableBuilder137 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record137) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO ExportDecl (id, getExportLoc, getRBraceLoc, hasBraces, getEndLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXRecordDecl_methods
#[derive(Debug, Serialize, Deserialize)]
pub struct Record138 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder138 {
  db_url: String,
  records: Vec<Record138>,
}

impl TableBuilder138 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record138) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXRecordDecl_methods (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXRecordDecl_ctors
#[derive(Debug, Serialize, Deserialize)]
pub struct Record139 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder139 {
  db_url: String,
  records: Vec<Record139>,
}

impl TableBuilder139 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record139) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXRecordDecl_ctors (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXRecordDecl_friends
#[derive(Debug, Serialize, Deserialize)]
pub struct Record140 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder140 {
  db_url: String,
  records: Vec<Record140>,
}

impl TableBuilder140 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record140) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXRecordDecl_friends (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXRecordDecl_getLambdaExplicitTemplateParameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record141 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder141 {
  db_url: String,
  records: Vec<Record141>,
}

impl TableBuilder141 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record141) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXRecordDecl_getLambdaExplicitTemplateParameters (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXRecordDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record142 {
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

pub struct TableBuilder142 {
  db_url: String,
  records: Vec<Record142>,
}

impl TableBuilder142 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record142) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 115] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &record.c16,
        &record.c17,
        &record.c18,
        &record.c19,
        &record.c20,
        &record.c21,
        &record.c22,
        &record.c23,
        &record.c24,
        &record.c25,
        &record.c26,
        &record.c27,
        &record.c28,
        &record.c29,
        &record.c30,
        &record.c31,
        &record.c32,
        &record.c33,
        &record.c34,
        &record.c35,
        &record.c36,
        &record.c37,
        &record.c38,
        &record.c39,
        &record.c40,
        &record.c41,
        &record.c42,
        &record.c43,
        &record.c44,
        &record.c45,
        &record.c46,
        &record.c47,
        &record.c48,
        &(record.c49 as i64),
        &(record.c50 as i64),
        &record.c51,
        &record.c52,
        &record.c53,
        &record.c54,
        &record.c55,
        &record.c56,
        &record.c57,
        &record.c58,
        &record.c59,
        &record.c60,
        &record.c61,
        &record.c62,
        &record.c63,
        &record.c64,
        &record.c65,
        &record.c66,
        &record.c67,
        &record.c68,
        &record.c69,
        &record.c70,
        &record.c71,
        &record.c72,
        &record.c73,
        &record.c74,
        &record.c75,
        &record.c76,
        &record.c77,
        &record.c78,
        &record.c79,
        &record.c80,
        &record.c81,
        &record.c82,
        &record.c83,
        &record.c84,
        &record.c85,
        &record.c86,
        &record.c87,
        &record.c88,
        &record.c89,
        &record.c90,
        &record.c91,
        &record.c92,
        &record.c93,
        &record.c94,
        &record.c95,
        &record.c96,
        &record.c97,
        &record.c98,
        &record.c99,
        &record.c100,
        &(record.c101 as i64),
        &(record.c102 as i64),
        &(record.c103 as i64),
        &(record.c104 as i64),
        &(record.c105 as i64),
        &record.c106,
        &(record.c107 as i64),
        &record.c108,
        &record.c109,
        &record.c110,
        &(record.c111 as i64),
        &record.c112,
        &record.c113,
        &record.c114,
      ];
      conn.execute(
        "INSERT INTO CXXRecordDecl (id, getCanonicalDecl, getPreviousDecl, getMostRecentDecl, getDefinition, hasDefinition, isDynamicClass, mayBeDynamicClass, mayBeNonDynamicClass, isParsingBaseSpecifiers, getODRHash, getNumBases, getNumVBases, hasAnyDependentBases, hasFriends, hasSimpleCopyConstructor, hasSimpleMoveConstructor, hasSimpleCopyAssignment, hasSimpleMoveAssignment, hasSimpleDestructor, hasDefaultConstructor, needsImplicitDefaultConstructor, hasUserDeclaredConstructor, hasUserProvidedDefaultConstructor, hasUserDeclaredCopyConstructor, needsImplicitCopyConstructor, needsOverloadResolutionForCopyConstructor, implicitCopyConstructorHasConstParam, hasCopyConstructorWithConstParam, hasUserDeclaredMoveOperation, hasUserDeclaredMoveConstructor, hasMoveConstructor, needsImplicitMoveConstructor, needsOverloadResolutionForMoveConstructor, hasUserDeclaredCopyAssignment, needsImplicitCopyAssignment, needsOverloadResolutionForCopyAssignment, implicitCopyAssignmentHasConstParam, hasCopyAssignmentWithConstParam, hasUserDeclaredMoveAssignment, hasMoveAssignment, needsImplicitMoveAssignment, needsOverloadResolutionForMoveAssignment, hasUserDeclaredDestructor, needsImplicitDestructor, needsOverloadResolutionForDestructor, isLambda, isGenericLambda, lambdaIsDefaultConstructibleAndAssignable, getLambdaCallOperator, getDependentLambdaCallOperator, isCapturelessLambda, isAggregate, hasInClassInitializer, hasUninitializedReferenceMember, isPOD, isCLike, isEmpty, hasInitMethod, hasPrivateFields, hasProtectedFields, hasDirectFields, isPolymorphic, isAbstract, isStandardLayout, isCXX11StandardLayout, hasMutableFields, hasVariantMembers, hasTrivialDefaultConstructor, hasNonTrivialDefaultConstructor, hasConstexprNonCopyMoveConstructor, defaultedDefaultConstructorIsConstexpr, hasConstexprDefaultConstructor, hasTrivialCopyConstructor, hasTrivialCopyConstructorForCall, hasNonTrivialCopyConstructor, hasNonTrivialCopyConstructorForCall, hasTrivialMoveConstructor, hasTrivialMoveConstructorForCall, hasNonTrivialMoveConstructor, hasNonTrivialMoveConstructorForCall, hasTrivialCopyAssignment, hasNonTrivialCopyAssignment, hasTrivialMoveAssignment, hasNonTrivialMoveAssignment, defaultedDestructorIsConstexpr, hasConstexprDestructor, hasTrivialDestructor, hasTrivialDestructorForCall, hasNonTrivialDestructor, hasNonTrivialDestructorForCall, allowConstDefaultInit, hasIrrelevantDestructor, hasNonLiteralTypeFieldsOrBases, hasInheritedConstructor, hasInheritedAssignment, isTriviallyCopyable, isTriviallyCopyConstructible, isTrivial, isLiteral, isStructural, getInstantiatedFromMemberClass, getDescribedClassTemplate, getTemplateSpecializationKind, getTemplateInstantiationPattern, getDestructor, isAnyDestructorNoReturn, isLocalClass, mayBeAbstract, isEffectivelyFinal, getDeviceLambdaManglingNumber, getMSVtorDispMode, isDependentLambda, isNeverDependentLambda, getLambdaDependencyKind) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, $55, $56, $57, $58, $59, $60, $61, $62, $63, $64, $65, $66, $67, $68, $69, $70, $71, $72, $73, $74, $75, $76, $77, $78, $79, $80, $81, $82, $83, $84, $85, $86, $87, $88, $89, $90, $91, $92, $93, $94, $95, $96, $97, $98, $99, $100, $101, $102, $103, $104, $105, $106, $107, $108, $109, $110, $111, $112, $113, $114, $115)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypeAliasDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record143 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getDescribedAliasTemplate
}

pub struct TableBuilder143 {
  db_url: String,
  records: Vec<Record143>,
}

impl TableBuilder143 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record143) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO TypeAliasDecl (id, getSourceRange, getDescribedAliasTemplate) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DeclaratorDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record144 {
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

pub struct TableBuilder144 {
  db_url: String,
  records: Vec<Record144>,
}

impl TableBuilder144 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record144) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
        &(record.c7 as i64),
        &(record.c8 as i64),
      ];
      conn.execute(
        "INSERT INTO DeclaratorDecl (id, getInnerLocStart, getOuterLocStart, getSourceRange, getBeginLoc, getTrailingRequiresClause, getNumTemplateParameterLists, getTypeSpecStartLoc, getTypeSpecEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ExternCContextDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record145 {
  pub c0: u64, // id
}

pub struct TableBuilder145 {
  db_url: String,
  records: Vec<Record145>,
}

impl TableBuilder145 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record145) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO ExternCContextDecl (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ParmVarDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record146 {
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

pub struct TableBuilder146 {
  db_url: String,
  records: Vec<Record146>,
}

impl TableBuilder146 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record146) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 18] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &record.c7,
        &record.c8,
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &(record.c12 as i64),
        &record.c13,
        &record.c14,
        &record.c15,
        &record.c16,
        &(record.c17 as i64),
      ];
      conn.execute(
        "INSERT INTO ParmVarDecl (id, getSourceRange, isObjCMethodParameter, isDestroyedInCallee, getFunctionScopeDepth, getFunctionScopeIndex, getObjCDeclQualifier, isKNRPromoted, isExplicitObjectParameter, getExplicitObjectParamThisLoc, getDefaultArg, getDefaultArgRange, getUninstantiatedDefaultArg, hasDefaultArg, hasUnparsedDefaultArg, hasUninstantiatedDefaultArg, hasInheritedDefaultArg, getOriginalType) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// NonTypeTemplateParmDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record147 {
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

pub struct TableBuilder147 {
  db_url: String,
  records: Vec<Record147>,
}

impl TableBuilder147 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record147) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 11] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &(record.c9 as i64),
        &record.c10,
      ];
      conn.execute(
        "INSERT INTO NonTypeTemplateParmDecl (id, getSourceRange, hasDefaultArgument, getDefaultArgument, getDefaultArgumentLoc, defaultArgumentWasInherited, isParameterPack, isPackExpansion, isExpandedParameterPack, getPlaceholderTypeConstraint, hasPlaceholderTypeConstraint) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSPropertyDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record148 {
  pub c0: u64, // id
  pub c1: bool, // hasGetter
  pub c2: bool, // hasSetter
}

pub struct TableBuilder148 {
  db_url: String,
  records: Vec<Record148>,
}

impl TableBuilder148 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record148) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
      ];
      conn.execute(
        "INSERT INTO MSPropertyDecl (id, hasGetter, hasSetter) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TranslationUnitDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record149 {
  pub c0: u64, // id
  pub c1: u64, // getAnonymousNamespace
}

pub struct TableBuilder149 {
  db_url: String,
  records: Vec<Record149>,
}

impl TableBuilder149 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record149) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO TranslationUnitDecl (id, getAnonymousNamespace) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// LabelDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record150 {
  pub c0: u64, // id
  pub c1: u64, // getStmt
  pub c2: bool, // isGnuLocal
  pub c3: u64, // getSourceRange
  pub c4: bool, // isMSAsmLabel
  pub c5: bool, // isResolvedMSAsmLabel
  pub c6: String, // getMSAsmLabel
}

pub struct TableBuilder150 {
  db_url: String,
  records: Vec<Record150>,
}

impl TableBuilder150 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record150) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &record.c4,
        &record.c5,
        &record.c6,
      ];
      conn.execute(
        "INSERT INTO LabelDecl (id, getStmt, isGnuLocal, getSourceRange, isMSAsmLabel, isResolvedMSAsmLabel, getMSAsmLabel) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// VarTemplatePartialSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record151 {
  pub c0: u64, // id
  pub c1: bool, // hasAssociatedConstraints
  pub c2: u64, // getInstantiatedFromMember
  pub c3: u64, // getSourceRange
}

pub struct TableBuilder151 {
  db_url: String,
  records: Vec<Record151>,
}

impl TableBuilder151 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record151) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO VarTemplatePartialSpecializationDecl (id, hasAssociatedConstraints, getInstantiatedFromMember, getSourceRange) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// IndirectFieldDecl_chain
#[derive(Debug, Serialize, Deserialize)]
pub struct Record152 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder152 {
  db_url: String,
  records: Vec<Record152>,
}

impl TableBuilder152 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record152) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO IndirectFieldDecl_chain (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// IndirectFieldDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record153 {
  pub c0: u64, // id
  pub c1: u32, // getChainingSize
  pub c2: u64, // getAnonField
  pub c3: u64, // getVarDecl
  pub c4: u64, // getCanonicalDecl
}

pub struct TableBuilder153 {
  db_url: String,
  records: Vec<Record153>,
}

impl TableBuilder153 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record153) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO IndirectFieldDecl (id, getChainingSize, getAnonField, getVarDecl, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// EnumConstantDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record154 {
  pub c0: u64, // id
  pub c1: u64, // getInitExpr
  pub c2: u64, // getSourceRange
  pub c3: u64, // getCanonicalDecl
}

pub struct TableBuilder154 {
  db_url: String,
  records: Vec<Record154>,
}

impl TableBuilder154 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record154) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO EnumConstantDecl (id, getInitExpr, getSourceRange, getCanonicalDecl) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FieldDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record155 {
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

pub struct TableBuilder155 {
  db_url: String,
  records: Vec<Record155>,
}

impl TableBuilder155 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record155) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 17] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &record.c7,
        &(record.c8 as i64),
        &record.c9,
        &record.c10,
        &(record.c11 as i64),
        &record.c12,
        &(record.c13 as i64),
        &(record.c14 as i64),
        &(record.c15 as i64),
        &(record.c16 as i64),
      ];
      conn.execute(
        "INSERT INTO FieldDecl (id, getFieldIndex, isMutable, isBitField, isUnnamedBitfield, isAnonymousStructOrUnion, getBitWidth, isPotentiallyOverlapping, getInClassInitStyle, hasInClassInitializer, hasNonNullInClassInitializer, getInClassInitializer, hasCapturedVLAType, getCapturedVLAType, getParent, getSourceRange, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// EnumDecl_enumerators
#[derive(Debug, Serialize, Deserialize)]
pub struct Record156 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder156 {
  db_url: String,
  records: Vec<Record156>,
}

impl TableBuilder156 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record156) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO EnumDecl_enumerators (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// EnumDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record157 {
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

pub struct TableBuilder157 {
  db_url: String,
  records: Vec<Record157>,
}

impl TableBuilder157 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record157) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 21] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &record.c16,
        &record.c17,
        &(record.c18 as i64),
        &(record.c19 as i64),
        &(record.c20 as i64),
      ];
      conn.execute(
        "INSERT INTO EnumDecl (id, getCanonicalDecl, getPreviousDecl, getMostRecentDecl, getDefinition, getSourceRange, getPromotionType, getIntegerType, getIntegerTypeRange, getNumPositiveBits, getNumNegativeBits, isScoped, isScopedUsingClassTag, isFixed, isComplete, isClosed, isClosedFlag, isClosedNonFlag, getTemplateInstantiationPattern, getInstantiatedFromMemberEnum, getTemplateSpecializationKind) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXConversionDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record158 {
  pub c0: u64, // id
  pub c1: bool, // isExplicit
  pub c2: u64, // getConversionType
  pub c3: bool, // isLambdaToBlockPointerConversion
  pub c4: u64, // getCanonicalDecl
}

pub struct TableBuilder158 {
  db_url: String,
  records: Vec<Record158>,
}

impl TableBuilder158 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record158) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXConversionDecl (id, isExplicit, getConversionType, isLambdaToBlockPointerConversion, getCanonicalDecl) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// Decl_attrs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record159 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder159 {
  db_url: String,
  records: Vec<Record159>,
}

impl TableBuilder159 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record159) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO Decl_attrs (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// Decl_redecls
#[derive(Debug, Serialize, Deserialize)]
pub struct Record160 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder160 {
  db_url: String,
  records: Vec<Record160>,
}

impl TableBuilder160 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record160) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO Decl_redecls (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// Decl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record161 {
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

pub struct TableBuilder161 {
  db_url: String,
  records: Vec<Record161>,
}

impl TableBuilder161 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record161) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 60] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &record.c8,
        &record.c9,
        &record.c10,
        &(record.c11 as i64),
        &(record.c12 as i64),
        &record.c13,
        &record.c14,
        &record.c15,
        &record.c16,
        &record.c17,
        &record.c18,
        &record.c19,
        &record.c20,
        &record.c21,
        &record.c22,
        &record.c23,
        &record.c24,
        &record.c25,
        &(record.c26 as i64),
        &record.c27,
        &record.c28,
        &record.c29,
        &record.c30,
        &record.c31,
        &record.c32,
        &record.c33,
        &(record.c34 as i64),
        &record.c35,
        &record.c36,
        &record.c37,
        &record.c38,
        &record.c39,
        &record.c40,
        &(record.c41 as i64),
        &record.c42,
        &(record.c43 as i64),
        &record.c44,
        &(record.c45 as i64),
        &(record.c46 as i64),
        &record.c47,
        &(record.c48 as i64),
        &record.c49,
        &record.c50,
        &record.c51,
        &record.c52,
        &record.c53,
        &(record.c54 as i64),
        &(record.c55 as i64),
        &record.c56,
        &(record.c57 as i64),
        &record.c58,
        &record.c59,
      ];
      conn.execute(
        "INSERT INTO Decl (id, getSourceRange, getBeginLoc, getEndLoc, getLocation, getNextDeclInContext, getNonClosureContext, getTranslationUnitDecl, isInAnonymousNamespace, isInStdNamespace, isFileContextDecl, getAccess, getAccessUnsafe, hasAttrs, isInvalidDecl, isImplicit, isReferenced, isThisDeclarationReferenced, isTopLevelDeclInObjCContainer, isModulePrivate, isInExportDeclContext, isInvisibleOutsideTheOwningModule, isInAnotherModuleUnit, isDiscardedInGlobalModuleFragment, shouldSkipCheckingODR, hasDefiningAttr, getDefiningAttr, isWeakImported, isFromASTFile, getGlobalID, getOwningModuleID, hasOwningModule, isUnconditionallyVisible, isReachable, getModuleOwnershipKind, getIdentifierNamespace, hasTagIdentifierNamespace, isOutOfLine, isTemplated, getTemplateDepth, isDefinedOutsideFunctionOrMethod, getCanonicalDecl, isCanonicalDecl, getPreviousDecl, isFirstDecl, getMostRecentDecl, getBody, hasBody, getBodyRBrace, isTemplateParameter, isTemplateParameterPack, isParameterPack, isTemplateDecl, isFunctionOrFunctionTemplate, getDescribedTemplate, getAsFunction, isLocalExternDecl, getFriendObjectKind, getID, isFunctionPointerType) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, $55, $56, $57, $58, $59, $60)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnresolvedUsingIfExistsDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record162 {
  pub c0: u64, // id
}

pub struct TableBuilder162 {
  db_url: String,
  records: Vec<Record162>,
}

impl TableBuilder162 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record162) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO UnresolvedUsingIfExistsDecl (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TemplateTypeParmDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record163 {
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

pub struct TableBuilder163 {
  db_url: String,
  records: Vec<Record163>,
}

impl TableBuilder163 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record163) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 12] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &(record.c3 as i64),
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &(record.c11 as i64),
      ];
      conn.execute(
        "INSERT INTO TemplateTypeParmDecl (id, wasDeclaredWithTypename, hasDefaultArgument, getDefaultArgumentLoc, defaultArgumentWasInherited, getDepth, getIndex, isParameterPack, isPackExpansion, isExpandedParameterPack, hasTypeConstraint, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DecompositionDecl_bindings
#[derive(Debug, Serialize, Deserialize)]
pub struct Record164 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder164 {
  db_url: String,
  records: Vec<Record164>,
}

impl TableBuilder164 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record164) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO DecompositionDecl_bindings (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DecompositionDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record165 {
  pub c0: u64, // id
}

pub struct TableBuilder165 {
  db_url: String,
  records: Vec<Record165>,
}

impl TableBuilder165 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record165) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO DecompositionDecl (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// VarTemplateSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record166 {
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

pub struct TableBuilder166 {
  db_url: String,
  records: Vec<Record166>,
}

impl TableBuilder166 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record166) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO VarTemplateSpecializationDecl (id, getSpecializedTemplate, getSpecializationKind, isExplicitSpecialization, isClassScopeExplicitSpecialization, isExplicitInstantiationOrSpecialization, getPointOfInstantiation, getExternLoc, getTemplateKeywordLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// LifetimeExtendedTemporaryDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record167 {
  pub c0: u64, // id
  pub c1: u64, // getExtendingDecl
  pub c2: u64, // getStorageDuration
  pub c3: u64, // getTemporaryExpr
  pub c4: u32, // getManglingNumber
}

pub struct TableBuilder167 {
  db_url: String,
  records: Vec<Record167>,
}

impl TableBuilder167 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record167) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO LifetimeExtendedTemporaryDecl (id, getExtendingDecl, getStorageDuration, getTemporaryExpr, getManglingNumber) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ValueDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record168 {
  pub c0: u64, // id
  pub c1: u64, // getType
  pub c2: bool, // isWeak
  pub c3: bool, // isInitCapture
  pub c4: u64, // getPotentiallyDecomposedVarDecl
}

pub struct TableBuilder168 {
  db_url: String,
  records: Vec<Record168>,
}

impl TableBuilder168 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record168) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO ValueDecl (id, getType, isWeak, isInitCapture, getPotentiallyDecomposedVarDecl) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CapturedDecl_parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record169 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder169 {
  db_url: String,
  records: Vec<Record169>,
}

impl TableBuilder169 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record169) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CapturedDecl_parameters (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CapturedDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record170 {
  pub c0: u64, // id
  pub c1: u64, // getBody
  pub c2: bool, // isNothrow
  pub c3: u32, // getNumParams
  pub c4: u64, // getContextParam
  pub c5: u32, // getContextParamPosition
}

pub struct TableBuilder170 {
  db_url: String,
  records: Vec<Record170>,
}

impl TableBuilder170 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record170) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO CapturedDecl (id, getBody, isNothrow, getNumParams, getContextParam, getContextParamPosition) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// VarDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record171 {
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

pub struct TableBuilder171 {
  db_url: String,
  records: Vec<Record171>,
}

impl TableBuilder171 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record171) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 54] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &(record.c9 as i64),
        &(record.c10 as i64),
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &record.c16,
        &record.c17,
        &(record.c18 as i64),
        &(record.c19 as i64),
        &(record.c20 as i64),
        &(record.c21 as i64),
        &(record.c22 as i64),
        &record.c23,
        &record.c24,
        &(record.c25 as i64),
        &record.c26,
        &(record.c27 as i64),
        &(record.c28 as i64),
        &record.c29,
        &(record.c30 as i64),
        &record.c31,
        &record.c32,
        &record.c33,
        &record.c34,
        &record.c35,
        &record.c36,
        &record.c37,
        &record.c38,
        &record.c39,
        &record.c40,
        &record.c41,
        &record.c42,
        &record.c43,
        &record.c44,
        &record.c45,
        &record.c46,
        &(record.c47 as i64),
        &(record.c48 as i64),
        &(record.c49 as i64),
        &(record.c50 as i64),
        &(record.c51 as i64),
        &(record.c52 as i64),
        &record.c53,
      ];
      conn.execute(
        "INSERT INTO VarDecl (id, getSourceRange, getStorageClass, getTSCSpec, getTLSKind, hasLocalStorage, isStaticLocal, hasExternalStorage, hasGlobalStorage, getStorageDuration, getLanguageLinkage, isExternC, isInExternCContext, isInExternCXXContext, isLocalVarDecl, isLocalVarDeclOrParm, isFunctionOrMethodVarDecl, isStaticDataMember, getCanonicalDecl, isThisDeclarationADefinition, hasDefinition, getActingDefinition, getDefinition, isOutOfLine, isFileVarDecl, getAnyInitializer, hasInit, getInit, getInitializingDeclaration, hasConstantInitialization, getInitStyle, isDirectInit, isThisDeclarationADemotedDefinition, isExceptionVariable, isNRVOVariable, isCXXForRangeDecl, isObjCForDecl, isARCPseudoStrong, isInline, isInlineSpecified, isConstexpr, isInitCapture, isParameterPack, isPreviousDeclInSameBlockScope, isEscapingByref, isNonEscapingByref, hasDependentAlignment, getTemplateInstantiationPattern, getInstantiatedFromStaticDataMember, getTemplateSpecializationKind, getTemplateSpecializationKindForInstantiation, getPointOfInstantiation, getDescribedVarTemplate, isKnownToBeDefined) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXDestructorDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record172 {
  pub c0: u64, // id
  pub c1: u64, // getOperatorDelete
  pub c2: u64, // getOperatorDeleteThisArg
  pub c3: u64, // getCanonicalDecl
}

pub struct TableBuilder172 {
  db_url: String,
  records: Vec<Record172>,
}

impl TableBuilder172 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record172) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXDestructorDecl (id, getOperatorDelete, getOperatorDeleteThisArg, getCanonicalDecl) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ImportDecl_getIdentifierLocs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record173 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder173 {
  db_url: String,
  records: Vec<Record173>,
}

impl TableBuilder173 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record173) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO ImportDecl_getIdentifierLocs (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ImportDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record174 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
}

pub struct TableBuilder174 {
  db_url: String,
  records: Vec<Record174>,
}

impl TableBuilder174 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record174) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO ImportDecl (id, getSourceRange) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FriendTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record175 {
  pub c0: u64, // id
  pub c1: u64, // getFriendDecl
  pub c2: u64, // getFriendLoc
  pub c3: u32, // getNumTemplateParameters
}

pub struct TableBuilder175 {
  db_url: String,
  records: Vec<Record175>,
}

impl TableBuilder175 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record175) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
      ];
      conn.execute(
        "INSERT INTO FriendTemplateDecl (id, getFriendDecl, getFriendLoc, getNumTemplateParameters) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXDeductionGuideDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record176 {
  pub c0: u64, // id
  pub c1: bool, // isExplicit
  pub c2: u64, // getDeducedTemplate
  pub c3: u64, // getCorrespondingConstructor
  pub c4: u64, // getDeductionCandidateKind
}

pub struct TableBuilder176 {
  db_url: String,
  records: Vec<Record176>,
}

impl TableBuilder176 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record176) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXDeductionGuideDecl (id, isExplicit, getDeducedTemplate, getCorrespondingConstructor, getDeductionCandidateKind) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BlockDecl_parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record177 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder177 {
  db_url: String,
  records: Vec<Record177>,
}

impl TableBuilder177 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record177) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO BlockDecl_parameters (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BlockDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record178 {
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

pub struct TableBuilder178 {
  db_url: String,
  records: Vec<Record178>,
}

impl TableBuilder178 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record178) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 18] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &(record.c16 as i64),
        &(record.c17 as i64),
      ];
      conn.execute(
        "INSERT INTO BlockDecl (id, getCaretLocation, isVariadic, getCompoundBody, getBody, param_empty, param_size, getNumParams, hasCaptures, getNumCaptures, capturesCXXThis, blockMissingReturnType, isConversionFromLambda, doesNotEscape, canAvoidCopyToHeap, getBlockManglingNumber, getBlockManglingContextDecl, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// NamedDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record179 {
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

pub struct TableBuilder179 {
  db_url: String,
  records: Vec<Record179>,
}

impl TableBuilder179 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record179) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 16] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &(record.c13 as i64),
        &(record.c14 as i64),
        &(record.c15 as i64),
      ];
      conn.execute(
        "INSERT INTO NamedDecl (id, getNameAsString, getQualifiedNameAsString, hasLinkage, isCXXClassMember, isCXXInstanceMember, getLinkageInternal, getFormalLinkage, hasExternalFormalLinkage, isExternallyVisible, isExternallyDeclarable, isLinkageValid, hasLinkageBeenComputed, getUnderlyingDecl, getMostRecentDecl, getObjCFStringFormattingFamily) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ImplicitParamDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record180 {
  pub c0: u64, // id
  pub c1: u64, // getParameterKind
}

pub struct TableBuilder180 {
  db_url: String,
  records: Vec<Record180>,
}

impl TableBuilder180 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record180) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO ImplicitParamDecl (id, getParameterKind) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FileScopeAsmDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record181 {
  pub c0: u64, // id
  pub c1: u64, // getAsmLoc
  pub c2: u64, // getRParenLoc
  pub c3: u64, // getSourceRange
  pub c4: u64, // getAsmString
}

pub struct TableBuilder181 {
  db_url: String,
  records: Vec<Record181>,
}

impl TableBuilder181 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record181) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO FileScopeAsmDecl (id, getAsmLoc, getRParenLoc, getSourceRange, getAsmString) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// HLSLBufferDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record182 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getLocStart
  pub c3: u64, // getLBraceLoc
  pub c4: u64, // getRBraceLoc
  pub c5: bool, // isCBuffer
}

pub struct TableBuilder182 {
  db_url: String,
  records: Vec<Record182>,
}

impl TableBuilder182 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record182) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO HLSLBufferDecl (id, getSourceRange, getLocStart, getLBraceLoc, getRBraceLoc, isCBuffer) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AccessSpecDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record183 {
  pub c0: u64, // id
  pub c1: u64, // getAccessSpecifierLoc
  pub c2: u64, // getColonLoc
  pub c3: u64, // getSourceRange
}

pub struct TableBuilder183 {
  db_url: String,
  records: Vec<Record183>,
}

impl TableBuilder183 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record183) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO AccessSpecDecl (id, getAccessSpecifierLoc, getColonLoc, getSourceRange) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FriendDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record184 {
  pub c0: u64, // id
  pub c1: u32, // getFriendTypeNumTemplateParameterLists
  pub c2: u64, // getFriendDecl
  pub c3: u64, // getFriendLoc
  pub c4: u64, // getSourceRange
  pub c5: bool, // isUnsupportedFriend
}

pub struct TableBuilder184 {
  db_url: String,
  records: Vec<Record184>,
}

impl TableBuilder184 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record184) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO FriendDecl (id, getFriendTypeNumTemplateParameterLists, getFriendDecl, getFriendLoc, getSourceRange, isUnsupportedFriend) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RedeclarableTemplateDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record185 {
  pub c0: u64, // id
  pub c1: u64, // getCanonicalDecl
  pub c2: bool, // isMemberSpecialization
  pub c3: u64, // getInstantiatedFromMemberTemplate
}

pub struct TableBuilder185 {
  db_url: String,
  records: Vec<Record185>,
}

impl TableBuilder185 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record185) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO RedeclarableTemplateDecl (id, getCanonicalDecl, isMemberSpecialization, getInstantiatedFromMemberTemplate) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ImplicitConceptSpecializationDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record186 {
  pub c0: u64, // id
}

pub struct TableBuilder186 {
  db_url: String,
  records: Vec<Record186>,
}

impl TableBuilder186 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record186) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO ImplicitConceptSpecializationDecl (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// StaticAssertDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record187 {
  pub c0: u64, // id
  pub c1: u64, // getAssertExpr
  pub c2: u64, // getMessage
  pub c3: bool, // isFailed
  pub c4: u64, // getRParenLoc
  pub c5: u64, // getSourceRange
}

pub struct TableBuilder187 {
  db_url: String,
  records: Vec<Record187>,
}

impl TableBuilder187 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record187) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO StaticAssertDecl (id, getAssertExpr, getMessage, isFailed, getRParenLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TopLevelStmtDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record188 {
  pub c0: u64, // id
  pub c1: u64, // getSourceRange
  pub c2: u64, // getStmt
  pub c3: bool, // isSemiMissing
}

pub struct TableBuilder188 {
  db_url: String,
  records: Vec<Record188>,
}

impl TableBuilder188 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record188) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
      ];
      conn.execute(
        "INSERT INTO TopLevelStmtDecl (id, getSourceRange, getStmt, isSemiMissing) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// LinkageSpecDecl
#[derive(Debug, Serialize, Deserialize)]
pub struct Record189 {
  pub c0: u64, // id
  pub c1: u64, // getLanguage
  pub c2: bool, // hasBraces
  pub c3: u64, // getExternLoc
  pub c4: u64, // getRBraceLoc
  pub c5: u64, // getEndLoc
  pub c6: u64, // getSourceRange
}

pub struct TableBuilder189 {
  db_url: String,
  records: Vec<Record189>,
}

impl TableBuilder189 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record189) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO LinkageSpecDecl (id, getLanguage, hasBraces, getExternLoc, getRBraceLoc, getEndLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FullExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record190 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
}

pub struct TableBuilder190 {
  db_url: String,
  records: Vec<Record190>,
}

impl TableBuilder190 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record190) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO FullExpr (id, getSubExpr) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ContinueStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record191 {
  pub c0: u64, // id
  pub c1: u64, // getContinueLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

pub struct TableBuilder191 {
  db_url: String,
  records: Vec<Record191>,
}

impl TableBuilder191 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record191) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO ContinueStmt (id, getContinueLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXTemporaryObjectExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record192 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

pub struct TableBuilder192 {
  db_url: String,
  records: Vec<Record192>,
}

impl TableBuilder192 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record192) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXTemporaryObjectExpr (id, getBeginLoc, getEndLoc) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ForStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record193 {
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

pub struct TableBuilder193 {
  db_url: String,
  records: Vec<Record193>,
}

impl TableBuilder193 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record193) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 12] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
      ];
      conn.execute(
        "INSERT INTO ForStmt (id, getConditionVariable, getConditionVariableDeclStmt, getInit, getCond, getInc, getBody, getForLoc, getLParenLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXDefaultInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record194 {
  pub c0: u64, // id
  pub c1: bool, // hasRewrittenInit
  pub c2: u64, // getField
  pub c3: u64, // getExpr
  pub c4: u64, // getRewrittenExpr
  pub c5: u64, // getUsedLocation
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder194 {
  db_url: String,
  records: Vec<Record194>,
}

impl TableBuilder194 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record194) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXDefaultInitExpr (id, hasRewrittenInit, getField, getExpr, getRewrittenExpr, getUsedLocation, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXConstructExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record195 {
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

pub struct TableBuilder195 {
  db_url: String,
  records: Vec<Record195>,
}

impl TableBuilder195 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record195) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 14] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &(record.c8 as i64),
        &record.c9,
        &record.c10,
        &(record.c11 as i64),
        &(record.c12 as i64),
        &(record.c13 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXConstructExpr (id, getConstructor, getLocation, isElidable, hadMultipleCandidates, isListInitialization, isStdInitListInitialization, requiresZeroInitialization, getConstructionKind, getNumArgs, isImmediateEscalating, getBeginLoc, getEndLoc, getParenOrBraceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// Expr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record196 {
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

pub struct TableBuilder196 {
  db_url: String,
  records: Vec<Record196>,
}

impl TableBuilder196 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record196) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 36] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &(record.c12 as i64),
        &(record.c13 as i64),
        &record.c14,
        &record.c15,
        &(record.c16 as i64),
        &(record.c17 as i64),
        &(record.c18 as i64),
        &record.c19,
        &record.c20,
        &record.c21,
        &record.c22,
        &(record.c23 as i64),
        &(record.c24 as i64),
        &(record.c25 as i64),
        &(record.c26 as i64),
        &(record.c27 as i64),
        &(record.c28 as i64),
        &(record.c29 as i64),
        &(record.c30 as i64),
        &(record.c31 as i64),
        &(record.c32 as i64),
        &record.c33,
        &record.c34,
        &(record.c35 as i64),
      ];
      conn.execute(
        "INSERT INTO Expr (id, getType, getDependence, isValueDependent, isTypeDependent, isInstantiationDependent, containsUnexpandedParameterPack, containsErrors, isLValue, isPRValue, isXValue, isGLValue, getValueKind, getObjectKind, isOrdinaryOrBitFieldObject, refersToBitField, getSourceBitField, getReferencedDeclOfCallee, getObjCProperty, isObjCSelfExpr, refersToVectorElement, refersToMatrixElement, refersToGlobalRegisterVar, IgnoreImpCasts, IgnoreCasts, IgnoreImplicit, IgnoreImplicitAsWritten, IgnoreParens, IgnoreParenImpCasts, IgnoreParenCasts, IgnoreConversionOperatorSingleStep, IgnoreParenLValueCasts, IgnoreParenBaseCasts, isDefaultArgument, isImplicitCXXThis, skipRValueSubobjectAdjustments) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// WhileStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record197 {
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

pub struct TableBuilder197 {
  db_url: String,
  records: Vec<Record197>,
}

impl TableBuilder197 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record197) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 11] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
      ];
      conn.execute(
        "INSERT INTO WhileStmt (id, hasVarStorage, getCond, getBody, getConditionVariable, getConditionVariableDeclStmt, getWhileLoc, getLParenLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ValueStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record198 {
  pub c0: u64, // id
  pub c1: u64, // getExprStmt
}

pub struct TableBuilder198 {
  db_url: String,
  records: Vec<Record198>,
}

impl TableBuilder198 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record198) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO ValueStmt (id, getExprStmt) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// GotoStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record199 {
  pub c0: u64, // id
  pub c1: u64, // getLabel
  pub c2: u64, // getGotoLoc
  pub c3: u64, // getLabelLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder199 {
  db_url: String,
  records: Vec<Record199>,
}

impl TableBuilder199 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record199) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO GotoStmt (id, getLabel, getGotoLoc, getLabelLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BreakStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record200 {
  pub c0: u64, // id
  pub c1: u64, // getBreakLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

pub struct TableBuilder200 {
  db_url: String,
  records: Vec<Record200>,
}

impl TableBuilder200 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record200) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO BreakStmt (id, getBreakLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CharacterLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record201 {
  pub c0: u64, // id
  pub c1: u64, // getLocation
  pub c2: u64, // getKind
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u32, // getValue
}

pub struct TableBuilder201 {
  db_url: String,
  records: Vec<Record201>,
}

impl TableBuilder201 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record201) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO CharacterLiteral (id, getLocation, getKind, getBeginLoc, getEndLoc, getValue) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// GCCAsmStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record202 {
  pub c0: u64, // id
  pub c1: u64, // getRParenLoc
  pub c2: u64, // getAsmString
  pub c3: bool, // isAsmGoto
  pub c4: u32, // getNumLabels
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder202 {
  db_url: String,
  records: Vec<Record202>,
}

impl TableBuilder202 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record202) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO GCCAsmStmt (id, getRParenLoc, getAsmString, isAsmGoto, getNumLabels, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXCatchStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record203 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getCatchLoc
  pub c4: u64, // getExceptionDecl
  pub c5: u64, // getCaughtType
  pub c6: u64, // getHandlerBlock
}

pub struct TableBuilder203 {
  db_url: String,
  records: Vec<Record203>,
}

impl TableBuilder203 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record203) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXCatchStmt (id, getBeginLoc, getEndLoc, getCatchLoc, getExceptionDecl, getCaughtType, getHandlerBlock) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SEHExceptStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record204 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getExceptLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getFilterExpr
  pub c5: u64, // getBlock
}

pub struct TableBuilder204 {
  db_url: String,
  records: Vec<Record204>,
}

impl TableBuilder204 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record204) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO SEHExceptStmt (id, getBeginLoc, getExceptLoc, getEndLoc, getFilterExpr, getBlock) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BuiltinBitCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record205 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

pub struct TableBuilder205 {
  db_url: String,
  records: Vec<Record205>,
}

impl TableBuilder205 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record205) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO BuiltinBitCastExpr (id, getBeginLoc, getEndLoc) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXNamedCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record206 {
  pub c0: u64, // id
  pub c1: u64, // getOperatorLoc
  pub c2: u64, // getRParenLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u64, // getAngleBrackets
}

pub struct TableBuilder206 {
  db_url: String,
  records: Vec<Record206>,
}

impl TableBuilder206 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record206) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXNamedCastExpr (id, getOperatorLoc, getRParenLoc, getBeginLoc, getEndLoc, getAngleBrackets) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CapturedStmt_capture_inits
#[derive(Debug, Serialize, Deserialize)]
pub struct Record207 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder207 {
  db_url: String,
  records: Vec<Record207>,
}

impl TableBuilder207 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record207) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CapturedStmt_capture_inits (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CapturedStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record208 {
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

pub struct TableBuilder208 {
  db_url: String,
  records: Vec<Record208>,
}

impl TableBuilder208 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record208) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
      ];
      conn.execute(
        "INSERT INTO CapturedStmt (id, getCapturedStmt, getCapturedDecl, getCapturedRegionKind, getCapturedRecordDecl, capture_size, getBeginLoc, getEndLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXForRangeStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record209 {
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

pub struct TableBuilder209 {
  db_url: String,
  records: Vec<Record209>,
}

impl TableBuilder209 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record209) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 17] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &(record.c12 as i64),
        &(record.c13 as i64),
        &(record.c14 as i64),
        &(record.c15 as i64),
        &(record.c16 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXForRangeStmt (id, getInit, getLoopVariable, getRangeInit, getRangeStmt, getBeginStmt, getEndStmt, getCond, getInc, getLoopVarStmt, getBody, getForLoc, getCoawaitLoc, getColonLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXNewExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record210 {
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

pub struct TableBuilder210 {
  db_url: String,
  records: Vec<Record210>,
}

impl TableBuilder210 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record210) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 19] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &record.c5,
        &record.c6,
        &(record.c7 as i64),
        &record.c8,
        &record.c9,
        &(record.c10 as i64),
        &(record.c11 as i64),
        &(record.c12 as i64),
        &record.c13,
        &record.c14,
        &(record.c15 as i64),
        &(record.c16 as i64),
        &(record.c17 as i64),
        &(record.c18 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXNewExpr (id, getAllocatedType, getOperatorNew, getOperatorDelete, isArray, getNumPlacementArgs, isParenTypeId, getTypeIdParens, isGlobalNew, hasInitializer, getInitializationStyle, getInitializer, getConstructExpr, passAlignment, doesUsualArrayDeleteWantSize, getBeginLoc, getEndLoc, getDirectInitRange, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CoroutineBodyStmt_getParamMoves
#[derive(Debug, Serialize, Deserialize)]
pub struct Record211 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder211 {
  db_url: String,
  records: Vec<Record211>,
}

impl TableBuilder211 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record211) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CoroutineBodyStmt_getParamMoves (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CoroutineBodyStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record212 {
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

pub struct TableBuilder212 {
  db_url: String,
  records: Vec<Record212>,
}

impl TableBuilder212 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record212) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 18] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &(record.c12 as i64),
        &(record.c13 as i64),
        &(record.c14 as i64),
        &(record.c15 as i64),
        &(record.c16 as i64),
        &(record.c17 as i64),
      ];
      conn.execute(
        "INSERT INTO CoroutineBodyStmt (id, hasDependentPromiseType, getBody, getPromiseDeclStmt, getPromiseDecl, getInitSuspendStmt, getFinalSuspendStmt, getExceptionHandler, getFallthroughHandler, getAllocate, getDeallocate, getResultDecl, getReturnValueInit, getReturnValue, getReturnStmt, getReturnStmtOnAllocFailure, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ParenListExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record213 {
  pub c0: u64, // id
  pub c1: u32, // getNumExprs
  pub c2: u64, // getLParenLoc
  pub c3: u64, // getRParenLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder213 {
  db_url: String,
  records: Vec<Record213>,
}

impl TableBuilder213 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record213) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO ParenListExpr (id, getNumExprs, getLParenLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXScalarValueInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record214 {
  pub c0: u64, // id
  pub c1: u64, // getRParenLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

pub struct TableBuilder214 {
  db_url: String,
  records: Vec<Record214>,
}

impl TableBuilder214 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record214) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXScalarValueInitExpr (id, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSPropertyRefExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record215 {
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

pub struct TableBuilder215 {
  db_url: String,
  records: Vec<Record215>,
}

impl TableBuilder215 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record215) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &record.c7,
        &(record.c8 as i64),
      ];
      conn.execute(
        "INSERT INTO MSPropertyRefExpr (id, getSourceRange, isImplicitAccess, getBeginLoc, getEndLoc, getBaseExpr, getPropertyDecl, isArrow, getMemberLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSAsmStmt_getAllConstraints
#[derive(Debug, Serialize, Deserialize)]
pub struct Record216 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: String, // element
}

pub struct TableBuilder216 {
  db_url: String,
  records: Vec<Record216>,
}

impl TableBuilder216 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record216) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
      ];
      conn.execute(
        "INSERT INTO MSAsmStmt_getAllConstraints (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSAsmStmt_getClobbers
#[derive(Debug, Serialize, Deserialize)]
pub struct Record217 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: String, // element
}

pub struct TableBuilder217 {
  db_url: String,
  records: Vec<Record217>,
}

impl TableBuilder217 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record217) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
      ];
      conn.execute(
        "INSERT INTO MSAsmStmt_getClobbers (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSAsmStmt_getAllExprs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record218 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder218 {
  db_url: String,
  records: Vec<Record218>,
}

impl TableBuilder218 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record218) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO MSAsmStmt_getAllExprs (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSAsmStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record219 {
  pub c0: u64, // id
  pub c1: u64, // getLBraceLoc
  pub c2: u64, // getEndLoc
  pub c3: bool, // hasBraces
  pub c4: String, // getAsmString
  pub c5: u64, // getBeginLoc
}

pub struct TableBuilder219 {
  db_url: String,
  records: Vec<Record219>,
}

impl TableBuilder219 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record219) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO MSAsmStmt (id, getLBraceLoc, getEndLoc, hasBraces, getAsmString, getBeginLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CStyleCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record220 {
  pub c0: u64, // id
  pub c1: u64, // getLParenLoc
  pub c2: u64, // getRParenLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

pub struct TableBuilder220 {
  db_url: String,
  records: Vec<Record220>,
}

impl TableBuilder220 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record220) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO CStyleCastExpr (id, getLParenLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// IfStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record221 {
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

pub struct TableBuilder221 {
  db_url: String,
  records: Vec<Record221>,
}

impl TableBuilder221 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record221) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 22] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &(record.c16 as i64),
        &record.c17,
        &(record.c18 as i64),
        &(record.c19 as i64),
        &(record.c20 as i64),
        &(record.c21 as i64),
      ];
      conn.execute(
        "INSERT INTO IfStmt (id, hasInitStorage, hasVarStorage, hasElseStorage, getCond, getThen, getElse, getConditionVariable, getConditionVariableDeclStmt, getInit, getIfLoc, getElseLoc, isConsteval, isNonNegatedConsteval, isNegatedConsteval, isConstexpr, getStatementKind, isObjCAvailabilityCheck, getBeginLoc, getEndLoc, getLParenLoc, getRParenLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AsmStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record222 {
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

pub struct TableBuilder222 {
  db_url: String,
  records: Vec<Record222>,
}

impl TableBuilder222 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record222) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
      ];
      conn.execute(
        "INSERT INTO AsmStmt (id, getAsmLoc, isSimple, isVolatile, getBeginLoc, getEndLoc, getNumOutputs, getNumPlusOperands, getNumInputs, getNumClobbers) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXNullPtrLiteralExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record223 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getLocation
}

pub struct TableBuilder223 {
  db_url: String,
  records: Vec<Record223>,
}

impl TableBuilder223 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record223) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXNullPtrLiteralExpr (id, getBeginLoc, getEndLoc, getLocation) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SEHTryStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record224 {
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

pub struct TableBuilder224 {
  db_url: String,
  records: Vec<Record224>,
}

impl TableBuilder224 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record224) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
      ];
      conn.execute(
        "INSERT INTO SEHTryStmt (id, getBeginLoc, getTryLoc, getEndLoc, getIsCXXTry, getTryBlock, getHandler, getExceptHandler, getFinallyHandler) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ReturnStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record225 {
  pub c0: u64, // id
  pub c1: u64, // getRetValue
  pub c2: u64, // getNRVOCandidate
  pub c3: u64, // getReturnLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder225 {
  db_url: String,
  records: Vec<Record225>,
}

impl TableBuilder225 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record225) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO ReturnStmt (id, getRetValue, getNRVOCandidate, getReturnLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AsTypeExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record226 {
  pub c0: u64, // id
  pub c1: u64, // getSrcExpr
  pub c2: u64, // getBuiltinLoc
  pub c3: u64, // getRParenLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder226 {
  db_url: String,
  records: Vec<Record226>,
}

impl TableBuilder226 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record226) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO AsTypeExpr (id, getSrcExpr, getBuiltinLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MatrixSubscriptExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record227 {
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

pub struct TableBuilder227 {
  db_url: String,
  records: Vec<Record227>,
}

impl TableBuilder227 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record227) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
      ];
      conn.execute(
        "INSERT INTO MatrixSubscriptExpr (id, isIncomplete, getBase, getRowIdx, getColumnIdx, getBeginLoc, getEndLoc, getExprLoc, getRBracketLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SwitchStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record228 {
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

pub struct TableBuilder228 {
  db_url: String,
  records: Vec<Record228>,
}

impl TableBuilder228 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record228) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 15] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &record.c12,
        &(record.c13 as i64),
        &(record.c14 as i64),
      ];
      conn.execute(
        "INSERT INTO SwitchStmt (id, hasInitStorage, hasVarStorage, getCond, getBody, getInit, getConditionVariable, getConditionVariableDeclStmt, getSwitchCaseList, getSwitchLoc, getLParenLoc, getRParenLoc, isAllEnumCasesCovered, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AttributedStmt_getAttrs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record229 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder229 {
  db_url: String,
  records: Vec<Record229>,
}

impl TableBuilder229 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record229) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO AttributedStmt_getAttrs (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AttributedStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record230 {
  pub c0: u64, // id
  pub c1: u64, // getAttrLoc
  pub c2: u64, // getSubStmt
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

pub struct TableBuilder230 {
  db_url: String,
  records: Vec<Record230>,
}

impl TableBuilder230 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record230) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO AttributedStmt (id, getAttrLoc, getSubStmt, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SEHFinallyStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record231 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getFinallyLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getBlock
}

pub struct TableBuilder231 {
  db_url: String,
  records: Vec<Record231>,
}

impl TableBuilder231 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record231) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO SEHFinallyStmt (id, getBeginLoc, getFinallyLoc, getEndLoc, getBlock) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MemberExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record232 {
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

pub struct TableBuilder232 {
  db_url: String,
  records: Vec<Record232>,
}

impl TableBuilder232 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record232) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 19] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &record.c7,
        &record.c8,
        &record.c9,
        &(record.c10 as i64),
        &record.c11,
        &(record.c12 as i64),
        &(record.c13 as i64),
        &(record.c14 as i64),
        &(record.c15 as i64),
        &record.c16,
        &record.c17,
        &(record.c18 as i64),
      ];
      conn.execute(
        "INSERT INTO MemberExpr (id, getBase, getMemberDecl, hasQualifier, getTemplateKeywordLoc, getLAngleLoc, getRAngleLoc, hasTemplateKeyword, hasExplicitTemplateArgs, getNumTemplateArgs, getOperatorLoc, isArrow, getMemberLoc, getBeginLoc, getEndLoc, getExprLoc, isImplicitAccess, hadMultipleCandidates, isNonOdrUse) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// IndirectGotoStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record233 {
  pub c0: u64, // id
  pub c1: u64, // getGotoLoc
  pub c2: u64, // getStarLoc
  pub c3: u64, // getTarget
  pub c4: u64, // getConstantTarget
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder233 {
  db_url: String,
  records: Vec<Record233>,
}

impl TableBuilder233 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record233) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO IndirectGotoStmt (id, getGotoLoc, getStarLoc, getTarget, getConstantTarget, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder234 {
  db_url: String,
  records: Vec<Record234>,
}

impl TableBuilder234 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record234) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO CXXNoexceptExpr (id, getOperand, getBeginLoc, getEndLoc, getSourceRange, getValue) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// NullStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record235 {
  pub c0: u64, // id
  pub c1: u64, // getSemiLoc
  pub c2: bool, // hasLeadingEmptyMacro
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

pub struct TableBuilder235 {
  db_url: String,
  records: Vec<Record235>,
}

impl TableBuilder235 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record235) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO NullStmt (id, getSemiLoc, hasLeadingEmptyMacro, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BlockExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record236 {
  pub c0: u64, // id
  pub c1: u64, // getBlockDecl
  pub c2: u64, // getCaretLocation
  pub c3: u64, // getBody
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
  pub c6: u64, // getFunctionType
}

pub struct TableBuilder236 {
  db_url: String,
  records: Vec<Record236>,
}

impl TableBuilder236 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record236) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO BlockExpr (id, getBlockDecl, getCaretLocation, getBody, getBeginLoc, getEndLoc, getFunctionType) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// Stmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record237 {
  pub c0: u64, // id
  pub c1: u64, // stripLabelLikeStatements
}

pub struct TableBuilder237 {
  db_url: String,
  records: Vec<Record237>,
}

impl TableBuilder237 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record237) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO Stmt (id, stripLabelLikeStatements) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXTryStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record238 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getTryLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getTryBlock
  pub c5: u32, // getNumHandlers
}

pub struct TableBuilder238 {
  db_url: String,
  records: Vec<Record238>,
}

impl TableBuilder238 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record238) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO CXXTryStmt (id, getBeginLoc, getTryLoc, getEndLoc, getTryBlock, getNumHandlers) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnresolvedMemberExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record239 {
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

pub struct TableBuilder239 {
  db_url: String,
  records: Vec<Record239>,
}

impl TableBuilder239 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record239) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 11] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
      ];
      conn.execute(
        "INSERT INTO UnresolvedMemberExpr (id, isImplicitAccess, getBaseType, hasUnresolvedUsing, isArrow, getOperatorLoc, getNamingClass, getMemberLoc, getExprLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// StmtExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record240 {
  pub c0: u64, // id
  pub c1: u64, // getSubStmt
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getLParenLoc
  pub c5: u64, // getRParenLoc
  pub c6: u32, // getTemplateDepth
}

pub struct TableBuilder240 {
  db_url: String,
  records: Vec<Record240>,
}

impl TableBuilder240 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record240) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
      ];
      conn.execute(
        "INSERT INTO StmtExpr (id, getSubStmt, getBeginLoc, getEndLoc, getLParenLoc, getRParenLoc, getTemplateDepth) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CaseStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record241 {
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

pub struct TableBuilder241 {
  db_url: String,
  records: Vec<Record241>,
}

impl TableBuilder241 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record241) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
      ];
      conn.execute(
        "INSERT INTO CaseStmt (id, caseStmtIsGNURange, getCaseLoc, getEllipsisLoc, getLHS, getRHS, getSubStmt, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FunctionParmPackExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record242 {
  pub c0: u64, // id
  pub c1: u64, // getParameterPack
  pub c2: u64, // getParameterPackLocation
  pub c3: u32, // getNumExpansions
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder242 {
  db_url: String,
  records: Vec<Record242>,
}

impl TableBuilder242 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record242) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO FunctionParmPackExpr (id, getParameterPack, getParameterPackLocation, getNumExpansions, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ImplicitCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record243 {
  pub c0: u64, // id
  pub c1: bool, // isPartOfExplicitCast
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

pub struct TableBuilder243 {
  db_url: String,
  records: Vec<Record243>,
}

impl TableBuilder243 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record243) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO ImplicitCastExpr (id, isPartOfExplicitCast, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// OpaqueValueExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record244 {
  pub c0: u64, // id
  pub c1: u64, // getLocation
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getExprLoc
  pub c5: u64, // getSourceExpr
  pub c6: bool, // isUnique
}

pub struct TableBuilder244 {
  db_url: String,
  records: Vec<Record244>,
}

impl TableBuilder244 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record244) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
      ];
      conn.execute(
        "INSERT INTO OpaqueValueExpr (id, getLocation, getBeginLoc, getEndLoc, getExprLoc, getSourceExpr, isUnique) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder245 {
  db_url: String,
  records: Vec<Record245>,
}

impl TableBuilder245 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record245) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO UserDefinedLiteral (id, getLiteralOperatorKind, getCookedLiteral, getBeginLoc, getEndLoc, getUDSuffixLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXDynamicCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record246 {
  pub c0: u64, // id
  pub c1: bool, // isAlwaysNull
}

pub struct TableBuilder246 {
  db_url: String,
  records: Vec<Record246>,
}

impl TableBuilder246 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record246) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &record.c1,
      ];
      conn.execute(
        "INSERT INTO CXXDynamicCastExpr (id, isAlwaysNull) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder247 {
  db_url: String,
  records: Vec<Record247>,
}

impl TableBuilder247 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record247) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO ArrayTypeTraitExpr (id, getBeginLoc, getEndLoc, getTrait, getQueriedType, getDimensionExpression) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// BinaryOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record248 {
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

pub struct TableBuilder248 {
  db_url: String,
  records: Vec<Record248>,
}

impl TableBuilder248 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record248) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 23] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &record.c16,
        &record.c17,
        &record.c18,
        &record.c19,
        &record.c20,
        &record.c21,
        &record.c22,
      ];
      conn.execute(
        "INSERT INTO BinaryOperator (id, getExprLoc, getOperatorLoc, getOpcode, getLHS, getRHS, getBeginLoc, getEndLoc, getOpcodeStr, isPtrMemOp, isMultiplicativeOp, isAdditiveOp, isShiftOp, isBitwiseOp, isRelationalOp, isEqualityOp, isComparisonOp, isCommaOp, isLogicalOp, isAssignmentOp, isCompoundAssignmentOp, isShiftAssignOp, hasStoredFPFeatures) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXPseudoDestructorExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record249 {
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

pub struct TableBuilder249 {
  db_url: String,
  records: Vec<Record249>,
}

impl TableBuilder249 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record249) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 11] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXPseudoDestructorExpr (id, getBase, hasQualifier, isArrow, getOperatorLoc, getColonColonLoc, getTildeLoc, getDestroyedType, getDestroyedTypeLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// StringLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record250 {
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

pub struct TableBuilder250 {
  db_url: String,
  records: Vec<Record250>,
}

impl TableBuilder250 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record250) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 19] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &record.c15,
        &record.c16,
        &(record.c17 as i64),
        &(record.c18 as i64),
      ];
      conn.execute(
        "INSERT INTO StringLiteral (id, getString, getBytes, getByteLength, getLength, getCharByteWidth, getKind, isOrdinary, isWide, isUTF8, isUTF16, isUTF32, isUnevaluated, isPascal, containsNonAscii, containsNonAsciiOrNull, getNumConcatenated, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXBindTemporaryExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record251 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

pub struct TableBuilder251 {
  db_url: String,
  records: Vec<Record251>,
}

impl TableBuilder251 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record251) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXBindTemporaryExpr (id, getSubExpr, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXOperatorCallExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record252 {
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

pub struct TableBuilder252 {
  db_url: String,
  records: Vec<Record252>,
}

impl TableBuilder252 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record252) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXOperatorCallExpr (id, getOperator, isAssignmentOp, isComparisonOp, isInfixBinaryOp, getOperatorLoc, getExprLoc, getBeginLoc, getEndLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXDefaultArgExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record253 {
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

pub struct TableBuilder253 {
  db_url: String,
  records: Vec<Record253>,
}

impl TableBuilder253 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record253) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXDefaultArgExpr (id, getParam, hasRewrittenInit, getExpr, getRewrittenExpr, getAdjustedRewrittenExpr, getUsedLocation, getBeginLoc, getEndLoc, getExprLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXThisExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record254 {
  pub c0: u64, // id
  pub c1: u64, // getLocation
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: bool, // isImplicit
}

pub struct TableBuilder254 {
  db_url: String,
  records: Vec<Record254>,
}

impl TableBuilder254 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record254) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO CXXThisExpr (id, getLocation, getBeginLoc, getEndLoc, isImplicit) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXUuidofExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record255 {
  pub c0: u64, // id
  pub c1: bool, // isTypeOperand
  pub c2: u64, // getExprOperand
  pub c3: u64, // getGuidDecl
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
  pub c6: u64, // getSourceRange
}

pub struct TableBuilder255 {
  db_url: String,
  records: Vec<Record255>,
}

impl TableBuilder255 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record255) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXUuidofExpr (id, isTypeOperand, getExprOperand, getGuidDecl, getBeginLoc, getEndLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder256 {
  db_url: String,
  records: Vec<Record256>,
}

impl TableBuilder256 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record256) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXStdInitializerListExpr (id, getSubExpr, getBeginLoc, getEndLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ShuffleVectorExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record257 {
  pub c0: u64, // id
  pub c1: u64, // getBuiltinLoc
  pub c2: u64, // getRParenLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u32, // getNumSubExprs
}

pub struct TableBuilder257 {
  db_url: String,
  records: Vec<Record257>,
}

impl TableBuilder257 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record257) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO ShuffleVectorExpr (id, getBuiltinLoc, getRParenLoc, getBeginLoc, getEndLoc, getNumSubExprs) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AtomicExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record258 {
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

pub struct TableBuilder258 {
  db_url: String,
  records: Vec<Record258>,
}

impl TableBuilder258 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record258) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 19] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &record.c10,
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &(record.c15 as i64),
        &(record.c16 as i64),
        &(record.c17 as i64),
        &(record.c18 as i64),
      ];
      conn.execute(
        "INSERT INTO AtomicExpr (id, getPtr, getOrder, getScope, getVal1, getOrderFail, getVal2, getWeak, getValueType, getOp, getOpAsString, getNumSubExprs, isVolatile, isCmpXChg, isOpenCL, getBuiltinLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ImplicitValueInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record259 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

pub struct TableBuilder259 {
  db_url: String,
  records: Vec<Record259>,
}

impl TableBuilder259 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record259) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO ImplicitValueInitExpr (id, getBeginLoc, getEndLoc) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// NoInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record260 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

pub struct TableBuilder260 {
  db_url: String,
  records: Vec<Record260>,
}

impl TableBuilder260 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record260) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO NoInitExpr (id, getBeginLoc, getEndLoc) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXThrowExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record261 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getThrowLoc
  pub c3: bool, // isThrownVariableInScope
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder261 {
  db_url: String,
  records: Vec<Record261>,
}

impl TableBuilder261 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record261) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXThrowExpr (id, getSubExpr, getThrowLoc, isThrownVariableInScope, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CompoundStmt_body
#[derive(Debug, Serialize, Deserialize)]
pub struct Record262 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder262 {
  db_url: String,
  records: Vec<Record262>,
}

impl TableBuilder262 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record262) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CompoundStmt_body (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CompoundStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record263 {
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

pub struct TableBuilder263 {
  db_url: String,
  records: Vec<Record263>,
}

impl TableBuilder263 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record263) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 11] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
      ];
      conn.execute(
        "INSERT INTO CompoundStmt (id, body_empty, size, hasStoredFPFeatures, body_front, body_back, getStmtExprResult, getBeginLoc, getEndLoc, getLBracLoc, getRBracLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder264 {
  db_url: String,
  records: Vec<Record264>,
}

impl TableBuilder264 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record264) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO AbstractConditionalOperator (id, getCond, getTrueExpr, getFalseExpr, getQuestionLoc, getColonLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RecoveryExpr_subExpressions
#[derive(Debug, Serialize, Deserialize)]
pub struct Record265 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder265 {
  db_url: String,
  records: Vec<Record265>,
}

impl TableBuilder265 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record265) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO RecoveryExpr_subExpressions (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RecoveryExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record266 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

pub struct TableBuilder266 {
  db_url: String,
  records: Vec<Record266>,
}

impl TableBuilder266 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record266) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO RecoveryExpr (id, getBeginLoc, getEndLoc) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder267 {
  db_url: String,
  records: Vec<Record267>,
}

impl TableBuilder267 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record267) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO BinaryConditionalOperator (id, getCommon, getOpaqueValue, getCond, getTrueExpr, getFalseExpr, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DeclStmt_decls
#[derive(Debug, Serialize, Deserialize)]
pub struct Record268 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder268 {
  db_url: String,
  records: Vec<Record268>,
}

impl TableBuilder268 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record268) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO DeclStmt_decls (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DeclStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record269 {
  pub c0: u64, // id
  pub c1: bool, // isSingleDecl
  pub c2: u64, // getEndLoc
  pub c3: u64, // getBeginLoc
}

pub struct TableBuilder269 {
  db_url: String,
  records: Vec<Record269>,
}

impl TableBuilder269 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record269) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO DeclStmt (id, isSingleDecl, getEndLoc, getBeginLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ExtVectorElementExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record270 {
  pub c0: u64, // id
  pub c1: u64, // getBase
  pub c2: u64, // getAccessorLoc
  pub c3: u32, // getNumElements
  pub c4: bool, // containsDuplicateElements
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
  pub c7: bool, // isArrow
}

pub struct TableBuilder270 {
  db_url: String,
  records: Vec<Record270>,
}

impl TableBuilder270 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record270) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
        &record.c7,
      ];
      conn.execute(
        "INSERT INTO ExtVectorElementExpr (id, getBase, getAccessorLoc, getNumElements, containsDuplicateElements, getBeginLoc, getEndLoc, isArrow) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXBoolLiteralExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record271 {
  pub c0: u64, // id
  pub c1: bool, // getValue
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getLocation
}

pub struct TableBuilder271 {
  db_url: String,
  records: Vec<Record271>,
}

impl TableBuilder271 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record271) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXBoolLiteralExpr (id, getValue, getBeginLoc, getEndLoc, getLocation) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnresolvedLookupExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record272 {
  pub c0: u64, // id
  pub c1: bool, // requiresADL
  pub c2: bool, // isOverloaded
  pub c3: u64, // getNamingClass
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder272 {
  db_url: String,
  records: Vec<Record272>,
}

impl TableBuilder272 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record272) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO UnresolvedLookupExpr (id, requiresADL, isOverloaded, getNamingClass, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ArrayInitIndexExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record273 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

pub struct TableBuilder273 {
  db_url: String,
  records: Vec<Record273>,
}

impl TableBuilder273 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record273) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO ArrayInitIndexExpr (id, getBeginLoc, getEndLoc) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnaryExprOrTypeTraitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record274 {
  pub c0: u64, // id
  pub c1: u64, // getKind
  pub c2: bool, // isArgumentType
  pub c3: u64, // getTypeOfArgument
  pub c4: u64, // getOperatorLoc
  pub c5: u64, // getRParenLoc
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder274 {
  db_url: String,
  records: Vec<Record274>,
}

impl TableBuilder274 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record274) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO UnaryExprOrTypeTraitExpr (id, getKind, isArgumentType, getTypeOfArgument, getOperatorLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// OffsetOfExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record275 {
  pub c0: u64, // id
  pub c1: u64, // getOperatorLoc
  pub c2: u64, // getRParenLoc
  pub c3: u32, // getNumComponents
  pub c4: u32, // getNumExpressions
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder275 {
  db_url: String,
  records: Vec<Record275>,
}

impl TableBuilder275 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record275) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO OffsetOfExpr (id, getOperatorLoc, getRParenLoc, getNumComponents, getNumExpressions, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DeclRefExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record276 {
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

pub struct TableBuilder276 {
  db_url: String,
  records: Vec<Record276>,
}

impl TableBuilder276 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record276) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 19] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
        &record.c7,
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &record.c11,
        &record.c12,
        &record.c13,
        &record.c14,
        &(record.c15 as i64),
        &record.c16,
        &record.c17,
        &record.c18,
      ];
      conn.execute(
        "INSERT INTO DeclRefExpr (id, getDecl, getLocation, getBeginLoc, getEndLoc, hasQualifier, getFoundDecl, hasTemplateKWAndArgsInfo, getTemplateKeywordLoc, getLAngleLoc, getRAngleLoc, hasTemplateKeyword, hasExplicitTemplateArgs, getNumTemplateArgs, hadMultipleCandidates, isNonOdrUse, refersToEnclosingVariableOrCapture, isImmediateEscalating, isCapturedByCopyInLambdaWithExplicitObjectParameter) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXTypeidExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record277 {
  pub c0: u64, // id
  pub c1: bool, // isPotentiallyEvaluated
  pub c2: bool, // isTypeOperand
  pub c3: u64, // getExprOperand
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
  pub c6: u64, // getSourceRange
}

pub struct TableBuilder277 {
  db_url: String,
  records: Vec<Record277>,
}

impl TableBuilder277 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record277) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXTypeidExpr (id, isPotentiallyEvaluated, isTypeOperand, getExprOperand, getBeginLoc, getEndLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// GenericSelectionExpr_getAssocExprs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record278 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder278 {
  db_url: String,
  records: Vec<Record278>,
}

impl TableBuilder278 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record278) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO GenericSelectionExpr_getAssocExprs (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// GenericSelectionExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record279 {
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

pub struct TableBuilder279 {
  db_url: String,
  records: Vec<Record279>,
}

impl TableBuilder279 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record279) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 13] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &(record.c12 as i64),
      ];
      conn.execute(
        "INSERT INTO GenericSelectionExpr (id, getNumAssocs, getResultIndex, isResultDependent, isExprPredicate, isTypePredicate, getControllingExpr, getResultExpr, getGenericLoc, getDefaultLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SwitchCase
#[derive(Debug, Serialize, Deserialize)]
pub struct Record280 {
  pub c0: u64, // id
  pub c1: u64, // getNextSwitchCase
  pub c2: u64, // getKeywordLoc
  pub c3: u64, // getColonLoc
  pub c4: u64, // getSubStmt
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder280 {
  db_url: String,
  records: Vec<Record280>,
}

impl TableBuilder280 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record280) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO SwitchCase (id, getNextSwitchCase, getKeywordLoc, getColonLoc, getSubStmt, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// IntegerLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record281 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getLocation
}

pub struct TableBuilder281 {
  db_url: String,
  records: Vec<Record281>,
}

impl TableBuilder281 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record281) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO IntegerLiteral (id, getBeginLoc, getEndLoc, getLocation) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXStaticCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record282 {
  pub c0: u64, // id
}

pub struct TableBuilder282 {
  db_url: String,
  records: Vec<Record282>,
}

impl TableBuilder282 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record282) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXStaticCastExpr (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder283 {
  db_url: String,
  records: Vec<Record283>,
}

impl TableBuilder283 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record283) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
      ];
      conn.execute(
        "INSERT INTO LabelStmt (id, getIdentLoc, getDecl, getSubStmt, getBeginLoc, getEndLoc, isSideEntry) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// VAArgExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record284 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: bool, // isMicrosoftABI
  pub c3: u64, // getBuiltinLoc
  pub c4: u64, // getRParenLoc
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder284 {
  db_url: String,
  records: Vec<Record284>,
}

impl TableBuilder284 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record284) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO VAArgExpr (id, getSubExpr, isMicrosoftABI, getBuiltinLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ImaginaryLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record285 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

pub struct TableBuilder285 {
  db_url: String,
  records: Vec<Record285>,
}

impl TableBuilder285 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record285) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO ImaginaryLiteral (id, getSubExpr, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record286 {
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

pub struct TableBuilder286 {
  db_url: String,
  records: Vec<Record286>,
}

impl TableBuilder286 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record286) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
      ];
      conn.execute(
        "INSERT INTO CastExpr (id, getCastKind, getSubExpr, getSubExprAsWritten, getConversionFunction, path_empty, path_size, hasStoredFPFeatures, changesVolatileQualification) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PackExpansionExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record287 {
  pub c0: u64, // id
  pub c1: u64, // getPattern
  pub c2: u64, // getEllipsisLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

pub struct TableBuilder287 {
  db_url: String,
  records: Vec<Record287>,
}

impl TableBuilder287 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record287) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO PackExpansionExpr (id, getPattern, getEllipsisLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ConstantExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record288 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getResultAPValueKind
  pub c4: u64, // getResultStorageKind
  pub c5: bool, // isImmediateInvocation
  pub c6: bool, // hasAPValueResult
}

pub struct TableBuilder288 {
  db_url: String,
  records: Vec<Record288>,
}

impl TableBuilder288 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record288) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &record.c6,
      ];
      conn.execute(
        "INSERT INTO ConstantExpr (id, getBeginLoc, getEndLoc, getResultAPValueKind, getResultStorageKind, isImmediateInvocation, hasAPValueResult) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ConvertVectorExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record289 {
  pub c0: u64, // id
  pub c1: u64, // getSrcExpr
  pub c2: u64, // getBuiltinLoc
  pub c3: u64, // getRParenLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder289 {
  db_url: String,
  records: Vec<Record289>,
}

impl TableBuilder289 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record289) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO ConvertVectorExpr (id, getSrcExpr, getBuiltinLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DefaultStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record290 {
  pub c0: u64, // id
  pub c1: u64, // getSubStmt
  pub c2: u64, // getDefaultLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

pub struct TableBuilder290 {
  db_url: String,
  records: Vec<Record290>,
}

impl TableBuilder290 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record290) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO DefaultStmt (id, getSubStmt, getDefaultLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ArraySubscriptExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record291 {
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

pub struct TableBuilder291 {
  db_url: String,
  records: Vec<Record291>,
}

impl TableBuilder291 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record291) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
      ];
      conn.execute(
        "INSERT INTO ArraySubscriptExpr (id, getLHS, getRHS, getBase, getIdx, getBeginLoc, getEndLoc, getRBracketLoc, getExprLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RequiresExpr_getLocalParameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record292 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder292 {
  db_url: String,
  records: Vec<Record292>,
}

impl TableBuilder292 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record292) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO RequiresExpr_getLocalParameters (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// RequiresExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record293 {
  pub c0: u64, // id
  pub c1: u64, // getBody
  pub c2: u64, // getRequiresKWLoc
  pub c3: u64, // getLParenLoc
  pub c4: u64, // getRParenLoc
  pub c5: u64, // getRBraceLoc
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder293 {
  db_url: String,
  records: Vec<Record293>,
}

impl TableBuilder293 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record293) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO RequiresExpr (id, getBody, getRequiresKWLoc, getLParenLoc, getRParenLoc, getRBraceLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ExpressionTraitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record294 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getTrait
  pub c4: u64, // getQueriedExpression
  pub c5: bool, // getValue
}

pub struct TableBuilder294 {
  db_url: String,
  records: Vec<Record294>,
}

impl TableBuilder294 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record294) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
      ];
      conn.execute(
        "INSERT INTO ExpressionTraitExpr (id, getBeginLoc, getEndLoc, getTrait, getQueriedExpression, getValue) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// InitListExpr_inits
#[derive(Debug, Serialize, Deserialize)]
pub struct Record295 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder295 {
  db_url: String,
  records: Vec<Record295>,
}

impl TableBuilder295 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record295) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO InitListExpr_inits (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// InitListExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record296 {
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

pub struct TableBuilder296 {
  db_url: String,
  records: Vec<Record296>,
}

impl TableBuilder296 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record296) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 18] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &record.c6,
        &record.c7,
        &record.c8,
        &(record.c9 as i64),
        &(record.c10 as i64),
        &record.c11,
        &(record.c12 as i64),
        &record.c13,
        &(record.c14 as i64),
        &record.c15,
        &(record.c16 as i64),
        &(record.c17 as i64),
      ];
      conn.execute(
        "INSERT INTO InitListExpr (id, getNumInits, getArrayFiller, hasArrayFiller, hasDesignatedInit, getInitializedFieldInUnion, isExplicit, isStringLiteralInit, isTransparent, getLBraceLoc, getRBraceLoc, isSemanticForm, getSemanticForm, isSyntacticForm, getSyntacticForm, hadArrayRangeDesignator, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXMemberCallExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record297 {
  pub c0: u64, // id
  pub c1: u64, // getImplicitObjectArgument
  pub c2: u64, // getObjectType
  pub c3: u64, // getMethodDecl
  pub c4: u64, // getRecordDecl
  pub c5: u64, // getExprLoc
}

pub struct TableBuilder297 {
  db_url: String,
  records: Vec<Record297>,
}

impl TableBuilder297 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record297) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXMemberCallExpr (id, getImplicitObjectArgument, getObjectType, getMethodDecl, getRecordDecl, getExprLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DesignatedInitUpdateExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record298 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getBase
  pub c4: u64, // getUpdater
}

pub struct TableBuilder298 {
  db_url: String,
  records: Vec<Record298>,
}

impl TableBuilder298 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record298) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO DesignatedInitUpdateExpr (id, getBeginLoc, getEndLoc, getBase, getUpdater) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXReinterpretCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record299 {
  pub c0: u64, // id
}

pub struct TableBuilder299 {
  db_url: String,
  records: Vec<Record299>,
}

impl TableBuilder299 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record299) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXReinterpretCastExpr (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSPropertySubscriptExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record300 {
  pub c0: u64, // id
  pub c1: u64, // getBase
  pub c2: u64, // getIdx
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u64, // getRBracketLoc
  pub c6: u64, // getExprLoc
}

pub struct TableBuilder300 {
  db_url: String,
  records: Vec<Record300>,
}

impl TableBuilder300 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record300) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO MSPropertySubscriptExpr (id, getBase, getIdx, getBeginLoc, getEndLoc, getRBracketLoc, getExprLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentScopeDeclRefExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record301 {
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

pub struct TableBuilder301 {
  db_url: String,
  records: Vec<Record301>,
}

impl TableBuilder301 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record301) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &record.c6,
        &record.c7,
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentScopeDeclRefExpr (id, getLocation, getTemplateKeywordLoc, getLAngleLoc, getRAngleLoc, hasTemplateKeyword, hasExplicitTemplateArgs, getNumTemplateArgs, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXAddrspaceCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record302 {
  pub c0: u64, // id
}

pub struct TableBuilder302 {
  db_url: String,
  records: Vec<Record302>,
}

impl TableBuilder302 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record302) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXAddrspaceCastExpr (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXFunctionalCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record303 {
  pub c0: u64, // id
  pub c1: u64, // getLParenLoc
  pub c2: u64, // getRParenLoc
  pub c3: bool, // isListInitialization
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder303 {
  db_url: String,
  records: Vec<Record303>,
}

impl TableBuilder303 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record303) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXFunctionalCastExpr (id, getLParenLoc, getRParenLoc, isListInitialization, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ConditionalOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record304 {
  pub c0: u64, // id
  pub c1: u64, // getCond
  pub c2: u64, // getTrueExpr
  pub c3: u64, // getFalseExpr
  pub c4: u64, // getLHS
  pub c5: u64, // getRHS
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder304 {
  db_url: String,
  records: Vec<Record304>,
}

impl TableBuilder304 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record304) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO ConditionalOperator (id, getCond, getTrueExpr, getFalseExpr, getLHS, getRHS, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypeTraitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record305 {
  pub c0: u64, // id
  pub c1: u64, // getTrait
  pub c2: u32, // getNumArgs
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

pub struct TableBuilder305 {
  db_url: String,
  records: Vec<Record305>,
}

impl TableBuilder305 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record305) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO TypeTraitExpr (id, getTrait, getNumArgs, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXInheritedCtorInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record306 {
  pub c0: u64, // id
  pub c1: u64, // getConstructor
  pub c2: bool, // constructsVBase
  pub c3: u64, // getConstructionKind
  pub c4: bool, // inheritedFromVBase
  pub c5: u64, // getLocation
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder306 {
  db_url: String,
  records: Vec<Record306>,
}

impl TableBuilder306 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record306) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXInheritedCtorInitExpr (id, getConstructor, constructsVBase, getConstructionKind, inheritedFromVBase, getLocation, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CompoundAssignOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record307 {
  pub c0: u64, // id
  pub c1: u64, // getComputationLHSType
  pub c2: u64, // getComputationResultType
}

pub struct TableBuilder307 {
  db_url: String,
  records: Vec<Record307>,
}

impl TableBuilder307 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record307) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CompoundAssignOperator (id, getComputationLHSType, getComputationResultType) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ParenExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record308 {
  pub c0: u64, // id
  pub c1: u64, // getSubExpr
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
  pub c4: u64, // getLParen
  pub c5: u64, // getRParen
}

pub struct TableBuilder308 {
  db_url: String,
  records: Vec<Record308>,
}

impl TableBuilder308 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record308) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO ParenExpr (id, getSubExpr, getBeginLoc, getEndLoc, getLParen, getRParen) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PseudoObjectExpr_semantics
#[derive(Debug, Serialize, Deserialize)]
pub struct Record309 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder309 {
  db_url: String,
  records: Vec<Record309>,
}

impl TableBuilder309 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record309) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO PseudoObjectExpr_semantics (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PseudoObjectExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record310 {
  pub c0: u64, // id
  pub c1: u64, // getSyntacticForm
  pub c2: u32, // getResultExprIndex
  pub c3: u64, // getResultExpr
  pub c4: u32, // getNumSemanticExprs
  pub c5: u64, // getExprLoc
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder310 {
  db_url: String,
  records: Vec<Record310>,
}

impl TableBuilder310 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record310) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO PseudoObjectExpr (id, getSyntacticForm, getResultExprIndex, getResultExpr, getNumSemanticExprs, getExprLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// FixedPointLiteral
#[derive(Debug, Serialize, Deserialize)]
pub struct Record311 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getLocation
  pub c4: u32, // getScale
}

pub struct TableBuilder311 {
  db_url: String,
  records: Vec<Record311>,
}

impl TableBuilder311 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record311) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
      ];
      conn.execute(
        "INSERT INTO FixedPointLiteral (id, getBeginLoc, getEndLoc, getLocation, getScale) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// AddrLabelExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record312 {
  pub c0: u64, // id
  pub c1: u64, // getAmpAmpLoc
  pub c2: u64, // getLabelLoc
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
  pub c5: u64, // getLabel
}

pub struct TableBuilder312 {
  db_url: String,
  records: Vec<Record312>,
}

impl TableBuilder312 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record312) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO AddrLabelExpr (id, getAmpAmpLoc, getLabelLoc, getBeginLoc, getEndLoc, getLabel) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// GNUNullExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record313 {
  pub c0: u64, // id
  pub c1: u64, // getTokenLocation
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

pub struct TableBuilder313 {
  db_url: String,
  records: Vec<Record313>,
}

impl TableBuilder313 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record313) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO GNUNullExpr (id, getTokenLocation, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXRewrittenBinaryOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record314 {
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

pub struct TableBuilder314 {
  db_url: String,
  records: Vec<Record314>,
}

impl TableBuilder314 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record314) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 15] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &record.c6,
        &record.c7,
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &(record.c12 as i64),
        &(record.c13 as i64),
        &(record.c14 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXRewrittenBinaryOperator (id, getSemanticForm, isReversed, getOperator, getOpcode, getOpcodeStr, isComparisonOp, isAssignmentOp, getLHS, getRHS, getOperatorLoc, getExprLoc, getBeginLoc, getEndLoc, getSourceRange) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// OverloadExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record315 {
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

pub struct TableBuilder315 {
  db_url: String,
  records: Vec<Record315>,
}

impl TableBuilder315 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record315) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &record.c7,
        &record.c8,
        &record.c9,
      ];
      conn.execute(
        "INSERT INTO OverloadExpr (id, getNamingClass, getNumDecls, getNameLoc, getTemplateKeywordLoc, getLAngleLoc, getRAngleLoc, hasTemplateKeyword, hasExplicitTemplateArgs, getNumTemplateArgs) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXUnresolvedConstructExpr_arguments
#[derive(Debug, Serialize, Deserialize)]
pub struct Record316 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder316 {
  db_url: String,
  records: Vec<Record316>,
}

impl TableBuilder316 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record316) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXUnresolvedConstructExpr_arguments (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXUnresolvedConstructExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record317 {
  pub c0: u64, // id
  pub c1: u64, // getTypeAsWritten
  pub c2: u64, // getLParenLoc
  pub c3: u64, // getRParenLoc
  pub c4: bool, // isListInitialization
  pub c5: u32, // getNumArgs
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder317 {
  db_url: String,
  records: Vec<Record317>,
}

impl TableBuilder317 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record317) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXUnresolvedConstructExpr (id, getTypeAsWritten, getLParenLoc, getRParenLoc, isListInitialization, getNumArgs, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ExplicitCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record318 {
  pub c0: u64, // id
  pub c1: u64, // getTypeAsWritten
}

pub struct TableBuilder318 {
  db_url: String,
  records: Vec<Record318>,
}

impl TableBuilder318 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record318) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO ExplicitCastExpr (id, getTypeAsWritten) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ChooseExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record319 {
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

pub struct TableBuilder319 {
  db_url: String,
  records: Vec<Record319>,
}

impl TableBuilder319 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record319) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 11] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
      ];
      conn.execute(
        "INSERT INTO ChooseExpr (id, isConditionTrue, isConditionDependent, getChosenSubExpr, getCond, getLHS, getRHS, getBuiltinLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXDependentScopeMemberExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record320 {
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

pub struct TableBuilder320 {
  db_url: String,
  records: Vec<Record320>,
}

impl TableBuilder320 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record320) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 15] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &record.c10,
        &record.c11,
        &record.c12,
        &(record.c13 as i64),
        &(record.c14 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXDependentScopeMemberExpr (id, isImplicitAccess, getBaseType, isArrow, getOperatorLoc, getFirstQualifierFoundInScope, getMemberLoc, getTemplateKeywordLoc, getLAngleLoc, getRAngleLoc, hasTemplateKeyword, hasExplicitTemplateArgs, getNumTemplateArgs, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SizeOfPackExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record321 {
  pub c0: u64, // id
  pub c1: u64, // getOperatorLoc
  pub c2: u64, // getPackLoc
  pub c3: u64, // getRParenLoc
  pub c4: u64, // getPack
  pub c5: bool, // isPartiallySubstituted
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder321 {
  db_url: String,
  records: Vec<Record321>,
}

impl TableBuilder321 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record321) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO SizeOfPackExpr (id, getOperatorLoc, getPackLoc, getRParenLoc, getPack, isPartiallySubstituted, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// PredefinedExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record322 {
  pub c0: u64, // id
  pub c1: u64, // getIdentKind
  pub c2: bool, // isTransparent
  pub c3: u64, // getLocation
  pub c4: u64, // getFunctionName
  pub c5: String, // getIdentKindName
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder322 {
  db_url: String,
  records: Vec<Record322>,
}

impl TableBuilder322 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record322) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO PredefinedExpr (id, getIdentKind, isTransparent, getLocation, getFunctionName, getIdentKindName, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SubstNonTypeTemplateParmExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record323 {
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

pub struct TableBuilder323 {
  db_url: String,
  records: Vec<Record323>,
}

impl TableBuilder323 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record323) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &record.c6,
        &(record.c7 as i64),
        &record.c8,
      ];
      conn.execute(
        "INSERT INTO SubstNonTypeTemplateParmExpr (id, getNameLoc, getBeginLoc, getEndLoc, getReplacement, getAssociatedDecl, getIndex, getParameter, isReferenceParameter) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SubstNonTypeTemplateParmPackExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record324 {
  pub c0: u64, // id
  pub c1: u64, // getAssociatedDecl
  pub c2: u32, // getIndex
  pub c3: u64, // getParameterPack
  pub c4: u64, // getParameterPackLocation
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder324 {
  db_url: String,
  records: Vec<Record324>,
}

impl TableBuilder324 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record324) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO SubstNonTypeTemplateParmPackExpr (id, getAssociatedDecl, getIndex, getParameterPack, getParameterPackLocation, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXParenListInitExpr_getInitExprs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record325 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder325 {
  db_url: String,
  records: Vec<Record325>,
}

impl TableBuilder325 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record325) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXParenListInitExpr_getInitExprs (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXParenListInitExpr_getUserSpecifiedInitExprs
#[derive(Debug, Serialize, Deserialize)]
pub struct Record326 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder326 {
  db_url: String,
  records: Vec<Record326>,
}

impl TableBuilder326 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record326) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXParenListInitExpr_getUserSpecifiedInitExprs (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXParenListInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record327 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getInitLoc
  pub c4: u64, // getSourceRange
  pub c5: u64, // getArrayFiller
  pub c6: u64, // getInitializedFieldInUnion
}

pub struct TableBuilder327 {
  db_url: String,
  records: Vec<Record327>,
}

impl TableBuilder327 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record327) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXParenListInitExpr (id, getBeginLoc, getEndLoc, getInitLoc, getSourceRange, getArrayFiller, getInitializedFieldInUnion) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CoyieldExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record328 {
  pub c0: u64, // id
}

pub struct TableBuilder328 {
  db_url: String,
  records: Vec<Record328>,
}

impl TableBuilder328 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record328) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO CoyieldExpr (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CUDAKernelCallExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record329 {
  pub c0: u64, // id
  pub c1: u64, // getConfig
}

pub struct TableBuilder329 {
  db_url: String,
  records: Vec<Record329>,
}

impl TableBuilder329 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record329) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
      ];
      conn.execute(
        "INSERT INTO CUDAKernelCallExpr (id, getConfig) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CoroutineSuspendExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record330 {
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

pub struct TableBuilder330 {
  db_url: String,
  records: Vec<Record330>,
}

impl TableBuilder330 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record330) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO CoroutineSuspendExpr (id, getCommonExpr, getOpaqueValue, getReadyExpr, getSuspendExpr, getResumeExpr, getOperand, getKeywordLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// UnaryOperator
#[derive(Debug, Serialize, Deserialize)]
pub struct Record331 {
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

pub struct TableBuilder331 {
  db_url: String,
  records: Vec<Record331>,
}

impl TableBuilder331 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record331) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 15] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &record.c5,
        &record.c6,
        &record.c7,
        &record.c8,
        &record.c9,
        &record.c10,
        &(record.c11 as i64),
        &(record.c12 as i64),
        &(record.c13 as i64),
        &record.c14,
      ];
      conn.execute(
        "INSERT INTO UnaryOperator (id, getOpcode, getSubExpr, getOperatorLoc, canOverflow, isPrefix, isPostfix, isIncrementOp, isDecrementOp, isIncrementDecrementOp, isArithmeticOp, getBeginLoc, getEndLoc, getExprLoc, hasStoredFPFeatures) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DependentCoawaitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record332 {
  pub c0: u64, // id
  pub c1: u64, // getOperand
  pub c2: u64, // getOperatorCoawaitLookup
  pub c3: u64, // getKeywordLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder332 {
  db_url: String,
  records: Vec<Record332>,
}

impl TableBuilder332 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record332) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO DependentCoawaitExpr (id, getOperand, getOperatorCoawaitLookup, getKeywordLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ConceptSpecializationExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record333 {
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

pub struct TableBuilder333 {
  db_url: String,
  records: Vec<Record333>,
}

impl TableBuilder333 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record333) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO ConceptSpecializationExpr (id, getNamedConcept, hasExplicitTemplateArgs, getConceptNameLoc, getTemplateKWLoc, getFoundDecl, getSpecializationDecl, getBeginLoc, getEndLoc, getExprLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// LambdaExpr_capture_inits
#[derive(Debug, Serialize, Deserialize)]
pub struct Record334 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder334 {
  db_url: String,
  records: Vec<Record334>,
}

impl TableBuilder334 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record334) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO LambdaExpr_capture_inits (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// LambdaExpr_getExplicitTemplateParameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Record335 {
  pub c0: u64, // id
  pub c1: u64, // idx
  pub c2: u64, // element
}

pub struct TableBuilder335 {
  db_url: String,
  records: Vec<Record335>,
}

impl TableBuilder335 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record335) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO LambdaExpr_getExplicitTemplateParameters (id, idx, element) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// LambdaExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record336 {
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

pub struct TableBuilder336 {
  db_url: String,
  records: Vec<Record336>,
}

impl TableBuilder336 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record336) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 17] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &record.c9,
        &(record.c10 as i64),
        &(record.c11 as i64),
        &record.c12,
        &record.c13,
        &record.c14,
        &(record.c15 as i64),
        &(record.c16 as i64),
      ];
      conn.execute(
        "INSERT INTO LambdaExpr (id, getCaptureDefault, getCaptureDefaultLoc, capture_size, getIntroducerRange, getLambdaClass, getCallOperator, getDependentCallOperator, getTrailingRequiresClause, isGenericLambda, getBody, getCompoundStmtBody, isMutable, hasExplicitParameters, hasExplicitResultType, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ExprWithCleanups
#[derive(Debug, Serialize, Deserialize)]
pub struct Record337 {
  pub c0: u64, // id
  pub c1: u32, // getNumObjects
  pub c2: bool, // cleanupsHaveSideEffects
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

pub struct TableBuilder337 {
  db_url: String,
  records: Vec<Record337>,
}

impl TableBuilder337 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record337) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO ExprWithCleanups (id, getNumObjects, cleanupsHaveSideEffects, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SYCLUniqueStableNameExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record338 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
  pub c3: u64, // getLocation
  pub c4: u64, // getLParenLocation
  pub c5: u64, // getRParenLocation
}

pub struct TableBuilder338 {
  db_url: String,
  records: Vec<Record338>,
}

impl TableBuilder338 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record338) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO SYCLUniqueStableNameExpr (id, getBeginLoc, getEndLoc, getLocation, getLParenLocation, getRParenLocation) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DesignatedInitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record339 {
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

pub struct TableBuilder339 {
  db_url: String,
  records: Vec<Record339>,
}

impl TableBuilder339 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record339) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &record.c6,
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO DesignatedInitExpr (id, size, getEqualOrColonLoc, isDirectInit, usesGNUSyntax, getInit, getNumSubExprs, getDesignatorsSourceRange, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// TypoExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record340 {
  pub c0: u64, // id
  pub c1: u64, // getBeginLoc
  pub c2: u64, // getEndLoc
}

pub struct TableBuilder340 {
  db_url: String,
  records: Vec<Record340>,
}

impl TableBuilder340 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record340) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 3] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
      ];
      conn.execute(
        "INSERT INTO TypoExpr (id, getBeginLoc, getEndLoc) VALUES ($1, $2, $3)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXDeleteExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record341 {
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

pub struct TableBuilder341 {
  db_url: String,
  records: Vec<Record341>,
}

impl TableBuilder341 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record341) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 10] = [
        &(record.c0 as i64),
        &record.c1,
        &record.c2,
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXDeleteExpr (id, isGlobalDelete, isArrayForm, isArrayFormAsWritten, doesUsualArrayDeleteWantSize, getOperatorDelete, getArgument, getDestroyedType, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CallExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record342 {
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

pub struct TableBuilder342 {
  db_url: String,
  records: Vec<Record342>,
}

impl TableBuilder342 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record342) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 13] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &record.c3,
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
        &record.c7,
        &record.c8,
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &record.c12,
      ];
      conn.execute(
        "INSERT INTO CallExpr (id, getCallee, getADLCallKind, usesADL, hasStoredFPFeatures, getCalleeDecl, getDirectCallee, getNumArgs, getBuiltinCallee, getRParenLoc, getBeginLoc, getEndLoc, isCallToStdMove) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CompoundLiteralExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record343 {
  pub c0: u64, // id
  pub c1: u64, // getInitializer
  pub c2: bool, // isFileScope
  pub c3: u64, // getLParenLoc
  pub c4: u64, // getBeginLoc
  pub c5: u64, // getEndLoc
}

pub struct TableBuilder343 {
  db_url: String,
  records: Vec<Record343>,
}

impl TableBuilder343 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record343) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 6] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
      ];
      conn.execute(
        "INSERT INTO CompoundLiteralExpr (id, getInitializer, isFileScope, getLParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MaterializeTemporaryExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record344 {
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

pub struct TableBuilder344 {
  db_url: String,
  records: Vec<Record344>,
}

impl TableBuilder344 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record344) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 9] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &record.c5,
        &record.c6,
        &(record.c7 as i64),
        &(record.c8 as i64),
      ];
      conn.execute(
        "INSERT INTO MaterializeTemporaryExpr (id, getSubExpr, getStorageDuration, getLifetimeExtendedTemporaryDecl, getExtendingDecl, getManglingNumber, isBoundToLvalueReference, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SourceLocExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record345 {
  pub c0: u64, // id
  pub c1: String, // getBuiltinStr
  pub c2: u64, // getIdentKind
  pub c3: bool, // isIntType
  pub c4: u64, // getLocation
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder345 {
  db_url: String,
  records: Vec<Record345>,
}

impl TableBuilder345 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record345) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &record.c1,
        &(record.c2 as i64),
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO SourceLocExpr (id, getBuiltinStr, getIdentKind, isIntType, getLocation, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CoawaitExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record346 {
  pub c0: u64, // id
  pub c1: bool, // isImplicit
}

pub struct TableBuilder346 {
  db_url: String,
  records: Vec<Record346>,
}

impl TableBuilder346 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record346) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 2] = [
        &(record.c0 as i64),
        &record.c1,
      ];
      conn.execute(
        "INSERT INTO CoawaitExpr (id, isImplicit) VALUES ($1, $2)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CoreturnStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record347 {
  pub c0: u64, // id
  pub c1: u64, // getKeywordLoc
  pub c2: u64, // getOperand
  pub c3: u64, // getPromiseCall
  pub c4: bool, // isImplicit
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder347 {
  db_url: String,
  records: Vec<Record347>,
}

impl TableBuilder347 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record347) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO CoreturnStmt (id, getKeywordLoc, getOperand, getPromiseCall, isImplicit, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXConstCastExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record348 {
  pub c0: u64, // id
}

pub struct TableBuilder348 {
  db_url: String,
  records: Vec<Record348>,
}

impl TableBuilder348 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record348) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 1] = [
        &(record.c0 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXConstCastExpr (id) VALUES ($1)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// ArrayInitLoopExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record349 {
  pub c0: u64, // id
  pub c1: u64, // getCommonExpr
  pub c2: u64, // getSubExpr
  pub c3: u64, // getBeginLoc
  pub c4: u64, // getEndLoc
}

pub struct TableBuilder349 {
  db_url: String,
  records: Vec<Record349>,
}

impl TableBuilder349 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record349) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 5] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
      ];
      conn.execute(
        "INSERT INTO ArrayInitLoopExpr (id, getCommonExpr, getSubExpr, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// MSDependentExistsStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record350 {
  pub c0: u64, // id
  pub c1: u64, // getKeywordLoc
  pub c2: bool, // isIfExists
  pub c3: bool, // isIfNotExists
  pub c4: u64, // getSubStmt
  pub c5: u64, // getBeginLoc
  pub c6: u64, // getEndLoc
}

pub struct TableBuilder350 {
  db_url: String,
  records: Vec<Record350>,
}

impl TableBuilder350 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record350) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO MSDependentExistsStmt (id, getKeywordLoc, isIfExists, isIfNotExists, getSubStmt, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// CXXFoldExpr
#[derive(Debug, Serialize, Deserialize)]
pub struct Record351 {
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

pub struct TableBuilder351 {
  db_url: String,
  records: Vec<Record351>,
}

impl TableBuilder351 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record351) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 14] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &record.c4,
        &record.c5,
        &(record.c6 as i64),
        &(record.c7 as i64),
        &(record.c8 as i64),
        &(record.c9 as i64),
        &(record.c10 as i64),
        &(record.c11 as i64),
        &(record.c12 as i64),
        &(record.c13 as i64),
      ];
      conn.execute(
        "INSERT INTO CXXFoldExpr (id, getCallee, getLHS, getRHS, isRightFold, isLeftFold, getPattern, getInit, getLParenLoc, getRParenLoc, getEllipsisLoc, getOperator, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// SEHLeaveStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record352 {
  pub c0: u64, // id
  pub c1: u64, // getLeaveLoc
  pub c2: u64, // getBeginLoc
  pub c3: u64, // getEndLoc
}

pub struct TableBuilder352 {
  db_url: String,
  records: Vec<Record352>,
}

impl TableBuilder352 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record352) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 4] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
      ];
      conn.execute(
        "INSERT INTO SEHLeaveStmt (id, getLeaveLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

// DoStmt
#[derive(Debug, Serialize, Deserialize)]
pub struct Record353 {
  pub c0: u64, // id
  pub c1: u64, // getCond
  pub c2: u64, // getBody
  pub c3: u64, // getDoLoc
  pub c4: u64, // getWhileLoc
  pub c5: u64, // getRParenLoc
  pub c6: u64, // getBeginLoc
  pub c7: u64, // getEndLoc
}

pub struct TableBuilder353 {
  db_url: String,
  records: Vec<Record353>,
}

impl TableBuilder353 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record353) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 8] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &(record.c2 as i64),
        &(record.c3 as i64),
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
        &(record.c7 as i64),
      ];
      conn.execute(
        "INSERT INTO DoStmt (id, getCond, getBody, getDoLoc, getWhileLoc, getRParenLoc, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

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

pub struct TableBuilder354 {
  db_url: String,
  records: Vec<Record354>,
}

impl TableBuilder354 {
  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
    Self {
      db_url: db_url.as_ref().to_owned(),
      records: Vec::new(),
    }
  }

  pub fn push(&mut self, record: Record354) {
    self.records.push(record);
  }

  pub async fn flush(&self) -> Result<()> {
    if self.records.is_empty() {
      return Ok(());
    }
    let pool = self.get_pool()?;
    let mut conn = pool.get().await?;
    for record in &self.records {
      let params: [&(dyn types::ToSql + Sync); 7] = [
        &(record.c0 as i64),
        &(record.c1 as i64),
        &record.c2,
        &record.c3,
        &(record.c4 as i64),
        &(record.c5 as i64),
        &(record.c6 as i64),
      ];
      conn.execute(
        "INSERT INTO FloatingLiteral (id, getRawSemantics, isExact, getValueAsApproximateDouble, getLocation, getBeginLoc, getEndLoc) VALUES ($1, $2, $3, $4, $5, $6, $7)", 
        &params
      ).await?;
    }
    Ok(())
  }

  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {
    let mut config = deadpool_postgres::Config::new();
    config.url = Some(self.db_url.clone());
    use deadpool_postgres::tokio_postgres::NoTls;
    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
  }

  pub async fn cancel(&self) -> Result<()> {
    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back
    Ok(())
  }

}

////   END ARBORETUM GENERATED CODE ////
