use math::big::Int;

use strconv::NumErrorCause;

mod helper;

lazy_static::lazy_static! {
  static ref BITWISE_TESTS: Vec<BitwiseTest> = vec![
    BitwiseTest::new("0x00", "0x00", "0x00", "0x00", "0x00", "0x00"),
    BitwiseTest::new("0x00", "0x01", "0x00", "0x01", "0x01", "0x00"),
    BitwiseTest::new("0x01", "0x00", "0x00", "0x01", "0x01", "0x01"),
    BitwiseTest::new("-0x01", "0x00", "0x00", "-0x01", "-0x01", "-0x01"),
    BitwiseTest::new("-0xaf", "-0x50", "-0xf0", "-0x0f", "0xe1", "0x41"),
    BitwiseTest::new("0x00", "-0x01", "0x00", "-0x01", "-0x01", "0x00"),
    BitwiseTest::new("0x01", "0x01", "0x01", "0x01", "0x00", "0x00"),
    BitwiseTest::new("-0x01", "-0x01", "-0x01", "-0x01", "0x00", "0x00"),
    BitwiseTest::new("0x07", "0x08", "0x00", "0x0f", "0x0f", "0x07"),
    BitwiseTest::new("0x05", "0x0f", "0x05", "0x0f", "0x0a", "0x00"),
    BitwiseTest::new("0xff", "-0x0a", "0xf6", "-0x01", "-0xf7", "0x09"),
    BitwiseTest::new("0x013ff6", "0x9a4e", "0x1a46", "0x01bffe", "0x01a5b8", "0x0125b0"),
    BitwiseTest::new("-0x013ff6", "0x9a4e", "0x800a", "-0x0125b2", "-0x01a5bc", "-0x01c000"),
    BitwiseTest::new("-0x013ff6", "-0x9a4e", "-0x01bffe", "-0x1a46", "0x01a5b8", "0x8008"),
    BitwiseTest::new(
        "0x1000009dc6e3d9822cba04129bcbe3401",
        "0xb9bd7d543685789d57cb918e833af352559021483cdb05cc21fd",
        "0x1000001186210100001000009048c2001",
        "0xb9bd7d543685789d57cb918e8bfeff7fddb2ebe87dfbbdfe35fd",
        "0xb9bd7d543685789d57ca918e8ae69d6fcdb2eae87df2b97215fc",
        "0x8c40c2d8822caa04120b8321400",
    ),
    BitwiseTest::new(
        "0x1000009dc6e3d9822cba04129bcbe3401",
        "-0xb9bd7d543685789d57cb918e833af352559021483cdb05cc21fd",
        "0x8c40c2d8822caa04120b8321401",
        "-0xb9bd7d543685789d57ca918e82229142459020483cd2014001fd",
        "-0xb9bd7d543685789d57ca918e8ae69d6fcdb2eae87df2b97215fe",
        "0x1000001186210100001000009048c2000",
    ),
    BitwiseTest::new(
        "-0x1000009dc6e3d9822cba04129bcbe3401",
        "-0xb9bd7d543685789d57cb918e833af352559021483cdb05cc21fd",
        "-0xb9bd7d543685789d57cb918e8bfeff7fddb2ebe87dfbbdfe35fd",
        "-0x1000001186210100001000009048c2001",
        "0xb9bd7d543685789d57ca918e8ae69d6fcdb2eae87df2b97215fc",
        "0xb9bd7d543685789d57ca918e82229142459020483cd2014001fc",
    ),
  ];

  static ref CMP_ABS_TESTS: Vec<&'static str> = vec![
    "0",
    "1",
    "2",
    "10",
    "10000000",
    "2783678367462374683678456387645876387564783686583485",
    "2783678367462374683678456387645876387564783686583486",
    "32957394867987420967976567076075976570670947609750670956097509670576075067076027578341538",
  ];

  static ref LSH_TESTS: Vec<IntShiftTest> = vec![
    IntShiftTest::new("0", 0, "0"),
    IntShiftTest::new("0", 1, "0"),
    IntShiftTest::new("0", 2, "0"),
    IntShiftTest::new("1", 0, "1"),
    IntShiftTest::new("1", 1, "2"),
    IntShiftTest::new("1", 2, "4"),
    IntShiftTest::new("2", 0, "2"),
    IntShiftTest::new("2", 1, "4"),
    IntShiftTest::new("2", 2, "8"),
    IntShiftTest::new("-87", 1, "-174"),
    IntShiftTest::new("4294967296", 0, "4294967296"),
    IntShiftTest::new("4294967296", 1, "8589934592"),
    IntShiftTest::new("4294967296", 2, "17179869184"),
    IntShiftTest::new("18446744073709551616", 0, "18446744073709551616"),
    IntShiftTest::new("9223372036854775808", 1, "18446744073709551616"),
    IntShiftTest::new("4611686018427387904", 2, "18446744073709551616"),
    IntShiftTest::new("1", 64, "18446744073709551616"),
    IntShiftTest::new(
        "18446744073709551616",
        64,
        "340282366920938463463374607431768211456",
    ),
    IntShiftTest::new("1", 128, "340282366920938463463374607431768211456"),
  ];

  static ref PROD_ZZ: Vec<ArgZz> = vec![
    ArgZz::from_i64s(0, 0, 0),
    ArgZz::from_i64s(0, 1, 0),
    ArgZz::from_i64s(1, 1, 1),
    ArgZz::from_i64s(-991*991, 991, -991),
  ];

  static ref RSH_TESTS: Vec<IntShiftTest> = vec![
    IntShiftTest::new("0", 0, "0"),
    IntShiftTest::new("-0", 0, "0"),
    IntShiftTest::new("0", 1, "0"),
    IntShiftTest::new("0", 2, "0"),
    IntShiftTest::new("1", 0, "1"),
    IntShiftTest::new("1", 1, "0"),
    IntShiftTest::new("1", 2, "0"),
    IntShiftTest::new("2", 0, "2"),
    IntShiftTest::new("2", 1, "1"),
    IntShiftTest::new("-1", 0, "-1"),
    IntShiftTest::new("-1", 1, "-1"),
    IntShiftTest::new("-1", 10, "-1"),
    IntShiftTest::new("-100", 2, "-25"),
    IntShiftTest::new("-100", 3, "-13"),
    IntShiftTest::new("-100", 100, "-1"),
    IntShiftTest::new("4294967296", 0, "4294967296"),
    IntShiftTest::new("4294967296", 1, "2147483648"),
    IntShiftTest::new("4294967296", 2, "1073741824"),
    IntShiftTest::new("18446744073709551616", 0, "18446744073709551616"),
    IntShiftTest::new("18446744073709551616", 1, "9223372036854775808"),
    IntShiftTest::new("18446744073709551616", 2, "4611686018427387904"),
    IntShiftTest::new("18446744073709551616", 64, "1"),
    IntShiftTest::new(
        "340282366920938463463374607431768211456",
        64,
        "18446744073709551616",
    ),
    IntShiftTest::new("340282366920938463463374607431768211456", 128, "1"),
  ];

  static ref SUM_ZZ: Vec<ArgZz> = vec![
    ArgZz::new(Int::new(0), Int::new(0), Int::new(0)),
    ArgZz::new(Int::new(1), Int::new(1), Int::new(0)),
    ArgZz::new(Int::new(1111111110), Int::new(123456789), Int::new(987654321)),
    ArgZz::from_i64s(-1, -1, 0),
    ArgZz::from_i64s(864197532, -123456789, 987654321),
    ArgZz::from_i64s(-1111111110, -123456789, -987654321),
  ];
}

