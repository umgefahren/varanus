use crate::{version::VersionNumber, Version};

#[cfg(nightly)]
use crate::simd::simd_version_req;

#[inline]
pub fn serial_version_req<N: VersionNumber>(ver: [N; 3], lower: [N; 3], upper: [N; 3]) -> bool {
    for i in 0..3 {
        let v = ver[i];
        let l = lower[i];
        let u = upper[i];
        let res = l <= v && v <= u;
        
        if !res {
            return false
        }
    }

    true
}

#[derive(Debug, Copy, Clone)]
pub enum VersionRegType<N: VersionNumber> {
    Strict(Version<N>),
    GreaterMajor {
        major: N,
    },
    GreaterMinor {
        major: N,
        minor: N
    },
    GreaterPatch {
        major: N,
        minor: N,
        patch: N
    },
    GreaterEqMajor {
        major: N,
    },
    GreaterEqMinor {
        major: N,
        minor: N,
    },
    GreaterEqPatch {
        major: N,
        minor: N,
        patch: N
    },
    LesserMajor {
        major: N
    },
    LesserMinor {
        major: N,
        minor: N
    },
    LesserPatch {
        major: N,
        minor: N,
        patch: N
    },
    LesserEqMajor {
        major: N
    },
    LesserEqMinor {
        major: N,
        minor: N
    },
    LesserEqPatch {
        major: N,
        minor: N,
        patch: N
    }
}

impl<N: VersionNumber> VersionRegType<N> {
    fn is_greater(&self) -> bool {
        match self {
            VersionRegType::GreaterMajor { .. } => true,
            VersionRegType::GreaterMinor { .. } => true,
            VersionRegType::GreaterPatch { .. } => true,
            VersionRegType::GreaterEqMajor { .. } => true,
            VersionRegType::GreaterEqMinor { .. } => true,
            VersionRegType::GreaterEqPatch { .. } => true,
            _ => false,
        }
    }

    fn is_lower(&self) -> bool {
        match self {
            VersionRegType::LesserMajor { .. } => true,
            VersionRegType::LesserMinor { .. } => true,
            VersionRegType::LesserPatch { .. } => true,
            VersionRegType::LesserEqMajor { .. } => true,
            VersionRegType::LesserEqMinor { .. } => true,
            VersionRegType::LesserEqPatch { .. } => true,
            _ => false
        }
    }

    fn is_strict(&self) -> bool {
        match self {
            VersionRegType::Strict(_) => true,
            _ => false,
        }
    }
}

pub enum VersionRegCompType<N: VersionNumber> {
    Pure(VersionRegType<N>),
    Composite {
        lower: VersionRegType<N>,
        higher: VersionRegType<N>
    }
}

enum VersionComperatorLower {
    Strict,
    GreaterMajor,
    GreaterMinor,
    GreaterPatch,
    GreaterEqMajor,
    GreaterEqMinor,
    GreaterEqPatch,
    None
}

impl Default for VersionComperatorLower {
    fn default() -> Self {
        Self::None
    }
}

enum VersionComperatorUpper {
    LesserMajor,
    LesserMinor,
    LesserPatch,
    LesserEqMajor,
    LesserEqMinor,
    LesserEqPatch,
    None
}

impl Default for VersionComperatorUpper {
    fn default() -> Self {
        Self::None
    }
}

pub struct VersionReq<N: VersionNumber> {
    comperator_lower: VersionComperatorLower,
    comperator_higher: VersionComperatorUpper,
    major_lower: N,
    minor_lower: N,
    patch_lower: N,
    major_upper: N,
    minor_upper: N,
    patch_upper: N
}

fn validate_num<N: VersionNumber>(input: N) -> Result<(), VersionRegError> {
    let min = N::min();
    let max = N::max();

    if input == min {
        return Err(VersionRegError::MinValueNotAllowed)
    } else if input == max {
        return Err(VersionRegError::MaxValueNotAllowed)
    }

    Ok(())
}

