use self::lambert::Lambert;

pub mod lambert;

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum BSDF {
    Lambert(Lambert),
}
