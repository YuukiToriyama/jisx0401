use crate::Prefecture;

struct PrefectureData {
    code: &'static str,
    name_ja: &'static str,
    name_en: &'static str,
}

impl Prefecture {
    /// Returns prefecture identification code
    ///
    /// # Example
    /// ```rust
    /// use jisx0401::Prefecture;
    ///
    /// let tokyo = Prefecture::TOKYO;
    /// assert_eq!(tokyo.code(), "13");
    /// let kyoto = Prefecture::KYOTO;
    /// assert_eq!(kyoto.code(), "26");
    /// ```
    pub fn code(&self) -> &'static str {
        self.value().code
    }

    /// Returns prefecture name in Japanese
    ///
    /// # Example
    /// ```rust
    /// use jisx0401::Prefecture;
    ///
    /// let gunma = Prefecture::GUNMA;
    /// assert_eq!(gunma.name_ja(), "群馬県");
    /// let yamanashi = Prefecture::YAMANASHI;
    /// assert_eq!(yamanashi.name_ja(), "山梨県");
    /// ```
    pub fn name_ja(&self) -> &'static str {
        self.value().name_ja
    }

    /// Returns prefecture name in English
    ///
    /// # Example
    /// ```rust
    /// use jisx0401::Prefecture;
    ///
    /// let tottori = Prefecture::TOTTORI;
    /// assert_eq!(tottori.name_en(), "tottori");
    /// let ehime = Prefecture::EHIME;
    /// assert_eq!(ehime.name_en(), "ehime");
    /// ```
    pub fn name_en(&self) -> &'static str {
        self.value().name_en
    }

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
                name_ja: "東京都",
                name_en: "tokyo",
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
            Prefecture::FUKUI => PrefectureData {
                code: "18",
                name_ja: "福井県",
                name_en: "fukui",
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

impl TryFrom<&str> for Prefecture {
    type Error = &'static str;

    /// Convert prefecture code or prefecture name to `Prefecture`
    ///
    /// # Example
    /// ```rust
    /// use jisx0401::Prefecture;
    ///
    /// let result = Prefecture::try_from("47");
    /// assert_eq!(result.unwrap(), Prefecture::OKINAWA);
    ///
    /// let result = Prefecture::try_from("48");
    /// assert!(result.is_err());
    ///
    /// let result = Prefecture::try_from("北海道");
    /// assert_eq!(result.unwrap(), Prefecture::HOKKAIDO);
    ///
    /// let result = Prefecture::try_from("ほっかいどう");
    /// assert!(result.is_err());
    ///
    /// let result = Prefecture::try_from("aichi");
    /// assert_eq!(result.unwrap(), Prefecture::AICHI);
    ///
    /// let result = Prefecture::try_from("aiti");
    /// assert!(result.is_err());
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "01" | "北海道" | "hokkaido" => Ok(Prefecture::HOKKAIDO),
            "02" | "青森県" | "aomori" => Ok(Prefecture::AOMORI),
            "03" | "岩手県" | "iwate" => Ok(Prefecture::IWATE),
            "04" | "宮城県" | "miyagi" => Ok(Prefecture::MIYAGI),
            "05" | "秋田県" | "akita" => Ok(Prefecture::AKITA),
            "06" | "山形県" | "yamagata" => Ok(Prefecture::YAMAGATA),
            "07" | "福島県" | "fukushima" => Ok(Prefecture::FUKUSHIMA),
            "08" | "茨城県" | "ibaraki" => Ok(Prefecture::IBARAKI),
            "09" | "栃木県" | "tochigi" => Ok(Prefecture::TOCHIGI),
            "10" | "群馬県" | "gunma" => Ok(Prefecture::GUNMA),
            "11" | "埼玉県" | "saitama" => Ok(Prefecture::SAITAMA),
            "12" | "千葉県" | "chiba" => Ok(Prefecture::CHIBA),
            "13" | "東京都" | "tokyo" => Ok(Prefecture::TOKYO),
            "14" | "神奈川県" | "kanagawa" => Ok(Prefecture::KANAGAWA),
            "15" | "新潟県" | "niigata" => Ok(Prefecture::NIIGATA),
            "16" | "富山県" | "toyama" => Ok(Prefecture::TOYAMA),
            "17" | "石川県" | "ishikawa" => Ok(Prefecture::ISHIKAWA),
            "18" | "福井県" | "fukui" => Ok(Prefecture::FUKUI),
            "19" | "山梨県" | "yamanashi" => Ok(Prefecture::YAMANASHI),
            "20" | "長野県" | "nagano" => Ok(Prefecture::NAGANO),
            "21" | "岐阜県" | "gifu" => Ok(Prefecture::GIFU),
            "22" | "静岡県" | "shizuoka" => Ok(Prefecture::SHIZUOKA),
            "23" | "愛知県" | "aichi" => Ok(Prefecture::AICHI),
            "24" | "三重県" | "mie" => Ok(Prefecture::MIE),
            "25" | "滋賀県" | "shiga" => Ok(Prefecture::SHIGA),
            "26" | "京都府" | "kyoto" => Ok(Prefecture::KYOTO),
            "27" | "大阪府" | "osaka" => Ok(Prefecture::OSAKA),
            "28" | "兵庫県" | "hyogo" => Ok(Prefecture::HYOGO),
            "29" | "奈良県" | "nara" => Ok(Prefecture::NARA),
            "30" | "和歌山県" | "wakayama" => Ok(Prefecture::WAKAYAMA),
            "31" | "鳥取県" | "tottori" => Ok(Prefecture::TOTTORI),
            "32" | "島根県" | "shimane" => Ok(Prefecture::SHIMANE),
            "33" | "岡山県" | "okayama" => Ok(Prefecture::OKAYAMA),
            "34" | "広島県" | "hiroshima" => Ok(Prefecture::HIROSHIMA),
            "35" | "山口県" | "yamaguchi" => Ok(Prefecture::YAMAGUCHI),
            "36" | "徳島県" | "tokushima" => Ok(Prefecture::TOKUSHIMA),
            "37" | "香川県" | "kagawa" => Ok(Prefecture::KAGAWA),
            "38" | "愛媛県" | "ehime" => Ok(Prefecture::EHIME),
            "39" | "高知県" | "kochi" => Ok(Prefecture::KOCHI),
            "40" | "福岡県" | "fukuoka" => Ok(Prefecture::FUKUOKA),
            "41" | "佐賀県" | "saga" => Ok(Prefecture::SAGA),
            "42" | "長崎県" | "nagasaki" => Ok(Prefecture::NAGASAKI),
            "43" | "熊本県" | "kumamoto" => Ok(Prefecture::KUMAMOTO),
            "44" | "大分県" | "oita" => Ok(Prefecture::OITA),
            "45" | "宮崎県" | "miyazaki" => Ok(Prefecture::MIYAZAKI),
            "46" | "鹿児島県" | "kagoshima" => Ok(Prefecture::KAGOSHIMA),
            "47" | "沖縄県" | "okinawa" => Ok(Prefecture::OKINAWA),
            _ => Err("No matching prefectures were found."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Prefecture;

    #[test]
    fn code() {
        assert_eq!(Prefecture::HOKKAIDO.code(), "01")
    }

    #[test]
    fn name_ja() {
        assert_eq!(Prefecture::AOMORI.name_ja(), "青森県");
    }

    #[test]
    fn name_en() {
        assert_eq!(Prefecture::FUKUOKA.name_en(), "fukuoka");
    }

    #[test]
    fn try_from_success() {
        let result = Prefecture::try_from("14");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Prefecture::KANAGAWA);
        let result = Prefecture::try_from("新潟県");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Prefecture::NIIGATA);
        let result = Prefecture::try_from("fukui");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Prefecture::FUKUI);
    }

    #[test]
    fn try_from_failure() {
        let result = Prefecture::try_from("48");
        assert!(result.is_err());
        let result = Prefecture::try_from("東京県");
        assert!(result.is_err());
        let result = Prefecture::try_from("hukuoka");
        assert!(result.is_err());
    }
}
