#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct DecodeLimits {
    pub max_width: usize,
    pub max_height: usize,
    pub max_bytes: usize,
}

impl DecodeLimits {
    pub const DEFAULT: Self = Self::new(16_384, 16_384, 256 * 1024 * 1024);

    #[must_use]
    pub const fn new(max_width: usize, max_height: usize, max_bytes: usize) -> Self {
        Self {
            max_width,
            max_height,
            max_bytes,
        }
    }

    #[must_use]
    pub const fn allows(&self, width: usize, height: usize) -> bool {
        if width > self.max_width || height > self.max_height {
            return false;
        }
        match width.checked_mul(4) {
            Some(bytes_per_row) => match bytes_per_row.checked_mul(height) {
                Some(bytes) => bytes <= self.max_bytes,
                None => false,
            },
            None => false,
        }
    }

    pub(crate) fn thumbnail_max_pixel_size(self, requested: usize) -> usize {
        let mut side = self.max_width.min(self.max_height);
        if !self.allows(side, side) {
            let (mut low, mut high) = (0, side);
            while low < high {
                let middle = low + (high - low).div_ceil(2);
                if self.allows(middle, middle) {
                    low = middle;
                } else {
                    high = middle - 1;
                }
            }
            side = low;
        }
        if requested == 0 {
            side
        } else {
            requested.min(side)
        }
    }

    pub(crate) fn bridge_values(self) -> (usize, usize, usize) {
        let clamp = |value: usize| value.min(isize::MAX.unsigned_abs());
        (
            clamp(self.max_width),
            clamp(self.max_height),
            clamp(self.max_bytes),
        )
    }
}

impl Default for DecodeLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::DecodeLimits;

    #[test]
    fn default_limits_allow_large_photos_and_refuse_huge_images() {
        let limits = DecodeLimits::default();
        assert_eq!(limits, DecodeLimits::new(16_384, 16_384, 256 * 1024 * 1024));
        assert!(limits.allows(8_064, 6_048));
        assert!(limits.allows(8_192, 8_192));
        assert!(!limits.allows(8_193, 8_193));
        assert!(!limits.allows(16_385, 1));
        assert!(!limits.allows(1, 16_385));
        assert!(!limits.allows(40_000, 40_000));
        assert!(!limits.allows(usize::MAX, 1));
    }

    #[test]
    fn byte_limit_uses_checked_bgra_size() {
        let limits = DecodeLimits::new(usize::MAX, usize::MAX, usize::MAX);
        assert!(limits.allows(1 << 20, 1 << 20));
        assert!(!limits.allows(usize::MAX / 2, 1));
        assert!(!limits.allows(1 << 32, 1 << 32));
        assert!(limits.allows(0, 0));
    }

    #[test]
    fn thumbnail_size_is_capped_by_every_limit() {
        let limits = DecodeLimits::default();
        assert_eq!(limits.thumbnail_max_pixel_size(256), 256);
        assert_eq!(limits.thumbnail_max_pixel_size(0), 8_192);
        assert_eq!(limits.thumbnail_max_pixel_size(usize::MAX), 8_192);
        let narrow = DecodeLimits::new(64, 1_000, usize::MAX);
        assert_eq!(narrow.thumbnail_max_pixel_size(500), 64);
        let tiny = DecodeLimits::new(100, 100, 3);
        assert_eq!(tiny.thumbnail_max_pixel_size(50), 0);
    }

    #[test]
    fn bridge_values_fit_swift_int() {
        let limits = DecodeLimits::new(usize::MAX, 5, usize::MAX);
        let (width, height, bytes) = limits.bridge_values();
        assert_eq!(width, isize::MAX.unsigned_abs());
        assert_eq!(height, 5);
        assert_eq!(bytes, isize::MAX.unsigned_abs());
    }
}