#[derive(Clone, Debug)]
struct ArgZz {
    z: Int,
    x: Int,
    y: Int,
}

struct BitwiseTest {
    x: &'static str,
    y: &'static str,
    and: &'static str,
    or: &'static str,
    xor: &'static str,
    and_not: &'static str,
}

struct IntShiftTest {
    input: &'static str,
    shift: usize,
    out: &'static str,
}

impl ArgZz {
    fn new(z: Int, x: Int, y: Int) -> Self {
        Self { z, x, y }
    }

    fn from_i64s(z: i64, x: i64, y: i64) -> Self {
        Self {
            z: Int::new(z),
            x: Int::new(x),
            y: Int::new(y),
        }
    }
}

impl BitwiseTest {
    fn new(
        x: &'static str,
        y: &'static str,
        and: &'static str,
        or: &'static str,
        xor: &'static str,
        and_not: &'static str,
    ) -> Self {
        Self {
            x,
            y,
            and,
            or,
            xor,
            and_not,
        }
    }
}

impl IntShiftTest {
    fn new(input: &'static str, shift: usize, out: &'static str) -> Self {
        Self { input, shift, out }
    }
}

#[test]
fn abs_z() {
    let zero = Int::default();
    for a in SUM_ZZ.iter() {
        let mut z = Int::default();
        z.abs(&a.z);
        let mut e = Int::default();
        e.set(&a.z);
        if e.cmp(&zero) < 0 {
            let v = e.clone();
            e.sub(&zero, &v);
        }

        assert_eq!(z, e);
    }
}

#[test]
fn binomial() {
    struct Case {
        n: i64,
        k: i64,
        want: &'static str,
    }
    let new_case = |n, k, want| -> Case { Case { n, k, want } };

    let test_vector = vec![
        new_case(0, 0, "1"),
        new_case(0, 1, "0"),
        new_case(1, 0, "1"),
        new_case(1, 1, "1"),
        new_case(1, 10, "0"),
        new_case(4, 0, "1"),
        new_case(4, 1, "4"),
        new_case(4, 2, "6"),
        new_case(4, 3, "4"),
        new_case(4, 4, "1"),
        new_case(10, 1, "10"),
        new_case(10, 9, "10"),
        new_case(10, 5, "252"),
        new_case(11, 5, "462"),
        new_case(11, 6, "462"),
        new_case(100, 10, "17310309456440"),
        new_case(100, 90, "17310309456440"),
        new_case(1000, 10, "263409560461970212832400"),
        new_case(1000, 990, "263409560461970212832400"),
    ];

    let mut z = Int::default();
    for c in test_vector {
        let got = z.binomial(c.n, c.k).to_string();
        assert_eq!(c.want, got, "binomial({},{})", c.n, c.k);
    }
}

#[test]
fn bit_len() {
    struct Case {
        input: &'static str,
        out: usize,
    }

    let new_case = |input, out| Case { input, out };

    let test_vector = vec![
        new_case("-1", 1),
        new_case("0", 0),
        new_case("1", 1),
        new_case("2", 2),
        new_case("4", 3),
        new_case("0xabc", 12),
        new_case("0x8000", 16),
        new_case("0x80000000", 32),
        new_case("0x800000000000", 48),
        new_case("0x8000000000000000", 64),
        new_case("0x80000000000000000000", 80),
        new_case("-0x4000000000000000000000", 87),
    ];

    for (i, c) in test_vector.iter().enumerate() {
        let mut x = Int::default();
        assert!(
            x.set_string(c.input, 0).is_some(),
            "#{i} set_string({}, 0)",
            c.input
        );

        assert_eq!(x.bit_len(), c.out, "#{i} bit_len");
    }
}

#[test]
fn bit_set() {
    struct Case {
        x: &'static str,
        i: usize,
        b: u8,
    }

    let new_case = |x, i, b| Case { x, i, b };

    let bitset_test_vector = vec![
        new_case("0", 0, 0),
        new_case("0", 200, 0),
        new_case("1", 0, 1),
        new_case("1", 1, 0),
        new_case("-1", 0, 1),
        new_case("-1", 200, 1),
        new_case("0x2000000000000000000000000000", 108, 0),
        new_case("0x2000000000000000000000000000", 109, 1),
        new_case("0x2000000000000000000000000000", 110, 0),
        new_case("-0x2000000000000000000000000001", 108, 1),
        new_case("-0x2000000000000000000000000001", 109, 0),
        new_case("-0x2000000000000000000000000001", 110, 1),
    ];

    for c in BITWISE_TESTS.iter() {
        let x = int_from_str(c.x, None);
        test_bitset(&x);

        let y = int_from_str(c.y, None);
        test_bitset(&y);
    }

    for (i, c) in bitset_test_vector.iter().enumerate() {
        let x = int_from_str(c.x, None);
        assert_eq!(x.bit(c.i), c.b, "#{i} x={x}");
    }
}

#[test]
fn bits() {
    let test_vector = vec![
        vec![0u32],
        vec![1],
        vec![0, 1, 2, 3, 4],
        vec![4, 3, 2, 1, 0],
        vec![4, 3, 2, 1, 0, 0, 0, 0],
    ];

    for c in test_vector {
        let mut z = Int::default();

        let got = z.set_bits(c.as_slice());

        assert!(got.sign() >= 0, "set_bits({c:?}): get negative result");

        let want = norm(c.as_slice());
        let got: Vec<u32> = got.bits().collect();
        assert_eq!(got, want, "set_bits({c:?})");

        let bits: Vec<u32> = z.bits().collect();
        assert_eq!(
            bits,
            want,
            "{:?}.bits() = {:?}; want {:?}",
            z.bytes(),
            bits,
            want
        );
    }
}

#[test]
fn bitwise() {
    let mut x = Int::default();
    let mut y = Int::default();
    for c in BITWISE_TESTS.iter() {
        x.set_string(c.x, 0)
            .expect(&format!("x.set_string({}, 0)", c.x));
        y.set_string(c.y, 0)
            .expect(&format!("y.set_string({}, 0)", c.y));

        test_bit_fun("and", Int::and, &x, &y, c.and);
        test_bit_fun("and_not", Int::and_not, &x, &y, c.and_not);
        test_bit_fun("or", Int::or, &x, &y, c.or);
        test_bit_fun("xor", Int::xor, &x, &y, c.xor);
    }
}