impl<N: VersionNumber> VersionReq<N> {
    pub fn fits(&self, version: Version<N>) -> bool {
        let ver = [version.major, version.minor, version.patch];
        let lower = [self.major_lower, self.minor_lower, self.patch_lower];
        let upper = [self.major_upper, self.minor_upper, self.patch_upper];

        #[cfg(nightly)]
        let ret = simd_version_req(ver, lower, upper);

        #[cfg(not(nightly))]
        let ret = serial_version_req(ver, lower, upper);
        
        ret
    }

    fn apply_reg_type(&mut self, reg_type: VersionRegType<N>) -> Result<(), VersionRegError> {
        match reg_type {
            VersionRegType::Strict(version) => {
                return self.apply_strict(version);
            },
            VersionRegType::GreaterMajor { major } => {
                return self.apply_greater_major(major);
            },
            VersionRegType::GreaterMinor { major, minor } => {
                return self.apply_greater_minor(major, minor);
            },
            VersionRegType::GreaterPatch { major, minor, patch } => {
                return self.apply_greater_patch(major, minor, patch);
            },
            VersionRegType::GreaterEqMajor { major } => {
                return self.apply_greater_eq_major(major);
            },
            VersionRegType::GreaterEqMinor { major, minor } => {
                return self.apply_greater_eq_minor(major, minor);
            },
            VersionRegType::GreaterEqPatch { major, minor, patch } => {
                return self.apply_greater_eq_patch(major, minor, patch);
            },
            VersionRegType::LesserMajor { major } => {
                return self.apply_lesser_major(major);
            },
            VersionRegType::LesserMinor { major, minor } => {
                return self.apply_lesser_minor(major, minor);
            },
            VersionRegType::LesserPatch { major, minor, patch } => {
                return self.apply_lesser_patch(major, minor, patch);
            },
            VersionRegType::LesserEqMajor { major } => {
                return self.apply_lesser_eq_major(major);
            },
            VersionRegType::LesserEqMinor { major, minor } => {
                return self.apply_lesser_eq_minor(major, minor);
            },
            VersionRegType::LesserEqPatch { major, minor, patch } => {
                return self.apply_lesser_eq_patch(major, minor, patch);
            },
        }

    }

    fn apply_strict(&mut self, reg_type: Version<N>) -> Result<(), VersionRegError> {
        let major = reg_type.major;
        let minor = reg_type.minor;
        let patch = reg_type.patch;

        self.major_lower = major;
        self.minor_lower = minor;
        self.patch_lower = patch;

        self.major_upper = major;
        self.minor_upper = minor;
        self.patch_upper = patch;

        self.comperator_lower = VersionComperatorLower::Strict;

        Ok(())
    }

    fn apply_greater_major(&mut self, major: N) -> Result<(), VersionRegError> {
        validate_num(major)?;

        let one = N::one();

        let min_major = major + one;

        self.major_lower = min_major;

        self.comperator_lower = VersionComperatorLower::GreaterMajor;

        Ok(())
    }

    fn apply_greater_minor(&mut self, major: N, minor: N) -> Result<(), VersionRegError> {
        validate_num(minor)?;
       
        self.apply_greater_major(major)?;
        
        let one = N::one();

        let min_minor = minor + one;

        self.minor_lower = min_minor;

        self.comperator_lower = VersionComperatorLower::GreaterMinor;
        
        Ok(())
    }

    fn apply_greater_patch(&mut self, major: N, minor: N, patch: N) -> Result<(), VersionRegError> {
        validate_num(patch)?;

        self.apply_greater_minor(major, minor)?;


        let one = N::one();

        let min_patch = patch + one;

        self.patch_lower = min_patch;

        self.comperator_lower = VersionComperatorLower::GreaterPatch;

        Ok(())
    }

    fn apply_greater_eq_major(&mut self, major: N) -> Result<(), VersionRegError> {
        validate_num(major)?;

        self.major_lower = major;

        self.comperator_lower = VersionComperatorLower::GreaterEqMajor;

        Ok(())
    }

    fn apply_greater_eq_minor(&mut self, major: N, minor: N) -> Result<(), VersionRegError> {
        validate_num(minor)?;

        self.apply_greater_eq_major(major)?;

        self.minor_lower = minor;

        self.comperator_lower = VersionComperatorLower::GreaterEqMinor;

        Ok(())
    }

