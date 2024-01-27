use crate::Prefecture;
use std::slice::Iter;

impl Prefecture {
    /// Returns iterator
    ///
    /// # Example
    /// ```rust
    /// use jisx0401::Prefecture;
    ///
    /// for prefecture in Prefecture::values() {
    ///     let code = prefecture.code();
    ///     let name = prefecture.name_ja();
    ///     println!("{}: {}", code, name);
    /// }
    /// ```
    pub fn values() -> Iter<'static, Prefecture> {
        static PREFECTURES: [Prefecture; 47] = [
            Prefecture::HOKKAIDO,
            Prefecture::AOMORI,
            Prefecture::IWATE,
            Prefecture::MIYAGI,
            Prefecture::AKITA,
            Prefecture::YAMAGATA,
            Prefecture::FUKUSHIMA,
            Prefecture::IBARAKI,
            Prefecture::TOCHIGI,
            Prefecture::GUNMA,
            Prefecture::SAITAMA,
            Prefecture::CHIBA,
            Prefecture::TOKYO,
            Prefecture::KANAGAWA,
            Prefecture::NIIGATA,
            Prefecture::TOYAMA,
            Prefecture::ISHIKAWA,
            Prefecture::HUKUI,
            Prefecture::YAMANASHI,
            Prefecture::NAGANO,
            Prefecture::GIFU,
            Prefecture::SHIZUOKA,
            Prefecture::AICHI,
            Prefecture::MIE,
            Prefecture::SHIGA,
            Prefecture::KYOTO,
            Prefecture::OSAKA,
            Prefecture::HYOGO,
            Prefecture::NARA,
            Prefecture::WAKAYAMA,
            Prefecture::TOTTORI,
            Prefecture::SHIMANE,
            Prefecture::OKAYAMA,
            Prefecture::HIROSHIMA,
            Prefecture::YAMAGUCHI,
            Prefecture::TOKUSHIMA,
            Prefecture::KAGAWA,
            Prefecture::EHIME,
            Prefecture::KOCHI,
            Prefecture::FUKUOKA,
            Prefecture::SAGA,
            Prefecture::NAGASAKI,
            Prefecture::KUMAMOTO,
            Prefecture::OITA,
            Prefecture::MIYAZAKI,
            Prefecture::KAGOSHIMA,
            Prefecture::OKINAWA,
        ];
        PREFECTURES.iter()
    }
}