#[test]
fn bytes() {
    let n = randn(128, 256);
    for _ in 0..n {
        let b = rand_bytes(randn(1, 64));
        check_bytes(b.as_slice());
    }
}

#[test]
fn cmp_abs() {
    let mut values = Vec::with_capacity(CMP_ABS_TESTS.len());
    let mut prev = None::<Int>;
    for s in CMP_ABS_TESTS.iter() {
        let mut x = Int::default();
        x.set_string(*s, 0).expect(&format!("set_string({s}, 0)"));

        if let Some(v) = prev {
            assert!(
                v.cmp(&x) < 0,
                "CMP_ABS_TESTS entries not sorted in ascending order"
            );
        }

        values.push(x.clone());
        prev = Some(x);
    }

    fn negate(x: &mut Int) {
        let mut v = Int::default();
        v.neg(&x);
        *x = v;
    }

    for (i, x) in values.as_slice().iter().enumerate() {
        for (j, y) in values.as_slice().iter().enumerate() {
            for k in 0..4 {
                let mut a = x.clone();
                let mut b = y.clone();

                if (k & 1) != 0 {
                    negate(&mut a);
                }

                if (k & 2) != 0 {
                    negate(&mut b);
                }

                let got = a.cmp_abs(&b);
                let want = if i > j {
                    1
                } else if i < j {
                    -1
                } else {
                    0
                };
                assert_eq!(got, want, "cmp_abs |{a}|, |{b}|");
            }
        }
    }
}

#[test]
fn division_signs() {
    struct Case {
        x: i64,
        y: i64,
        q: i64,
        r: i64,
        d: i64,
        m: i64,
    }

    let new_case = |x, y, q, r, d, m| Case { x, y, q, r, d, m };

    let test_vector = vec![
        new_case(5, 3, 1, 2, 1, 2),
        new_case(-5, 3, -1, -2, -2, 1),
        new_case(5, -3, -1, 2, -1, 2),
        new_case(-5, -3, 1, -2, 2, 1),
        new_case(1, 2, 0, 1, 0, 1),
        new_case(8, 4, 2, 0, 2, 0),
    ];

    for (i, c) in test_vector.iter().enumerate() {
        let x = Int::new(c.x);
        let y = Int::new(c.y);
        let q = Int::new(c.q);
        let r = Int::new(c.r);
        let d = Int::new(c.d);
        let m = Int::new(c.m);

        let mut q1 = Int::default();
        q1.quo(&x, &y);

        let mut r1 = Int::default();
        r1.rem(&x, &y);

        assert!(is_normalized(&q1), "#{i} quo: {q1} is not normalized");
        assert!(is_normalized(&r1), "#{i} rem: {r1} is not normalized");
        assert!(
            (q1 == q) && (r1 == r),
            "#{i} quo/rem: got ({q1}, {r1}), want ({q}, {r})"
        );

        let mut q2 = Int::default();
        let mut r2 = Int::default();
        q2.quo_rem(&x, &y, &mut r2);

        assert!(is_normalized(&q2), "#{i} quo: {q2} is not normalized");
        assert!(is_normalized(&r2), "#{i} rem: {r2} is not normalized");
        assert!(
            (q2 == q) && (r2 == r),
            "#{i} quo_rem: got ({q2}, {r2}), want ({q}, {r})"
        );

        let mut d1 = Int::default();
        let mut m1 = Int::default();
        d1.div(&x, &y);
        m1.r#mod(&x, &y);

        assert!(is_normalized(&d1), "#{i} div: {d1} is not normalized");
        assert!(is_normalized(&m1), "#{i} mod: {m1} is not normalized");
        assert!(
            (d1 == d) && (m1 == m),
            "#{i} div/mod: got ({d1}, {m1}), want ({d}, {m})"
        );

        let mut d2 = Int::default();
        let mut m2 = Int::default();
        d2.div_mod(&x, &y, &mut m2);

        assert!(is_normalized(&d2), "#{i} div: {d2} is not normalized");
        assert!(is_normalized(&m2), "#{i} mod: {m2} is not normalized");
        assert!(
            (d2 == d) && (m2 == m),
            "#{i} div_mod: got ({d2}, {m2}), want ({d}, {m})"
        );
    }
}

