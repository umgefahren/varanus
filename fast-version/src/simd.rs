
cfg_if::cfg_if! {
    if #[cfg(nightly)] {
        use core::simd::SimdElement;
        use crate::version::VersionNumber;
        pub trait PortableSimdElement: SimdElement + PartialEq {}
    } else {
        pub trait PortableSimdElement: PartialEq {}
    }
}

impl PortableSimdElement for u8 {}

impl PortableSimdElement for u16 {}

impl PortableSimdElement for u32 {}

impl PortableSimdElement for u64 {}

impl PortableSimdElement for usize {}

impl PortableSimdElement for i8 {}

impl PortableSimdElement for i16 {}

impl PortableSimdElement for i32 {}

impl PortableSimdElement for i64 {}

impl PortableSimdElement for isize {}

#[cfg(nightly)]
#[inline]
pub fn fast_compare_simd<N: VersionNumber>(major: N, minor: N, patch: N) -> bool {
    let max = N::max();
    let min = N::min();
    let arr = [ max, max, max, max, min, min, min, min ];
    let simd_arr = [ major, minor, patch, min, major, minor, patch, max ];
    
    let limit_simd = core::simd::Simd::from_array(arr);
    let version_simd = core::simd::Simd::from_array(simd_arr);

    let mask_res = limit_simd.lanes_ne(version_simd);
    mask_res.all()
}

#[cfg(test)]
mod compare_tests {
    use rand::{thread_rng, distributions::uniform::SampleUniform, Rng};

    use crate::{version::{VersionNumber, serial_compare}, Version};

    #[cfg(nightly)]
    use super::fast_compare_simd;

    const SAMPLE_SIZE: usize = 1_000;


    fn generate_version_part<N: VersionNumber + SampleUniform>() -> (N, bool) {
        let min = N::min();
        let max = N::max();

        let mut rng = thread_rng();

        let ret_num: N = rng.gen_range(min..=max); 

        (ret_num, ret_num == max || ret_num == min)
    }

    fn validity_test<N: VersionNumber + SampleUniform + core::fmt::Display + core::fmt::Debug>() {
        let (major, major_valid): (N, bool) = generate_version_part();
        let (minor, minor_valid): (N, bool) = generate_version_part();
        let (patch, patch_valid): (N, bool) = generate_version_part();

        let expected_result = (!major_valid) && (!minor_valid) && (!patch_valid); 

        
        #[cfg(nightly)]
        let simd_result = fast_compare_simd(major, minor, patch);


        let serial_result = crate::version::serial_compare(major, minor, patch);

        #[cfg(nightly)]
        assert_eq!(expected_result, simd_result, "SIMD doesn't perform as expected");
        
        assert_eq!(expected_result, serial_result, "Serial doesn't perform as expected; Major: {}, Minor: {}, Patch: {}", major, minor, patch);

        let try_version_result = Version::try_new(major, minor, patch);
        let version_result = Version::new(major, minor, patch);

        if expected_result {
            assert!(version_result.is_ok());
            assert!(try_version_result.is_some());
        } else {
            assert!(version_result.is_err());
            assert!(try_version_result.is_none(), "Try Version Result: {:?}, Major: {}, Minor: {}, Patch: {}", try_version_result, major, minor, patch);
        }

    }

    #[cfg(nightly)]
    fn simd_test<N: VersionNumber>(min: N, max: N) {
        let mut simd_result = fast_compare_simd(min, min, min);

        assert!(!simd_result);

        simd_result = fast_compare_simd(max, max, max);

        assert!(!simd_result);

    }

    fn whole_test<N: VersionNumber + SampleUniform + core::fmt::Display + core::fmt::Debug>() {
        (0..SAMPLE_SIZE).for_each(|_| {
            validity_test::<N>();
        });

        let min = N::min();
        let max = N::max();


        #[cfg(nightly)]
        simd_test(min, max);

        let mut serial_result = serial_compare(min, min, min);

        assert!(!serial_result);

        serial_result = serial_compare(max, max, max);

        assert!(!serial_result);
    }

    #[test]
    fn validity_u8_test() {
        whole_test::<u8>();
    }

    #[test]
    fn validity_u16_test() {
        whole_test::<u16>();
    }

    #[test]
    fn validity_u32_test() {
        whole_test::<u32>();
    }

    #[test]
    fn validity_u64_test() {
        whole_test::<u64>();
    }

    #[test]
    fn validity_usize_test() {
        whole_test::<usize>();
    }

    #[test]
    fn validity_i8_test() {
        whole_test::<i8>();
    }

    #[test]
    fn validity_i16_test() {
        whole_test::<i16>();
    }

    #[test]
    fn validity_i32_test() {
        whole_test::<i32>();
    }

    #[test]
    fn validity_i64_test() {
        whole_test::<i64>();
    }

    #[test]
    fn validity_isize_test() {
        whole_test::<isize>();
    }
}
