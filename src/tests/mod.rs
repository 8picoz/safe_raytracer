mod vec3 {
    use crate::vec3::*;

    #[test]
    fn vec3_dot() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);

        assert_eq!(a.dot(b), 6);
    }

    #[test]
    fn vec3_sqr_magnitude() {
        let a = Vec3::new(2, 2, 2);

        assert_eq!(a.sqr_magnitude(), 12);
    }

    #[test]
    fn vec3_normalized() {
        let a = Vec3::new(1.0, 0.0, 0.0);

        assert_eq!(a.normalized(), Vec3::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn vec3_cross() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);

        assert_eq!(a.cross(b), Vec3::new(-3, 6, -3))
    }

    #[test]
    fn vec3_eq() {
        let a = Vec3::new(0, 0, 0);
        let b = Vec3::new(0, 0, 0);

        assert_eq!(a, b);
    }

    #[test]
    fn vec3_add_1() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);

        assert_eq!(a + b, Vec3::new(3, 3, 3));
    }

    #[test]
    fn vec3_add_2() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);

        assert_eq!(b + a, Vec3::new(3, 3, 3));
    }

    #[test]
    fn vec3_sub_1() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);

        assert_eq!(a - b, Vec3::new(-1, -1, -1));
    }

    #[test]
    fn vec3_sub_2() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);

        assert_eq!(b - a, Vec3::new(1, 1, 1))
    }

    /*
    #[test]
    fn vec3_scalar_times_1() {
        let k = 2;
        let a = Vec3::new(1, 1, 1);

        assert_eq!(k * a, Vec3::new(2, 2, 2));
    }
    */

    #[test]
    fn vec3_scalar_times_2() {
        let k = 2;
        let a = Vec3::new(1, 1, 1);

        assert_eq!(a * k, Vec3::new(2, 2, 2));
    }

    #[test]
    fn hadamard_1() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);

        assert_eq!(a * b, Vec3::new(2, 2, 2));
    }

    #[test]
    fn hadamard_2() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);

        assert_eq!(b * a, Vec3::new(2, 2, 2));
    }
}