#[test]
fn exp() {
    struct Case {
        x: &'static str,
        y: &'static str,
        m: &'static str,
        out: &'static str,
    }
    let new_case = |x, y, m, out| Case { x, y, m, out };

    let test_vector = vec![
    // y <= 0
	new_case("0", "0", "", "1"),
	new_case("1", "0", "", "1"),
	new_case("-10", "0", "", "1"),
	new_case("1234", "-1", "", "1"),
	new_case("1234", "-1", "0", "1"),
	new_case("17", "-100", "1234", "865"),
	new_case("2", "-100", "1234", ""),

	// m == 1
	new_case("0", "0", "1", "0"),
	new_case("1", "0", "1", "0"),
	new_case("-10", "0", "1", "0"),
	new_case("1234", "-1", "1", "0"),

	// misc
	new_case("5", "1", "3", "2"),
	new_case("5", "-7", "", "1"),
	new_case("-5", "-7", "", "1"),
	new_case("5", "0", "", "1"),
	new_case("-5", "0", "", "1"),
	new_case("5", "1", "", "5"),
	new_case("-5", "1", "", "-5"),
	new_case("-5", "1", "7", "2"),
	new_case("-2", "3", "2", "0"),
	new_case("5", "2", "", "25"),
	new_case("1", "65537", "2", "1"),
	new_case("0x8000000000000000", "2", "", "0x40000000000000000000000000000000"),
	new_case("0x8000000000000000", "2", "6719", "4944"),
	new_case("0x8000000000000000", "3", "6719", "5447"),
	new_case("0x8000000000000000", "1000", "6719", "1603"),
	new_case("0x8000000000000000", "1000000", "6719", "3199"),
	new_case("0x8000000000000000", "-1000000", "6719", "3663"), // 3663 = ModInverse(3199, 6719) Issue #25865

	new_case("0xffffffffffffffffffffffffffffffff", "0x12345678123456781234567812345678123456789", "0x01112222333344445555666677778889", "0x36168FA1DB3AAE6C8CE647E137F97A"),

new_case(	
		"2938462938472983472983659726349017249287491026512746239764525612965293865296239471239874193284792387498274256129746192347",
		"298472983472983471903246121093472394872319615612417471234712061",
		"29834729834729834729347290846729561262544958723956495615629569234729836259263598127342374289365912465901365498236492183464",
		"23537740700184054162508175125554701713153216681790245129157191391322321508055833908509185839069455749219131480588829346291",
),
	// test case for issue 8822
new_case(	
		"11001289118363089646017359372117963499250546375269047542777928006103246876688756735760905680604646624353196869572752623285140408755420374049317646428185270079555372763503115646054602867593662923894140940837479507194934267532831694565516466765025434902348314525627418515646588160955862839022051353653052947073136084780742729727874803457643848197499548297570026926927502505634297079527299004267769780768565695459945235586892627059178884998772989397505061206395455591503771677500931269477503508150175717121828518985901959919560700853226255420793148986854391552859459511723547532575574664944815966793196961286234040892865",
		"0xB08FFB20760FFED58FADA86DFEF71AD72AA0FA763219618FE022C197E54708BB1191C66470250FCE8879487507CEE41381CA4D932F81C2B3F1AB20B539D50DCD",
		"0xAC6BDB41324A9A9BF166DE5E1389582FAF72B6651987EE07FC3192943DB56050A37329CBB4A099ED8193E0757767A13DD52312AB4B03310DCD7F48A9DA04FD50E8083969EDB767B0CF6095179A163AB3661A05FBD5FAAAE82918A9962F0B93B855F97993EC975EEAA80D740ADBF4FF747359D041D5C33EA71D281E446B14773BCA97B43A23FB801676BD207A436C6481F1D2B9078717461A5B9D32E688F87748544523B524B0D57D5EA77A2775D2ECFA032CFBDBF52FB3786160279004E57AE6AF874E7303CE53299CCC041C7BC308D82A5698F3A8D0C38271AE35F8E9DBFBB694B5C803D89F7AE435DE236D525F54759B65E372FCD68EF20FA7111F9E4AFF73",
		"21484252197776302499639938883777710321993113097987201050501182909581359357618579566746556372589385361683610524730509041328855066514963385522570894839035884713051640171474186548713546686476761306436434146475140156284389181808675016576845833340494848283681088886584219750554408060556769486628029028720727393293111678826356480455433909233520504112074401376133077150471237549474149190242010469539006449596611576612573955754349042329130631128234637924786466585703488460540228477440853493392086251021228087076124706778899179648655221663765993962724699135217212118535057766739392069738618682722216712319320435674779146070442",
),
new_case(	
		"-0x1BCE04427D8032319A89E5C4136456671AC620883F2C4139E57F91307C485AD2D6204F4F87A58262652DB5DBBAC72B0613E51B835E7153BEC6068F5C8D696B74DBD18FEC316AEF73985CF0475663208EB46B4F17DD9DA55367B03323E5491A70997B90C059FB34809E6EE55BCFBD5F2F52233BFE62E6AA9E4E26A1D4C2439883D14F2633D55D8AA66A1ACD5595E778AC3A280517F1157989E70C1A437B849F1877B779CC3CDDEDE2DAA6594A6C66D181A00A5F777EE60596D8773998F6E988DEAE4CCA60E4DDCF9590543C89F74F603259FCAD71660D30294FBBE6490300F78A9D63FA660DC9417B8B9DDA28BEB3977B621B988E23D4D954F322C3540541BC649ABD504C50FADFD9F0987D58A2BF689313A285E773FF02899A6EF887D1D4A0D2",
		"0xB08FFB20760FFED58FADA86DFEF71AD72AA0FA763219618FE022C197E54708BB1191C66470250FCE8879487507CEE41381CA4D932F81C2B3F1AB20B539D50DCD",
		"0xAC6BDB41324A9A9BF166DE5E1389582FAF72B6651987EE07FC3192943DB56050A37329CBB4A099ED8193E0757767A13DD52312AB4B03310DCD7F48A9DA04FD50E8083969EDB767B0CF6095179A163AB3661A05FBD5FAAAE82918A9962F0B93B855F97993EC975EEAA80D740ADBF4FF747359D041D5C33EA71D281E446B14773BCA97B43A23FB801676BD207A436C6481F1D2B9078717461A5B9D32E688F87748544523B524B0D57D5EA77A2775D2ECFA032CFBDBF52FB3786160279004E57AE6AF874E7303CE53299CCC041C7BC308D82A5698F3A8D0C38271AE35F8E9DBFBB694B5C803D89F7AE435DE236D525F54759B65E372FCD68EF20FA7111F9E4AFF73",
		"21484252197776302499639938883777710321993113097987201050501182909581359357618579566746556372589385361683610524730509041328855066514963385522570894839035884713051640171474186548713546686476761306436434146475140156284389181808675016576845833340494848283681088886584219750554408060556769486628029028720727393293111678826356480455433909233520504112074401376133077150471237549474149190242010469539006449596611576612573955754349042329130631128234637924786466585703488460540228477440853493392086251021228087076124706778899179648655221663765993962724699135217212118535057766739392069738618682722216712319320435674779146070442",
),

	// test cases for issue 13907
	new_case("0xffffffff00000001", "0xffffffff00000001", "0xffffffff00000001", "0"),
	new_case("0xffffffffffffffff00000001", "0xffffffffffffffff00000001", "0xffffffffffffffff00000001", "0"),
	new_case("0xffffffffffffffffffffffff00000001", "0xffffffffffffffffffffffff00000001", "0xffffffffffffffffffffffff00000001", "0"),
	new_case("0xffffffffffffffffffffffffffffffff00000001", "0xffffffffffffffffffffffffffffffff00000001", "0xffffffffffffffffffffffffffffffff00000001", "0"),

	new_case(
		"2",
		"0xB08FFB20760FFED58FADA86DFEF71AD72AA0FA763219618FE022C197E54708BB1191C66470250FCE8879487507CEE41381CA4D932F81C2B3F1AB20B539D50DCD",
		"0xAC6BDB41324A9A9BF166DE5E1389582FAF72B6651987EE07FC3192943DB56050A37329CBB4A099ED8193E0757767A13DD52312AB4B03310DCD7F48A9DA04FD50E8083969EDB767B0CF6095179A163AB3661A05FBD5FAAAE82918A9962F0B93B855F97993EC975EEAA80D740ADBF4FF747359D041D5C33EA71D281E446B14773BCA97B43A23FB801676BD207A436C6481F1D2B9078717461A5B9D32E688F87748544523B524B0D57D5EA77A2775D2ECFA032CFBDBF52FB3786160279004E57AE6AF874E7303CE53299CCC041C7BC308D82A5698F3A8D0C38271AE35F8E9DBFBB694B5C803D89F7AE435DE236D525F54759B65E372FCD68EF20FA7111F9E4AFF73", // odd
		"0x6AADD3E3E424D5B713FCAA8D8945B1E055166132038C57BBD2D51C833F0C5EA2007A2324CE514F8E8C2F008A2F36F44005A4039CB55830986F734C93DAF0EB4BAB54A6A8C7081864F44346E9BC6F0A3EB9F2C0146A00C6A05187D0C101E1F2D038CDB70CB5E9E05A2D188AB6CBB46286624D4415E7D4DBFAD3BCC6009D915C406EED38F468B940F41E6BEDC0430DD78E6F19A7DA3A27498A4181E24D738B0072D8F6ADB8C9809A5B033A09785814FD9919F6EF9F83EEA519BEC593855C4C10CBEEC582D4AE0792158823B0275E6AEC35242740468FAF3D5C60FD1E376362B6322F78B7ED0CA1C5BBCD2B49734A56C0967A1D01A100932C837B91D592CE08ABFF",
    ),
    new_case(	
		"2",
		"0xB08FFB20760FFED58FADA86DFEF71AD72AA0FA763219618FE022C197E54708BB1191C66470250FCE8879487507CEE41381CA4D932F81C2B3F1AB20B539D50DCD",
		"0xAC6BDB41324A9A9BF166DE5E1389582FAF72B6651987EE07FC3192943DB56050A37329CBB4A099ED8193E0757767A13DD52312AB4B03310DCD7F48A9DA04FD50E8083969EDB767B0CF6095179A163AB3661A05FBD5FAAAE82918A9962F0B93B855F97993EC975EEAA80D740ADBF4FF747359D041D5C33EA71D281E446B14773BCA97B43A23FB801676BD207A436C6481F1D2B9078717461A5B9D32E688F87748544523B524B0D57D5EA77A2775D2ECFA032CFBDBF52FB3786160279004E57AE6AF874E7303CE53299CCC041C7BC308D82A5698F3A8D0C38271AE35F8E9DBFBB694B5C803D89F7AE435DE236D525F54759B65E372FCD68EF20FA7111F9E4AFF72", // even
		"0x7858794B5897C29F4ED0B40913416AB6C48588484E6A45F2ED3E26C941D878E923575AAC434EE2750E6439A6976F9BB4D64CEDB2A53CE8D04DD48CADCDF8E46F22747C6B81C6CEA86C0D873FBF7CEF262BAAC43A522BD7F32F3CDAC52B9337C77B3DCFB3DB3EDD80476331E82F4B1DF8EFDC1220C92656DFC9197BDC1877804E28D928A2A284B8DED506CBA304435C9D0133C246C98A7D890D1DE60CBC53A024361DA83A9B8775019083D22AC6820ED7C3C68F8E801DD4EC779EE0A05C6EB682EF9840D285B838369BA7E148FA27691D524FAEAF7C6ECE2A4B99A294B9F2C241857B5B90CC8BFFCFCF18DFA7D676131D5CD3855A5A3E8EBFA0CDFADB4D198B4A",
    ),
    ];

    let from_decimal_string = |s: &str| {
        if s.is_empty() {
            return Ok(None);
        }

        let mut out = Int::default();
        match out.set_string(s, 0) {
            Some(_) => Ok(Some(out)),
            None => Err(None::<Int>),
        }
    };

    for (i, c) in test_vector.iter().enumerate() {
        let x = from_decimal_string(c.x)
            .expect(&format!("x.set_string({})", c.x))
            .expect("unwrap x");
        let y = from_decimal_string(c.y)
            .expect(&format!("y.set_string({})", c.y))
            .expect("unwrap y");

        let m = from_decimal_string(c.m).expect("parse m");
        let out = from_decimal_string(c.out).expect("parse out");

        let mut z1 = Int::default();
        let zz = z1.exp(&x, &y, m.as_ref());

        match &zz {
            Some(v) => assert!(is_normalized(*v), "#{i}: {v} is not normalized"),
            _ => {}
        }

        match &zz {
            None if out.is_none() => {}
            Some(v) if out.is_some() && (v.cmp(out.as_ref().unwrap()) == 0) => {}
            _ => panic!("#{i}: got {zz:?}, want {out:?}"),
        }

        if m.is_none() {
            let m = Int::default();
            let mut z2 = Int::default();
            z2.exp(&x, &y, Some(&m));
            assert_eq!(z2, z1, "#{i}");
        }
    }
}