    fn apply_greater_eq_patch(&mut self, major: N, minor: N, patch: N) -> Result<(), VersionRegError> {
        validate_num(patch)?;

        self.apply_greater_eq_minor(major, minor)?;

        self.patch_lower = patch;

        self.comperator_lower = VersionComperatorLower::GreaterEqPatch;

        Ok(())
    }

    fn apply_lesser_major(&mut self, major: N) -> Result<(), VersionRegError> {
        validate_num(major)?;

        let one = N::one();

        let max_major = major - one;

        self.major_upper = max_major;

        self.comperator_higher = VersionComperatorUpper::LesserMajor;

        Ok(())
    }

    fn apply_lesser_minor(&mut self, major: N, minor: N) -> Result<(), VersionRegError> {
        validate_num(minor)?;

        self.apply_lesser_major(major)?;

        let one = N::one();

        let max_minor = minor - one;

        self.minor_upper = max_minor;

        self.comperator_higher = VersionComperatorUpper::LesserMinor;

        Ok(())
    }

    fn apply_lesser_patch(&mut self, major: N, minor: N, patch: N) -> Result<(), VersionRegError> {
        validate_num(patch)?;

        self.apply_lesser_minor(major, minor)?;

        let one = N::one();

        let max_patch = patch - one;

        self.patch_upper = max_patch;

        self.comperator_higher = VersionComperatorUpper::LesserPatch;

        Ok(())
    }

    fn apply_lesser_eq_major(&mut self, major: N) -> Result<(), VersionRegError> {
        validate_num(major)?;

        self.major_upper = major;

        self.comperator_higher = VersionComperatorUpper::LesserEqMajor;

        Ok(())
    }

    fn apply_lesser_eq_minor(&mut self, major: N, minor: N) -> Result<(), VersionRegError> {
        validate_num(minor)?;

        self.apply_lesser_eq_major(major)?;

        self.minor_upper = minor;

        self.comperator_higher = VersionComperatorUpper::LesserEqMinor;

        Ok(())
    }

    fn apply_lesser_eq_patch(&mut self, major: N, minor: N, patch: N) -> Result<(), VersionRegError> {
        validate_num(patch)?;

        self.apply_lesser_eq_minor(major, minor)?;

        self.patch_upper = patch;

        self.comperator_higher = VersionComperatorUpper::LesserEqPatch;

        Ok(())
    }
}

impl<N: VersionNumber> Default for VersionReq<N> {
    fn default() -> Self {
       let min = N::min(); 
       let max = N::max();

       Self {
           comperator_lower: Default::default(),
           comperator_higher: Default::default(),
           major_lower: max,
           minor_lower: max,
           patch_lower: max,
           major_upper: min,
           minor_upper: min,
           patch_upper: min
       }
    }
}

pub enum VersionRegError {
    StrictNotAllowedInComposite,
    LowerOnPlaceOfGreater,
    GreaterOnPlaceOfLower,
    MinValueNotAllowed,
    MaxValueNotAllowed,
}

impl<N: VersionNumber> TryFrom<VersionRegCompType<N>> for VersionReq<N> {
    type Error = VersionRegError;

    fn try_from(value: VersionRegCompType<N>) -> Result<Self, Self::Error> {
        let mut ret = Self::default();

        match value {
            VersionRegCompType::Pure(inner) => {
                ret.apply_reg_type(inner)?;
            },
            VersionRegCompType::Composite { lower, higher } => {
                if lower.is_greater() {
                    return Err(VersionRegError::GreaterOnPlaceOfLower);
                }

                if lower.is_strict() || higher.is_strict() {
                    return Err(VersionRegError::StrictNotAllowedInComposite);
                }

                if higher.is_lower() {
                    return Err(VersionRegError::LowerOnPlaceOfGreater);
                }

                ret.apply_reg_type(lower)?;
                ret.apply_reg_type(higher)?;
            }
        }

        Ok(ret)
    }
}
