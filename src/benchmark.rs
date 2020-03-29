
use ::criterion::{black_box, criterion_group, criterion_main, Criterion};
use ::number2name::Charset;
use ::number2name::BASE32HUMAN;

pub fn encode(c: &mut Criterion) {
    let charset: &Charset = &*BASE32HUMAN;
    c.bench_function("encoding", |b| b.iter(|| {
        charset.encode(black_box(576));
        charset.encode(black_box(120));
        charset.encode(black_box(184));
        charset.encode(black_box(604));
        charset.encode(black_box(986));
        charset.encode(black_box(834));
        charset.encode(black_box(556));
        charset.encode(black_box(280));
        charset.encode(black_box(775));
        charset.encode(black_box(228));
        charset.encode(black_box(042));
        charset.encode(black_box(860));
        charset.encode(black_box(680));
        charset.encode(black_box(757));
        charset.encode(black_box(488));
        charset.encode(black_box(344));
        charset.encode(black_box(389));
        charset.encode(black_box(111));
        charset.encode(black_box(678));
        charset.encode(black_box(915));
        charset.encode(black_box(094));
        charset.encode(black_box(168));
        charset.encode(black_box(087));
        charset.encode(black_box(441));
        charset.encode(black_box(416));
        charset.encode(black_box(651_749));
        charset.encode(black_box(110_129));
        charset.encode(black_box(071_308));
        charset.encode(black_box(519_164));
        charset.encode(black_box(357_502));
        charset.encode(black_box(825_540));
        charset.encode(black_box(688_862));
        charset.encode(black_box(517_890));
        charset.encode(black_box(536_327));
        charset.encode(black_box(809_673));
        charset.encode(black_box(789_323));
        charset.encode(black_box(281_633));
        charset.encode(black_box(692_341));
        charset.encode(black_box(045_006));
        charset.encode(black_box(455_303));
        charset.encode(black_box(158_880));
        charset.encode(black_box(271_265));
        charset.encode(black_box(363_808));
        charset.encode(black_box(418_819));
        charset.encode(black_box(395_972));
        charset.encode(black_box(405_352));
        charset.encode(black_box(461_224));
        charset.encode(black_box(481_338));
        charset.encode(black_box(166_457));
        charset.encode(black_box(082_585));
        charset.encode(black_box(917_997_698_978));
        charset.encode(black_box(059_046_100_344));
        charset.encode(black_box(258_757_884_037));
        charset.encode(black_box(586_675_931_082));
        charset.encode(black_box(352_445_193_904));
        charset.encode(black_box(576_897_310_672));
        charset.encode(black_box(435_733_284_516));
        charset.encode(black_box(001_082_163_840));
        charset.encode(black_box(313_979_438_321));
        charset.encode(black_box(090_446_256_394));
        charset.encode(black_box(311_050_840_270));
        charset.encode(black_box(870_567_988_398));
        charset.encode(black_box(310_485_975_096));
        charset.encode(black_box(796_509_295_345));
        charset.encode(black_box(476_110_984_672));
        charset.encode(black_box(231_570_407_893));
        charset.encode(black_box(540_557_813_570));
        charset.encode(black_box(693_931_292_960));
        charset.encode(black_box(443_883_629_278));
        charset.encode(black_box(313_478_642_651));
        charset.encode(black_box(982_091_285_302));
        charset.encode(black_box(918_351_405_980));
        charset.encode(black_box(614_205_465_822));
        charset.encode(black_box(462_214_570_388));
        charset.encode(black_box(129_207_888_704));
        charset.encode(black_box(704_375_709_625_294_469));
        charset.encode(black_box(677_658_304_480_761_669));
        charset.encode(black_box(480_706_666_433_187_535));
        charset.encode(black_box(390_302_265_436_167_379));
        charset.encode(black_box(448_145_180_750_793_245));
        charset.encode(black_box(291_807_797_351_780_789));
        charset.encode(black_box(925_047_676_495_726_651));
        charset.encode(black_box(011_459_256_733_501_417));
        charset.encode(black_box(951_535_399_062_262_497));
        charset.encode(black_box(211_460_552_440_647_076));
        charset.encode(black_box(821_280_206_634_023_006));
        charset.encode(black_box(579_570_999_092_290_400));
        charset.encode(black_box(279_205_006_687_550_235));
        charset.encode(black_box(278_327_264_214_350_732));
        charset.encode(black_box(740_442_064_276_382_784));
        charset.encode(black_box(869_850_708_965_439_213));
        charset.encode(black_box(941_066_654_859_166_871));
        charset.encode(black_box(807_685_469_414_522_852));
        charset.encode(black_box(854_376_460_486_286_172));
        charset.encode(black_box(273_843_778_936_877_580));
        charset.encode(black_box(324_889_628_346_191_041));
        charset.encode(black_box(596_578_767_027_239_687));
        charset.encode(black_box(426_071_395_988_457_303));
        charset.encode(black_box(428_636_909_288_626_891));
        charset.encode(black_box(870_130_888_288_961_047));
    }));
}