#[test]
fn gcd() {
    struct Case {
        d: &'static str,
        x: &'static str,
        y: &'static str,
        a: &'static str,
        b: &'static str,
    }

    let new_case = |d, x, y, a, b| Case { d, x, y, a, b };

    let test_vector = vec![
        // a <= 0 || b <= 0
        new_case("0", "0", "0", "0", "0"),
        new_case("7", "0", "1", "0", "7"),
        new_case("7", "0", "-1", "0", "-7"),
        new_case("11", "1", "0", "11", "0"),
        new_case("7", "-1", "-2", "-77", "35"),
        new_case("935", "-3", "8", "64515", "24310"),
        new_case("935", "-3", "-8", "64515", "-24310"),
        new_case("935", "3", "-8", "-64515", "-24310"),
        new_case("1", "-9", "47", "120", "23"),
        new_case("7", "1", "-2", "77", "35"),
        new_case("935", "-3", "8", "64515", "24310"),
        new_case(
            "935000000000000000",
            "-3",
            "8",
            "64515000000000000000",
            "24310000000000000000",
        ),
        new_case(
            "1",
            "-221",
            "22059940471369027483332068679400581064239780177629666810348940098015901108344",
            "98920366548084643601728869055592650835572950932266967461790948584315647051443",
            "991",
        ),
    ];

    fn must_from_decimal_str(s: &str) -> Int {
        let mut out = Int::default();
        out.set_string(s, 0).expect(&format!("Int from {s}"));
        out
    }

    for c in test_vector {
        let d = must_from_decimal_str(c.d);
        let x = must_from_decimal_str(c.x);
        let y = must_from_decimal_str(c.y);
        let a = must_from_decimal_str(c.a);
        let b = must_from_decimal_str(c.b);

        test_gcd(&d, None, None, &a, &b);
        test_gcd(&d, Some(&x), None, &a, &b);
        test_gcd(&d, None, Some(&y), &a, &b);
        test_gcd(&d, Some(&x), Some(&y), &a, &b);
    }

    let n = randn(128, 256);
    for _ in 0..n {
        let a = rand_bytes(randn(1, 64));
        let b = rand_bytes(randn(1, 64));
        check_gcd(a.as_slice(), b.as_slice());
    }
}

#[test]
fn int64() {
    let test_vector = vec![
        // int64
        "0",
        "1",
        "-1",
        "4294967295",
        "-4294967295",
        "4294967296",
        "-4294967296",
        "9223372036854775807",
        "-9223372036854775807",
        "-9223372036854775808",
        // not int64
        "0x8000000000000000",
        "-0x8000000000000001",
        "38579843757496759476987459679745",
        "-38579843757496759476987459679745",
    ];

    for s in test_vector {
        let mut x = Int::default();
        x.set_string(s, 0).expect(&format!("set_string({s}, 0)"));

        let want = match strconv::parse_int(s, 0, 64) {
            Err(err) => {
                match err.err {
                    NumErrorCause::OutOfRangeSigned { .. } => {
                        assert!(!x.is_int64(), "is_int64({x}) succeeded unexpectedly")
                    }
                    _ => panic!("parse_int({s}) failed"),
                }
                continue;
            }
            Ok(v) => v,
        };

        assert!(x.is_int64(), "is_int64({x}) failed unexpectedly");

        assert_eq!(x.int64(), want, "int64({s})");
    }
}

