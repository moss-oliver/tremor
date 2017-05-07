#[cfg(test)]
mod vector3_tests {
    use vector::Vector3f;

    #[test]
    #[allow(unused_variables)]
    fn vector3_create() {
        let new_vec = Vector3f::new(0.0,1.0,2.0);
    }

    #[test]
    fn vector3_get_by_index() {
        let new_vec = Vector3f::new(0.0,1.0,2.0);
        assert_eq!(new_vec[0], 0.0);
        assert_eq!(new_vec[1], 1.0);
        assert_eq!(new_vec[2], 2.0);
    }
    
    #[test]
    fn vector3_get_by_element() {
        let new_vec = Vector3f::new(0.0,10.0,2.0);
        assert_eq!(new_vec.x(), 0.0);
        assert_eq!(new_vec.y(), 10.0);
        assert_eq!(new_vec.z(), 2.0);
    }

    #[test]
    fn vector3_setter() {
        let mut new_vec = Vector3f::new(0.0,0.0,0.0);

        assert_eq!(new_vec[0], 0.0);
        new_vec[0] = 1.0;
        assert_eq!(new_vec[0], 1.0);
        new_vec[0] = 300.0;
        assert_eq!(new_vec[0], 300.0);
        new_vec[0] = 800.0;
        assert_eq!(new_vec[0], 800.0);

        assert_eq!(new_vec[2], 0.0);
        new_vec[2] = 100.0;
        assert_eq!(new_vec[2], 100.0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 40")]
    fn vector3_getter_out_of_bounds() {
        let new_vec = Vector3f::new(0.0,0.0,0.0);

        assert_eq!(new_vec[40], 0.0);
    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 20")]
    fn vector3_setter_out_of_bounds() {
        let mut new_vec = Vector3f::new(0.0,0.0,0.0);
        new_vec[20] = 10.0;
        assert_eq!(new_vec[0], 0.0);
    }
}

#[cfg(test)]
mod vector4_tests {
    use vector::Vector4f;
    use matrix::Matrix4f;

    #[test]
    #[allow(unused_variables)]
    fn vector4_create() {
        let new_vec = Vector4f::new(0.0,1.0,2.0,3.0);
    }

    #[test]
    fn vector4_get_by_index() {
        let new_vec = Vector4f::new(0.0,1.0,2.0,3.0);
        assert_eq!(new_vec[0], 0.0);
        assert_eq!(new_vec[1], 1.0);
        assert_eq!(new_vec[2], 2.0);
        assert_eq!(new_vec[3], 3.0);
    }
    
    #[test]
    fn vector4_get_by_element() {
        let new_vec = Vector4f::new(0.0,10.0,2.0,30.0);
        assert_eq!(new_vec.x(), 0.0);
        assert_eq!(new_vec.y(), 10.0);
        assert_eq!(new_vec.z(), 2.0);
        assert_eq!(new_vec.w(), 30.0);
    }

    #[test]
    fn vector4_setter() {
        let mut new_vec = Vector4f::new(0.0,0.0,0.0,0.0);

        assert_eq!(new_vec[0], 0.0);
        new_vec[0] = 1.0;
        assert_eq!(new_vec[0], 1.0);
    }

    #[test]
    fn vector4_mul_matrix4() {
        let vec = Vector4f::new(1.0,2.0,3.0,4.0);

        let m1 = Matrix4f::new( 01.0,05.0,09.0,13.0,
                                02.0,06.0,10.0,14.0,
                                03.0,07.0,11.0,15.0,
                                04.0,08.0,12.0,16.0);
        
        assert_eq!(vec.transform(m1), Vector4f::new(90.0,100.0,110.0,120.0));
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 4 but the index is 40")]
    fn vector4_getter_out_of_bounds() {
        let new_vec = Vector4f::new(0.0,0.0,0.0,0.0);

        assert_eq!(new_vec[40], 0.0);
    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 4 but the index is 20")]
    fn vector4_setter_out_of_bounds() {
        let mut new_vec = Vector4f::new(0.0,0.0,0.0,0.0);
        new_vec[20] = 10.0;
        assert_eq!(new_vec[0], 0.0);
    }
}


#[cfg(test)]
mod matrix4_tests {
    use matrix::Matrix4f;
    use vector::Position3f;

    #[test]
    fn matrix4_create_identity()
    {
        let m4 = Matrix4f::identity();
        assert_eq!(m4[0][0], 1.0);
        assert_eq!(m4[0][1], 0.0);
        assert_eq!(m4[0][2], 0.0);
        assert_eq!(m4[0][3], 0.0);
        
        assert_eq!(m4[1][0], 0.0);
        assert_eq!(m4[1][1], 1.0);
        assert_eq!(m4[1][2], 0.0);
        assert_eq!(m4[1][3], 0.0);
        
        assert_eq!(m4[2][0], 0.0);
        assert_eq!(m4[2][1], 0.0);
        assert_eq!(m4[2][2], 1.0);
        assert_eq!(m4[2][3], 0.0);
        
        assert_eq!(m4[3][0], 0.0);
        assert_eq!(m4[3][1], 0.0);
        assert_eq!(m4[3][2], 0.0);
        assert_eq!(m4[3][3], 1.0);
    }

    #[test]
    fn matrix4_setter() {
        let mut m4 = Matrix4f::identity();

        m4[2][2] = 390.0;

        assert_eq!(m4[0][0], 1.0);
        assert_eq!(m4[0][1], 0.0);
        assert_eq!(m4[0][2], 0.0);
        assert_eq!(m4[0][3], 0.0);
        
        assert_eq!(m4[1][0], 0.0);
        assert_eq!(m4[1][1], 1.0);
        assert_eq!(m4[1][2], 0.0);
        assert_eq!(m4[1][3], 0.0);
        
        assert_eq!(m4[2][0], 0.0);
        assert_eq!(m4[2][1], 0.0);
        assert_eq!(m4[2][2], 390.0);
        assert_eq!(m4[2][3], 0.0);
        
        assert_eq!(m4[3][0], 0.0);
        assert_eq!(m4[3][1], 0.0);
        assert_eq!(m4[3][2], 0.0);
        assert_eq!(m4[3][3], 1.0);

        m4[2][2] = m4[2][2]+ 1.0;
        assert_eq!(m4[2][2], 391.0);
    }

    #[test]
    fn matrix4_mul() {
        let m1 = Matrix4f::new( 01.0,05.0,09.0,13.0,
                                02.0,06.0,10.0,14.0,
                                03.0,07.0,11.0,15.0,
                                04.0,08.0,12.0,16.0);

        let m2 = Matrix4f::new( 02.0,06.0,10.0,14.0,
                                03.0,07.0,11.0,15.0,
                                04.0,08.0,12.0,16.0,
                                05.0,09.0,13.0,17.0);
                                
        let m_res=Matrix4f::new(100.0,228.0,356.0,484.0,
                                110.0,254.0,398.0,542.0,
                                120.0,280.0,440.0,600.0,
                                130.0,306.0,482.0,658.0);

        assert_eq!(m1 * m2, m_res);
    }

    #[test]
    fn matrix4_mul_identity() {
        let m4_1 = Matrix4f::identity();
        let m4_2 = Matrix4f::identity();

        let m4 = m4_1 * m4_2;
        assert_eq!(m4[0][0], 1.0);
        assert_eq!(m4[0][1], 0.0);
        assert_eq!(m4[0][2], 0.0);
        assert_eq!(m4[0][3], 0.0);
        
        assert_eq!(m4[1][0], 0.0);
        assert_eq!(m4[1][1], 1.0);
        assert_eq!(m4[1][2], 0.0);
        assert_eq!(m4[1][3], 0.0);
        
        assert_eq!(m4[2][0], 0.0);
        assert_eq!(m4[2][1], 0.0);
        assert_eq!(m4[2][2], 1.0);
        assert_eq!(m4[2][3], 0.0);
        
        assert_eq!(m4[3][0], 0.0);
        assert_eq!(m4[3][1], 0.0);
        assert_eq!(m4[3][2], 0.0);
        assert_eq!(m4[3][3], 1.0);
    }

    #[test]
    fn matrix4_translation() {
        let m4_1 = Matrix4f::translation(Position3f::new(1.0,2.0,3.0));
        let m4_2 = Matrix4f::identity();

        assert_eq!(m4_1 * m4_2, Matrix4f::new(
            1.0,0.0,0.0,1.0,
            0.0,1.0,0.0,2.0,
            0.0,0.0,1.0,3.0,
            0.0,0.0,0.0,1.0));
    }

    #[test]
    fn matrix4_perspective() {

        let m4_1 = Matrix4f::perspective_fov(2.65, 0.8, 0.01, 1.0);

        let m4_2 = Matrix4f::new(
            0.31358612,  0.0,        0.0,        0.0,
            0.0,        0.2508689, 0.0,        0.0,
            0.0,        0.0,        -1.010101,  -1.0,
            0.0,        0.0,        -0.01010101, 0.0);
        
        assert_eq!(m4_1, m4_2);

        let m4_3 = Matrix4f::perspective_fov(1.65, 0.8, 0.1, 100.0);

        let m4_4 = Matrix4f::new(
            1.1547189,  0.0,        0.0,        0.0,
            0.0,        0.92377514, 0.0,        0.0,
            0.0,        0.0,        -1.001001,  -1.0,
            0.0,        0.0,        -0.1001001, 0.0);
        
        assert_eq!(m4_3, m4_4);
    }
}