pub fn decode(c: &mut Criterion) {
    let charset: &Charset = &*BASE32HUMAN;
    c.bench_function("encoding", |b| b.iter(|| {
        charset.decode(black_box("x52")).unwrap();
        charset.decode(black_box("dha")).unwrap();
        charset.decode(black_box("d8r")).unwrap();
        charset.decode(black_box("2ar")).unwrap();
        charset.decode(black_box("_82")).unwrap();
        charset.decode(black_box("8mr")).unwrap();
        charset.decode(black_box("xx2")).unwrap();
        charset.decode(black_box("h8a")).unwrap();
        charset.decode(black_box("55x")).unwrap();
        charset.decode(black_box("hh8")).unwrap();
        charset.decode(black_box("arh")).unwrap();
        charset.decode(black_box("82a")).unwrap();
        charset.decode(black_box("28a")).unwrap();
        charset.decode(black_box("5x5")).unwrap();
        charset.decode(black_box("r88")).unwrap();
        charset.decode(black_box("mrr")).unwrap();
        charset.decode(black_box("m8_")).unwrap();
        charset.decode(black_box("ddd")).unwrap();
        charset.decode(black_box("258")).unwrap();
        charset.decode(black_box("_dx")).unwrap();
        charset.decode(black_box("a_r")).unwrap();
        charset.decode(black_box("d28")).unwrap();
        charset.decode(black_box("a85")).unwrap();
        charset.decode(black_box("rrd")).unwrap();
        charset.decode(black_box("rd2")).unwrap();
        charset.decode(black_box("2xd5r_")).unwrap();
        charset.decode(black_box("ddadh_")).unwrap();
        charset.decode(black_box("a5dma8")).unwrap();
        charset.decode(black_box("xd_d2r")).unwrap();
        charset.decode(black_box("mx5xah")).unwrap();
        charset.decode(black_box("8hxxra")).unwrap();
        charset.decode(black_box("28882h")).unwrap();
        charset.decode(black_box("xd58_a")).unwrap();
        charset.decode(black_box("xm2mh5")).unwrap();
        charset.decode(black_box("8a_25m")).unwrap();
        charset.decode(black_box("58_mhm")).unwrap();
        charset.decode(black_box("h8d2mm")).unwrap();
        charset.decode(black_box("2_hmrd")).unwrap();
        charset.decode(black_box("arxaa2")).unwrap();
        charset.decode(black_box("rxxmam")).unwrap();
        charset.decode(black_box("dx888a")).unwrap();
        charset.decode(black_box("h5dh2x")).unwrap();
        charset.decode(black_box("m2m8a8")).unwrap();
        charset.decode(black_box("rd88d_")).unwrap();
        charset.decode(black_box("m_x_5h")).unwrap();
        charset.decode(black_box("raxmxh")).unwrap();
        charset.decode(black_box("r2dhhr")).unwrap();
        charset.decode(black_box("r8dmm8")).unwrap();
        charset.decode(black_box("d22rx5")).unwrap();
        charset.decode(black_box("a8hx8x")).unwrap();
        charset.decode(black_box("_d5__52_8_58")).unwrap();
        charset.decode(black_box("ax_ar2daamrr")).unwrap();
        charset.decode(black_box("hx85x588ram5")).unwrap();
        charset.decode(black_box("x8225x_mda8h")).unwrap();
        charset.decode(black_box("mxhrrxd_m_ar")).unwrap();
        charset.decode(black_box("x528_5mda25h")).unwrap();
        charset.decode(black_box("rmx5mmh8rxd2")).unwrap();
        charset.decode(black_box("aada8hd2m8ra")).unwrap();
        charset.decode(black_box("mdm_5_rm8mhd")).unwrap();
        charset.decode(black_box("a_arr2hx2m_r")).unwrap();
        charset.decode(black_box("mddaxa8rah5a")).unwrap();
        charset.decode(black_box("85ax25_88m_8")).unwrap();
        charset.decode(black_box("mdar8x_5xa_2")).unwrap();
        charset.decode(black_box("5_2xa_h_xmrx")).unwrap();
        charset.decode(black_box("r52dda_8r25h")).unwrap();
        charset.decode(black_box("hmdx5ara58_m")).unwrap();
        charset.decode(black_box("xraxx58dmx5a")).unwrap();
        charset.decode(black_box("2_m_mdh_h_2a")).unwrap();
        charset.decode(black_box("rrm88m2h_h58")).unwrap();
        charset.decode(black_box("mdmr582rh2xd")).unwrap();
        charset.decode(black_box("_8ha_dh8xmah")).unwrap();
        charset.decode(black_box("_d8mxdrax_8a")).unwrap();
        charset.decode(black_box("2drhaxr2x8hh")).unwrap();
        charset.decode(black_box("r2hhdrx5am88")).unwrap();
        charset.decode(black_box("dh_ha58885ar")).unwrap();
        charset.decode(black_box("5arm5x5a_2hxh")).unwrap();
        charset.decode(black_box("2552x8marr8a5")).unwrap();
        charset.decode(black_box("r8a5a2222rmmd")).unwrap();
        charset.decode(black_box("m_amahh2xrm2d")).unwrap();
        charset.decode(black_box("rr8drxd8a5xa5")).unwrap();
        charset.decode(black_box("h_d8a55_5mxd5")).unwrap();
        charset.decode(black_box("_hxar5252r_x5")).unwrap();
        charset.decode(black_box("addrx_hx25mmx")).unwrap();
        charset.decode(black_box("_xdxmxm__a2hh")).unwrap();
        charset.decode(black_box("hddr2axxhrra2")).unwrap();
        charset.decode(black_box("8hdh8aha22mra")).unwrap();
        charset.decode(black_box("x5_x5a___a_hh")).unwrap();
        charset.decode(black_box("h5_haxaa2285x")).unwrap();
        charset.decode(black_box("h58mh5h2rhdrm")).unwrap();
        charset.decode(black_box("5rarrha2rh52m")).unwrap();
        charset.decode(black_box("82_8xa5a8_2xr")).unwrap();
        charset.decode(black_box("_rda222xr8x_d")).unwrap();
        charset.decode(black_box("8a528xr2_rdrx")).unwrap();
        charset.decode(black_box("8xrm52r2ar82h")).unwrap();
        charset.decode(black_box("h5m8rm558_m28")).unwrap();
        charset.decode(black_box("mhr88_2h8mr2d2")).unwrap_err();
        charset.decode(black_box("x_2x58525ah5h2")).unwrap_err();
        charset.decode(black_box("rh2a5dm_x_88r2")).unwrap_err();
        charset.decode(black_box("rh82m2_a_h8822")).unwrap_err();
        charset.decode(black_box("85adma888h88_2")).unwrap_err();
    }));
}

criterion_group!(benches, decode);
criterion_main!(benches);