#[test]
fn int_cmp_self() {
    for s in CMP_ABS_TESTS.iter() {
        let mut x = Int::default();
        x.set_string(s, 0).expect(&format!("set_string({s}, 0)"));

        let got = x.cmp(&x);
        assert_eq!(got, 0, "x = {x}: x.cmp(x)");
    }
}

#[test]
fn lsh() {
    for (i, c) in LSH_TESTS.iter().enumerate() {
        let input = int_from_decimal_str(c.input);
        let expected = int_from_decimal_str(c.out);

        let mut out = Int::default();
        out.lsh(&input, c.shift);

        assert!(is_normalized(&out), "#{i}: {out} is not normalized");
        assert_eq!(out, expected, "#{i}");
    }
}

#[test]
fn lsh_rsh() {
    for (i, c) in RSH_TESTS.iter().enumerate() {
        let input = int_from_decimal_str(c.input);

        let mut out = Int::default();
        out.lsh(&input, c.shift);
        out.rsh(&out.clone(), c.shift);

        assert!(
            is_normalized(&out),
            "#{i} rsh test vector: {out} is not normalized"
        );
        assert_eq!(out, input, "#{i} rsh test vector");
    }

    for (i, c) in LSH_TESTS.iter().enumerate() {
        let input = int_from_decimal_str(c.input);

        let mut out = Int::default();
        out.lsh(&input, c.shift);
        out.rsh(&out.clone(), c.shift);

        assert!(
            is_normalized(&out),
            "#{i} lsh test vector: {out} is not normalized"
        );
        assert_eq!(out, input, "#{i} lsh test vector");
    }
}

#[test]
fn mod_inverse() {
    struct Case {
        element: &'static str,
        modulus: &'static str,
    }

    let new_case = |element, modulus| Case { element, modulus };

    let test_vector = vec![
        new_case("1234567", "458948883992"),
	new_case("239487239847", "2410312426921032588552076022197566074856950548502459942654116941958108831682612228890093858261341614673227141477904012196503648957050582631942730706805009223062734745341073406696246014589361659774041027169249453200378729434170325843778659198143763193776859869524088940195577346119843545301547043747207749969763750084308926339295559968882457872412993810129130294592999947926365264059284647209730384947211681434464714438488520940127459844288859336526896320919633919"),
	new_case("-10", "13"), // issue #16984
	new_case("10", "-13"),
	new_case("-17", "-13"),
    ];

    let mut element = Int::default();
    let mut modulus = Int::default();
    //let mut gcd = Int::default();
    let mut inverse = Int::default();
    let one = Int::new(1);

    for c in test_vector {
        element.set_string(c.element, 10);
        modulus.set_string(c.modulus, 10);
        inverse.mod_inverse(&element, &modulus);

        let inv = inverse.clone();
        inverse.mul(&inv, &element);

        let inv = inverse.clone();
        inverse.r#mod(&inv, &modulus);
        assert_eq!(
            inverse, one,
            "mod_inverse({element},{modulus})*{element}%{modulus}={inverse}, not 1"
        );
    }
}

#[test]
fn mul() {
    let n = randn(128, 256);
    for _ in 0..n {
        let a = rand_bytes(randn(1, 64));
        let b = rand_bytes(randn(1, 64));
        assert!(
            check_mul(a.as_slice(), b.as_slice()),
            "a={:?}, b={:?}",
            a,
            b
        );
    }
}

#[test]
fn mul_range_z() {
    struct S<T> {
        a: T,
        b: T,
        prod: &'static str,
    }

    impl<T> S<T> {
        fn new(a: T, b: T, prod: &'static str) -> Self {
            Self { a, b, prod }
        }
    }

    let e9 = 10i64.pow(9);
    let e9u = 10u64.pow(9);

    let mul_ranges_n = vec![
        S::new(0u64, 0, "0"),
        S::new(1, 1, "1"),
        S::new(1, 2, "2"),
        S::new(1, 3, "6"),
        S::new(10, 10, "10"),
        S::new(0, 100, "0"),
        S::new(0, e9u, "0"),
        S::new(1, 0, "1"),                    // empty range
        S::new(100, 1, "1"),                  // empty range
        S::new(1, 10, "3628800"),             // 10!
        S::new(1, 20, "2432902008176640000"), // 20!
        S::new(
            1,
            100,
            concat!(
                "933262154439441526816992388562667004907159682643816214685929",
                "638952175999932299156089414639761565182862536979208272237582",
                "51185210916864000000000000000000000000"
            ),
        ), // 100!
    ];

    let mul_ranges_z = vec![
        // entirely positive ranges are covered by mulRangesN
        S::new(-1, 1, "0"),
        S::new(-2, -1, "2"),
        S::new(-3, -2, "6"),
        S::new(-3, -1, "-6"),
        S::new(1, 3, "6"),
        S::new(-10, -10, "-10"),
        S::new(0, -1, "1"),                      // empty range
        S::new(-1, -100, "1"),                   // empty range
        S::new(-1, 1, "0"),                      // range includes 0
        S::new(-1 * e9, 0, "0"),                 // range includes 0
        S::new(-1 * e9, e9, "0"),                // range includes 0
        S::new(-10, -1, "3628800"),              // 10!
        S::new(-20, -2, "-2432902008176640000"), // -20!
        S::new(
            -99,
            -1,
            concat!(
                "-933262154439441526816992388562667004907159682643816214685929",
                "638952175999932299156089414639761565182862536979208272237582",
                "511852109168640000000000000000000000"
            ), // -99!
        ),
    ];

    let mut tmp = Int::default();
    for (i, r) in mul_ranges_n.iter().enumerate() {
        let prod = tmp.mul_range(r.a as i64, r.b as i64).string();
        assert_eq!(&prod, r.prod, "{i}");
    }

    for (i, r) in mul_ranges_z.iter().enumerate() {
        let prod = tmp.mul_range(r.a as i64, r.b as i64).string();
        assert_eq!(&prod, r.prod, "{i}");
    }
}

#[test]
fn prod_zz() {
    fn mul_zz<'a>(z: &'a mut Int, x: &Int, y: &Int) -> &'a mut Int {
        z.mul(x, y)
    }

    for a in PROD_ZZ.iter() {
        test_fun_zz("mul_zz", mul_zz, a);

        let arg = ArgZz::new(a.z.clone(), a.y.clone(), a.x.clone());
        test_fun_zz("mul_zz symmetric", mul_zz, &arg);
    }
}

