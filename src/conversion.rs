use crate::Prefecture;

macro_rules! prefecture_data {
    ($($variant:ident => { code: $code:expr, name_ja: $name_ja:expr, name_en: $name_en:expr }),* $(,)?) => {
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
                    $(Prefecture::$variant => $code,)*
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
                    $(Prefecture::$variant => $name_ja,)*
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
                    $(Prefecture::$variant => $name_en,)*
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
                    $($code | $name_ja | $name_en => Ok(Prefecture::$variant),)*
                    _ => Err("No matching prefectures were found."),
                }
            }
        }
    };
}

prefecture_data! {
    HOKKAIDO => { code: "01", name_ja: "北海道", name_en: "hokkaido" },
    AOMORI => { code: "02", name_ja: "青森県", name_en: "aomori" },
    IWATE => { code: "03", name_ja: "岩手県", name_en: "iwate" },
    MIYAGI => { code: "04", name_ja: "宮城県", name_en: "miyagi" },
    AKITA => { code: "05", name_ja: "秋田県", name_en: "akita" },
    YAMAGATA => { code: "06", name_ja: "山形県", name_en: "yamagata" },
    FUKUSHIMA => { code: "07", name_ja: "福島県", name_en: "fukushima" },
    IBARAKI => { code: "08", name_ja: "茨城県", name_en: "ibaraki" },
    TOCHIGI => { code: "09", name_ja: "栃木県", name_en: "tochigi" },
    GUNMA => { code: "10", name_ja: "群馬県", name_en: "gunma" },
    SAITAMA => { code: "11", name_ja: "埼玉県", name_en: "saitama" },
    CHIBA => { code: "12", name_ja: "千葉県", name_en: "chiba" },
    TOKYO => { code: "13", name_ja: "東京都", name_en: "tokyo" },
    KANAGAWA => { code: "14", name_ja: "神奈川県", name_en: "kanagawa" },
    NIIGATA => { code: "15", name_ja: "新潟県", name_en: "niigata" },
    TOYAMA => { code: "16", name_ja: "富山県", name_en: "toyama" },
    ISHIKAWA => { code: "17", name_ja: "石川県", name_en: "ishikawa" },
    FUKUI => { code: "18", name_ja: "福井県", name_en: "fukui" },
    YAMANASHI => { code: "19", name_ja: "山梨県", name_en: "yamanashi" },
    NAGANO => { code: "20", name_ja: "長野県", name_en: "nagano" },
    GIFU => { code: "21", name_ja: "岐阜県", name_en: "gifu" },
    SHIZUOKA => { code: "22", name_ja: "静岡県", name_en: "shizuoka" },
    AICHI => { code: "23", name_ja: "愛知県", name_en: "aichi" },
    MIE => { code: "24", name_ja: "三重県", name_en: "mie" },
    SHIGA => { code: "25", name_ja: "滋賀県", name_en: "shiga" },
    KYOTO => { code: "26", name_ja: "京都府", name_en: "kyoto" },
    OSAKA => { code: "27", name_ja: "大阪府", name_en: "osaka" },
    HYOGO => { code: "28", name_ja: "兵庫県", name_en: "hyogo" },
    NARA => { code: "29", name_ja: "奈良県", name_en: "nara" },
    WAKAYAMA => { code: "30", name_ja: "和歌山県", name_en: "wakayama" },
    TOTTORI => { code: "31", name_ja: "鳥取県", name_en: "tottori" },
    SHIMANE => { code: "32", name_ja: "島根県", name_en: "shimane" },
    OKAYAMA => { code: "33", name_ja: "岡山県", name_en: "okayama" },
    HIROSHIMA => { code: "34", name_ja: "広島県", name_en: "hiroshima" },
    YAMAGUCHI => { code: "35", name_ja: "山口県", name_en: "yamaguchi" },
    TOKUSHIMA => { code: "36", name_ja: "徳島県", name_en: "tokushima" },
    KAGAWA => { code: "37", name_ja: "香川県", name_en: "kagawa" },
    EHIME => { code: "38", name_ja: "愛媛県", name_en: "ehime" },
    KOCHI => { code: "39", name_ja: "高知県", name_en: "kochi" },
    FUKUOKA => { code: "40", name_ja: "福岡県", name_en: "fukuoka" },
    SAGA => { code: "41", name_ja: "佐賀県", name_en: "saga" },
    NAGASAKI => { code: "42", name_ja: "長崎県", name_en: "nagasaki" },
    KUMAMOTO => { code: "43", name_ja: "熊本県", name_en: "kumamoto" },
    OITA => { code: "44", name_ja: "大分県", name_en: "oita" },
    MIYAZAKI => { code: "45", name_ja: "宮崎県", name_en: "miyazaki" },
    KAGOSHIMA => { code: "46", name_ja: "鹿児島県", name_en: "kagoshima" },
    OKINAWA => { code: "47", name_ja: "沖縄県", name_en: "okinawa" },
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
