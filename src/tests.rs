mod vec3 {
    use crate::vec3::*;

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
}