#[test]
fn quo() {
    let n = randn(128, 256);
    for _ in 0..n {
        let a = rand_bytes(randn(1, 64));
        let b = rand_bytes(randn(1, 64));
        check_quo(a.as_slice(), b.as_slice());
    }

    struct Case {
        x: &'static str,
        y: &'static str,
        q: &'static str,
        r: &'static str,
    }
    let new_case = |x, y, q, r| Case { x, y, q, r };

    let test_vector = vec![
        new_case(
            "476217953993950760840509444250624797097991362735329973741718102894495832294430498335824897858659711275234906400899559094370964723884706254265559534144986498357",
            "9353930466774385905609975137998169297361893554149986716853295022578535724979483772383667534691121982974895531435241089241440253066816724367338287092081996",
            "50911",
            "1",
        ),
       new_case("11510768301994997771168",
            "1328165573307167369775",
            "8",
            "885443715537658812968",
       ),
    ];

    fn int_from_decimal_string(s: &str) -> Int {
        let mut out = Int::default();
        out.set_string(s, 10);
        out
    }

    for (i, c) in test_vector.iter().enumerate() {
        let x = int_from_decimal_string(c.x);
        let y = int_from_decimal_string(c.y);
        let expected_q = int_from_decimal_string(c.q);
        let expected_r = int_from_decimal_string(c.r);

        let mut r = Int::default();
        let mut q = Int::default();
        q.quo_rem(&x, &y, &mut r);

        assert!(
            (q == expected_q) && (r == expected_r),
            "#{i} got ({q},{r}) want ({expected_q}, {expected_r})"
        );
    }
}

#[test]
fn quo_step_d6() {
    let u = {
        let mut v = Int::default();

        let abs = [0, 0, 0, 0, 1, 1 << 31, u32::MAX, u32::MAX ^ (1 << 31)];

        v.set_bits(&abs);
        v
    };

    let v = {
        let mut v = Int::default();

        let abs = [5, 0, 2, 1 << 31, 0, 1 << 31];

        v.set_bits(&abs);
        v
    };

    let mut r = Int::default();
    let mut q = Int::default();
    q.quo_rem(&u, &v, &mut r);

    const EXPECTED_Q64: &'static str = "18446744073709551613";
    const EXPECTED_R64: &'static str = "3138550867693340382088035895064302439801311770021610913807";
    //const EXPECTED_Q32: &'static str = "4294967293";
    //const EXPECTED_R32: &'static str = "39614081266355540837921718287";

    assert_eq!(q.to_string(), EXPECTED_Q64, "bad q64");
    //assert_eq!(q.to_string(), EXPECTED_Q32, "bad q32");
    assert_eq!(r.to_string(), EXPECTED_R64, "bad r64");
    //assert_eq!(r.to_string(), EXPECTED_R32, "bad r32");
}

#[test]
fn rsh() {
    for (i, c) in RSH_TESTS.iter().enumerate() {
        let input = int_from_decimal_str(c.input);
        let expected = int_from_decimal_str(c.out);

        let mut out = Int::default();
        out.rsh(&input, c.shift);

        assert!(is_normalized(&out), "#{i}: {out} is not normalized");
        assert_eq!(out, expected, "#{i}");
    }
}

#[test]
fn set_bytes() {
    let n = randn(128, 256);
    for _ in 0..n {
        let b = rand_bytes(randn(1, 64));
        check_set_bytes(b.as_slice());
    }
}

#[test]
fn set_z() {
    for a in SUM_ZZ.iter() {
        let mut z = Int::default();
        z.set(&a.z);
        assert!(is_normalized(&z), "{z} is not normalized");
        assert!(z.cmp(&a.z) == 0, "got z={}; want {}", z, a.z);
    }
}

#[test]
fn sign_z() {
    let zero = Int::default();
    for a in SUM_ZZ.iter() {
        let s = a.z.sign();
        let e = a.z.cmp(&zero);
        assert_eq!(s, e, "z = {}", a.z);
    }
}

#[test]
fn sum_zz() {
    fn add_zz<'a>(z: &'a mut Int, x: &Int, y: &Int) -> &'a mut Int {
        z.add(x, y)
    }

    fn sub_zz<'a>(z: &'a mut Int, x: &Int, y: &Int) -> &'a mut Int {
        z.sub(x, y)
    }

    for a in SUM_ZZ.iter() {
        let arg = a;
        test_fun_zz("add_zz", add_zz, arg);

        let arg = ArgZz::new(a.z.clone(), a.y.clone(), a.x.clone());
        test_fun_zz("add_zz symmetric", add_zz, &arg);

        let arg = ArgZz::new(a.x.clone(), a.z.clone(), a.y.clone());
        test_fun_zz("sub_zz", sub_zz, &arg);

        let arg = ArgZz::new(a.y.clone(), a.z.clone(), a.x.clone());
        test_fun_zz("sub_zz symmetric", sub_zz, &arg);
    }
}

#[test]
fn trailing_zero_bits() {
    struct Case {
        input: &'static str,
        output: usize,
    }

    fn new_case(input: &'static str, output: usize) -> Case {
        Case { input, output }
    }

    let test_vector = vec![
        new_case("0", 0),
        new_case("1", 0),
        new_case("-1", 0),
        new_case("4", 2),
        new_case("-8", 3),
        new_case("0x4000000000000000000", 74),
        new_case("-0x8000000000000000000", 75),
    ];

    for (i, c) in test_vector.iter().enumerate() {
        let input = int_from_str(c.input, None);
        let got = input.trailing_zero_bits();
        assert_eq!(got, c.output, "#{i}");
    }
}

#[test]
fn uint64() {
    let test_vector = vec![
        // uint64
        "0",
        "1",
        "4294967295",
        "4294967296",
        "8589934591",
        "8589934592",
        "9223372036854775807",
        "9223372036854775808",
        "0x08000000000000000",
        // not uint64
        "0x10000000000000000",
        "-0x08000000000000000",
        "-1",
    ];

    for s in test_vector {
        let mut x = Int::default();
        assert!(x.set_string(s, 0).is_some(), "set_string({s}, 0) failed");

        let want = match strconv::parse_uint(s, 0, 64) {
            Ok(v) => v,
            Err(err) => {
                let ok = s.starts_with('-')
                    || match err.err {
                        NumErrorCause::OutOfRangeUnsigned { .. } => true,
                        _ => false,
                    };

                if ok {
                    assert!(!x.is_uint64(), "is_uint64({s}) succeed unexpectedly");
                } else {
                    panic!("parse_uint({s}) failed");
                }
                continue;
            }
        };

        assert!(x.is_uint64(), "is_uint64({s}) failed unexpectedly");
        assert_eq!(x.uint64(), want, "uint64({s})");
    }
}

// private functions
fn alt_bit(x: &Int, i: usize) -> u8 {
    let mut z = Int::default();
    z.rsh(x, i);
    z.and(&z.clone(), &Int::new(1));

    if z.cmp(&Int::new(0)) != 0 {
        1
    } else {
        0
    }
}

fn alt_set_bit(z: Int, x: &Int, i: usize, b: bool) -> Int {
    let mut m = Int::new(1);
    m.lsh(&m.clone(), i);

    let mut z = z;
    if b {
        z.or(x, &m);
    } else {
        z.and_not(x, &m);
    }

    z
}

