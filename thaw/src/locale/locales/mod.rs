use pure_rust_locales::Locale;

pub trait LocaleExt {
    fn locale(&self) -> Locale;
    fn today(&self) -> &'static str;
}

pub struct EnUS;
impl LocaleExt for EnUS {
    fn locale(&self) -> Locale {
        Locale::en_US
    }
    fn today(&self) -> &'static str {
        "Today"
    }
}

pub struct EnGB;
impl LocaleExt for EnGB {
    fn locale(&self) -> Locale {
        Locale::en_GB
    }
    fn today(&self) -> &'static str {
        "Today"
    }
}

pub struct EsES;
impl LocaleExt for EsES {
    fn locale(&self) -> Locale {
        Locale::es_ES
    }
    fn today(&self) -> &'static str {
        "Hoy"
    }
}

pub struct PlPL;
impl LocaleExt for PlPL {
    fn locale(&self) -> Locale {
        Locale::pl_PL
    }
    fn today(&self) -> &'static str {
        "Dzisiaj"
    }
}

pub struct FrFR;
impl LocaleExt for FrFR {
    fn locale(&self) -> Locale {
        Locale::fr_FR
    }
    fn today(&self) -> &'static str {
        "Aujourd'hui"
    }
}
