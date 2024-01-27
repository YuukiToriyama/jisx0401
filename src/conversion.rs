use crate::Prefecture;

struct PrefectureData {
    code: &'static str,
    name_ja: &'static str,
    name_en: &'static str,
}

impl Prefecture {
    fn value(&self) -> PrefectureData {
        match self {
            Prefecture::HOKKAIDO => PrefectureData {
                code: "01",
                name_ja: "北海道",
                name_en: "hokkaido",
            },
            Prefecture::AOMORI => PrefectureData {
                code: "02",
                name_ja: "青森県",
                name_en: "aomori",
            },
            Prefecture::IWATE => PrefectureData {
                code: "03",
                name_ja: "岩手県",
                name_en: "iwate",
            },
            Prefecture::MIYAGI => PrefectureData {
                code: "04",
                name_ja: "宮城県",
                name_en: "miyagi",
            },
            Prefecture::AKITA => PrefectureData {
                code: "05",
                name_ja: "秋田県",
                name_en: "akita",
            },
            Prefecture::YAMAGATA => PrefectureData {
                code: "06",
                name_ja: "山形県",
                name_en: "yamagata",
            },
            Prefecture::FUKUSHIMA => PrefectureData {
                code: "07",
                name_ja: "福島県",
                name_en: "fukushima",
            },
            Prefecture::IBARAKI => PrefectureData {
                code: "08",
                name_ja: "茨城県",
                name_en: "ibaraki",
            },
            Prefecture::TOCHIGI => PrefectureData {
                code: "09",
                name_ja: "栃木県",
                name_en: "tochigi",
            },
            Prefecture::GUNMA => PrefectureData {
                code: "10",
                name_ja: "群馬県",
                name_en: "gunma",
            },
            Prefecture::SAITAMA => PrefectureData {
                code: "11",
                name_ja: "埼玉県",
                name_en: "saitama",
            },
            Prefecture::CHIBA => PrefectureData {
                code: "12",
                name_ja: "千葉県",
                name_en: "chiba",
            },
            Prefecture::TOKYO => PrefectureData {
                code: "13",
                name_ja: "",
                name_en: "",
            },
            Prefecture::KANAGAWA => PrefectureData {
                code: "14",
                name_ja: "神奈川県",
                name_en: "kanagawa",
            },
            Prefecture::NIIGATA => PrefectureData {
                code: "15",
                name_ja: "新潟県",
                name_en: "niigata",
            },
            Prefecture::TOYAMA => PrefectureData {
                code: "16",
                name_ja: "富山県",
                name_en: "toyama",
            },
            Prefecture::ISHIKAWA => PrefectureData {
                code: "17",
                name_ja: "石川県",
                name_en: "ishikawa",
            },
            Prefecture::HUKUI => PrefectureData {
                code: "18",
                name_ja: "福井県",
                name_en: "hukui",
            },
            Prefecture::YAMANASHI => PrefectureData {
                code: "19",
                name_ja: "山梨県",
                name_en: "yamanashi",
            },
            Prefecture::NAGANO => PrefectureData {
                code: "20",
                name_ja: "長野県",
                name_en: "nagano",
            },
            Prefecture::GIFU => PrefectureData {
                code: "21",
                name_ja: "岐阜県",
                name_en: "gifu",
            },
            Prefecture::SHIZUOKA => PrefectureData {
                code: "22",
                name_ja: "静岡県",
                name_en: "shizuoka",
            },
            Prefecture::AICHI => PrefectureData {
                code: "23",
                name_ja: "愛知県",
                name_en: "aichi",
            },
            Prefecture::MIE => PrefectureData {
                code: "24",
                name_ja: "三重県",
                name_en: "mie",
            },
            Prefecture::SHIGA => PrefectureData {
                code: "25",
                name_ja: "滋賀県",
                name_en: "shiga",
            },
            Prefecture::KYOTO => PrefectureData {
                code: "26",
                name_ja: "京都府",
                name_en: "kyoto",
            },
            Prefecture::OSAKA => PrefectureData {
                code: "27",
                name_ja: "大阪府",
                name_en: "osaka",
            },
            Prefecture::HYOGO => PrefectureData {
                code: "28",
                name_ja: "兵庫県",
                name_en: "hyogo",
            },
            Prefecture::NARA => PrefectureData {
                code: "29",
                name_ja: "奈良県",
                name_en: "nara",
            },
            Prefecture::WAKAYAMA => PrefectureData {
                code: "30",
                name_ja: "和歌山県",
                name_en: "wakayama",
            },
            Prefecture::TOTTORI => PrefectureData {
                code: "31",
                name_ja: "鳥取県",
                name_en: "tottori",
            },
            Prefecture::SHIMANE => PrefectureData {
                code: "32",
                name_ja: "島根県",
                name_en: "shimane",
            },
            Prefecture::OKAYAMA => PrefectureData {
                code: "33",
                name_ja: "岡山県",
                name_en: "okayama",
            },
            Prefecture::HIROSHIMA => PrefectureData {
                code: "34",
                name_ja: "広島県",
                name_en: "hiroshima",
            },
            Prefecture::YAMAGUCHI => PrefectureData {
                code: "35",
                name_ja: "山口県",
                name_en: "yamaguchi",
            },
            Prefecture::TOKUSHIMA => PrefectureData {
                code: "36",
                name_ja: "徳島県",
                name_en: "tokushima",
            },
            Prefecture::KAGAWA => PrefectureData {
                code: "37",
                name_ja: "香川県",
                name_en: "kagawa",
            },
            Prefecture::EHIME => PrefectureData {
                code: "38",
                name_ja: "愛媛県",
                name_en: "ehime",
            },
            Prefecture::KOCHI => PrefectureData {
                code: "39",
                name_ja: "高知県",
                name_en: "kochi",
            },
            Prefecture::FUKUOKA => PrefectureData {
                code: "40",
                name_ja: "福岡県",
                name_en: "fukuoka",
            },
            Prefecture::SAGA => PrefectureData {
                code: "41",
                name_ja: "佐賀県",
                name_en: "saga",
            },
            Prefecture::NAGASAKI => PrefectureData {
                code: "42",
                name_ja: "長崎県",
                name_en: "nagasaki",
            },
            Prefecture::KUMAMOTO => PrefectureData {
                code: "43",
                name_ja: "熊本県",
                name_en: "kumamoto",
            },
            Prefecture::OITA => PrefectureData {
                code: "44",
                name_ja: "大分県",
                name_en: "oita",
            },
            Prefecture::MIYAZAKI => PrefectureData {
                code: "45",
                name_ja: "宮崎県",
                name_en: "miyazaki",
            },
            Prefecture::KAGOSHIMA => PrefectureData {
                code: "46",
                name_ja: "鹿児島県",
                name_en: "kagoshima",
            },
            Prefecture::OKINAWA => PrefectureData {
                code: "47",
                name_ja: "沖縄県",
                name_en: "okinawa",
            },
        }
    }
}