fn check_bytes(b: &[u8]) {
    let b = {
        let mut v = b;
        while (v.len() > 0) && (v[0] == 0) {
            v = &v[1..];
        }
        v
    };

    let mut v = Int::default();
    let b2 = v.set_bytes(b).bytes();

    assert_eq!(b, b2.as_slice())
}

fn check_gcd(a: &[u8], b: &[u8]) {
    let mut x = Int::default();
    let mut y = Int::default();

    let mut aa = Int::default();
    aa.set_bytes(a);

    let mut bb = Int::default();
    bb.set_bytes(b);

    let mut d = Int::default();
    d.gcd(Some(&mut x), Some(&mut y), &aa, &bb);

    x.mul(&x.clone(), &aa);
    y.mul(&y.clone(), &bb);
    x.add(&x.clone(), &y);

    assert_eq!(x, d, "check_gcd({a:?},{b:?})");
}

fn check_mul(a: &[u8], b: &[u8]) -> bool {
    let mut x = Int::default();
    let mut y = Int::default();
    let mut z1 = Int::default();

    x.set_bytes(a);
    y.set_bytes(b);

    z1.mul(&x, &y);

    let mut z2 = Int::default();
    z2.set_bytes(&mul_bytes(a, b));

    z1 == z2
}

fn check_quo(x: &[u8], y: &[u8]) {
    let mut u = Int::default();
    u.set_bytes(x);

    let mut v = Int::default();
    v.set_bytes(y);

    let zero = Int::default();

    if &v == &zero {
        return;
    }

    let mut r = Int::default();
    let mut q = Int::default();
    q.quo_rem(&u, &v, &mut r);

    assert!(r.cmp(&v) < 0, "remainder not less than divisor");

    let uprime = {
        let mut out = q.clone();
        out.mul(&q, &v);

        let vv = out.clone();
        out.add(&vv, &r);

        out
    };

    assert_eq!(uprime, u);
}

fn check_set_bytes(b: &[u8]) {
    let b1 = {
        let mut v = Int::default();
        v.set_bytes(b).bytes()
    };

    fn normalize(b: &[u8]) -> &[u8] {
        let mut out = b;
        while (out.len() > 1) && (out[0] == 0) {
            out = &out[1..];
        }
        out
    }

    assert_eq!(normalize(b1.as_slice()), normalize(b));
}

fn int_from_decimal_str(s: &str) -> Int {
    let mut out = Int::default();
    out.set_string(s, 10).expect("set_string");
    out
}

fn int_from_str(s: &str, base: Option<u8>) -> Int {
    let mut out = Int::default();
    out.set_string(s, base.unwrap_or_default())
        .expect("set_string");
    out
}

fn is_normalized(x: &Int) -> bool {
    match x.bits().next_back() {
        None => x.sign() == 0,
        Some(v) => v != 0,
    }
}

fn mul_bytes(x: &[u8], y: &[u8]) -> Vec<u8> {
    let mut z = vec![0u8; x.len() + y.len()];

    let mut k0 = z.len() - 1;
    for j in (0..y.len()).rev() {
        let d = y[j] as i32;
        if d != 0 {
            let mut k = k0;
            let mut carry = 0;
            for i in (0..x.len()).rev() {
                let t = (z[k] as i32) + (x[i] as i32) * d + carry;
                z[k] = t as u8;
                carry = t >> 8;
                k -= 1;
            }
            z[k] = carry as u8;
        }
        println!("j={j}, k0={k0}, x.len()={}", x.len());
        k0 -= 1;
    }

    let mut i = 0;
    while (i < z.len()) && (z[i] == 0) {
        i += 1;
    }

    let (_, b) = z.split_at(i);

    b.to_vec()
}

fn norm(x: &[u32]) -> Vec<u32> {
    let mut i = x.len();
    while (i > 0) && (x[i - 1] == 0) {
        i -= 1;
    }

    x[..i].to_vec()
}

fn rand_bytes(n: usize) -> Vec<u8> {
    let mut out = vec![0u8; n];
    helper::rand::read(&mut out).unwrap();
    out
}

fn randn(lo: u64, hi: u64) -> usize {
    let mut b = [0u8; 8];
    helper::rand::read(&mut b).unwrap();

    (u64::from_be_bytes(b) % (hi - lo) + lo) as usize
}

fn test_bit_fun<F>(msg: &'static str, f: F, x: &Int, y: &Int, exp: &'static str)
where
    F: for<'a> Fn(&'a mut Int, &Int, &Int) -> &'a mut Int,
{
    let mut expected = Int::default();
    expected.set_string(exp, 0);

    let mut got = Int::default();
    let _ = f(&mut got, x, y);
    assert_eq!(got, expected, "{msg}");
}


fn test_bitset(x: &Int) {
    let n = x.bit_len();
    let z = x.clone();
    let z1 = x.clone();

    println!("x = {x}");
    for i in 0..(n + 10) {
        let old = z.bit(i);
        let old1 = alt_bit(&z1, i);
        assert_eq!(old, old1, "bitset: inconsistent value for bit({z1}, {i})");

        let z0 = &z;

        let mut z = Int::default();

        z.set_bit(z0, i, true);
        let z1 = alt_set_bit(Int::default(), &z1, i, true);
        assert_ne!(z.bit(i), 0, "bitset: bit {i} of {z}");
        assert_eq!(z, z1, "bitset: inconsistent value after set_bit 1");

        z.set_bit(&z.clone(), i, false);
        let z1 = alt_set_bit(Int::default(), &z1, i, false);
        assert_eq!(z.bit(i), 0, "bitset: bit {i} of {z}");
        assert_eq!(z, z1, "bitset: inconsistent value after set_bit 0");

        let z1 = alt_set_bit(z1.clone(), &z1, i, old == 1);
        z.set_bit(&z.clone(), i, old == 1);
        assert_eq!(z, z1, "bitset: inconsistent value after set_bit old={old}");
    }

    assert_eq!(&z, x, "bitset");
}

fn test_fun_zz<F>(msg: &str, f: F, a: &ArgZz)
where
    F: for<'a> Fn(&'a mut Int, &Int, &Int) -> &'a mut Int,
{
    let mut z = Int::default();
    f(&mut z, &a.x, &a.y);
    assert!(is_normalized(&z), "{msg}{z} is not normalized");
    assert_eq!(z, a.z, "{msg}{a:?}");
}

fn test_gcd(d: &Int, x: Option<&Int>, y: Option<&Int>, a: &Int, b: &Int) {
    let mut xx = if x.is_some() {
        Some(Int::default())
    } else {
        None
    };

    let mut yy = if y.is_some() {
        Some(Int::default())
    } else {
        None
    };

    let mut dd = Int::default();
    dd.gcd(xx.as_mut(), yy.as_mut(), a, b);

    assert_eq!(&dd, d, "gcd({x:?},{y:?},{a},{b}): bad d");
    assert_eq!(x, xx.as_ref(), "gcd({x:?},{y:?},{a},{b}): bad x");
    assert_eq!(y, yy.as_ref(), "gcd({x:?},{y:?},{a},{b}): bad y");
}
