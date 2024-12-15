use crate::Prefecture;

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
        match self {
            Prefecture::HOKKAIDO => "01",
            Prefecture::AOMORI => "02",
            Prefecture::IWATE => "03",
            Prefecture::MIYAGI => "04",
            Prefecture::AKITA => "05",
            Prefecture::YAMAGATA => "06",
            Prefecture::FUKUSHIMA => "07",
            Prefecture::IBARAKI => "08",
            Prefecture::TOCHIGI => "09",
            Prefecture::GUNMA => "10",
            Prefecture::SAITAMA => "11",
            Prefecture::CHIBA => "12",
            Prefecture::TOKYO => "13",
            Prefecture::KANAGAWA => "14",
            Prefecture::NIIGATA => "15",
            Prefecture::TOYAMA => "16",
            Prefecture::ISHIKAWA => "17",
            Prefecture::FUKUI => "18",
            Prefecture::YAMANASHI => "19",
            Prefecture::NAGANO => "20",
            Prefecture::GIFU => "21",
            Prefecture::SHIZUOKA => "22",
            Prefecture::AICHI => "23",
            Prefecture::MIE => "24",
            Prefecture::SHIGA => "25",
            Prefecture::KYOTO => "26",
            Prefecture::OSAKA => "27",
            Prefecture::HYOGO => "28",
            Prefecture::NARA => "29",
            Prefecture::WAKAYAMA => "30",
            Prefecture::TOTTORI => "31",
            Prefecture::SHIMANE => "32",
            Prefecture::OKAYAMA => "33",
            Prefecture::HIROSHIMA => "34",
            Prefecture::YAMAGUCHI => "35",
            Prefecture::TOKUSHIMA => "36",
            Prefecture::KAGAWA => "37",
            Prefecture::EHIME => "38",
            Prefecture::KOCHI => "39",
            Prefecture::FUKUOKA => "40",
            Prefecture::SAGA => "41",
            Prefecture::NAGASAKI => "42",
            Prefecture::KUMAMOTO => "43",
            Prefecture::OITA => "44",
            Prefecture::MIYAZAKI => "45",
            Prefecture::KAGOSHIMA => "46",
            Prefecture::OKINAWA => "47",
        }
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
        match self {
            Prefecture::HOKKAIDO => "北海道",
            Prefecture::AOMORI => "青森県",
            Prefecture::IWATE => "岩手県",
            Prefecture::MIYAGI => "宮城県",
            Prefecture::AKITA => "秋田県",
            Prefecture::YAMAGATA => "山形県",
            Prefecture::FUKUSHIMA => "福島県",
            Prefecture::IBARAKI => "茨城県",
            Prefecture::TOCHIGI => "栃木県",
            Prefecture::GUNMA => "群馬県",
            Prefecture::SAITAMA => "埼玉県",
            Prefecture::CHIBA => "千葉県",
            Prefecture::TOKYO => "東京都",
            Prefecture::KANAGAWA => "神奈川県",
            Prefecture::NIIGATA => "新潟県",
            Prefecture::TOYAMA => "富山県",
            Prefecture::ISHIKAWA => "石川県",
            Prefecture::FUKUI => "福井県",
            Prefecture::YAMANASHI => "山梨県",
            Prefecture::NAGANO => "長野県",
            Prefecture::GIFU => "岐阜県",
            Prefecture::SHIZUOKA => "静岡県",
            Prefecture::AICHI => "愛知県",
            Prefecture::MIE => "三重県",
            Prefecture::SHIGA => "滋賀県",
            Prefecture::KYOTO => "京都府",
            Prefecture::OSAKA => "大阪府",
            Prefecture::HYOGO => "兵庫県",
            Prefecture::NARA => "奈良県",
            Prefecture::WAKAYAMA => "和歌山県",
            Prefecture::TOTTORI => "鳥取県",
            Prefecture::SHIMANE => "島根県",
            Prefecture::OKAYAMA => "岡山県",
            Prefecture::HIROSHIMA => "広島県",
            Prefecture::YAMAGUCHI => "山口県",
            Prefecture::TOKUSHIMA => "徳島県",
            Prefecture::KAGAWA => "香川県",
            Prefecture::EHIME => "愛媛県",
            Prefecture::KOCHI => "高知県",
            Prefecture::FUKUOKA => "福岡県",
            Prefecture::SAGA => "佐賀県",
            Prefecture::NAGASAKI => "長崎県",
            Prefecture::KUMAMOTO => "熊本県",
            Prefecture::OITA => "大分県",
            Prefecture::MIYAZAKI => "宮崎県",
            Prefecture::KAGOSHIMA => "鹿児島県",
            Prefecture::OKINAWA => "沖縄県",
        }
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
        match self {
            Prefecture::HOKKAIDO => "hokkaido",
            Prefecture::AOMORI => "aomori",
            Prefecture::IWATE => "iwate",
            Prefecture::MIYAGI => "miyagi",
            Prefecture::AKITA => "akita",
            Prefecture::YAMAGATA => "yamagata",
            Prefecture::FUKUSHIMA => "fukushima",
            Prefecture::IBARAKI => "ibaraki",
            Prefecture::TOCHIGI => "tochigi",
            Prefecture::GUNMA => "gunma",
            Prefecture::SAITAMA => "saitama",
            Prefecture::CHIBA => "chiba",
            Prefecture::TOKYO => "tokyo",
            Prefecture::KANAGAWA => "kanagawa",
            Prefecture::NIIGATA => "niigata",
            Prefecture::TOYAMA => "toyama",
            Prefecture::ISHIKAWA => "ishikawa",
            Prefecture::FUKUI => "fukui",
            Prefecture::YAMANASHI => "yamanashi",
            Prefecture::NAGANO => "nagano",
            Prefecture::GIFU => "gifu",
            Prefecture::SHIZUOKA => "shizuoka",
            Prefecture::AICHI => "aichi",
            Prefecture::MIE => "mie",
            Prefecture::SHIGA => "shiga",
            Prefecture::KYOTO => "kyoto",
            Prefecture::OSAKA => "osaka",
            Prefecture::HYOGO => "hyogo",
            Prefecture::NARA => "nara",
            Prefecture::WAKAYAMA => "wakayama",
            Prefecture::TOTTORI => "tottori",
            Prefecture::SHIMANE => "shimane",
            Prefecture::OKAYAMA => "okayama",
            Prefecture::HIROSHIMA => "hiroshima",
            Prefecture::YAMAGUCHI => "yamaguchi",
            Prefecture::TOKUSHIMA => "tokushima",
            Prefecture::KAGAWA => "kagawa",
            Prefecture::EHIME => "ehime",
            Prefecture::KOCHI => "kochi",
            Prefecture::FUKUOKA => "fukuoka",
            Prefecture::SAGA => "saga",
            Prefecture::NAGASAKI => "nagasaki",
            Prefecture::KUMAMOTO => "kumamoto",
            Prefecture::OITA => "oita",
            Prefecture::MIYAZAKI => "miyazaki",
            Prefecture::KAGOSHIMA => "kagoshima",
            Prefecture::OKINAWA => "okinawa",
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
