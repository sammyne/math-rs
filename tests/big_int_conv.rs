use math::big::Int;

mod helper;

use helper::is_big_int_normalized as is_normalized;

lazy_static::lazy_static! {
  static ref STRING_TESTS: Vec<StringTest> = vec![
    // invalid inputs
    StringTest{input: "", ..Default::default()},
    StringTest{input: "a", ..Default::default()},
    StringTest{input: "z", ..Default::default()},
    StringTest{input: "+", ..Default::default()},
    StringTest{input: "-", ..Default::default()},
    StringTest{input: "0b", ..Default::default()},
    StringTest{input: "0o", ..Default::default()},
    StringTest{input: "0x", ..Default::default()},
    StringTest{input: "0y", ..Default::default()},
    StringTest{input: "2", base: 2, ..Default::default()},
    StringTest{input: "0b2", base: 0, ..Default::default()},
    StringTest{input: "08", ..Default::default()},
    StringTest{input: "8", base: 8, ..Default::default()},
    StringTest{input: "0xg", base: 0, ..Default::default()},
    StringTest{input: "g", base: 16, ..Default::default()},

    // invalid inputs with separators
    // (smoke tests only - a comprehensive set of tests is in natconv_test.go)
    StringTest{input: "_", ..Default::default()},
    StringTest{input: "0_", ..Default::default()},
    StringTest{input: "_0", ..Default::default()},
    StringTest{input: "-1__0", ..Default::default()},
    StringTest{input: "0x10_", ..Default::default()},
    StringTest{input: "1_000", base: 10, ..Default::default()}, // separators are not permitted for bases != 0
    StringTest{input: "d_e_a_d", base: 16, ..Default::default()},

    // valid inputs
    StringTest::new("0", "0", 0, 0, true),
    StringTest::new("0", "0", 10, 0, true),
    StringTest::new("0", "0", 16, 0, true),
    StringTest::new("+0", "0", 0, 0, true),
    StringTest::new("-0", "0", 0, 0, true),
    StringTest::new("10", "10", 0, 10, true),
    StringTest::new("10", "10", 10, 10, true),
    StringTest::new("10", "10", 16, 16, true),
    StringTest::new("-10", "-10", 16, -16, true),
    StringTest::new("+10", "10", 16, 16, true),
    StringTest::new("0b10", "2", 0, 2, true),
    StringTest::new("0o10", "8", 0, 8, true),
    StringTest::new("0x10", "16", 0, 16, true),
    StringTest{input: "0x10", base: 16,..Default::default()},
    StringTest::new("-0x10", "-16", 0, -16, true),
    StringTest::new("+0x10", "16", 0, 16, true),
    StringTest::new("00", "0", 0, 0, true),
    StringTest::new("0", "0", 8, 0, true),
    StringTest::new("07", "7", 0, 7, true),
    StringTest::new("7", "7", 8, 7, true),
    StringTest::new("023", "19", 0, 19, true),
    StringTest::new("23", "23", 8, 19, true),
    StringTest::new("cafebabe", "cafebabe", 16, 0xcafebabe, true),
    StringTest::new("0b0", "0", 0, 0, true),
    StringTest::new("-111", "-111", 2, -7, true),
    StringTest::new("-0b111", "-7", 0, -7, true),
    StringTest::new("0b1001010111", "599", 0, 0x257, true),
    StringTest::new("1001010111", "1001010111", 2, 0x257, true),
    StringTest::new("A", "a", 36, 10, true),
    StringTest::new("A", "A", 37, 36, true),
    StringTest::new("ABCXYZ", "abcxyz", 36, 623741435, true),
    StringTest::new("ABCXYZ", "ABCXYZ", 62, 33536793425, true),

    // valid input with separators
    // (smoke tests only - a comprehensive set of tests is in natconv_test.go)
    StringTest::new("1_000", "1000", 0, 1000, true),
    StringTest::new("0b_1010", "10", 0, 10, true),
    StringTest::new("+0o_660", "432", 0, 0o660, true),
    StringTest::new("-0xF00D_1E", "-15731998", 0, -0xf00d1e, true),
  ];
}

#[derive(Default, Debug)]
struct StringTest {
    input: &'static str,
    output: &'static str,
    base: u8,
    val: i64,
    ok: bool,
}

impl StringTest {
    pub fn new(input: &'static str, output: &'static str, base: u8, val: i64, ok: bool) -> Self {
        Self {
            input,
            output,
            base,
            val,
            ok,
        }
    }
}

#[test]
fn set_string() {
    let mut tmp = Int::default();
    for (i, c) in STRING_TESTS.iter().enumerate() {
        tmp.set_int64(1234567890);

        let mut n1 = Int::default();
        let ok1 = n1.set_string(c.input, c.base).is_some();

        let ok2 = tmp.set_string(c.input, c.base).is_some();

        assert_eq!(
            ok1, c.ok,
            "#{i} (input '{}', base '{}') ok1 incorrect",
            c.input, c.base
        );
        assert_eq!(
            ok2, c.ok,
            "#{i} (input '{}', base '{}') ok2 incorrect",
            c.input, c.base
        );

        if !c.ok {
            continue;
        }

        assert!(
            is_normalized(&n1),
            "#{i} (input '{}', base '{}'): n1={n1} is not normalized",
            c.input,
            c.base,
        );
        assert!(
            is_normalized(&tmp),
            "#{i} (input '{}', base '{}'): n2={tmp} is not normalized",
            c.input,
            c.base,
        );

        let expected = Int::new(c.val);

        assert_eq!(
            n1, expected,
            "#{i} (input '{}', base '{}'): bad n1",
            c.input, c.base
        );
        assert_eq!(
            tmp, expected,
            "#{i} (input '{}', base '{}'): bad n2",
            c.input, c.base
        );
    }

    struct Good {
        input: &'static str,
        base: u8,
        expect: i64,
    }

    fn new_good(input: &'static str, base: u8, expect: i64) -> Good {
        Good {
            input,
            base,
            expect,
        }
    }

    let good_test_vector = vec![new_good("0x9a4e", 0, 39502)];

    println!("-----------");
    println!("testing good cases");
    for (i, c) in good_test_vector.iter().enumerate() {
        tmp.set_int64(1234567890);

        let _ = tmp
            .set_string(c.input, c.base)
            .expect(&format!("#{i} (input '{}')", c.input));

        let mut expect = Int::default();
        expect.set_int64(c.expect);

        assert_eq!(tmp, expect, "#{i} good");
    }
}

#[test]
fn text() {
    let mut z = Int::default();
    for c in STRING_TESTS.iter().filter(|v| v.ok) {
        z.set_string(c.input, c.base)
            .expect(&format!("{c:?} failed to parse"));

        let base = if c.base != 0 { c.base } else { 10 };

        let got = z.text(base);
        assert_eq!(&got, c.output, "{c:?}");
    }
}
